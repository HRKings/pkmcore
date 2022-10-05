use self::base::PokemonBase;

pub mod base;
pub mod utils;
pub mod variants;

#[derive(Debug, Default)]
pub struct Generation1 {
  pub base: PokemonBase,
  pub is_japanese: bool,
}

#[derive(Debug, Default)]
pub struct Generation3 {
  pub base: PokemonBase,
  pub is_japanese: bool,
}
