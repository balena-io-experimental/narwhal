pub enum CommsBackend {
    Unix,
    TCP,
    TLS,
}

pub struct TcpClient {
    pub host: String,
    pub port: u16,
}

pub struct TlsFiles {
    pub key: String,
    pub cert: String,
    pub ca: String,
}

pub struct Client {
    pub backend: CommsBackend,
    pub socket_path: Option<String>,
    pub tcp_options: Option<TcpClient>,
    pub use_tls: bool,
    pub tls_files: Option<TlsFiles>,
}

impl Client {
    pub fn new_unix(socket_path: String) -> Client {
        Client {
            backend: CommsBackend::Unix,
            socket_path: Some(socket_path),
            tcp_options: None,
            use_tls: false,
            tls_files: None,
        }
    }

    pub fn new_tcp(tcp: TcpClient) -> Client {
        Client {
            backend: CommsBackend::TCP,
            socket_path: None,
            tcp_options: Some(tcp),
            use_tls: false,
            tls_files: None,
        }
    }

    pub fn new_tls(tcp: TcpClient, tls_files: TlsFiles) -> Client {
        Client {
            backend: CommsBackend::TLS,
            socket_path: None,
            tcp_options: Some(tcp),
            use_tls: true,
            tls_files: Some(tls_files),
        }
    }
}
