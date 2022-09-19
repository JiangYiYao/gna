use super::common::{Message, Signal};
use super::clipboard;

use message_io::network::{NetEvent, Transport};
use message_io::node::{self, NodeEvent};
use std::time::{Duration};
use arboard::Clipboard;

pub fn run(server_addr: &String, uid: &String) {
    let (handler, listener) = node::split();

    let (remote_id, _addr) = handler.network().connect(Transport::FramedTcp, server_addr).unwrap();

    let mut ctx = Clipboard::new().unwrap();
    let mut last_text = clipboard::get_text(&mut ctx);
    let mut last_image = clipboard::get_image(&mut ctx);

    println!("Client mode running, uid is {}", uid);

    listener.for_each(move |event| match event {
        NodeEvent::Network(net_event) => match net_event {
            NetEvent::Connected(_, established) => {
                if established {
                    println!("Connected to server at {}", remote_id.addr());
                    let request = Message { category: 0, dimension: 0, dimension_data: Vec::new(),
                        content_data: uid.as_str().as_bytes().to_vec()};
                    let output_data = bincode::serialize(&request).unwrap();
                    handler.network().send(remote_id, &output_data);
                }
                else {
                    println!("Can not connect to server at {}", server_addr)
                }
            }
            NetEvent::Accepted(_, _) => unreachable!(),
            NetEvent::Message(_, input_data) => {
                let message: Message = bincode::deserialize(&input_data).unwrap();
                match message.category {
                    0 => {
                        println!("Client can send data to server");
                        handler.signals().send(Signal::CheckClipboard);
                    }
                    1 => {
                        let remote_text = String::from_utf8(message.content_data).unwrap();
                        if remote_text.ne(&last_text) {
                            clipboard::set_text(&mut ctx, &remote_text);
                        }
                    }
                    2 => {
                        let remote_image_content = message.content_data;
                        if remote_image_content != last_image.bytes.to_vec().clone() {
                            clipboard::set_image(&mut ctx, message.dimension_data[0], message.dimension_data[1], &remote_image_content);
                        }
                    }
                    _ => {}
                }
            }
            NetEvent::Disconnected(_) => {
                println!("Server is disconnected");
                handler.stop();
            }
        },
        NodeEvent::Signal(signal) => match signal {
            Signal::CheckClipboard => {
                match clipboard::sync_clipboard_data_change(&mut ctx, &mut last_text, &mut last_image) {
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
                handler.network().send(remote_id, &output_data);
            }
            Signal::SendClipboardImage => {
                println!("May cost a litte time to send an image, please wait a moment");
                let request = Message { category: 2, dimension: 2, 
                    dimension_data: vec![last_image.width, last_image.height], content_data: last_image.clone().bytes.to_vec()};
                let output_data = bincode::serialize(&request).unwrap();
                handler.network().send(remote_id, &output_data);
            }
        },
    });
}