use std::fs::File;
use std::io::Read;
use std::process::Command;

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
            // TODO - Parse from an 'aconnect' subprocess
            let output = Command::new("ssh")
                .arg("pizerow.local")
                .args(&["aconnect", "-l"])
                .output()
                .expect("failed to execute process");
            let contents = String::from_utf8(output.stdout).expect("unable to read output");
            // (Vec::new(), Vec::new())
            patchberry::parse_aconnect(contents)
        },
    };

    clients.iter().for_each(&|cli| println!("{:#?}", cli));
    connections.iter().for_each(&|conn| println!("{:#?}", conn));
}
