use std::{
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use log::{info, trace};
use shecat_server::ThreadPool;

fn main() {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();

    let listener = TcpListener::bind("127.0.0.1:17080").unwrap();

    let pool = ThreadPool::new(4);
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        // info!("handle_connection");

        let lines: Vec<String> = BufReader::new(&stream)
            .lines()
            .map(|line| line.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();
        trace!("{:?}", lines);
        if lines.is_empty() {
            return;
        }
        if lines[0].starts_with("POST") {
            break;
        }
        pool.exec(|| {
            handle_connection(stream);
        });
        // handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // stream.write(&mut buffer).unwrap();

    thread::sleep(Duration::from_secs(2));
    let response = "HTTP/2 200 OK\r\nContent-Length: 1\r\n\r\nok\n";
    stream.write_all(response.as_bytes()).unwrap();
    info!("done");
}
