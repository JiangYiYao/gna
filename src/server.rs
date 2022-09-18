use super::common::{Message, Signal};
use super::clipboard;

use message_io::network::{NetEvent, Transport, Endpoint};
use message_io::node::{self, NodeEvent};
use std::collections::{HashMap};
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io;
use std::error::Error;
use serde::{Serialize, Deserialize};
use std::time::{Duration};

struct ClientInfo {
    is_auth: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ClientList {
    list: Vec<String>,
}

pub fn run(server_addr: &String, uid: &String) {
    let (handler, listener) = node::split();

    let mut clients_status: HashMap<Endpoint, ClientInfo> = HashMap::new();

    match handler.network().listen(Transport::FramedTcp, server_addr) {
        Ok((_id, _addr)) => println!("Server mode running, uid is {}", uid),
        Err(_) => return println!("Can not working at {}", server_addr),
    }

    let mut client_list: ClientList = ClientList { list: Vec::new() };

    match load_client_info() {
        Ok(list) => {
            client_list = list;
        }
        Err(_) => {}
    }

    let mut last_text = clipboard::get_text();
    let mut last_image = clipboard::get_image();

    handler.signals().send(Signal::CheckClipboard);

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, _) => (), 
            NetEvent::Accepted(endpoint, _listener_id) => {
                clients_status.insert(endpoint, ClientInfo { is_auth: false });
                println!("Client ({}) connected (total clients: {})", endpoint.addr(), clients_status.len());
            }
            NetEvent::Message(endpoint, input_data) => {
                let message: Message = bincode::deserialize(&input_data).unwrap();
                match message.category {
                    // uid
                    0 => {
                        let client_uid = String::from_utf8(message.content_data).unwrap();
                        if client_list.list.iter().any(|e| e == &client_uid) {
                            match clients_status.get_mut(&endpoint) {
                                Some(status) => {
                                    status.is_auth = true
                                }
                                None => {}
                            }
                            let response = Message { category: 0, dimension: 0, dimension_data: Vec::new(), content_data: Vec::new()};
                            let output_data = bincode::serialize(&response).unwrap();
                            handler.network().send(endpoint, &output_data);
                        } else {
                            println!("Client [uid:{} addr:{}] wants to join, accept it? (y/n)", client_uid, endpoint.addr());
                            loop {
                                let mut choice = String::new();
                                io::stdin().read_line(&mut choice).unwrap();
                                match choice.trim().to_lowercase().as_str() {
                                    "y" | "yes" => {
                                        match clients_status.get_mut(&endpoint) {
                                            Some(status) => {
                                                status.is_auth = true
                                            }
                                            None => {}
                                        }
                                        client_list.list.push(client_uid);
                                        save_client_info(&client_list).unwrap();
                                        let response = Message { category: 0, dimension: 0, dimension_data: Vec::new(), content_data: Vec::new()};
                                        let output_data = bincode::serialize(&response).unwrap();
                                        handler.network().send(endpoint, &output_data);
                                        break;
                                    }
                                    "n" | "no" => {
                                        println!("Reject client [uid:{} addr:{:?}]", client_uid, endpoint.addr());
                                        clients_status.remove(&endpoint).unwrap();
                                        handler.network().remove(endpoint.resource_id());
                                        println!("Client ({}) disconnected (total clients: {})", endpoint.addr(), clients_status.len());
                                        break;
                                    }
                                    _ => {
                                        println!("Client [uid:{} addr:{:?}] wants to connect, accept it? (y/n)", client_uid, endpoint.addr());
                                    }
                                }
                            }
                        }
                    }
                    // text_content
                    1 => {
                        match clients_status.get(&endpoint) {
                            Some(status) => {
                                if !status.is_auth {
                                    println!("Close unauthed client({})", endpoint.addr());
                                    clients_status.remove(&endpoint).unwrap();
                                    handler.network().remove(endpoint.resource_id());
                                    println!("Client ({}) disconnected (total clients: {})", endpoint.addr(), clients_status.len());
                                } else {
                                    let remote_text = String::from_utf8(message.content_data).unwrap();
                                    if remote_text.ne(&last_text) {
                                        clipboard::set_text(&remote_text);
                                    }
                                }
                            }
                            None => {
                                println!("Close unknown client({})", endpoint.addr());
                                handler.network().remove(endpoint.resource_id());
                            }
                        }
                    }
                    // image_content
                    2 => {
                        match clients_status.get(&endpoint) {
                            Some(status) => {
                                if !status.is_auth {
                                    println!("Close unauthed client({})", endpoint.addr());
                                    clients_status.remove(&endpoint).unwrap();
                                    handler.network().remove(endpoint.resource_id());
                                    println!("Client ({}) disconnected (total clients: {})", endpoint.addr(), clients_status.len());
                                } else {
                                    let remote_image_content = message.content_data;
                                    if remote_image_content != last_image.bytes.to_vec().clone() {
                                        clipboard::set_image(message.dimension_data[0], message.dimension_data[1], &remote_image_content);
                                    }
                                }
                            }
                            None => {
                                println!("Close unknown client({})", endpoint.addr());
                                handler.network().remove(endpoint.resource_id());
                            }
                        }
                    }
                    _ => {}
                }
            }
            NetEvent::Disconnected(endpoint) => {
                // Only connection oriented protocols will generate this event
                clients_status.remove(&endpoint).unwrap();
                println!("Client ({}) disconnected (total clients: {})", endpoint.addr(), clients_status.len());
            }
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::CheckClipboard => {
                match clipboard::sync_clipboard_data_change(&mut last_text, &mut last_image) {
                    clipboard::Change::TextChange => {
                        handler.signals().send(Signal::SendClipboardText);
                    }
                    clipboard::Change::ImageChange => {
                        handler.signals().send(Signal::SendClipboardImage);
                    }
                    clipboard::Change::NoChange => {}
                }
                handler.signals().send_with_timer(Signal::CheckClipboard, Duration::from_millis(200));
            }
            Signal::SendClipboardText => {
                let request = Message { category: 1, dimension: 0, dimension_data: Vec::new(),
                    content_data: last_text.clone().as_str().as_bytes().to_vec()};
                let output_data = bincode::serialize(&request).unwrap();
                for (endpoint, status) in &clients_status {
                    if status.is_auth {
                        handler.network().send(endpoint.clone(), &output_data);
                    }
                }
            }
            Signal::SendClipboardImage => {
                println!("May cost a litte time to send an image, please wait a moment");
                let request = Message { category: 2, dimension: 2, 
                    dimension_data: vec![last_image.width, last_image.height], content_data: last_image.clone().bytes.to_vec()};
                let output_data = bincode::serialize(&request).unwrap();
                for (endpoint, status) in &clients_status {
                    if status.is_auth {
                        handler.network().send(endpoint.clone(), &output_data);
                    }
                }
            }
        },
    });
}

fn load_client_info() -> Result<ClientList, Box<dyn Error>> {
    let file = OpenOptions::new()
    .read(true)
    .open("client_info.json")?;
    let reader = BufReader::new(file);
    let list = serde_json::from_reader(reader)?;
    Ok(list)
}

fn save_client_info(list: &ClientList) -> Result<(), Box<dyn Error>> {
    let file = OpenOptions::new()
    .create(true)
    .write(true)
    .truncate(true)
    .open("client_info.json")?;
    serde_json::to_writer(&file, &list)?;
    Ok(())
}