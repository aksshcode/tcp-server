//game plan
//listen for stream
//take stream parse
//construct res
//send res
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:6767").expect("Failed to bind.");
    let incoming_stream = tcp_listener.incoming();
    println!("Tcp server listening on 127.0.0.1:6767.");
    for stream in incoming_stream {
        println!("Request recieved");
        let mutable_stream = stream.unwrap();
        let path = get_path_from_stream(&mutable_stream);
        handle_path_based_response(mutable_stream, path);
    }
}

fn get_path_from_stream(stream: &TcpStream) -> String {
    let mut stream = stream;
    let mut buffer = [0u8; 1024];
    stream
        .read(&mut buffer)
        .expect("failed to read from stream");
    let request = String::from_utf8_lossy(&buffer);
    let buffer_line = request
        .lines()
        .next()
        .expect("Faild to get header")
        .split(" ")
        .nth(1)
        .expect("Failed to get first header");
    // println!("Constructed path:{}", buffer_line);
    buffer_line.to_owned()
}
fn handle_path_based_response(mut stream: TcpStream, path: String) {
    if path == "/greet" {
        let res = generate_http_response("Haro!");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
        // println!("Res /greet sent")
    } else {
        let res = generate_http_response("valid end points:\n 1./greet");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
        // println!("Res /greet sent")
    }
}

fn generate_http_response(body: &str) -> String {
    let body_length = body.len();
    let res = format!(
        "HTTP/1.1 200 OK \n content-type:plain/text \n content-length:{} \n\n {}",
        body_length, body
    );
    res
}
