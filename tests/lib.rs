extern crate narwhal;

extern crate error_chain;
extern crate url;

#[cfg(test)]
mod tests {

    use std::env;
    use std::path::Path;
    use url::Url;
    use narwhal::types::{Client, TcpClient, TlsFiles};

    fn get_client() -> Client {
        match env::var("CIRCLECI") {
            Ok(_) => {
                // Return a TLS client
                let cert_path_str = env::var("DOCKER_CERT_PATH").unwrap();
                let cert_path = Path::new(&cert_path_str);
                let url_str = env::var("DOCKER_HOST").unwrap();

                let parsed = Url::parse(&url_str).unwrap();

                Client::new_tls(
                    TcpClient {
                        host: String::from(parsed.host_str().unwrap()),
                        port: parsed.port().unwrap(),
                    },
                    TlsFiles {
                        ca: String::from(cert_path.join("ca.pem").to_string_lossy()),
                        cert: String::from(cert_path.join("cert.pem").to_string_lossy()),
                        key: String::from(cert_path.join("key.pem").to_string_lossy()),
                    },
                )
            }
            Err(_) => Client::new_unix(String::from("/var/run/docker.sock")),
        }
    }

    fn read_fixture(fixture: &str) -> String {
        use std::io::Read;
        use std::fs::File;
        use std::path::PathBuf;

        let mut filename = String::from(fixture);
        filename.push_str(".json");

        let path: PathBuf = [".", "tests", "fixtures", &filename].iter().collect();

        let mut f = File::open(path)
            .expect(&format!("Could not open fixture {}", filename));

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect(&format!("Something went wrong reading the fixture {}", filename));

        contents
    }

    mod engine {

        use narwhal::engine;
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

    mod containers {
        use narwhal::{ containers };
        use super::{ get_client, read_fixture };

        #[test]
        pub fn parse_get_containers_empty() {
            assert_eq!(containers::get_containers_parse("[]").unwrap().len(), 0);
        }

        #[test]
        pub fn parse_get_containers() {
            let test_str = read_fixture("get_containers");
            let parsed = containers::get_containers_parse(&test_str)
                .expect("Error parsing get_containers fixture");

            assert_eq!(parsed.len(), 2);

            let first = &parsed[0];
            assert_eq!(first.image, "alpine");
            assert_eq!(first.state, "created");
            assert_eq!(first.host_config.get("NetworkMode").unwrap(), "default");
            let network = first.network_settings.networks.get("bridge").unwrap();
            assert_eq!(network.network_id, "");
            assert_eq!(network.ip_prefix_len, 0);
        }

        #[test]
        pub fn get_containers() {
            let c = get_client();
            let containers = containers::get_containers(c);

            if let Err(ref e) = containers {
                use error_chain::ChainedError;
                println!("{}", e.display_chain());
                assert!(false, "Could not get list of containers");
            }
        }

        // TODO: Once we can create containers, add this back in
        // and also add a filter test
        // #[test]
        // pub fn get_container_with_params() {
        //     let c = get_client();
        //     let mut params = QueryParameters::new();
        //     params.add("all", true);
        //
        //     let containers = containers::get_container_with_params(c, &mut params)
        //         .unwrap();
        //     assert!(containers.len() > 0);
        // }

    }

    mod queries {
        use narwhal::{ QueryFilter, QueryParameters };

        #[test]
        pub fn simple_params() {
            let mut q = QueryParameters::new();
            q.add("test", "str");
            q.add("int", 42);
            q.add("bool", false);

            let query = q.to_string();
            assert_eq!(query, "test=str&int=42&bool=false");
        }

        #[test]
        pub fn filter_param() {
            let mut q = QueryParameters::new();
            let mut filter = QueryFilter::new();
            filter.insert(
                String::from("status"),
                vec![String::from("paused"), String::from("running")]
            );
            q.add_filter(filter);
            assert_eq!(
                q.to_string(),
                "filter=%7B%22status%22%3A%5B%22paused%22%2C%22running%22%5D%7D"
            );
        }
    }

    mod utils {
        use narwhal::utils::http;

        #[test]
        pub fn http_response_parsing() {
            let response = "HTTP/1.1 304 test\r\nheader: value\r\n\
                header2: value2\r\n\r\nbody\r\nbody2";
            let parsed = http::parse_response(response);
            if let Err(ref e) = parsed {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not parse HTTP response");
            } else {
                let r = parsed.unwrap();
                assert_eq!(r.status_code, 304);

                assert!(
                    r.headers.contains_key("header"),
                    "HTTP headers not correctly parsed"
                );
                assert!(
                    r.headers.contains_key("header2"),
                    "HTTP headers not correctly parsed"
                );

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
            request.headers.insert(
                String::from("header"),
                String::from("value"),
            );
            let request_str = ::narwhal::utils::http::gen_request_string(request);
            assert_eq!(request_str, "GET /test HTTP/1.1\r\nheader: value\r\n\r\n");
        }

        #[test]
        pub fn chunked_parsing() {
            let test_str = "4\r\n\
                Wiki\r\n\
                5\r\n\
                pedia\r\n\
                E\r\n in\r\n\
                \r\n\
                chunks.\r\n\
                0\r\n\
                \r\n";

            assert_eq!(http::parse_chunked(test_str).unwrap(), "Wikipedia in\r\n\r\nchunks.");
        }
    }
}
