use std::sync::Arc;

use crate::{repositories::pokemon::Repository, domain::fetch_all_pokemons};

#[derive(Debug)]
struct Response {
    number: u16,
    name: String,
    types: Vec<String>,
}

pub fn run(repo: Arc<dyn Repository>) {
    match fetch_all_pokemons::execute(repo) {
        Ok(res) => res.into_iter().for_each(|p| {
            println!("{:?}", Response {
                number: p.number,
                name: p.name,
                types: p.types,
            });
        }),
        Err(fetch_all_pokemons::Error::Unknown) => println!("An unknown error occurred"),
    }
}