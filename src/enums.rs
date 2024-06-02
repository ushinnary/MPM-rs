#[derive(PartialEq, Debug)]
pub enum Distribution {
    /// https://doc.ubuntu-fr.org/apt-cli
    DebianLike,
    Fedora,
    AtomicRpmOstree,
    /// https://wiki.archlinux.org/title/Pacman
    Arch,
}

impl Distribution {
    pub fn get_package_name(&self) -> &str {
        match self {
            Distribution::DebianLike => "apt",
            Distribution::Fedora => "dnf",
            Distribution::AtomicRpmOstree => "rpm-ostree",
            Distribution::Arch => "pacman",
        }
    }

    pub fn get_package_install_command(&self) -> &str {
        match self {
            Distribution::DebianLike => "install",
            Distribution::Fedora => "install",
            Distribution::AtomicRpmOstree => "install",
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
            Distribution::AtomicRpmOstree => false,
            Distribution::DebianLike => true,
            Distribution::Fedora => true,
            Distribution::Arch => true,
        }
    }

    pub fn get_package_upgrade_commands(&self) -> &str {
        match self {
            Distribution::DebianLike => "upgrade",
            Distribution::Fedora => "upgrade",
            Distribution::AtomicRpmOstree => "upgrade",
            Distribution::Arch => "-Syu",
        }
    }

    pub fn get_package_remove_command(&self) -> &str {
        match self {
            Distribution::DebianLike => "remove",
            Distribution::Fedora => "remove",
            Distribution::AtomicRpmOstree => "uninstall",
            Distribution::Arch => "-R", // -Rs for remove with deps
        }
    }

    pub fn is_available(&self) -> bool {
        use std::path::Path;

        match self {
            Distribution::DebianLike => Path::new("/bin/apt"),
            Distribution::Fedora => Path::new("/bin/dnf"),
            Distribution::AtomicRpmOstree => Path::new("/bin/rpm-ostree"),
            Distribution::Arch => Path::new("/bin/pacman"),
        }
        .exists()
    }

    pub fn get_all_possible_options() -> Vec<Self> {
        vec![
            Distribution::DebianLike,
            Distribution::Fedora,
            Distribution::AtomicRpmOstree,
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
