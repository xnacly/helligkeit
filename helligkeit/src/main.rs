use std::{collections::HashMap, process};

use clap::Parser;
use helligkeit_shared::Device;

mod cli;

fn die(msg: impl std::fmt::Display) -> ! {
    eprintln!("error: {msg}");
    process::exit(1)
}

struct Helligkeit<'h> {
    args: &'h cli::Cli,
    devices: HashMap<String, Box<dyn Device>>,
}

impl<'h> Helligkeit<'h> {
    pub fn new(args: &'h cli::Cli) -> Self {
        let leds = helligkeit_dev::leds().expect("Failed to enumerate leds");
        // let backlight = helligkeit_dev::backlight().expect("Failed to enumerate backlights");
        // let ddc = helligkeit_ddc::ddc().expect("Failed to enumerate ddc devices");
        let verbose = args.verbose;

        Self {
            args,
            devices: leds
                .map(|x| x.map(|x| Box::new(x) as Box<dyn Device>))
                // .chain(backlight.map(|x| x.map(|x| Box::new(x) as Box<dyn Device>)))
                // .chain(ddc.map(|x| x.map(|x| Box::new(x) as Box<dyn Device>)))
                .filter_map(|result| match result {
                    Ok(device) => Some(device),
                    Err(err) => {
                        if verbose {
                            eprintln!("failed to enumerate device: {err}");
                        }
                        None
                    }
                })
                .map(|device| (device.name().to_owned(), device))
                .collect(),
        }
    }

    fn find_device(&self) -> Vec<&Box<dyn Device>> {
        let cli::Cli {
            like,
            target: device,
            ..
        } = &self.args;

        if *like {
            self.devices
                .iter()
                .filter(|(name, _)| name.contains(device))
                .map(|(_, d)| d)
                .collect()
        } else {
            self.devices
                .get(device)
                .map(|d| vec![d])
                .unwrap_or_default()
        }
    }

    pub fn info(&self) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.args.target));
        }

        found.iter().for_each(|d| println!("{d}"))
    }

    pub fn get(&self) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.args.target));
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
        self.devices.values().for_each(|d| println!("{d}"))
    }

    fn set(&self, value: usize) {
        let found = self.find_device();
        if found.is_empty() {
            die(format!("device {:?} not found", self.args.target));
        }

        for dev in found {
            if let Err(err) = dev.set(value) {
                die(format!(
                    "failed to set brightness of '{}' to '{}': {}",
                    dev.name(),
                    value,
                    err,
                ))
            }
        }
    }

    pub fn run(self) {
        let Some(cmd) = &self.args.command else {
            self.get();
            return;
        };

        match cmd {
            cli::Action::Info => self.info(),
            cli::Action::Get => self.get(),
            cli::Action::Set { value } => self.set(*value),
            cli::Action::List => self.list(),
        }
    }
}

fn main() {
    let args = cli::Cli::parse();
    Helligkeit::new(&args).run();
}
