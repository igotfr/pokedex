use std::sync::Arc;

use crate::domain::entities::{Pokemon, PokemonNumber, PokemonName, PokemonTypes};
use crate::repositories::pokemon::{Repository, InsertError};

pub struct Request {
  pub number: u16,
  pub name: String,
  pub types: Vec<String>,
}

/*enum Response {
  Ok(u16),
  BadRequest,
  Conflict,
  Error,
}*/

pub struct Response {
  pub number: u16,
  pub name: String,
  pub types: Vec<String>,
}

pub enum Error {
  BadRequest,
  Conflict,
  Unknown,
}

//fn execute(repo: &mut dyn Repository, req: Request) -> Response {
pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
  match (
    PokemonNumber::try_from(req.number),
    PokemonName::try_from(req.name),
    PokemonTypes::try_from(req.types),
  ) {
      (Ok(number), Ok(name), Ok(types)) => match repo.insert(number, name, types) {
        /*Insert::Ok(number) => Response::Ok(u16::from(number)),
        Insert::Conflict => Response::Conflict,
        Insert::Error => Response::Error,*/
        Ok(Pokemon {
          number,
          name,
          types,
        }) => Ok(Response {
          number: u16::from(number),
          name: String::from(name),
          types: Vec::<String>::from(types),
        }),
        Err(InsertError::Conflict) => Err(Error::Conflict),
        Err(InsertError::Unknown) => Err(Error::Unknown),
      },
      //_ => Response::BadRequest,
      _ => Err(Error::BadRequest),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::repositories::pokemon::InMemoryRepository;

  #[test]
  fn it_should_return_a_bad_request_error_when_request_is_invalid() {
    let repo = Arc::new(InMemoryRepository::new());
    /*let req = Request {
      number: 25,
      name: String::from(""),
      types: vec![String::from("Electric")],
    };*/
    let req = Request::new(
      PokemonNumber::pikachu(),
      PokemonName::bad(),
      PokemonTypes::pikachu(),
    );

    let res = execute(repo, req);

    match res {
      //Response::BadRequest => {},
      Err(Error::BadRequest) => {}
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_return_a_conflict_error_when_pokemon_number_already_exists() {
    /*let number = PokemonNumber::try_from(25).unwrap();
    let name = PokemonName::try_from(String::from("Pikachu")).unwrap();
    let types = PokemonTypes::try_from(vec![String::from("Electric")]).unwrap();*/
    let repo = Arc::new(InMemoryRepository::new());
    /*repo.insert(number, name, types);
    let req = Request {
      number: 25,
      name: String::from("Charmander"),
      types: vec![String::from("Fire")],
    };*/
    repo.insert(
      PokemonNumber::pikachu(),
      PokemonName::pikachu(),
      PokemonTypes::pikachu(),
    )
    .ok();

    let req = Request::new(
      PokemonNumber::pikachu(),
      PokemonName::charmander(),
      PokemonTypes::charmander(),
    );

    let res = execute(repo, req);

    match res {
      //Response::Conflict => {}
      Err(Error::Conflict) => {}
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_return_an_error_when_an_unexpected_error_happens() {
    let repo = Arc::new(InMemoryRepository::new().with_error());
    /*let number = 25;
    let req = Request {
      number,
      name: String::from("Pikachu"),
      types: vec![String::from("Electric")],
    };*/
    let req = Request::new(
      PokemonNumber::pikachu(),
      PokemonName::pikachu(),
      PokemonTypes::pikachu(),
    );

    let res = execute(repo, req);

    match res {
      //Response::Error => {},
      Err(Error::Unknown) => {}
      _ => unreachable!(),
    }
  }

  #[test]
  fn it_should_return_the_pokemon_otherwise() {
    let repo = Arc::new(InMemoryRepository::new());
    //let number = 25;
    /*let req = Request {
      number,
      name: String::from("Pikachu"),
      types: vec![String::from("Electric")],
    };*/

    let req = Request::new(
      PokemonNumber::pikachu(),
      PokemonName::pikachu(),
      PokemonTypes::pikachu(),
    );

    let res = execute(repo, req);

    match res {
      Ok(res) => {
        assert_eq!(res.number, u16::from(PokemonNumber::pikachu()));
        assert_eq!(res.name, String::from(PokemonName::pikachu()));
        assert_eq!(res.types, Vec::<String>::from(PokemonTypes::pikachu()));
      }
      _ => unreachable!(),
    }
  }

  impl Request {
    fn new(number: PokemonNumber, name: PokemonName, types: PokemonTypes) -> Self {
      Self {
        number: u16::from(number),
        name: String::from(name),
        types: Vec::<String>::from(types),
      }
    }
  }
}