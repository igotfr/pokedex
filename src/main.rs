mod api;
mod cli;
mod domain;
mod repositories;

#[macro_use]
extern crate rouille;
//extern crate serde;

use std::sync::Arc;
use repositories::pokemon::{InMemoryRepository, Repository, SqliteRepository, AirtableRepository};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, action = clap::ArgAction::Count, help = "Runs in CLI mode")]
  cli: u8,
  #[arg(short, long, value_name = "PATH")]
  sqlite: Option<String>,
  #[arg(short, long, value_names = &["API_KEY", "WORKSPACE_ID"])]
  airtable: Option<Vec<String>>,
}

fn build_repo(sqlite_value: Option<&str>, airtable_values: Option<(&str, &str)>) -> Arc<dyn Repository> {
  if let Some(values) = airtable_values {
    match AirtableRepository::try_new(values.0, values.1) {
      Ok(repo) => return Arc::new(repo),
      _ => panic!("Error while creating airtable repo"),
    }
  }
  if let Some(path) = sqlite_value {
    match SqliteRepository::try_new(path) {
      Ok(repo) => return Arc::new(repo),
      _ => panic!("Error while creating sqlite repo"),
    }
  }

  Arc::new(InMemoryRepository::new())
}

fn main() {
  //let repo = Arc::new(InMemoryRepository::new());
  let args = Args::parse();

  let repo = build_repo(
    args.sqlite.as_deref(),
    args.airtable.as_ref().and_then(|v| v.get(0)
      .and_then(|s1| v.get(1).map(|s2| (s1.as_str(), s2.as_str())))));

  match args.cli {
    0 => api::serve("localhost:8000", repo),
    _ => cli::run(repo),
  }
}
// curl https://api.airtable.com/v0/appfJP7WA4phjUyUw/pokemons -H "Authorization: Bearer keyv1UYH52iTjFoSk"