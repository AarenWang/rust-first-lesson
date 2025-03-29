use std::io::BufRead;
use std::{
    env, fs,
    io::{BufReader, Read, Write},
    net::TcpListener,
    net::TcpStream,
    path::Path,
};

struct HttpRequestLine {
    method: String,
    uri: String,
    version: String,
}
fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    //let mut reader = BufReader::new(&mut stream);
    //let lines = reader.lines();

    let len = stream.read(&mut buffer).unwrap();
    //将buffer里的\r\n之前 请求行 提取出来
    let req = String::from_utf8_lossy(&buffer[..]).to_string();
    let lines = req.lines();
    let mut first = true;
    let mut http_request_line = None;
    for line in lines {
        //let line = line.unwrap();
        println!("{}", line);
        if first {
            http_request_line = parseHttpRequestLine(String::from(line));
        }
        first = false;
    }

    match http_request_line {
        Some(t) => {
            if t.uri.ends_with(".html") {
                process_with_file(t.uri, stream);
            } else {
                default_response(stream);
            }
        }
        None => bad_response(stream),
    }
}

fn default_response(mut stream: TcpStream) {
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let content = "<font color='black'>Something is Response from Server</font>";
    let header = "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0";
    let response = format!("{}\r\n{}\r\n\r\n{}", "HTTP/1.1 200 OK", header, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn bad_response(mut stream: TcpStream) {
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let content = "<font color='red'>Bad Response</font>";
    let header = "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0";
    let response = format!("{}\r\n{}\r\n\r\n{}", "HTTP/1.1 500 OK", header, content);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn parseHttpRequestLine(line: String) -> Option<HttpRequestLine> {
    let mut requestLine = HttpRequestLine {
        method: String::from(""),
        uri: String::from("/"),
        version: String::from(""),
    };

    let mut spit = line.split(" ");
    match spit.next() {
        Some(t) => {
            requestLine.method = String::from(t);
        }
        None => {
            return None;
        }
    };

    match spit.next() {
        Some(t) => {
            requestLine.uri = String::from(t);
        }
        None => {
            return None;
        }
    };

    match spit.next() {
        Some(t) => {
            requestLine.version = String::from(t);
        }
        None => {
            return None;
        }
    };

    return Some(requestLine);
}

fn process_with_file(uri: String, mut stream: TcpStream) {
    //todo!()
    let path = env::current_dir();
    //println!("current work directory is {}",path.unwrap().display());
    let f_path = path.unwrap().join(uri.replacen("/", "", 1));
    if f_path.exists() {
        //let content = "<font color='green'>Not Found</font>";
        let html_content = fs::read_to_string(f_path);
        let header = format!(
            "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0\r\nContent-Length: {}",
            &html_content.as_ref().unwrap().len().to_string()[..]
        );
        let response = format!(
            "{}\r\n{}\r\n\r\n{}",
            "HTTP/1.1 200 OK",
            header,
            &html_content.as_ref().unwrap()
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    } else {
        let content = "<font color='green'>Not Found</font>";
        let header = format!(
            "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0\r\nContent-Length: {}",
            &content.len().to_string()[..]
        );
        let response = format!("{}\r\n{}\r\n\r\n{}", "HTTP/1.1 404 OK", header, content);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
