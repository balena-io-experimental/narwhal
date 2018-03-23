extern crate narwhal;

#[macro_use]
extern crate error_chain;

#[cfg(test)]
mod tests {

    mod network {
        #[test]
        pub fn path_format() {
            assert_eq!(::narwhal::network::format_path("/info", "GET"), "GET /info HTTP/1.1\r\nHost: /docker\r\n\r\n");
        }

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
    }
}
