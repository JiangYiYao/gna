use serde::{Serialize, Deserialize};
use std::env;
use std::process;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io;
use clipboard_master::{Master, ClipboardHandler, CallbackResult};
use std::sync::{Arc, Mutex};
use arboard::{Clipboard, ImageData};
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use std::time;
use std::str;


#[derive(Serialize, Deserialize, Debug)]
struct Frame {
    width: usize,
    height: usize,
    content: Vec<u8>,
}

type MutStream = Arc<Mutex<TcpStream>>;
type StreamList = Arc<Mutex<Vec<MutStream>>>;

struct Handler {
    stream_list: StreamList,
    clipboard: Clipboard,
}

impl ClipboardHandler for Handler {
    fn on_clipboard_change(&mut self) -> CallbackResult {
        let list = self.stream_list.lock().unwrap();
		for stream in &list[..] {
            match self.clipboard.get_text() {
                Ok(text) => {
                    let frame = Frame {width: 0, height: 0, content: text.into_bytes()};
                    write_to_stream(stream, &frame);
                },
                Err(_error) => {
                    match self.clipboard.get_image() {
                        Ok(image) => {
                            let frame = Frame {width: image.width, height: image.height, content: image.bytes.to_vec()};
                            write_to_stream(stream, &frame);
                        },
                        Err(_error) => ()
                    }
                }
            }
        }
        CallbackResult::Next
    }

    fn on_clipboard_error(&mut self, error: io::Error) -> CallbackResult {
        eprintln!("Error: {}", error);
        CallbackResult::Next
    }

	fn sleep_interval(&self) -> core::time::Duration {
        core::time::Duration::from_millis(100)
    }
}


#[derive(Serialize, Deserialize, Debug)]
struct Config {
    is_server: bool,
    addr: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() == 4 {
            let _cmd = &args[1];
            if _cmd.ne("-c") {
               Config::deal_wrong_args();
            }
            let ip = args[2].clone();
            let port = args[3].clone();
            let addr = ip + ":" + &port;
            let config = Config {is_server: false, addr: addr.to_string()};
            match Config::save_cache(&config) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("fail to save cache, error: {}", error);
                }
            }
            return config
        } else if args.len() == 3 {
            let _cmd = &args[1];
            if _cmd.ne("-p") {
               Config::deal_wrong_args();
            }
            let port = args[2].clone();
            let addr = "0.0.0.0".to_owned() + ":" + &port;
            let config = Config {is_server: true, addr: addr.to_string()};
            match Config::save_cache(&config) {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("fail to save cache, error: {}", error);
                }
            }
            return config
        } else if args.len() == 1 {
            match Config::load_cache() {
                Ok(config) => {
                    return config;
                },
                Err(error) => {
                    println!("fail to read 'cache.json', error: {}
try to use 'gna -p server_port' (server mode)
or 'gna -c server_ip server_port' (client mode)", error);
                process::exit(1);
                }
            }
        } else {
            Config::deal_wrong_args();
        }
        Config {is_server: true, addr: "0.0.0.0:8888".to_string()}
    }

    fn deal_wrong_args() {
        println!("wrong arguments! 
try to use 'gna' (last mode)
or 'gna -p server_port' (server mode)
or 'gna -c server_ip server_port' (client mode)" );
        process::exit(1);
    }
    
    fn load_cache() -> Result<Config, Box<dyn Error>> {
        let file = OpenOptions::new()
        .read(true)
        .open("cache.json")?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }
    
    fn save_cache(cache: &Config) -> Result<(), Box<dyn Error>> {
        let file = OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open("cache.json")?;
        serde_json::to_writer(&file, &cache)?;
        Ok(())
    }
}

fn write_to_stream(stream: &MutStream, frame: &Frame) {
    let serialized = bincode::serialize(&frame).unwrap();
    let len : u64 = serialized.len().try_into().unwrap();
    let len_buf = bincode::serialize(&len).unwrap();
    
    match stream.lock() {
        Ok(mut stream) => {
            match stream.set_nonblocking(false) {
                Ok(_) => {
                    match stream.write(&len_buf[..]) {
                        Ok(_) => {
                            match stream.write(&serialized[..]) {
                                Ok(_) => {
                                    match stream.set_nonblocking(true) {
                                        _ => ()
                                    }
                                },
                                Err(_) => ()
                            }
                        },
                        Err(_) => ()
                    }
                },
                Err(_) => ()
            }
        },
        Err(_) => ()
    }
}

