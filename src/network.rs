use std::os::unix::net::UnixStream;
use std::io::prelude::*;

use errors::*;
use types::Client;

/// The `Response` type stores the response from the container engine
pub struct Response {
    pub status_code: u32,
    pub body: String,
}

/// Get a `Response` from the container engine, defined by the `Client` and
/// using the `path` endpoint.
///
/// # Examples
/// ```
/// let client = narwhal::types::Client {
///     socket_path: String::from("/var/run/docker.sock"),
/// };
/// let path = "/version";
/// let version =  narwhal::network::simple_get(client, &path);
/// ```
pub fn simple_get(client: Client, path: &str) -> Result<Response> {
    let mut stream =
        UnixStream::connect(client.socket_path).chain_err(|| "Could not connect to unix socket")?;
    let req = format_path(path, "GET");

    stream
        .write_all(req.as_bytes())
        .chain_err(|| "Could not write to unix socket")?;

    let mut response = String::new();
    stream
        .read_to_string(&mut response)
        .chain_err(|| "Could not read from unix socket")?;

    // This is awful and hacky, but should work ok...
    let mut first = true;
    let mut in_body = false;
    let mut status_code: u32 = 0;
    let mut body = String::new();

    for l in response.split("\r\n") {
        if first {
            match l.split(" ").nth(1) {
                Some(str) => {
                    status_code = str.parse::<u32>()
                        .chain_err(|| "Error parsing HTTP status code")?
                }
                None => bail!("Error parsing HTTP response headers"),
            };
            first = false;
        } else if in_body {
            body.push_str(l);
        } else {
            if l == "" {
                in_body = true;
            }
        }
    }

    if status_code == 0 {
        bail!("Error locating HTTP status code");
    }

    Ok(Response {
        status_code: status_code,
        body: body,
    })
}

/// Preparing a query with a specific `path` for the given `method`.
///
/// # Examples
/// ```
/// let client = narwhal::types::Client {
///     socket_path: String::from("/var/run/docker.sock"),
/// };
/// let path = "/version";
/// let method = "GET";
/// let reques = narwhal::network::format_path(path, method);
/// ```
pub fn format_path(path: &str, method: &str) -> String {
    let footer = String::from(" HTTP/1.1\r\nHost: /docker\r\n\r\n");
    let mut buf = String::with_capacity(method.len() + 1 + path.len() + footer.len());

    buf.push_str(method);
    buf.push(' ');
    buf.push_str(path);
    buf.push_str(&footer);

    buf
}
