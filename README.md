<h1 align=center>Recall</h1>

<p align=center>Recall keybinds, shortcuts, commands and more</p>

<p align=center>
  <img alt="recall_screenshot" src="https://github.com/user-attachments/assets/3fd3dfd9-82be-48cd-8c68-50f514a5f427" width="500">
</p>

Recall is a simple utility tool to display predefined entries, like keybinds, shortcuts, commands and other one-liners you want to remember. You can group these entries on pages and quickly navigate through them.

## Usage

```raw
Usage: recall [OPTIONS] [COMMAND]

Commands:
  init  Initialize example config
  help  Print this message or the help of the given subcommand(s)

Options:
  -c, --config <FILE>  Specify a different configuration file
  -h, --help           Print help
  -V, --version        Print version
```

## Installation

Install with cargo:

```sh
cargo install --git https://github.com/HerrPixel/recall
```

## Configuration

Configuration is done via a configuration placed in `{CONFIG_DIR}/recall/config.toml` where `{CONFIG_DIR}` is your platform specific configuration directory, for example `$XDG_CONFIG_HOME` or `$HOME/.config` on Linux. See also the path mentioned when running `recall init`.

You may also specifiy a different location for the configuration file by using the `--config` flag.

A configuration file has the following format:

```toml
# General settings for recall reside in this table
[recall]        
# Colors are u8-encoded numbers as specified by the ANSI Color Table
primary_color = 2
highlight_color = 105

# Each subtable under keys specifies a new page
# The name of a page is the name of the subtable
[keys.general]
# The keys-value takes an array of strings used as the keys to press for a shortcut
# The description-value takes a string to be displayed as the description for the corresponding entry
RecallClose = { keys = ["q"], description = "Closes recall" }
TTYSwitch = { keys = ["Ctrl","Alt","F2"], description = "Switches to TTY 2, replace Fn number with desired TTY" }

[keys.bash]
# Key names don't matter, these are just for uniquely identifying an entry in a table
Copy = { keys = ["Ctrl","Shift","C"], description = "Copy selected text" }
Paste = { keys = ["Ctrl","Shift","V"], description = "Paste selected text" }

# Empty tables are also allowed (but useless)
[keys.empty_page]
```

## Changelog

### [1.0.0] 5-2-2025

Initial Release
