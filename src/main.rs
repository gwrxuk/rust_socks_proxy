use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

async fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    // Step 1: Handle the greeting and select no authentication
    let mut buf = [0; 262];
    let _ = stream.read(&mut buf).await?;
    // Assuming the client supports no authentication, respond accordingly
    stream.write_all(&[0x05, 0x00]).await?;

    // Step 2: Handle the client's connection request
    let mut request_buf = [0; 4];
    stream.read_exact(&mut request_buf).await?;
    if request_buf[1] != 0x01 {
        // Only handle CONNECT command (0x01)
        return Ok(());
    }

    // Read the address type and destination address
    let addr_type = request_buf[3];
    let destination = match addr_type {
        0x01 => { // IPv4
            let mut addr_buf = [0; 4];
            stream.read_exact(&mut addr_buf).await?;
            format!("{}.{}.{}.{}", addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3])
        },
        0x03 => { // Domain name
            let mut len_buf = [0; 1];
            stream.read_exact(&mut len_buf).await?;
            let len = len_buf[0] as usize;
            let mut addr_buf = vec![0; len];
            stream.read_exact(&mut addr_buf).await?;
            String::from_utf8(addr_buf).unwrap_or_default()
        },
        // Add handling for IPv6 if needed
        _ => return Ok(()),
    };

    // Read the port
    let mut port_buf = [0; 2];
    stream.read_exact(&mut port_buf).await?;
    let port = u16::from_be_bytes(port_buf);

    // Step 3: Connect to the destination
    match TcpStream::connect(format!("{}:{}", destination, port)).await {
        Ok(mut dest_stream) => {
            stream.write_all(&[0x05, 0x00, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?; // Reply success
            let (mut client_reader, mut client_writer) = stream.split();
            let (mut dest_reader, mut dest_writer) = dest_stream.split();

            tokio::try_join!(
                tokio::io::copy(&mut client_reader, &mut dest_writer),
                tokio::io::copy(&mut dest_reader, &mut client_writer)
            )?;

            Ok(())
        }
        Err(_) => {
            stream.write_all(&[0x05, 0x01, 0x00, 0x01, 0, 0, 0, 0, 0, 0]).await?; // Reply with general failure
            Ok(())
        }
    }
}

// start_server function encapsulates the server initialization and loop
async fn start_server(addr: &str) -> io::Result<()> {
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on {}", addr);

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Failed to handle connection: {}", e);
            }
        });
    }
}

#[tokio::main]
async fn main() -> io::Result<()> {
    start_server("127.0.0.1:1080").await
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
