use std::sync::Arc;

use crate::repositories::pokemon::{Repository, FetchAllError};

pub struct Response {
  pub number: u16,
  pub name: String,
  pub types: Vec<String>,
}

pub struct Request {
  name: String
}

pub enum Error {
  Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Vec<Response>, Error> {
  match repo.fetch_by_name(req.name) {
    Ok(pokemons) => Ok(pokemons
      .into_iter()
      .map(|p| Response {
        number: u16::from(p.number),
        name: String::from(p.name),
        types: Vec::<String>::from(p.types),
      })
      .collect::<Vec<Response>>()),
    // (maybe) UNREACHABLE!
    Err(FetchAllError::Unknown) => Err(Error::Unknown),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::{domain::entities::{PokemonName, PokemonNumber, PokemonTypes},
    repositories::pokemon::InMemoryRepository};

  #[test]
  fn it_should_return_an_unknown_error_when_an_unexpected_error_happens() {
    let repo = Arc::new(InMemoryRepository::new().with_error());

    let res = execute(repo, Request { name: String::from("Pikachu") });

    match res {
      Err(Error::Unknown) => {}
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_return_all_the_pokemons_fetched_by_name_otherwise() {
    let repo = Arc::new(InMemoryRepository::new());
    let req = Request { name: String::from("chu") };
    
    repo.insert(
      PokemonNumber::try_from(1).unwrap(),
      PokemonName::try_from(String::from("Apikachu")).unwrap(),
      PokemonTypes::charmander(),
    ).ok();
    
    repo.insert(
      PokemonNumber::pikachu(),
      PokemonName::pikachu(),
      PokemonTypes::pikachu(),
    ).ok();

    repo.insert(
      PokemonNumber::charmander(),
      PokemonName::charmander(),
      PokemonTypes::charmander(),
    ).ok();

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.len(), 2);
        assert_eq!(res[0].name, String::from("Apikachu"));
      }
      _ => unreachable!(),
    }
  }
}