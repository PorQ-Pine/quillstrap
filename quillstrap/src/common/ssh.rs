use crate::prelude::*;

pub fn ssh_execute(command: &str, port: u16, options: &Options) {
    let ip_str = options
        .config
        .deploy_ip_addr
        .map(|b| b.to_string())
        .join(".");
    let to_exec = format!(
        "sshpass -p '{}' ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -p {} root@{} \"{}\"",
        options.config.root_password, port, ip_str, command
    );
    run_shell_command(&to_exec, options.config.command_output).unwrap();
}

pub fn ssh_send(local_path: &str, remote_path: &str, port: u16, options: &Options) {
    let ip_str = options
        .config
        .deploy_ip_addr
        .map(|b| b.to_string())
        .join(".");
    let to_exec = format!(
        "sshpass -p '{}' scp -r -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null -P {} {} root@{}:{}",
        options.config.root_password, port, local_path, ip_str, remote_path
    );
    ssh_execute(&format!("rm -rf {}", remote_path), port, options);
    run_shell_command(&to_exec, options.config.command_output).unwrap();
}
