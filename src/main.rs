use std::io::Read;
use std::io::Write;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");
    let mut request_count = 0;
    println!("Server started on localhost:8080.");
    //read , buffer
    let mut buffer = [0u8; 1024];
    for incoming_stream in listener.incoming() {
        // respond
        let body = "Hello there i am akash!";
        let reply = generate_http_response(body);
        handle_incoming_stream(&mut request_count, incoming_stream, &mut buffer, reply);
        println!("req no:{}", request_count);
        println!("req:\n{}", String::from_utf8_lossy(&buffer));
        let parse_buffer = String::from_utf8_lossy(&buffer);
        let path_assembler = parse_buffer
            .lines()
            .next()
            .expect("failed to parse first header");
        println!("Found path:{:?}", path_assembler.split(" ").nth(1));
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

fn generate_http_response(body: &str) -> String {
    format!(
        "HTTP/1.1 200 OK \n content-type:plain/text \n content-length:{} \n\n {}",
        body.len(),
        body
    )
}
