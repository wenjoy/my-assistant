use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

use strum_macros::{AsRefStr, EnumString};

#[derive(EnumString, AsRefStr)]
pub enum HttpMethod {
    #[strum(serialize = "GET")]
    GET,
    #[strum(serialize = "POST")]
    POST,
}

pub struct Router {
    pub method: HttpMethod,
    pub path: String,
    pub handler: Box<dyn Fn() -> ()>,
}

pub async fn http_server(router: Router) {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream, &router);
    }
}

fn handle_connection(mut stream: TcpStream, router: &Router) {
    let buffer = BufReader::new(&stream);
    // let http_request: Vec<_> = buffer
    //     .lines()
    //     .map(|result| result.unwrap())
    //     .take_while(|line| !line.is_empty())
    //     .collect();
    // println!("Request {http_request:?}");
    let request_line = buffer.lines().next().unwrap().unwrap();
    let request_line: Vec<&str> = request_line.split(" ").collect();
    let [method, path] = [request_line[0], request_line[1]];
    let (status_lien, content) = if method == router.method.as_ref() && path == router.path {
        (*router.handler)();
        ("HTTP/1.1 200 OK\r\n\r\n", "simple string")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "Not found")
    };
    let response = format!("{status_lien}\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
