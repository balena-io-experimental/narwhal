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

        let mut f = File::open(path).expect(&format!("Could not open fixture {}", filename));

        let mut contents = String::new();
        f.read_to_string(&mut contents).expect(&format!(
            "Something went wrong reading the fixture {}",
            filename
        ));

        contents
    }

    mod engine {

        use narwhal::engine;
        use super::{get_client, read_fixture};

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
        pub fn parse_info_docs() {
            let test_str = read_fixture("info_docs");
            let parsed = engine::info_parse(&test_str).expect("Error parsing info fixture");

            assert_eq!(parsed.architecture.unwrap(), "x86_64");
            assert_eq!(parsed.discovery_backend.is_none(), true);
            assert_eq!(parsed.debug.unwrap(), false);
            assert_eq!(parsed.images.unwrap(), 16);
            assert_eq!(parsed.labels.unwrap().len(), 1);
            assert_eq!(parsed.plugins.unwrap().network.unwrap().len(), 3);
            assert_eq!(parsed.system_status.unwrap()[0][1], "Healthy");
        }

        #[test]
        pub fn parse_info() {
            let test_str = read_fixture("info_real");
            let parsed = engine::info_parse(&test_str).expect("Error parsing info fixture");

            assert_eq!(
                parsed.index_server_address.unwrap(),
                "https://index.docker.io/v1/"
            );
            assert_eq!(parsed.labels.unwrap().len(), 0);
            assert_eq!(parsed.system_status.is_none(), true);
            assert_eq!(parsed.server_version.unwrap(), "18.03.0-ce");
        }

        #[test]
        pub fn get_info() {
            let c = get_client();
            let info = engine::info(c);

            if let Err(ref e) = info {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not get engine info");
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
        use narwhal::containers;
        use super::{get_client, read_fixture};

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
            let containers = containers::get_containers(c, None);

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

    mod images {
        use narwhal::images;
        use super::{get_client, read_fixture};

        #[test]
        pub fn parse_get_images_empty() {
            assert_eq!(images::get_images_parse("[]").unwrap().len(), 0);
        }

        #[test]
        pub fn parse_get_images_docs() {
            let test_str = read_fixture("get_images_docs");
            let parsed =
                images::get_images_parse(&test_str).expect("Error parsing get_images fixture");

            assert_eq!(parsed.len(), 2);

            let first = &parsed[0];
            assert_eq!(
                first.id,
                "sha256:e216a057b1cb1efc11f8a268f37ef62083e70b1b38323ba252e25ac88904a7e8"
            );
            assert_eq!(first.repo_tags.clone().unwrap().len(), 2);
            assert_eq!(first.repo_digests.clone().unwrap().len(), 1);
        }

        #[test]
        pub fn parse_get_images() {
            let test_str = read_fixture("get_images_real");
            let parsed =
                images::get_images_parse(&test_str).expect("Error parsing get_images fixture");

            assert_eq!(parsed.len(), 2);

            let first = &parsed[0];
            assert_eq!(
                first.id,
                "sha256:6d62985fe6c45b09beafde79d04f384ecca2ed1b3b764aa8adfcd98a555c5069"
            );
            let first_labels = first.labels.clone();
            assert_eq!(first_labels.is_some(), true);
            assert_eq!(first_labels.unwrap().len(), 3);
            let second = &parsed[1];
            assert_eq!(second.labels.clone().is_none(), true);
        }

        #[test]
        pub fn get_images() {
            let c = get_client();
            let images = images::get_images(c, None);

            if let Err(ref e) = images {
                use error_chain::ChainedError;
                println!("{}", e.display_chain());
                assert!(false, "Could not get list of containers");
            }
        }
    }

    mod queries {
        use narwhal::{QueryFilter, QueryParameters};

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
                vec![String::from("paused"), String::from("running")],
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
                body: None,
            };
            request
                .headers
                .insert(String::from("header"), String::from("value"));
            let request_str = http::gen_request_string(request);
            assert_eq!(request_str, "GET /test HTTP/1.1\r\nheader: value\r\n\r\n");
        }

        #[test]
        pub fn gen_http_request_with_body() {
            let request = http::Request {
                method: String::from("POST"),
                path: String::from("/test"),
                headers: ::std::collections::HashMap::new(),
                body: Some(String::from("testbody")),
            };
            let request_str = http::gen_request_string(request);
            assert_eq!(request_str, "POST /test HTTP/1.1\r\n\r\ntestbody");
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

            assert_eq!(
                http::parse_chunked(test_str).unwrap(),
                "Wikipedia in\r\n\r\nchunks."
            );
        }
    }
}
