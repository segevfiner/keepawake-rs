# keepawake-rs
[![Crates.io](https://img.shields.io/crates/v/keepawake)](https://crates.io/crates/keepawake)
[![docs.rs](https://img.shields.io/docsrs/keepawake)](https://docs.rs/keepawake)
[![CI](https://github.com/segevfiner/keepawake-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/segevfiner/keepawake-rs/actions/workflows/ci.yml)

Keep your computer awake. Like [`caffeinate`], [`systemd-inhibit`]/[`gnome-session-inhibit`], or [PowerToys Awake], but cross-platform and written in [Rust].

[`caffeinate`]: https://ss64.com/osx/caffeinate.html
[`systemd-inhibit`]: https://www.freedesktop.org/software/systemd/man/systemd-inhibit.html
[`gnome-session-inhibit`]: https://manpages.ubuntu.com/manpages/jammy/man1/gnome-session-inhibit.1.html
[PowerToys Awake]: https://learn.microsoft.com/en-us/windows/powertoys/awake
[Rust]: https://www.rust-lang.org/

## Usage
```
Keep your computer awake

Usage: keepawake [OPTIONS] [COMMAND]...

Arguments:
  [COMMAND]...  Run the command and wait for it to exit, keeping the computer awake while it runs

Options:
  -d, --display              Keep display on
  -i, --idle                 Keep system from idle sleeping
  -s, --sleep                Keep system from sleeping (Functionality and conditions for this to work vary by OS)
      --completions <SHELL>  Generate shell completions [possible values: bash, elvish, fish, powershell, zsh]
  -w <PID>                   Wait for the process with the specified pid to exit. This option is ignored when used with the COMMAND argument
  -h, --help                 Print help information
  -V, --version              Print version information
```

See [docs.rs/keepawake](https://docs.rs/keepawake) for library crate documentation and usage.

## Installation

### Cargo
```sh
cargo install keepawake -F bin
```

### Binaries
Download from https://github.com/segevfiner/keepawake-rs/releases/latest.

## Completions
Use: `keepawake --completions <SHELL>` to generate a completion script, you will have to install it
as appropriate for the specific shell you are using.

## Notes
Preventing the computer from explicitly sleeping, and/or by closing the lid, is often restricted in various ways by the OS, e.g. Only on AC power, not in any PC running Windows with [Modern Standby](https://learn.microsoft.com/en-us/windows-hardware/design/device-experiences/modern-standby).

## License
MIT License.
