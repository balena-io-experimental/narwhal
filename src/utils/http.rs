use std::collections::HashMap;

use regex::Regex;
use ::errors::*;

pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
}

pub struct Response {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
}

pub fn gen_request_string(request: Request) -> String {
    let mut ret = String::new();

    ret.push_str(&request.method);
    ret.push(' ');
    ret.push_str(&request.path);
    ret.push_str(" HTTP/1.1\r\n");

    for (k, v) in request.headers {
        ret.push_str(&k);
        ret.push_str(": ");
        ret.push_str(&v);
        ret.push_str("\r\n");
    }

    ret.push_str("\r\n");

    ret
}


pub fn parse_response(response: &str) -> Result<Response> {

    // setup some regexes
    lazy_static! {
        static ref STATUS_LINE_RE: Regex = Regex::new(r"HTTP/\d+\.\d+ (\d+) \w+").unwrap();
        static ref HEADER_LINE_RE: Regex = Regex::new(r"(\w+): (.*)$").unwrap();
    }

    // First split the input into lines based on delimeters
    let mut parts = response.split("\r\n");

    // The first line in the response should be the status line
    let status_line = parts.next().ok_or("Could not parse HTTP response")?;
    let captures = STATUS_LINE_RE.captures(status_line)
        .chain_err(|| "Could not parse status line of HTTP response")?;
    let status_code = captures.get(1)
        .chain_err(|| "Could not parse status code of HTTP response")?
        .as_str()
        .parse::<u16>()
        .chain_err(|| "Could not parse status code of HTTP response as int")?;

    let mut res = Response {
        status_code,
        headers: HashMap::new(),
        body: String::new(),
    };

    // Loop through the rest of the parts until we reach an empty line
    let mut in_body = false;
    let mut added_to_body = false;
    for l in parts {
        if in_body {
            // The remaining parts are all body
            if added_to_body {
                res.body.push_str("\r\n");
            }
            res.body.push_str(l);
            added_to_body = true;
        } else {
            if l == "" {
                in_body = true;
                continue;
            }
            // This must be a HTTP header, parse it as such
            let captures = HEADER_LINE_RE.captures(l)
                .chain_err(|| "Could not parse HTTP headers")?;

            let name = String::from(captures.get(1).unwrap().as_str());
            let value = String::from(captures.get(2).unwrap().as_str());
            res.headers.insert(name, value);
        }
    }

    Ok(res)
}
