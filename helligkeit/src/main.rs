use std::{io, process};

use clap::Parser;
use helligkeit_shared::{Device, IRRELEVANT};

mod cli;

fn die(msg: impl std::fmt::Display) -> ! {
    eprintln!("error: {msg}");
    process::exit(1)
}

struct Helligkeit<'h> {
    args: &'h cli::Cli,
    target: String,
    /// all devices sorted by (class, rank, name), the first one is the
    /// default used when no device argument is given
    devices: Vec<Box<dyn Device>>,
}

impl<'h> Helligkeit<'h> {
    pub fn new(args: &'h cli::Cli) -> Self {
        let verbose = args.verbose;
        let report = |err: io::Error| {
            if verbose {
                eprintln!("failed to enumerate device: {err}");
            }
        };

        let backlights = helligkeit_dev::backlight()
            .expect("Failed to enumerate backlights")
            .map(|r| r.map(|x| Box::new(x) as Box<dyn Device>));
        let leds = helligkeit_dev::leds()
            .expect("Failed to enumerate leds")
            .map(|r| r.map(|x| Box::new(x) as Box<dyn Device>));
        // let ddc = helligkeit_ddc::ddc()
        //     .expect("Failed to enumerate ddc devices")
        //     .map(|r| r.map(|x| Box::new(x) as Box<dyn Device>));

        let mut devices: Vec<Box<dyn Device>> = backlights
            .chain(leds)
            // .chain(ddc)
            .filter_map(|r| r.map_err(report).ok())
            .collect();
        devices.sort_by_cached_key(|d| helligkeit_shared::sort_key(d.as_ref()));

        if verbose {
            for d in &devices {
                let rank = d.rank();
                eprintln!(
                    "{}: {} rank {}{}",
                    d.name(),
                    d.class(),
                    rank,
                    if rank >= IRRELEVANT {
                        ", irrelevant"
                    } else {
                        ""
                    }
                );
            }
        }

        Self {
            args,
            target: args
                .command
                .as_ref()
                .and_then(cli::Action::device)
                .unwrap_or_default()
                .to_owned(),
            devices,
        }
    }

    fn find_device(&self) -> Vec<&Box<dyn Device>> {
        let cli::Cli { like, .. } = &self.args;

        if *like {
            self.devices
                .iter()
                .filter(|d| d.name().contains(&self.target))
                .collect()
        } else if self.target.is_empty() {
            self.devices.first().map(|d| vec![d]).unwrap_or_else(|| {
                die("no devices found, nothing to fall back to without a device argument")
            })
        } else {
            self.devices
                .iter()
                .find(|d| d.name() == self.target)
                .map(|d| vec![d])
                .unwrap_or_default()
        }
    }

    pub fn info(&self) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.target));
        }

        found.iter().for_each(|d| println!("{d}"))
    }

    pub fn get(&self) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.target));
        }

        for dev in found {
            let brightness = dev.get().unwrap_or_else(|err| {
                die(format!(
                    "failed to query brightness of '{}': {}",
                    dev.name(),
                    err,
                ))
            });

            println!("{}", brightness)
        }
    }

    fn list(&self) {
        let relevant = |d: &&Box<dyn Device>| self.args.verbose || d.rank() < IRRELEVANT;

        self.devices
            .iter()
            .filter(relevant)
            .for_each(|d| println!("{d}"));

        let hidden = self.devices.iter().filter(|d| !relevant(d)).count();
        if hidden > 0 {
            eprintln!("{hidden} possibly irrelevant devices hidden, use --verbose/-v to see all");
        }
    }

    fn set(&self, adjust: cli::Adjust) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.target));
        }

        for dev in found {
            let fail = |err: io::Error| -> ! {
                die(format!(
                    "failed to set brightness of '{}' (max={}) to '{}': {}",
                    dev.name(),
                    dev.max().map(|m| m.to_string()).unwrap_or_default(),
                    adjust,
                    err,
                ))
            };

            let max = dev.max().unwrap_or_else(|err| fail(err));
            let current = if adjust.is_relative() {
                dev.get().unwrap_or_else(|err| fail(err))
            } else {
                0
            };

            if let Err(err) = dev.set(adjust.resolve(current, max)) {
                fail(err)
            }
        }
    }

    pub fn run(self) {
        let Some(cmd) = &self.args.command else {
            self.get();
            return;
        };

        match cmd {
            cli::Action::List => self.list(),
            cli::Action::Info { .. } => self.info(),
            cli::Action::Get { .. } => self.get(),
            cli::Action::Set { value, .. } => self.set(*value),
        }
    }
}

fn main() {
    let args = cli::Cli::parse();
    Helligkeit::new(&args).run();
}
