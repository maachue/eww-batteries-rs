use clap::Parser;
use color_eyre::Result;

mod bat;
mod cli;
mod serializable;

fn main() -> Result<()> {
    color_eyre::install()?;

    let cmd = cli::Cli::parse();

    let mut first_cache = serializable::Serializeble {
        battery: crate::bat::get_battery_infomation(),
    };
    let json = serde_json::to_string(&first_cache)?;
    println!("{json}");

    loop {
        let after_cache = serializable::Serializeble {
            battery: crate::bat::get_battery_infomation(),
        };

        if first_cache != after_cache {
            first_cache = after_cache;
            let json = serde_json::to_string(&first_cache)?;
            println!("{json}");
        }

        std::thread::sleep(std::time::Duration::from_millis(cmd.sleep));
    }
}
