use std::collections::HashMap;

use clap::Parser;
use helligkeit_shared::Device;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    let leds = helligkeit_dev::leds().expect("Failed to enumerate leds");
    let backlight = helligkeit_dev::backlight().expect("Failed to enumerate backlights");
    let ddc = helligkeit_ddc::ddc().expect("Failed to enumerate ddc devices");

    let devices: HashMap<String, Box<dyn Device>> = leds
        .map(|x| x.map(|x| Box::new(x) as Box<dyn Device>))
        .chain(backlight.map(|x| x.map(|x| Box::new(x) as Box<dyn Device>)))
        .chain(ddc.map(|x| x.map(|x| Box::new(x) as Box<dyn Device>)))
        .filter_map(|result| match result {
            Ok(device) => Some(device),
            Err(err) => {
                eprintln!("failed to enumerate device: {err}");
                None
            }
        })
        .map(|device| (device.name().to_owned(), device))
        .collect();

    match args.command {
        cli::Action::Info { .. } => {
            todo!("info for a specific device")
        }
        cli::Action::Get { .. } => todo!(),
        cli::Action::Set { .. } => todo!(),
        cli::Action::List => {
            todo!()
        }
    }
}