fn read_from_stream(stream: MutStream) {
    let mut clipboard = Clipboard::new().unwrap();
    let mut len_buf = [0 as u8; 8];
    loop {
        match stream.lock() {
            Ok(mut stream) => {
                match stream.peek(&mut len_buf) {
                    Ok(size) => {
                        if size >= 8 {
                            stream.set_nonblocking(false).unwrap();
                            stream.read_exact(&mut len_buf).unwrap();
                            let len: u64 = bincode::deserialize(&len_buf).unwrap();
                            let len = len.try_into().unwrap();
                            let mut content_buf = vec![0 as u8];
                            content_buf.resize(len, 0);
                            stream.read_exact(&mut content_buf).unwrap();
                            stream.set_nonblocking(true).unwrap();
                            let content: Frame = bincode::deserialize(&content_buf).unwrap();

                            if content.width == 0 && content.height == 0 {
                                match str::from_utf8(&content.content) {
                                    Ok(text) => {
                                        match clipboard.get_text() {
                                            Ok(old_text) => {
                                                if old_text.ne(text) {
                                                    match clipboard.set_text(text.to_string()) {
                                                        _ => ()
                                                    }
                                                }
                                            },
                                            Err(_) => {
                                                match clipboard.set_text(text.to_string()) {
                                                    _ => ()
                                                }
                                            }
                                        }
                                    },
                                    Err(_) => ()
                                }
                            } else {
                                match clipboard.get_image() {
                                    Ok(old_image) => {
                                        if old_image.bytes.to_vec() != content.content {
                                            let image = ImageData { 
                                                width: content.width, 
                                                height: content.height, 
                                                bytes: content.content.into() };
                                            match clipboard.set_image(image) {
                                                _ => ()
                                            }
                                        }
                                    },
                                    Err(_) => {
                                        let image = ImageData { 
                                            width: content.width, 
                                            height: content.height, 
                                            bytes: content.content.into() };
                                        match clipboard.set_image(image) {
                                            _ => ()
                                        }
                                    }
                                }
                            }

                        } else if size == 0 {
                            stream.shutdown(Shutdown::Both).unwrap();
                        }
                    },
                    Err(_) => ()
                }
            },
            Err(_) => ()
        }
        thread::sleep(time::Duration::from_millis(100));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    let stream_list = Arc::new(Mutex::new(Vec::new()));
    let list_copy = stream_list.clone();
    let clipboard = Clipboard::new().unwrap();
    thread::spawn(move || {
		Master::new(Handler {stream_list: list_copy, clipboard: clipboard}).run().unwrap();
	});

    if config.is_server {
        match TcpListener::bind(&config.addr) {
            Ok(listener) => {
               println!("Gna is running in server mode, local_addr:{}", config.addr);
               loop {
                match listener.accept() {
                    Ok((stream, client_addr)) => {
                        loop {
                            println!("{:?} wants to connect me, accept it? (y/n)", client_addr);
                            let mut choice = String::new();
                            match io::stdin().read_line(&mut choice) {
                                Ok(_) => {
                                    choice = choice.trim().to_string();
                                    if choice.eq("y") || choice.eq("n") {
                                        if choice.eq("y") {
                                            println!("accept: {:?}", client_addr);
                                            println!("Hofvarpnir is flying...");
                                            stream.set_nonblocking(true).unwrap();
                                            let stream_vec = stream_list.clone();
                                            let mut stream_vec = stream_vec.lock().unwrap();
                                            let stream = Arc::new(Mutex::new(stream));
                                            stream_vec.push(stream.clone());
                                            let stream_copy = stream.clone();
                                            thread::spawn(move || {
                                                read_from_stream(stream_copy);
                                            });
                                        } else {
                                            println!("refuse: {:?}", client_addr);
                                            stream.shutdown(Shutdown::Both).unwrap();
                                        }
                                        break;
                                    }
                                },
                                Err(error) => {
                                    eprintln!("fail to read your choice, error: {}", error);
                                }
                            }
                        }
                    },
                    Err(error) => {
                        eprintln!("fail to accept, error: {}", error);
                    }
                }
                }
            },
            Err(error) => {
                eprintln!("fail to bind, error: {}", error);
                process::exit(2);
            }
        }

    } else {
        match TcpStream::connect(&config.addr) {
            Ok(stream) => {
                println!("Gna is running in client mode, connected to:{}", config.addr);
                let mut buffer = [0 as u8; 8];
                if stream.peek(&mut buffer).unwrap() == 0 {
                    println!("server refuse.");
                    process::exit(3);
                }
                println!("Hofvarpnir is flying...");
                stream.set_nonblocking(true).unwrap();
                let stream_vec = stream_list.clone();
                let stream = Arc::new(Mutex::new(stream));
                {
                    let mut stream_vec = stream_vec.lock().unwrap();
                    stream_vec.push(stream.clone());
                }
                let stream_copy = stream.clone();
                read_from_stream(stream_copy);
            },
            Err(error) => {
                eprintln!("fail to connect, error: {}", error);
                process::exit(2);
            }
        }
    }
}
