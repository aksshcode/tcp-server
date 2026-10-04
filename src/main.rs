//game plan
//listen for stream
//take stream parse
//construct res
//send res
use std::io::Read;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;
use std::process::Command;

fn main() {
    let tcp_listener = TcpListener::bind("0.0.0.0:6767").expect("Failed to bind.");
    let incoming_stream = tcp_listener.incoming();
    println!(
        "Tcp server listening on {}.",
        tcp_listener.local_addr().expect("Failed to get local_addr")
    );
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
        let res = generate_http_response("200 OK", "Haro!");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
        // println!("Res /greet sent")
    } else if path == "/ping" {
        let res = generate_http_response("200 OK", "pong!");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
    } else if path == "/hello-akash" {
        send_msg("Hello akash!");
        let res = generate_http_response("404 Not Found", "pinged akash");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
    } else {
        let res =
            generate_http_response("404 Not Found", "valid end points:\n 1./greet \n 2./ping");
        stream
            .write_all(res.as_bytes())
            .expect("Failed send response.");
        // println!("Res /greet sent")
    }
}

fn generate_http_response(status_line: &str, body: &str) -> String {
    let body_length = body.len();
    let res = format!(
        "HTTP/1.1 {status_line}\r\ncontent-type: text/plain\r\ncontent-length: {body_length}\r\n\r\n{body}"
    );
    res
}

fn send_msg(msg: &str) {
    Command::new("notify-send")
        .args([msg])
        .output()
        .expect("failed to run command and get output");
}
