use std::{error::Error, ops::RangeInclusive};

use clap::Parser;

#[tokio::main()]
async fn main() {
    let args = CLA::parse();
    let sets = ServerSettings::from(&args);
    println!("{:?}", sets.ports_http);
}

#[derive(Parser, Debug)]
struct CLA {
    #[arg(short = 'H',num_args = 0.. , value_terminator = "H-",value_parser = Ports::str_to_key_val)]
    ports_http: Vec<(String, u16)>,
}

#[derive(Debug)]
struct ServerSettings {
    ports_http: Ports,
}

impl From<&CLA> for ServerSettings {
    fn from(args: &CLA) -> Self {
        let mut ports_http = Ports::default();
        ports_http.key_val_owerwrite(&args.ports_http);
        Self { ports_http }
    }
}

#[derive(Debug)]
struct Ports {
    site: u16,
    game: u16,
}

impl Ports {
    pub fn str_to_key_val(
        s: &str,
    ) -> Result<(String, u16), Box<dyn Error + Send + Sync + 'static>> {
        let pos = s
            .find('=')
            .ok_or_else(|| format!("invalid PORT=value: no '=' found in '{s}'"))?;
        Ok((s[..pos].parse()?, s[pos + 1..].parse()?))
    }

    pub fn key_val_owerwrite(&mut self, keyval: &Vec<(String, u16)>) {
        const PORT_RANGE: RangeInclusive<u16> = 1..=65535;
        for (key, port) in keyval.iter() {
            if !PORT_RANGE.contains(port) {
                println!(
                    "Port for {key} not in range {}-{}, skipping",
                    PORT_RANGE.start(),
                    PORT_RANGE.end()
                );
                continue;
            }
            match key.as_str() {
                "site" => self.site = *port,
                "game" => self.game = *port,
                _ => println!("Invalid key '{key}', skipping"),
            }
        }
    }
}

impl Default for Ports {
    fn default() -> Self {
        Self {
            site: 8000,
            game: 3000,
        }
    }
}
