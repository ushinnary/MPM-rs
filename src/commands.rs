use std::process::{Command, Stdio};

use crate::{
    enums::{AvailableCommands, Distribution},
    MainCommands,
};

#[derive(Debug)]
pub struct PackageManager {
    available_managers: Vec<Distribution>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self { available_managers: Self::get_available_managers() }
    }

    /// List every usable package manager to be used
    fn get_available_managers() -> Vec<Distribution> {
        Distribution::get_all_possible_options()
            .into_iter()
            .filter(|distro| distro.is_available())
            .collect::<Vec<Distribution>>()
    }

    /// Determine command to run from args
    fn get_package_command(&self, cmd: &MainCommands) -> AvailableCommands {
        if cmd.update || cmd.upgrade {
            AvailableCommands::Update
        } else if cmd.remove {
            AvailableCommands::Remove
        } else {
            AvailableCommands::Install
        }
    }

    /// Running the command
    pub fn run_command(&self, cmd: &MainCommands, args: Option<Vec<String>>) {
        let command = self.get_package_command(cmd);

        self.available_managers.iter().for_each(|manager| {
            let args_copy = args.clone();
            let (main_command, all_other_commands) =
                get_main_and_additional_commands(
                    manager,
                    args_copy,
                    command.clone(),
                );

            let mut child = Command::new(main_command)
                .args(all_other_commands)
                .stdin(Stdio::inherit())
                .spawn()
                .expect("Failed to run command");

            child.wait().expect("Failed to wait for process");
        });
    }
}

/// Construction of all possible args for command construction
fn get_main_and_additional_commands(
    manager: &Distribution,
    args: Option<Vec<String>>,
    command: AvailableCommands,
) -> (&str, Vec<String>) {
    let mut all_other_commands: Vec<String> = Vec::new();
    let package_name = manager.get_package_name();

    let main_command = if manager.should_run_as_sudo() {
        all_other_commands.push(package_name.to_string());
        "sudo"
    } else {
        package_name
    };

    all_other_commands.push(command.get_str(manager));

    if let Some(args) = args.as_ref() {
        args.iter().for_each(|arg| all_other_commands.push(arg.clone()));
    }

    (main_command, all_other_commands)
}
