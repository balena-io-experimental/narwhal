use std::io::Read;

use types::Client;

use errors::*;
use utils::http::{Request, Response};

pub trait HttpStream: Sized {
    fn connect(client: Client) -> Result<Self>;
    fn request(&mut self, req: Request) -> Result<Response>;
}

pub fn read_from_stream<T: Read>(stream: &mut T) -> Result<String> {
    const BUFFER_SIZE: usize = 4096;
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
    let mut raw: Vec<u8> = Vec::new();

    loop {
        let len = stream
            .read(&mut buffer)
            .chain_err(|| "Could not read from engine stream")?;

        for b in buffer.iter().take(len) {
            raw.push(*b);
        }

        if len < BUFFER_SIZE {
            break;
        }
    }

    let str_response =
        String::from_utf8(raw).chain_err(|| "Could not convert response to utf8 string")?;
    Ok(str_response)
}
