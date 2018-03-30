use errors::*;
use types;
use types::Client;

use httpstream::HttpStream;
use tcp::TcpStream;
use unix::UnixStream;
use tls::TlsStream;

use utils::http;

pub fn get(client: Client, path: &str) -> Result<http::Response> {
    let req = gen_request("GET", path, None);

    match client.backend {
        types::CommsBackend::Unix => {
            let stream = UnixStream::connect(client).chain_err(
                || "Could not connect to unix socket",
            )?;

            perform_request(stream, req)
        }
        types::CommsBackend::TCP => {
            let stream = TcpStream::connect(client).chain_err(
                || "Could not connect to tcp address",
            )?;

            perform_request(stream, req)
        }
        types::CommsBackend::TLS => {
            let stream = TlsStream::connect(client).chain_err(
                || "Could not connect to tls address",
            )?;

            perform_request(stream, req)
        }
    }
}

pub fn perform_request<T: HttpStream>(mut stream: T, req: http::Request) -> Result<http::Response> {
    stream.request(req).chain_err(
        || "Could not perform HTTP request",
    )
}

pub fn gen_request(method: &str, path: &str, body: Option<String>) -> http::Request {
    let mut req = http::Request {
        method: String::from(method),
        path: String::from(path),
        headers: ::std::collections::HashMap::new(),
        body
    };
    req.headers.insert(
        String::from("Host"),
        String::from("narwhal"),
    );
    req
}
