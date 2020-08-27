extern crate regex;
use regex::Regex;

extern crate itertools;
use itertools::Itertools;

#[derive(Debug)]
pub struct Card {
    id: u32,
    usb: String,
    name: String,
}

#[derive(Debug)]
struct Port {
    client: u32,
    port: u32,
}

#[derive(Debug)]
pub struct Connection {
    src: Port,
    dst: Port,
}

#[derive(Debug)]
struct AlsaPort {
    id: u32,
    name: String,
}

#[derive(Debug)]
enum AlsaKind {
    Kernel,
    User,
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
    // Unavailable,
}

#[derive(Debug)]
pub struct Client {
    name: String,
    status: ClientStatus,
    alsa: AlsaInfo,
    ports: Vec<AlsaPort>,
}

pub fn parse_aconnect(contents: String) -> (Vec<Client>, Vec<Connection>) {
    let re = Regex::new(r"client ").unwrap();
    let rooms: Vec<_> = re.split(&contents).collect();

    parse_aconnect_rooms(rooms)
}

fn parse_aconnect_rooms(rooms: Vec<&str>) -> (Vec<Client>, Vec<Connection>) {
    let mut connections: Vec<Connection> = Vec::new();
    let mut clients: Vec<Client> = Vec::new();

    for room in rooms {
        let mut lines = room.lines();

        // First row should be the client definition...
        let alsa_client = match lines.next() {
            Some(line) => match_client(line),
            None => None,
        };

        // Next lines are ports

        // Look for connections
        if let Some(found_client) = &alsa_client {
            // Get line and the next one
            lines
                .clone()
                .tuple_windows()
                .filter(|(_, next)| next.contains("Connecting To"))
                .flat_map(|(line, next)| get_port_connections(&found_client, line, next))
                .for_each(|conn| connections.push(conn));
        }

        // Look for ports
        if let Some(found_client) = alsa_client {
            let ports: Vec<AlsaPort> = lines.filter_map(|line| get_port(line)).collect();

            let client = Client {
                name: String::from(&found_client.name),
                status: ClientStatus::Available,
                alsa: found_client,
                ports,
            };

            clients.push(client);
        }
    }

    (clients, connections)
}

fn match_client(line: &str) -> Option<AlsaInfo> {
    let re = Regex::new(r"^(\d+): '(.+)' \[type=(\S+),card=(\d+)\]$").unwrap();
    let caps = re.captures(line);

    match caps {
        Some(caps) => {
            let id = caps[1].parse().unwrap();
            let name = String::from(&caps[2]);
            let kind = match &caps[3] {
                "kernel" => AlsaKind::Kernel,
                "user" => AlsaKind::User,
                _ => AlsaKind::Unknown,
            };
            let card = caps[4].parse().unwrap();

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

fn get_port(line: &str) -> Option<AlsaPort> {
    let re = Regex::new(r"^\s+(\d) '(.+?)\s*'$").unwrap();
    let caps = re.captures(line);

    match caps {
        Some(caps) => {
            let id = caps[1].parse().unwrap();
            let name = String::from(&caps[2]);

            let alsa_port = AlsaPort { id, name };
            Some(alsa_port)
        }
        None => None,
    }
}

fn get_dst_ports(line: &str) -> Vec<Port> {
    let ports_re = Regex::new(r"(?:(\d+):(\d+))").unwrap();
    ports_re
        .captures_iter(line)
        .map(|cap| Port {
            client: cap[1].parse().unwrap(),
            port: cap[2].parse().unwrap(),
        })
        .collect()
}

fn get_port_connections(client: &AlsaInfo, line: &str, next: &str) -> Vec<Connection> {
    match get_port(line) {
        Some(src) => get_dst_ports(next)
            .iter()
            .map(|dst| Connection {
                src: Port {
                    client: client.id,
                    port: src.id,
                },
                dst: Port {
                    client: dst.client,
                    port: dst.port,
                },
            })
            .collect(),
        None => vec![],
    }
}

pub fn parse_cards(contents: String) -> Vec<Card> {
    fn get_usb_port(line: String) -> Option<Card> {
        let re =
            Regex::new(r"^\s+(\d)\s+\[(.+?)\s*\]: USB-Audio - .+usb-\d+\.(usb-.+),.*$").unwrap();
        let caps = re.captures(&line);

        match caps {
            Some(caps) => {
                let id = caps[1].parse().unwrap();
                let usb = String::from(&caps[2]);
                let name = String::from(&caps[3]);

                Some(Card { id, usb, name })
            }
            None => None,
        }
    }

    contents
        .lines()
        .tuples()
        .map(|(a, b)| [a, b].join(""))
        .filter_map(|line| get_usb_port(line))
        .collect()
}
