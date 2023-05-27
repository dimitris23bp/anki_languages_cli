use std::error::Error;
use crate::models::add_notes::AddNotes;

mod anki;

pub mod traits {
    pub mod to_string_json;
}

pub mod models {
    pub mod anki_base_model;
    pub mod add_notes;
    pub mod find_notes;
}

use clap::Parser;
use log::{error, info};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    note: Option<String>,

    /// Deck value. If it is an already existing deck, it will be selected
    #[arg(short, long, default_value_t = String::from("practice"))]
    deck: String
}



#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    if let Err(error) = run().await {
        error!("An error occurred: {}", error);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {

    let args = Args::parse();
    let note: AddNotes;

    info!("Deck is: {:?}", args.deck);
    if let Some(value) = args.note {
        let word = value;
        info!("Front is: {}", word);
        // TODO: Receive translation from API
        note = AddNotes::new(args.deck, word, "fd".to_string());
        anki::create_notes(note).await;
    } else {
        info!("No new note");
    }
    Ok(())

}