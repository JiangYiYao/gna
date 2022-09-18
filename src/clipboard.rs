use arboard::{Clipboard, ImageData};
use std::borrow::Cow;

pub enum Change {
    TextChange,
    ImageChange,
    NoChange,
}

pub fn sync_clipboard_data_change(last_text: &mut String, last_image: &mut ImageData) -> Change {
    let mut ctx = Clipboard::new().unwrap();
    match ctx.get_text() {
        Ok(text) => {
            if text.eq(last_text) {
                return Change::NoChange;
            } else {
                *last_text = text;
                return Change::TextChange;
            }
        }
        Err(_) => {
            match ctx.get_image() {
                Ok(image) => {
                    if image.bytes.to_vec() == last_image.bytes.to_vec().clone() {
                        return Change::NoChange;
                    } else {
                        *last_image = image;
                        return Change::ImageChange;
                    }
                }
                Err(_) => {}
            }
        }
    }
    return Change::NoChange;
}

pub fn get_text() -> String {
    let mut ctx = Clipboard::new().unwrap();
    match ctx.get_text() {
        Ok(text) => {return text;}
        Err(_) => {return String::from("");}
    }
}

pub fn get_image() -> ImageData<'static> {
    let mut ctx = Clipboard::new().unwrap();
    match ctx.get_image() {
        Ok(image) => {return image;}
        Err(_) => {return ImageData { width: 0, height: 0, bytes: Cow::Owned(vec![0])};}
    }
}

pub fn set_text(text: &String) {
	let mut ctx = Clipboard::new().unwrap();
    match ctx.set_text(text.to_string()) {
        Ok(_) => {}
        Err(_) => { println!("Fail to set text.") }
    }
}

pub fn set_image(width: usize, height: usize, content: &Vec<u8>) {
    let mut ctx = Clipboard::new().unwrap();
    let image = ImageData {width, height, bytes: Cow::Owned(content.clone())};
    match ctx.set_image(image) {
        Ok(_) => {}
        Err(_) => { println!("Fail to set image.") }
    }
}
