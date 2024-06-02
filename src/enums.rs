#[derive(PartialEq, Debug)]
pub enum Distribution {
    Debian,
    Fedora,
    FedoraAtomic,
    Arch,
}

impl Distribution {
    pub fn get_package_name(&self) -> &str {
        match self {
            Distribution::Debian => "apt",
            Distribution::Fedora => "dnf",
            Distribution::FedoraAtomic => "rpm-ostree",
            Distribution::Arch => "pacman",
        }
    }

    pub fn get_package_install_command(&self) -> &str {
        match self {
            Distribution::Debian => "install",
            Distribution::Fedora => "install",
            Distribution::FedoraAtomic => "install",
            Distribution::Arch => "-S",
        }
    }

    // pub fn should_update_before_upgrade(&self) -> bool {
    //     match self {
    //         Distribution::Debian => true,
    //         Distribution::Fedora => false,
    //         Distribution::FedoraAtomic => false,
    //         Distribution::Arch => false,
    //     }
    // }

    pub fn should_run_as_sudo(&self) -> bool {
        match self {
            Distribution::FedoraAtomic => false,
            Distribution::Debian => true,
            Distribution::Fedora => true,
            Distribution::Arch => true,
        }
    }

    pub fn get_package_upgrade_commands(&self) -> &str {
        match self {
            Distribution::Debian => "upgrade",
            Distribution::Fedora => "upgrade",
            Distribution::FedoraAtomic => "upgrade",
            Distribution::Arch => "-Sy",
        }
    }

    pub fn get_package_remove_command(&self) -> &str {
        match self {
            Distribution::Debian => "",
            Distribution::Fedora => "",
            Distribution::FedoraAtomic => "uninstall",
            Distribution::Arch => "",
        }
    }

    pub fn is_available(&self) -> bool {
        use std::path::Path;

        match self {
            Distribution::Debian => Path::new("/bin/apt"),
            Distribution::Fedora => Path::new("/bin/dnf"),
            Distribution::FedoraAtomic => Path::new("/bin/rpm-ostree"),
            Distribution::Arch => Path::new("/bin/pacman"),
        }
        .exists()
    }

    pub fn get_all_possible_options() -> Vec<Self> {
        vec![
            Distribution::Debian,
            Distribution::Fedora,
            Distribution::FedoraAtomic,
            Distribution::Arch,
        ]
    }
}

#[derive(Clone, Debug)]
pub enum AvailableCommands {
    Install,
    Update,
    Remove,
}

impl AvailableCommands {
    pub fn get_str(&self, distribution: &Distribution) -> String {
        match self {
            AvailableCommands::Install => {
                distribution.get_package_install_command()
            }
            AvailableCommands::Update => {
                distribution.get_package_upgrade_commands()
            }
            AvailableCommands::Remove => {
                distribution.get_package_remove_command()
            }
        }
        .to_string()
    }
}
