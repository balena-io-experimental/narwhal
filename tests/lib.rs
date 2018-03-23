
extern crate narwhal;
// use narwhal::types::Client;

#[cfg(test)]
mod tests {

    mod network {
        #[test]
        fn path_gen() {
            let c = ::narwhal::types::Client {
                socket_path: String::from("/var/run/docker.sock"),
                port: 80,
            };
            assert_eq!(::narwhal::network::generate_path(c, "test"), "/var/run/docker.sock/test");
        }
    }
}
