use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;

pub struct Network {
    stream: TcpStream,
    rx: mpsc::Receiver<(char, char, (i32, i32), (i32, i32))>,
}

impl Network {
    pub fn new(address: &str, connect: bool) -> Self {
        let (tx, rx) = mpsc::channel::<(char, char, (i32, i32), (i32, i32))>();
        let stream;
        if connect {
            // Connect to the specified address
            stream = TcpStream::connect(address).ok().unwrap();
            let mut socket = stream.try_clone().ok().unwrap();
            let mut buffer = [0; 32];
            std::thread::spawn(move || {
                while let Ok(msg) = socket.read(&mut buffer) {
                    if msg == 0 {
                        break;
                    }
                    tx.send(parse_buffer(buffer)).expect("Could not send");
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
            let mut buffer = [0; 32];
            std::thread::spawn(move || {
                while let Ok(msg) = socket.read(&mut buffer) {
                    if msg == 0 {
                        break;
                    }
                    tx.send(parse_buffer(buffer)).expect("Could not send");
                }
            });
        }
        Self { stream, rx }
    }

    pub fn recieve(&mut self) -> (char, char, (i32, i32), (i32, i32)) {
        println!("Recived");
        let mut message = (' ', ' ', (0, 0), (0, 0));
        while let Ok(read) = self.rx.try_recv() {
            message = read;
        }
        message
    }
    pub fn write(&mut self, message: Vec<u8>) {
        self.stream.write(&message).expect("Failed writing process");
    }
}

fn parse_buffer(buffer: [u8; 32]) -> (char, char, (i32, i32), (i32, i32)) {
    let msg: (char, char, (i32, i32), (i32, i32)) = match buffer[0] {
        0 => ('D', 'D', (0, 0), (0, 0)),
        1 => match buffer[1] {
            0 | 1 | 2 => (
                'M',
                'S',
                ((buffer[2] & 0x07).into(), ((buffer[2] & 0x38) >> 3).into()),
                ((buffer[3] & 0x07).into(), ((buffer[3] & 0x38) >> 3).into()),
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
