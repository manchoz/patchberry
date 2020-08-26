use std::fs::File;
use std::io::Read;

pub fn refresh(filename: Option<String>) -> () {
    let (clients, connections) = match filename {
        Some(filename) => {
            println!("In file {}", filename);
            let mut f = File::open(filename).expect("unable to open file");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("unable to read file");

            patchberry::parse_aconnect(contents)
        }
        None => {
            // Parse from aconnect subprocess
            (Vec::new(), Vec::new())
        },
    };

    clients.iter().for_each(&|cli| println!("{:#?}", cli));
    connections.iter().for_each(&|conn| println!("{:#?}", conn));
}
