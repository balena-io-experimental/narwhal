use std::os::unix;
use std::io::prelude::*;

use errors::*;
use types::Client;
use httpstream::{ HttpStream, read_from_stream };
use utils::http;
use utils::http::{ Request, Response };

pub struct UnixStream {
    stream: unix::net::UnixStream,
}

impl HttpStream for UnixStream {
    fn connect(client: Client) -> Result<UnixStream> {
        let socket_path = client.socket_path.ok_or("No socket path defined with unix backend")?;
        let stream = unix::net::UnixStream::connect(socket_path)
            .chain_err(|| "Could not connect to unix socket")?;

        Ok(UnixStream {
            stream
        })
    }

    fn request(&mut self, req: Request) -> Result<Response> {
        let req_str = http::gen_request_string(req);

        let mut stream = self.stream.try_clone()
            .chain_err(|| "Could not clone unix stream")?;

        stream.write_all(req_str.as_bytes())
            .chain_err(|| "Could not write to unix stream")?;

        let response_str = read_from_stream(&mut stream)
            .chain_err(|| "Could not read from unix stream")?;

        http::parse_response(&response_str)
            .chain_err(|| "Could not parse HTTP response")
    }
}
