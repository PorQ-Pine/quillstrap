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

        dir_change(&cur_dir);
    }

    // Now we check if is built
    for name in options.clone().args.is_built {
        let impl_name = get_thing_by_name(&name, &options.things);

        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        if impl_name.is_built() {
            info!("{} is built", name);
        } else {
            error!("{} is not built", name);
        }
        dir_change(&cur_dir);
    }

    // Now we clean
    for name in options.clone().args.clean {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Clean for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        impl_name
            .clean(&options)
            .expect(&format!("Failed to clean for {}", name));

        dir_change(&cur_dir);
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

        if impl_name.is_built() {
            info!("After built check passed");
        } else {
            panic!("Additional is built check after build failed");
        }

        dir_change(&cur_dir);
    }

    // Now we deploy
    for name in options.clone().args.deploy {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Deploy for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        impl_name
            .deploy(&options)
            .expect(&format!("Failed to deploy for {}", name));

        dir_change(&cur_dir);
    }

    // Now we run
    if let Some(name) = options.clone().args.run {
        let impl_name = get_thing_by_name(&name, &options.things);

        info!("Run for {}", name);
        let cur_dir = dir_current();
        mkdir_p(impl_name.path());
        dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

        impl_name
            .run(&options)
            .expect(&format!("Failed to run {}", name));

        dir_change(&cur_dir);
    }
}
