use crate::{prelude::*, things::get_thing_by_name};

pub fn manual_main(options: Options) {
    debug!("Manual mode selected...");

    things_setup();
    // First, get
    for name in options.clone().args.get {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Get for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(impl_name.path());

        impl_name
            .get(&options)
            .expect(&format!("Failed to get for {}", name));

        dir_change(
            cur_dir
                .to_str()
                .expect("Failed to change PathBuf to string?"),
        );
    }

    // Now we clean
    for name in options.clone().args.clean {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Clean for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        impl_name
            .clean()
            .expect(&format!("Failed to clean for {}", name));

        dir_change(
            cur_dir
                .to_str()
                .expect("Failed to change PathBuf to string?"),
        );
    }

    // Now we build
    for name in options.clone().args.build {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Build for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        impl_name
            .build(&options)
            .expect(&format!("Failed to build for {}", name));

        dir_change(
            cur_dir
                .to_str()
                .expect("Failed to change PathBuf to string?"),
        );
    }
}
