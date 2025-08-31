use crate::prelude::*;

pub const QINIT_SRC_DIR: &str = "qinit/";
pub const QINIT_BINARY: &str = "qinit";
pub const QINIT_GUI_ONLY_SUFFIX: &str = "_gui_only";

#[derive(Clone, Copy, Default)]
pub struct QuillInit;

impl SetupThing for QuillInit {
    fn name(&self) -> &'static str {
        "quill_init"
    }

    fn path(&self) -> &'static str {
        "init/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec!["sysroot", "branding"]
    }

    fn git(&self) -> &'static str {
        "quill-init"
    }

    fn get(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        git_get_manage(self, &_options);
        Ok(())
    }

    fn clean(&self) -> color_eyre::eyre::Result<(), String> {
        run_command("cargo clean", false).expect("Failed to clean quill-init");
        Ok(())
    }

    fn build(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        let cur_dir = dir_current();
        mkdir_p("out/");

        dir_change(&QINIT_SRC_DIR);

        let mut features_normal: Vec<&str> = vec![];
        let features_wrapper: Vec<&str> = vec!["init_wrapper"];

        if _options.config.unrestricted {
            features_normal.push("free_roam");
        }
        if _options.config.unsecure_debug {
            features_normal.push("debug");
        }

        // RUST_FLAGS=\"-C target-feature=-crt-static\" is applied in config.toml

        if _options.args.quill_init_options.qi_ssh_build {
            warn!("Building Quill-init ssh debug build");
            /* GUI deployment is enabled, hence we only need to build qinit once with gui_only flag and not the rest */
            // When debugging with gui_only feature, you normally want to have all the other features (except init_wrapper) enabled to make debugging easier, hence why the features set from config are bypassed here

            if !_options.config.unrestricted {
                warn!("This build is not unrestricted but qi_ssh_build needs it!");
            }
            if !_options.config.unsecure_debug {
                warn!("This build is not unsecure debug but qi_ssh_build needs it!");
            }

            features_normal.push("gui_only");
            Sysroot::execute_sysroot_command_dir(
                &format!("cargo build --features={}", features_normal.join(",")),
                &cur_dir,
                _options,
            );
            copy_file(
                &format!("../target/debug/{}", &QINIT_BINARY),
                &format!("../out/{}{}", &QINIT_BINARY, &QINIT_GUI_ONLY_SUFFIX),
            )
            .unwrap();
        } else {
            Sysroot::execute_sysroot_command_dir(
                &format!(
                    "cargo build --release --features={}",
                    features_normal.join(",")
                ),
                &cur_dir,
                _options,
            );
            copy_file(
                &format!("../target/release/{}", &QINIT_BINARY),
                &format!("../out/{}", &QINIT_BINARY),
            )
            .unwrap();
            Sysroot::execute_sysroot_command_dir(
                &format!(
                    "cargo build --release --features={}",
                    features_wrapper.join(",")
                ),
                &cur_dir,
                _options,
            );
            copy_file(
                &format!("../target/release/{}", &QINIT_BINARY),
                "../out/init",
            )
            .unwrap();
        }
        Ok(())
    }

    fn deploy(&self, _options: &crate::Options) -> color_eyre::eyre::Result<(), String> {
        if !_options.args.quill_init_options.qi_ssh_build {
            error!("This is not a ssh build, yet we are deplying to ssh. You have been warned!");
        }
        let ip_str = _options
            .config
            .qinit_options
            .deploy_ssh_ip_addr
            .map(|b| std::str::from_utf8(&[b]).unwrap().to_string())
            .join(".");

        run_command(
            &format!(
                "ssh -p {} root@{} killall {}",
                &_options.config.qinit_options.deploy_ssh_port, &ip_str, &QINIT_BINARY
            ),
            false,
        )
        .unwrap();
        run_command(
            &format!(
                "scp -P {} ../out/{}{} root@{}:/tmp",
                &_options.config.qinit_options.deploy_ssh_port,
                &QINIT_BINARY,
                &QINIT_GUI_ONLY_SUFFIX,
                &ip_str
            ),
            true,
        )
        .unwrap();
        run_shell_command(
            &format!(
                "ssh -t -p {} root@{} 'RUST_LOG=info SLINT_KMS_ROTATION=270 /tmp/{}{}'",
                &_options.config.qinit_options.deploy_ssh_port,
                &ip_str,
                &QINIT_BINARY,
                &QINIT_GUI_ONLY_SUFFIX
            ),
            true,
        )
        .unwrap();

        Ok(())
    }

    fn run(&self) -> color_eyre::eyre::Result<(), String> {
        todo!()
    }
}
