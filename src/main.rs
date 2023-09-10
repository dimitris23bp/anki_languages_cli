#[macro_use]
extern crate lazy_static;

use std::error::Error;

use clap::Parser;
use log::{error, info};

use crate::config::CONFIG;
use crate::models::add_notes::AddNotes;

pub mod config;

pub mod handlers {
    pub mod status_handler;
}

mod utils {
    pub mod anki;
    pub mod translation;
    pub mod reqwest_wrapper;
}

pub mod traits {
    pub mod to_string_json;
}

pub mod models {
    pub mod add_notes;
    pub mod find_notes;
    pub mod deck_name;
    pub mod create_deck;
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long = "source", default_value_t = String::from("en"))]
    source_lang: String,

    #[arg(short, long = "target", default_value_t = String::from("el"))]
    target_lang: String,

    #[arg(short, long)]
    note: Option<String>,

    /// Deck value. If it is an already existing deck, it will be selected
    #[arg(short, long, default_value_t = String::from("ExampleNewDeck"))]
    deck: String,
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

    info!("Source language is : {} and target language is: {}", args.source_lang, args.target_lang);
    info!("Deck is: {:?}", args.deck);
    if let Some(word) = args.note {
        info!("Front is: {}", word);
        let back = utils::translation::translate(word.as_str(), args.source_lang, args.target_lang, &CONFIG.urls.libre_translate).await?;
        note = AddNotes::new(args.deck, word, back);
        utils::anki::create_notes(note, &CONFIG.urls.anki).await;
    } else {
        info!("No new note");
    }
    Ok(())
}