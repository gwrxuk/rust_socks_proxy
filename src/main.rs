use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::rustls;
use rustls_pemfile::{certs, rsa_private_keys};
use tokio_rustls::server::TlsStream;
use std::fs::File;
use std::io::BufReader;



fn load_tls_config() -> io::Result<rustls::ServerConfig> {
    let cert_file = &mut BufReader::new(File::open("cert/cert.pem")?);
    let key_file = &mut BufReader::new(File::open("cert/key.pem")?);
    let key_file2 = &mut BufReader::new(File::open("cert/key.pem")?);
    
    let key2: Vec<_> = rsa_private_keys(key_file2).collect();

    println!("{:?}",key2);

    let cert_chain = certs(cert_file)
    .into_iter()
    .map(|x| rustls::Certificate(x.unwrap().to_vec()))
    .collect();

    let key = rsa_private_keys(key_file)
    .into_iter()
    .next()
    .map(|x| rustls::PrivateKey(x.unwrap().secret_pkcs1_der().to_vec()))
    .unwrap();

   let config = rustls::ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("Failed to build TLS configuration");

    Ok(config)
}

async fn handle_connection(mut stream: TlsStream<TcpStream>) -> io::Result<()> {
    let mut buf = [0; 1024];

    loop {
        let nbytes = stream.read(&mut buf).await?;
        if nbytes == 0 {
            break; // Connection was closed or we read all the data
        }

        // Echo the data back to the client as an example operation
        stream.write_all(&buf[0..nbytes]).await?;
    }

    Ok(())
}

// start_server function encapsulates the server initialization and loop
async fn start_server(addr: &str, tls_config: rustls::ServerConfig) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    let tls_acceptor = tokio_rustls::TlsAcceptor::from(std::sync::Arc::new(tls_config));

    println!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();

        tokio::spawn(async move {
            let tls_stream = match acceptor.accept(stream).await {
                Ok(tls_stream) => tls_stream,
                Err(e) => {
                    eprintln!("Failed to accept TLS connection: {}", e);
                    return;
                }
            };

            if let Err(e) = handle_connection(tls_stream).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}


#[tokio::main]
async fn main() -> io::Result<()> {
    let tls_config = load_tls_config()?;
    start_server("127.0.0.1:1080", tls_config).await
}


#[cfg(test)]
mod tests {
    use super::*;
    use tokio::net::TcpStream;

    #[tokio::test]
    async fn test_server_accepts_connections() {
        let server_addr = "127.0.0.1:12345";
        tokio::spawn(async move {
            // Start the server in a background task
            let _ = start_server(server_addr).await;
        });

        // Give the server a moment to start up
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Attempt to connect to the server
        let client_result = TcpStream::connect(server_addr).await;

        assert!(client_result.is_ok());
    }
}
