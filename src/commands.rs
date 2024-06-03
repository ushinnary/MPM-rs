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
            let mut child = if manager.should_run_as_sudo() {
                Command::new("sudo")
                    .arg(manager.get_package_name())
                    .arg(command.get_str(manager))
                    .args(args.as_ref().unwrap_or(&Vec::new()))
                    .stdin(Stdio::inherit())
                    .spawn()
            } else {
                Command::new(manager.get_package_name())
                    .arg(command.get_str(manager))
                    .args(args.as_ref().unwrap_or(&Vec::new()))
                    .stdin(Stdio::inherit())
                    .spawn()
            }
            .expect("Failed to run command");

            child.wait().expect("Failed to wait for process");
        });
    }
}
