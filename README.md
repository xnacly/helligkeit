# helligkeit

Control linux device brightness.

> helligkeit is a port/fork of brightness ctl with the aim of supporting more
> features, modernising the project and porting pull requests and issues from
> the original project in

## Features

- Dump and restore device brightness to and from a toml file
- Support devices conforming to ddci (with this, possible macos support)
- Init subcommand to set up udev rules, user grouping and man pages
- Doctor subcommand for checking:
    - brightness device permissions
    - ddci permissions
    - user permissions
    - udev rules

## Installation

```shell
cargo install --path helligkeit
```

## Permissions

Modifying brightness requires write permissions for device files.
`helligkeit` accomplishes this by:

1. installing relevant udev rules to add permissions to backlight class devices
for users in `video` and leds for users in `input`.

2. This requires your user to be in the `video` and `input` groups. (done by default)

## Usage

## Name

Helligkeit is german for brightness.

## Thanks

Special thanks to hummer12007's
[brightnessctl](https://github.com/Hummer12007/brightnessctl).
