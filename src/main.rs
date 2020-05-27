use std::env;
use std::fs::File;
use std::io::Read;

extern crate regex;
use regex::Regex;

#[derive(Debug)]
struct AlsaPort {
    name: String,
    port: u32,
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

    println!("Acconnect: \n");
    for line in contents.lines() {
        let re = Regex::new(r"^client (\d+): '(.+)' \[type=(\S+),card=(\d+)\]$").unwrap();

        for cap in re.captures_iter(line) {
            let id = (&cap[1]).parse().unwrap();
            let name = String::from(&cap[2]);
            let kind = match &cap[3] {
                "kernel" => AlsaKind::Kernel,
                _ => AlsaKind::Unknown,
            };
            let card = (&cap[4]).parse().unwrap();

            let client = AlsaInfo {
                id,
                name,
                kind,
                card,
            };
            println!("{:?}", client);
        }
    }

    // Split file in rooms
    // Parse rooms: client (first line) + ports (all the rest)
    // see https://doc.rust-lang.org/std/primitive.str.html#method.split
    let re = Regex::new(r"client ").unwrap();

    // let replaced = contents.replace("\n", "$");

    let rooms: Vec<_> = re.split(&contents).collect();
    // println!("{:?}", rooms);

    for room in rooms {
        println!("New Room");
        for line in room.lines() {
            // match client
            // match port
            println!("{:?}", line);
        }

    }

    // let re = Regex::new(r"^client").unwrap();
    // let rooms: Vec<&str> = re.split(&contents).collect();
    // let rooms_alt = test.contains(&re);

    // Keep status as RE match type (client/port) and add ports to current client.
    // Create new client if status is client.
}
