# MPM-rs

Multi-Package-Manager for linux distributions for mutual calls of system commands
like install / update

## What is it for ?

I'm using an Atomic version of Fedora distribution with fedora workstation as distrobox
instance for dev and some apps that i wont use as flatpaks.

With this package, i can use the same minimal command to update my systems.
Here are my steps :

- Open terminal
- Run `mpm -u` (And not `rpm-ostree update`)
- Run `distrobox-enter ...`
- Run `mpm -u` again (And not `sudo dnf update`)

## Usage

To update your system, you just type :
`mpm -u`

To install a package or some of them :
`mpm -i package_name other_package`

List of currently available parameters :

| Long                      | Short             | Description         |
| ------------------------- | ----------------- | ------------------- |
| `--install package_name`  | `-i package_name` | Install a package   |
| `--update` or `--upgrade` | `-u`              | Updates your system |
| `--remove package_name`   | `-r package_name` | Uninstall package   |
