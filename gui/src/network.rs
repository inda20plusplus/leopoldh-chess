use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

pub struct Network {
    stream: TcpStream,
    rx: mpsc::Receiver<[u8; 50]>,
}

impl Network {
    pub fn new(address: &str, connect: bool) -> Self {
        let (tx, rx) = mpsc::channel::<[u8; 50]>();
        let stream;
        if connect {
            // Connect to the specified address
            stream = TcpStream::connect(address).unwrap();
            let mut socket = stream.try_clone().unwrap();
            let mut buffer = [0; 50];
            std::thread::spawn(move || {
                while let Ok(msg) = socket.read(&mut buffer) {
                    if msg == 0 {
                        break;
                    }
                    tx.send(buffer).unwrap();
                }
            });
        } else {
            // Listening to the specified address
            let listener = TcpListener::bind(address).unwrap();
            println!("Listening to {}", address);
            // Getting the socket and the address of a new client.
            let (mut socket, addr) = listener.accept().unwrap();
            println!("New client is connected {}", addr);
            stream = socket.try_clone().ok().unwrap();
            let mut buffer = [0; 50];
            std::thread::spawn(move || {
                while let Ok(msg) = socket.read(&mut buffer) {
                    if msg == 0 {
                        break;
                    }
                    tx.send(buffer).unwrap();
                }
            });
        }
        Self { stream, rx }
    }

    pub fn recieve(&mut self) -> Vec<(char, char, (u8, u8), (u8, u8))> {
        let mut messages = Vec::new();
        while let Ok(read) = self.rx.try_recv() {
            messages.push(parse_buffer(read));
        }
        messages
    }
    pub fn write(&mut self, message: [u8; 50]) {
        self.stream.write(&message).expect("Failed writing process");
    }
}

fn parse_buffer(buffer: [u8; 50]) -> (char, char, (u8, u8), (u8, u8)) {
    let mut msg = match buffer[0] {
        0 => ('D', 'D', (0, 0), (0, 0)),
        1 => match buffer[1] {
            0 | 1 | 2 => (
                'M',
                'S',
                (buffer[2] & 0x07, (buffer[2] & 0x38) >> 3),
                (buffer[3] & 0x07, (buffer[3] & 0x38) >> 3),
            ),
            3 | 4 => ('M', 'S', (0, 0), (0, 0)),
            _ => ('C', 'C', (0, 0), (0, 0)),
        },
        2 => ('U', 'U', (0, 0), (0, 0)),
        3 => ('A', 'A', (0, 0), (0, 0)),
        4 => ('C', 'C', (0, 0), (0, 0)),
        5 => ('D', 'D', (0, 0), (0, 0)),
        6 => ('R', 'R', (0, 0), (0, 0)),
        _ => (' ', ' ', (0, 0), (0, 0)),
    };
    msg
}
