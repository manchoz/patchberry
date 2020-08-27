mod commands;

extern crate structopt;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about = "an USB MIDI patchbay")]
enum Opt {
    #[structopt(about = "Refresh info from 'aconnect'")]
    Refresh {
        #[structopt(short, help = "refresh from specified file")]
        filename: Option<String>,
    },

    #[structopt(about = "Load patchbay configuration")]
    Load {
        #[structopt(short, help = "load from specified file")]
        filename: String,
    },

    #[structopt(about = "Refresh Alsa Card - USB")]
    Cards {
        #[structopt(short, help = "load from specified file")]
        filename: Option<String>,
    },
}

fn main() {
    let opt = Opt::from_args();

    match opt {
        Opt::Refresh { filename } => {
            println!("Refreshing...");
            commands::refresh(filename);
        }

        Opt::Load { filename } => {
            println!("Loading...");
            println!("{}", filename);
        }

        Opt::Cards { filename } => {
            println!("Update Cards...");
            commands::cards(filename);
        }
    }
}
