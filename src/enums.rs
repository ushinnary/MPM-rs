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

    pub fn package_install_command(&self) -> &str {
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

    pub fn package_upgrade_commands(&self) -> &str {
        match self {
            Distribution::Debian => "upgrade",
            Distribution::Fedora => "upgrade",
            Distribution::FedoraAtomic => "upgrade",
            Distribution::Arch => "-Sy",
        }
    }

    pub fn is_available(&self) -> bool {
        use std::path::Path;

        match self {
            Distribution::Debian => Path::new("/bin/apt").exists(),
            Distribution::Fedora => Path::new("/bin/dnf").exists(),
            Distribution::FedoraAtomic => Path::new("/bin/rpm-ostree").exists(),
            Distribution::Arch => false,
        }
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
                distribution.package_install_command().to_string()
            }
            AvailableCommands::Update => {
                distribution.package_upgrade_commands().to_string()
            }
            AvailableCommands::Remove => "remove".to_string(),
        }
    }
}
