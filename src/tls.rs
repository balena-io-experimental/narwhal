use std;
use std::io::Write;

use errors::*;
use types::Client;
use utils::http;
use httpstream::{ HttpStream, read_from_stream };

use openssl;

pub struct TlsStream {
    pub tcp_stream: std::net::TcpStream,
    pub stream: openssl::ssl::SslStream<std::net::TcpStream>
}

impl HttpStream for TlsStream {
    fn connect(client: Client) -> Result<TlsStream> {
        let tcp_opts = client.tcp_options
            .chain_err(|| "TLS backend chosen with no TCP information")?;

        let tcp_stream = std::net::TcpStream::connect((&*tcp_opts.host, tcp_opts.port))
            .chain_err(|| "Could not initialise TCP stream to engine")?;

        let tls_opts = client.tls_files
            .chain_err(|| "TLS backend chosen with no TLS information")?;

        let mut context_builder = openssl::ssl::SslContextBuilder::new(openssl::ssl::SslMethod::tls())
            .chain_err(|| "Could not create SSL context")?;

        context_builder.set_private_key_file(tls_opts.key, openssl::ssl::SslFiletype::PEM)
            .chain_err(|| "Could not set key file for TLS")?;

        context_builder.set_certificate_file(tls_opts.cert, openssl::ssl::SslFiletype::PEM)
            .chain_err(|| "Could not set certificate for TLS")?;

        context_builder.set_ca_file(tls_opts.ca)
            .chain_err(|| "Could not set CA for TLS")?;

        let context = context_builder.build();
        let stream = tcp_stream.try_clone().chain_err(|| "Could not clone TCP stream")?;
        let ssl = openssl::ssl::Ssl::new(&context)
            .chain_err(|| "Could not create SSL object")?;
        let ssl_stream = ssl.connect(stream)
            .chain_err(|| "SSL handshake error")?;

        Ok(TlsStream {
            tcp_stream,
            stream: ssl_stream,
        })
    }

    fn request(&mut self, req: http::Request) -> Result<http::Response> {

        let req_str = http::gen_request_string(req);

        let _ = self.stream.write(req_str.as_bytes());

        let data = read_from_stream(&mut self.stream)
            .chain_err(|| "Could not read from TLS stream")?;

        http::parse_response((&data))
    }
}
