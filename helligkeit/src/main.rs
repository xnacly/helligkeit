use clap::Parser;

mod cli;

fn main() {
    let args = cli::Cli::parse();
    let leds = helligkeit_dev::leds().expect("Failed to enumerate leds");

    match args.command {
        cli::Action::Info { .. } => {
            todo!("info for a specific device")
        }
        cli::Action::Get { .. } => todo!(),
        cli::Action::Set { .. } => todo!(),
        cli::Action::List => {
            leds.flatten().for_each(|led| {
                println!("{led}");
            });
        }
    }
}
