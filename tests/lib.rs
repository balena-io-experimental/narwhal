extern crate narwhal;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests {

    mod network {

        #[test]
        pub fn get_request() {
            let c = ::narwhal::types::Client {
                socket_path: String::from("/var/run/docker.sock"),
            };
            let response = ::narwhal::network::simple_get(c, "/_ping").unwrap();
            assert_eq!(response.body, "OK");
            assert_eq!(response.status_code, 200);
        }

        #[test]
        pub fn cant_access_socket() {
            let c = ::narwhal::types::Client {
                socket_path: String::from("/should/not/exist"),
            };
            let response = ::narwhal::network::simple_get(c, "/_ping");
            match response {
                Ok(_) => assert!(false, "Should fail to connect to the docker socket"),
                Err(e) => assert_eq!(e.description(), "Could not connect to unix socket"),
            }
        }
    }

    mod engine {
        #[test]
        pub fn get_version() {
            let c = ::narwhal::types::Client {
                socket_path: String::from("/var/run/docker.sock"),
            };
            let version = ::narwhal::engine::version(c);

            if let Err(ref e) = version {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not get engine version");
            }
        }

        #[test]
        pub fn ping_engine() {
            let c = ::narwhal::types::Client {
                socket_path: String::from("/var/run/docker.sock"),
            };
            let ping = ::narwhal::engine::ping(c);
            if let Err(ref e) = ping {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not ping engine");
            }

        }
    }

    mod utils {
        #[test]
        pub fn http_response_parsing() {
            let response = "HTTP/1.1 304 test\r\nheader: value\r\nheader2: value2\r\n\r\nbody\r\nbody2";
            let parsed = ::narwhal::utils::http::parse_response(response);
            if let Err(ref e) = parsed {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not parse HTTP response");
            } else {
                let r = parsed.unwrap();
                assert_eq!(r.status_code, 304);

                assert!(r.headers.contains_key("header"), "HTTP headers not correctly parsed");
                assert!(r.headers.contains_key("header2"), "HTTP headers not correctly parsed");

                assert_eq!(r.headers.get("header").unwrap(), "value");
                assert_eq!(r.headers.get("header2").unwrap(), "value2");

                assert_eq!(r.body, "body\r\nbody2");
            }
        }

        #[test]
        pub fn http_request_generating() {

            let mut request = ::narwhal::utils::http::Request {
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
