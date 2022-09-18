use clap::Parser;
use uuid::Uuid;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::error::Error;
use serde::{Serialize, Deserialize};


#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Server mode, need port to bind
    #[clap(short = 'p', value_name = "PORT")]
    port: Option<u64>,

    /// Client mode, need ip and port to connect server
    #[clap(short = 'c', value_name = "IP:PORT")]
    connect: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
    pub is_server: bool,
    pub net_addr: String,
    pub uid: String,
}

pub fn process() -> Option<Info> {
    let args = Args::parse();
    if args.port.is_none() && args.connect.is_none() {
        match load_config_info() {
            Ok(info) => {
                return if info.net_addr.is_empty() || info.uid.is_empty() {
                    None
                } else {
                    Some(info)
                }
            }
            Err(error) => {
                println!("Fail to load 'config_info.json', err msg: {}", error);
                return None;
            }
        }
    } else if args.port.is_some() {
        match load_config_info() {
            Ok(mut info) => {
                info.is_server = true;
                info.net_addr = format!("0.0.0.0:{}", args.port.unwrap().to_string());
                if info.uid.is_empty() {
                    info.uid = Uuid::new_v4().to_string();
                }
                save_config_info(&info).unwrap();
                return Some(info);
            }
            Err(_) => {
                let id = Uuid::new_v4().to_string();
                let info = Info {is_server: true, net_addr: format!("0.0.0.0:{}", args.port.unwrap().to_string()), uid: id};
                save_config_info(&info).unwrap();
                return Some(info);
            }
        }
    } else {
        match load_config_info() {
            Ok(mut info) => {
                info.is_server = false;
                info.net_addr = args.connect.unwrap();
                if info.uid.is_empty() {
                    info.uid = Uuid::new_v4().to_string();
                }
                save_config_info(&info).unwrap();
                return Some(info);
            }
            Err(_) => {
                let id = Uuid::new_v4().to_string();
                let info = Info {is_server: false, net_addr: args.connect.unwrap(), uid: id};
                save_config_info(&info).unwrap();
                return Some(info);
            }
        }
    }
}

fn load_config_info() -> Result<Info, Box<dyn Error>> {
    let file = OpenOptions::new()
    .read(true)
    .open("config_info.json")?;
    let reader = BufReader::new(file);
    let info = serde_json::from_reader(reader)?;
    Ok(info)
}

fn save_config_info(info: &Info) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("config_info.json")?;
    serde_json::to_writer(&file, &info)?;
    Ok(())
}