extern crate narwhal;

extern crate error_chain;

#[cfg(test)]
mod tests {

    use std::env;
    use std::path::Path;
    use ::narwhal::types::{ Client, TcpClient, TlsFiles };

    fn get_client() -> Client {
        return match env::var("CIRCLECI") {
            Ok(_) => {
                // Return a TLS client
                let cert_path_str = env::var("DOCKER_CERT_PATH").unwrap();
                let cert_path = Path::new(&cert_path_str);
                let url = env::var("DOCKER_HOST").unwrap();
                let mut parts = url.split(':');
                let host = parts.next().unwrap();
                let port = parts.next().unwrap();
                Client::new_tls( TcpClient {
                    host: String::from(host),
                    port: port.parse().unwrap(),
                }, TlsFiles {
                    ca: String::from(cert_path.join("ca.pem").to_string_lossy()),
                    cert: String::from(cert_path.join("cert.pem").to_string_lossy()),
                    key: String::from(cert_path.join("key.pem").to_string_lossy()),
                })
            },
            Err(_) => {
                Client::new_unix(String::from("/var/run/docker.sock"))
            }
        }
    }

    mod engine {

        use ::narwhal::engine;
        use super::get_client;

        #[test]
        pub fn get_version() {
            let c = get_client();
            let version = engine::version(c);

            if let Err(ref e) = version {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not get engine version");
            }
        }

        #[test]
        pub fn ping_engine() {
            let c = get_client();
            let ping = engine::ping(c);
            if let Err(ref e) = ping {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not ping engine");
            }

        }
    }

    mod utils {
        use ::narwhal::utils::http;

        #[test]
        pub fn http_response_parsing() {
            let response = "HTTP/1.1 304 test\r\nheader: value\r\nheader2: value2\r\n\r\nbody\r\nbody2";
            let parsed = http::parse_response(response);
            if let Err(ref e) = parsed {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not parse HTTP response");
            } else {
                let r = parsed.unwrap();
                assert_eq!(r.status_code, 304);

                assert!(r.headers.contains_key("header"), "HTTP headers not correctly parsed");
                assert!(r.headers.contains_key("header2"), "HTTP headers not correctly parsed");

                assert_eq!(&r.headers["header"], "value");
                assert_eq!(&r.headers["header2"], "value2");

                assert_eq!(r.body, "body\r\nbody2");
            }
        }

        #[test]
        pub fn http_request_generating() {

            let mut request = http::Request {
                method: String::from("GET"),
                path: String::from("/test"),
                headers: ::std::collections::HashMap::new(),
            };
            request.headers.insert(String::from("header"), String::from("value"));
            let request_str = ::narwhal::utils::http::gen_request_string(request);
            assert_eq!(request_str, "GET /test HTTP/1.1\r\nheader: value\r\n\r\n");
        }
    }
}
