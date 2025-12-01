use crate::{prelude::*, things::get_thing_by_name};

pub fn auto_build(impl_name: &TraitWrapper, options: &Options, already_built: &mut Vec<String>) {
    info!("Build for {:?}", impl_name);

    // Check deps
    for deps in impl_name.deps() {
        info!("Checking dep: {} required by {:?}", deps, impl_name.name());
        let dep_impl: TraitWrapper = get_thing_by_name(&deps, &options.things);
        dir_change(&options.path_of_repo);
        dir_change(MAIN_BUILD_DIR);
        mkdir_p(dep_impl.path());
        dir_change(&format!("{}{}", dep_impl.path(), dep_impl.name()));
        if !options.args.ignore_built_checks {
            if !dep_impl.is_built() {
                info!(
                    "{} is not built, running auto build for it...",
                    dep_impl.name()
                );
                auto_build(&dep_impl, options, already_built);
            } else {
                info!("{} is already built", dep_impl.name());
            }
        } else {
            info!("Just built it is on, so building: {}", dep_impl.name());
            if !already_built.contains(&dep_impl.name().to_string()) {
                auto_build(&dep_impl, options, already_built);
                already_built.push(dep_impl.name().to_string());
            } else {
                info!("{} was already built in this session, ignoring it as a dependency, even tho just_built option is enabled", dep_impl.name());
            }
        }
    }

    // Build it
    info!("All deps checked for {:?}", impl_name);
    let cur_dir = dir_current();
    dir_change(&options.path_of_repo);
    dir_change(MAIN_BUILD_DIR);
    mkdir_p(impl_name.path());
    dir_change(&format!("{}{}", impl_name.path(), impl_name.name()));

    impl_name
        .build(&options)
        .expect(&format!("Failed to build for {:?}", impl_name));

    if impl_name.is_built() {
        info!("After built check passed");
    } else {
        panic!("Additional is built check after build failed");
    }

    dir_change(&cur_dir);
}

pub fn auto_main(options: Options) {
    debug!("Auto mode selected...");
    things_setup();

    let mut already_built: Vec<String> = vec![];

    // Now we build
    for name in options.clone().args.build {
        let impl_name: TraitWrapper = get_thing_by_name(&name, &options.things);
        auto_build(&impl_name, &options, &mut already_built);
    }
}
