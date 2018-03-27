use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use errors::*;
use types::Client;

use utils::http;

pub fn simple_get(client: Client, path: &str) -> Result<http::Response> {
    let mut stream = UnixStream::connect(client.socket_path).chain_err(|| "Could not connect to unix socket")?;
    let mut req = http::Request {
        method: String::from("GET"),
        path: String::from(path),
        headers: ::std::collections::HashMap::new()
    };
    req.headers.insert(String::from("Host"), String::from("/narwhal"));

    let req_str = http::gen_request_string(req);

    stream.write_all(req_str.as_bytes()).chain_err(|| "Could not write to unix socket")?;

    let mut response_str = String::new();
    stream.read_to_string(&mut response_str).chain_err(|| "Could not read from unix socket")?;

    let res = http::parse_response(&response_str).chain_err(|| "Could not parse response")?;
    Ok(res)
}

