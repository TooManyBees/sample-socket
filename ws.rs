use std::io::{TcpListener, TcpStream, Acceptor, Listener};

fn handle(mut stream: TcpStream) {
  println!("New client {}", stream.peer_name());
  let mut buf = [0u8, ..4096];
  loop {
    match stream.read(buf) {
      Ok(0) => break,
      Ok(n) => {
        println!("{}", std::str::from_utf8(buf.slice(0, n)));
        let key = "keeeeey";
        let proto = "chat";
        let response = upgrade_response(key, proto);
        println!("{}", response);
        stream.write(response.as_bytes()).unwrap();
      },
      Err(_) => break
    };
  }
}

fn upgrade_response<'a>(key: &'a str, protocol: &'a str) -> String {
  format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\nSec-WebSocket-Protocol: {}\r\n\r\n", key, protocol)
}

fn main() {
  let listener = TcpListener::bind("127.0.0.1", 8080);

  let mut acceptor = listener.listen();

  for stream in acceptor.incoming() {
    match stream {
      Err(e) => { panic!("fuck me running! {}", e) }
      Ok(stream) => spawn(proc() {
        handle(stream)
      })
    }
  }

  drop(acceptor);

}
