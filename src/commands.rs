use std::process::{Command, Stdio};

use crate::enums::{AvailableCommands, Distribution};

#[derive(Debug)]
pub struct PackageManager {
    available_managers: Vec<Distribution>,
}

impl PackageManager {
    pub fn new() -> Self {
        Self { available_managers: Self::get_available_managers() }
    }

    fn get_available_managers() -> Vec<Distribution> {
        Distribution::get_all_possible_options()
            .into_iter()
            .filter(|distro| distro.is_available())
            .collect::<Vec<Distribution>>()
    }

    /// Running the command
    pub fn run_command(
        &self,
        command: AvailableCommands,
        args: Option<Vec<String>>,
    ) {
        self.available_managers.iter().for_each(|manager| {
            let mut child = if manager.should_run_as_sudo() {
                Command::new("sudo")
                    .arg(manager.get_package_name())
                    .arg(command.get_str(manager))
                    .args(args.as_ref().unwrap_or(&Vec::new()))
                    .stdin(Stdio::inherit())
                    .spawn()
                    .expect("Well, it didn't work...")
            } else {
                Command::new(manager.get_package_name())
                    .arg(command.get_str(manager))
                    .args(args.as_ref().unwrap_or(&Vec::new()))
                    .stdin(Stdio::inherit())
                    .spawn()
                    .expect("Well, it didn't work...")
            };

            child.wait().expect("Failed to wait for process");
        });
    }
}
