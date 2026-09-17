# helligkeit

Control linux device brightness.

> helligkeit is a port/fork of brightness ctl with the aim of supporting more
> features, modernising the project and porting pull requests and issues from
> the original project in

## Features

| Status | Feature                                                                       |
| ------ | ----------------------------------------------------------------------------- |
| ✅     | List supported Linux LED devices                                              |
| ✅     | Inspect, read, and set LED brightness                                         |
| ✅     | Select a device by exact name or substring                                    |
| ✅     | Support Linux backlight devices                                               |
| ❌     | Support DDC/CI devices                                                        |
| ❌     | Dump and restore state from a TOML file                                       |
| ❌     | `init` command for udev rules, user groups, and man pages                     |
| ❌     | `doctor` command for device, DDC/CI, user, udev and systemd permission checks |

## Installation

```shell
cargo install --path helligkeit
```

## Permissions

Modifying brightness requires write permissions for device files, grant these
to helligkeit by installing `90-helligkeit.rules` rules to add permissions to
backlight, led and i2c devices for users in `video` and leds for users in
`input`:

```sh
sudo install -m 644 90-helligkeit.rules /etc/udev/rules.d/
sudo udevadm control --reload
sudo udevadm trigger
```

For DDC/CI support:

- The `i2c-dev` kernel module must be loaded for the `/dev/i2c-*` nodes to
exist. Load it via a file in `/etc/modules-load.d/`:

  ```sh
  echo i2c-dev | sudo tee /etc/modules-load.d/i2c-dev.conf
  sudo modprobe i2c-dev
  ```

- udev rules need to be reloaded after application

  ```sh
  sudo udevadm control --reload
  sudo udevadm trigger --subsystem-match=i2c-dev
  ```

## Usage

See [man](./man) and below:

```text
Control device linux device brightness

Usage: helligkeit [OPTIONS] [COMMAND]

Commands:
  info  Device info
  get   Get current brightness of device
  set   Set device brightness, either absolute (300), as percentage of the devices maximum (50%) or relative to the current value (+10, -5%)
  list  List all controllable and supported devices
  help  Print this message or the help of the given subcommand(s)

Options:
  -l, --like     Match devices by substring instead of exact name
  -s, --silent   disable stdout/stderr writes
  -v, --verbose
  -h, --help     Print help
  -V, --version  Print version
```

## Project structure

Helligkeit is split into the following crates:

- [helligkeit](./helligkeit): the cli entrypoint
- [ddc](./helligkeit-ddc): i2c based display data channel implementation
- [dev](./helligkeit-dev): enumerating, writing and reading leds and backlights devices
- [i2c](./helligkeit-i2c): userland inter-integrated circuit implementation
- [shared](./helligkeit-shared): shared abstrations, types

## Name

Helligkeit is german for brightness.

## Thanks

Special thanks to hummer12007's
[brightnessctl](https://github.com/Hummer12007/brightnessctl).
