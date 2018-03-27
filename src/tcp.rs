// We already have a simple HTTP request and response implementation for
// the unix socket backend, and rust http libs + client certs and more
// complex than necessary, so just use a raw TCP stream for connecting
// over TCP.
//
// Code inspired by: https://github.com/ghmlee/rust-docker/blob/master/src/tcp.rs

use std;
use std::io::Write;

use types::Client;
use httpstream::{ HttpStream, read_from_stream };
use errors::*;
use utils::http;

pub struct TcpStream {
    stream: std::net::TcpStream,
}

impl HttpStream for TcpStream {
    fn connect(client: Client) -> Result<TcpStream> {

        let tcp_opts = client.tcp_options
            .chain_err(|| "TCP backend chosen with no TCP information")?;

        // First connect with the TCP stream
        let tcp_stream = std::net::TcpStream::connect((&*tcp_opts.host, tcp_opts.port))
            .chain_err(|| "Could not initialise TCP stream to engine")?;

        Ok(TcpStream {
            stream: tcp_stream,
        })
    }

    fn request(&mut self, req: http::Request) -> Result<http::Response> {

        let req_str = http::gen_request_string(req);

        let mut stream = self.stream.try_clone()
            .chain_err(|| "Could not clone TCP stream")?;
        let _ = stream.write(req_str.as_bytes());
        let data = read_from_stream(&mut stream)
            .chain_err(|| "Could not read from tcp stream")?;

        let response = http::parse_response(&data)
            .chain_err(|| "Could not parse engine HTTP response")?;

        Ok(response)
    }
}

