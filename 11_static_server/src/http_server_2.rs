use std::{
    collections::HashMap,
    env, fs,
    io::{BufReader, Cursor, Error, Read, Write},
    net::TcpListener,
    net::TcpStream,
};

#[derive(Debug)]
struct HttpRequest {
    request_line: RequestLine,
    header: Option<HashMap<String, String>>,
    body: Option<Vec<u8>>,
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    loop {
        let (mut stream, addr) = listener.accept().unwrap();
        println!("Accepted a new connection: {}", addr);
        handle_connection(stream);
    }

    //let buffer = "GET /hello.html http1.1\r\nAccept: text/html\r\nHost: 127.0.0.1\r\nUser-Agent: curl-1.0\r\n\r\n".as_bytes().to_vec();
    // let result = RequestLine::parse(buffer);
    // match result  {
    //     Ok(r) => println!("request = {:?}",r),
    //     Err(why) => panic!("{:?}", why)
    // }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = Vec::new();
    let len = stream.read_to_end(&mut buffer).unwrap();
    //将buffer里的\r\n之前 请求行 提取出来
    let result = RequestLine::parse(buffer);
    match result {
        Ok(r) => {
            println!("request = {:?}", r);
            default_response(stream);
        }
        Err(why) => {
            panic!("parse error: {}", why);
        }
    }
}

#[derive(Debug)]
struct RequestLine {
    method: String,
    uri: String,
    version: String,
}

#[derive(Debug)]
enum ParserError {
    Other,
}

impl RequestLine {
    fn parse(buffer: Vec<u8>) -> Result<HttpRequest, String> {
        let position = Box::new(0 as usize);
        let (line, mut read_index) = read_line(position, &buffer);
        //println!("parse line {}",line);
        let request_line = parse_request_line(line);
        if request_line.method != String::from("GET") {
            return Err(String::from("method not suppor"));
        }
        //println!("position: {}",read_index);

        let mut header_map: HashMap<String, String> = HashMap::new();

        loop {
            let read_index_box = Box::new(read_index);
            let (header_line, position) = read_line(read_index_box, &buffer);
            read_index = position;
            //println!("position:{} header_line:{}",read_index,header_line);
            if header_line == String::from("") {
                break;
            }
            let header = header_line.split_once(": ");
            match header {
                Some((k, v)) => {
                    header_map.insert(k.to_string(), v.to_string());
                    ()
                }
                None => (),
            }
        }

        let http_request = HttpRequest {
            request_line: request_line,
            header: Some(header_map),
            body: None,
        };
        Ok(http_request)
    }
}

fn parse_request_line(line: String) -> RequestLine {
    let mut iter = line.split_ascii_whitespace();
    let request_line = RequestLine {
        method: iter.next().unwrap().to_string(),
        uri: iter.next().unwrap().to_string(),
        version: iter.next().unwrap().to_string(),
    };

    request_line
}

fn read_line(position: Box<usize>, buffer: &Vec<u8>) -> (String, usize) {
    let mut s: usize = *position;
    let mut i = s;

    loop {
        s = s + 1;
        if buffer[s] == '\r' as u8 && buffer[s + 1] == '\n' as u8 {
            s = s + 1;
            break;
        }
    }

    let mut v: Vec<u8> = Vec::new();
    while i < s {
        if buffer[i] != '\r' as u8 && buffer[i] != '\n' as u8 {
            v.push(buffer[i]);
        }
        i = i + 1;
    }
    (String::from_utf8(v).expect("Found invalid UTF-8"), s)
}

fn default_response(mut stream: TcpStream) {
    //println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let content = "<font color='black'>Something is Response from Server</font>";
    let header = "Content-Type: text/html;charset=utf-8\r\nServer: nginx/1.8.0";
    let response = format!("{}\r\n{}\r\n\r\n{}", "HTTP/1.1 200 OK", header, content);
    let result = stream.write(response.as_bytes());
    println!("write result = {:?}", result);
    stream.flush();
}
