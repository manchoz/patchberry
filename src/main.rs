use std::env;

extern crate patchberry;

fn main() {
    println!("Hello, world!");

    let args: Vec<String> = env::args().collect();

    let filename = args[1].clone();

    println!("In file {}", filename);

    let (clients, connections) = patchberry::parse_aconnect(filename);

    clients.iter().for_each(&|cli| println!("{:#?}", cli));
    connections.iter().for_each(&|conn| println!("{:#?}", conn));
}

