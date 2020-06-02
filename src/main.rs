use std::env;
use std::fs::File;
use std::io::Read;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct AlsaPort {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum AlsaKind {
    Kernel,
    Unknown,
}

#[derive(Debug)]
struct AlsaInfo {
    id: u32,
    name: String,
    kind: AlsaKind,
    card: u32,
}

#[derive(Debug)]
enum ClientStatus {
    Available,
    Unavailable,
}

#[derive(Debug)]
struct Client {
    name: String,
    status: ClientStatus,
    alsa: AlsaInfo,
    ports: Vec<AlsaPort>,
}

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let filename = &args[1];

    println!("In file {}", filename);

    let mut f = File::open(filename).expect("unable to open file");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("unable to read file");

    let re = Regex::new(r"client ").unwrap();
    let rooms: Vec<_> = re.split(&contents).collect();

    for room in rooms {
        let mut ports : Vec<AlsaPort> = Vec::new();

        let mut lines = room.lines();

        // First row should be the client definition
        let alsa_client = match lines.next() {
            Some(line) => match_client(line),
            None => None
        };

        // Next lines are ports
        match &alsa_client {
            Some(_found_client) => {
                for line in room.lines() {
                    let alsa_port = match_port(line);
                    match alsa_port {
                        Some(found_port) => {
                            ports.push(found_port);
                        },
                        None => ()
                    }
                }
            },
            None => ()                
        }

        let client = match alsa_client {
            Some(found) => {
                let client = Client {
                    name : String::from(&found.name),
                    status: ClientStatus::Unavailable,
                    alsa: found,
                    ports: ports };
                Some (client)
                },
            None => None
        };

        match client{
            Some(found) => println!("{:#?}", found),
            None => ()
        }

    }

    // Keep status as RE match type (client/port) and add ports to current client.
    // Create new client if status is client.
}

fn match_client(line: &str) -> Option<AlsaInfo> {
    let re = Regex::new(r"^(\d+): '(.+)' \[type=(\S+),card=(\d+)\]$").unwrap();
    let caps = re.captures(line);

    match caps {
        Some(caps) => {
            let id = (&caps[1]).parse().unwrap();
            let name = String::from(&caps[2]);
            let kind = match &caps[3] {
                "kernel" => AlsaKind::Kernel,
                _ => AlsaKind::Unknown,
            };
            let card = (&caps[4]).parse().unwrap();

            let client = AlsaInfo {
                id,
                name,
                kind,
                card,
            };

            Some(client)
        }
        None => None,
    }
}

fn match_port(line: &str) -> Option<AlsaPort> {
    let re = Regex::new(r"^\s+(\d) '(.+?)\s*'$").unwrap();
    let caps = re.captures(line);

    match caps {
        Some(caps) => {
            let id = (&caps[1]).parse().unwrap();
            let name = String::from(&caps[2]);

            let alsa_port = AlsaPort { id, name };
            Some(alsa_port)
        }
        None => None,
    }
}
