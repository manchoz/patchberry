use std::fs::File;
use std::io::Read;
use std::process::Command;

pub fn refresh(filename: Option<String>) -> () {
    let contents = match filename {
        Some(filename) => {
            let mut f = File::open(filename).expect("unable to open file");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("unable to read file");

            contents
        }
        None => {
            let output = Command::new("ssh")
                .arg("pizerow.local")
                .args(&["aconnect", "-l"])
                .output()
                .expect("failed to execute process");
            String::from_utf8(output.stdout).expect("unable to read output")
            
        }
    };

    let (clients, connections) = patchberry::parse_aconnect(contents);
    
    clients.iter().for_each(&|cli| println!("{:#?}", cli));
    connections.iter().for_each(&|conn| println!("{:#?}", conn));
}

pub fn cards(filename: Option<String>) -> () {
    let contents = match filename {
        Some(filename) => {
            let mut f = File::open(filename).expect("unable to open file");

            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("unable to read file");     
            
            contents
        }
        None => {
            let output = Command::new("ssh")
                .arg("pizerow.local")
                .args(&["cat", "/proc/asound/cards"])
                .output()
                .expect("failed to execute process");
            String::from_utf8(output.stdout).expect("unable to read output")
        }
    };

    let cards = patchberry::parse_cards(contents);

    cards.iter().for_each(&|conn| println!("{:#?}", conn));
}
