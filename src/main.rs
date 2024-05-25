use clap::Parser;

use commands::PackageManager;

use crate::enums::AvailableCommands;

mod commands;
mod enums;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Main commands
    #[clap(flatten)]
    main_command: MainCommands,
}

#[derive(Parser, Debug)]
#[group(required = true, multiple = false)]
struct MainCommands {
    /// Run install command
    #[arg(short, long)]
    install: bool,

    /// Run upgrade command
    #[arg(short, long)]
    update: bool,

    /// Run upgrade command
    #[arg(long)]
    upgrade: bool,

    /// Run remove command
    #[arg(short, long)]
    remove: bool,
}

fn main() {
    let args = Args::parse();

    let package_managers = PackageManager::new();

    if args.main_command.install {
        // TODO: implement package to install with other additional arguments
        package_managers.run_command(AvailableCommands::Install, None);
    } else if args.main_command.update || args.main_command.upgrade {
        package_managers.run_command(AvailableCommands::Update, None);
    } else if args.main_command.remove {
        // TODO: implement package to remove
        package_managers.run_command(AvailableCommands::Remove, None);
    }
}
