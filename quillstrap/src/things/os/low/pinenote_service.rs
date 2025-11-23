use crate::prelude::*;

#[derive(Clone, Copy, Default)]
pub struct PinenoteService;

impl SetupThing for PinenoteService {
    fn name(&self) -> &'static str {
        "pinenote_service"
    }

    fn path(&self) -> &'static str {
        "os/low/"
    }

    fn deps(&self) -> Vec<&'static str> {
        vec![]
    }

    fn git(&self) -> &'static str {
        "pinenote-service"
    }

    fn get(&self, _options: &Options) -> Result<(), String> {
        git_get_manage(self, _options);
        Ok(())
    }

    fn is_built(&self) -> bool {
        path_exists("target/aarch64-unknown-linux-gnu/release/pinenote-service")
    }

    fn clean(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }

    fn build(&self, _options: &Options) -> Result<(), String> {
        let full_path = get_path_of_thing_native(self, _options);
        set_var("PKG_CONFIG_ALLOW_CROSS", "1");
        set_var(
            "PKG_CONFIG_PATH",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );
        set_var(
            "PKG_CONFIG_SYSROOT_DIR",
            &format!("{}../../low/rootfs_sysroot/sysroot", full_path),
        );
        set_var(
            "RUSTFLAGS",
            &format!("-L {}../../low/rootfs_sysroot/sysroot/usr/lib64", full_path),
        );
        set_var(
            "PKG_CONFIG_LIBDIR",
            &format!(
                "{}../../low/rootfs_sysroot/sysroot/usr/lib64/pkgconfig/:{}../../low/rootfs_sysroot/sysroot/usr/share/pkgconfig",
                full_path, full_path
            ),
        );

        run_command(
            "cargo zigbuild --no-default-features --release --target aarch64-unknown-linux-gnu.2.41",
            _options.config.command_output,
        )
        .unwrap();

        set_var("PKG_CONFIG_ALLOW_CROSS", "");
        set_var("PKG_CONFIG_SYSROOT_DIR", "");
        set_var("RUSTFLAGS", "");

        Ok(())
    }

    fn deploy(&self, _options: &Options) -> Result<(), String> {
        let port = _options.config.rootfs_options.deploy_ssh_port;
        ssh_execute("systemctl stop pinenote", port, _options);
        ssh_execute("killall -9 pinenote-service", port, _options);
        ssh_execute("rm -rf /usr/bin/pinenote-service", port, _options);
        ssh_send("target/aarch64-unknown-linux-gnu/release/pinenote-service", "/usr/bin/pinenote-service", port, _options);
        Ok(())
    }

    fn run(&self, _options: &Options) -> Result<(), String> {
        todo!()
    }
}
