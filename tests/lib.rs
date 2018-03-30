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

    mod images {

        use narwhal::images;
        use super::get_client;

        #[test]
        pub fn images_parse_empty() {
            let test_str = "[]";
            assert_eq!(images::get_images_parse(test_str).unwrap().len(), 0);
        }

        #[test]
        pub fn images_parse_example() {
            let test_str = r#"[
              {
                "Id": "sha256:e216a057b1cb1efc11f8a268f37ef62083e70b1b38323ba252e25ac88904a7e8",
                "ParentId": "",
                "RepoTags": [
                  "ubuntu:12.04",
                  "ubuntu:precise"
                ],
                "RepoDigests": [
                  "ubuntu@sha256:992069aee4016783df6345315302fa59681aae51a8eeb2f889dea59290f21787"
                ],
                "Created": 1474925151,
                "Size": 103579269,
                "VirtualSize": 103579269,
                "SharedSize": 0,
                "Labels": {},
                "Containers": 2
              },
              {
                "Id": "sha256:3e314f95dcace0f5e4fd37b10862fe8398e3c60ed36600bc0ca5fda78b087175",
                "ParentId": "",
                "RepoTags": [
                  "ubuntu:12.10",
                  "ubuntu:quantal"
                ],
                "RepoDigests": [
                  "ubuntu@sha256:002fba3e3255af10be97ea26e476692a7ebed0bb074a9ab960b2e7a1526b15d7",
                  "ubuntu@sha256:68ea0200f0b90df725d99d823905b04cf844f6039ef60c60bf3e019915017bd3"
                ],
                "Created": 1403128455,
                "Size": 172064416,
                "VirtualSize": 172064416,
                "SharedSize": 0,
                "Labels": {},
                "Containers": 5
              }
            ]"#;
            assert_eq!(images::get_images_parse(test_str).unwrap().len(), 2);
        }

        #[test]
        pub fn get_images() {
            let c = get_client();
            let version = images::get_images(c);

            if let Err(ref e) = version {
                use error_chain::ChainedError;
                print!("{}", e.display_chain());
                assert!(false, "Could not get list of images");
            }
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
