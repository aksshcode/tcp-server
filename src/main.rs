use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    let mut request_count = 0;
    println!("Server started on localhost:8080.");
    for incoming_stream in listener.incoming() {
        //read
        let mut buffer = [0u8; 1024];

        // respond
        let body = "Hello there i am akash!";
        let reply = format!(
            "HTTP/1.1 200 OK \n content-type:text/plain \n content-length:{} \n\n {}",
            body.len(),
            body
        );
        handle_incoming_stream(&mut request_count, incoming_stream, &mut buffer, reply);
        println!("req no:{}", request_count);
        println!("req:\n{}", String::from_utf8_lossy(&buffer));
    }
}

fn handle_incoming_stream(
    request_count: &mut i32,
    incoming_stream: Result<std::net::TcpStream, std::io::Error>,
    buffer: &mut [u8; 1024],
    reply: String,
) {
    match incoming_stream {
        Ok(mut stream) => {
            *request_count += 1;
            //reply to stream
            stream
                .write_all(reply.as_bytes())
                .expect("Failed to send response");
            //read stream
            stream.read(buffer).expect("Failed to read stream");
        }
        Err(err) => {
            println!("Error getting stream : {}", err);
        }
    }
}
