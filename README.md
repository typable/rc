# rc
A config file management utility

## Installation

The "rc" tool can be installed directly via the cargo CLI.

```bash
cargo install --git https://github.com/typable/rc
```

The tool can be updated by adding the `--force` flag to the install command.

## Usage

The "rc" command allows you to manage your config files without the hassle of remembering the location of all your files.<br/>
After adding your config files to the "rc" configuration, you can now access them by typing "rc" and the name of the program.

```bash
rc <program>
```

* With the command `rc -l` you can list all config files.
* In order to get the path of a specific config file you can use the command `rc -p <program>`.

## Configuration

Use the command `rc rc` to edit the "rc" configuration file.

The config file contains all programs with the corresponding configuration paths.<br/>
In addition, the default editor for editing the configuration files can be specified.

Here is an example configuration:

```toml
editor = "vim"

[programs]
"rc" = "$HOME/.config/rc/config.toml"
"env" = "$HOME/.envrc"
"hx" = "$HOME/.config/helix/config.toml"
"hx.lang" = "$HOME/.config/helix/languages.toml"
```
