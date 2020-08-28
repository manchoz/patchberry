
pub fn refresh(filename: Option<String>) {
    let contents = match filename {
        Some(filename) => patchberry::get_contents_from_file(filename),
        None => patchberry::get_contents_from_pizero(&["aconnect", "-l"])
    };

    let (clients, connections) = patchberry::parse_aconnect(contents);

    for client in clients {
        println!("{:#?}", client)
    }

    for conn in connections {
        println!("{:#?}", conn)
    }
}

pub fn cards(filename: Option<String>) {
    let contents = match filename {
        Some(filename) => patchberry::get_contents_from_file(filename),
        None => patchberry::get_contents_from_pizero(&["cat", "/proc/asound/cards"])
    };

    let cards = patchberry::parse_cards(contents);

    for card in cards {
        println!("{:#?}", card);
    }
}
