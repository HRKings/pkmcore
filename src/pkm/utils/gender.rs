use crate::{species::base::SpeciesGenderCategory, game::enums::pokemon_gender::PokemonGender};

pub fn generate_from_pid_and_ratio(pid: u32, gender_ratio: SpeciesGenderCategory) -> PokemonGender{
  match gender_ratio {
    SpeciesGenderCategory::Genderless => PokemonGender::Genderless,
    SpeciesGenderCategory::FemaleOnly => PokemonGender::Female,
    SpeciesGenderCategory::MaleOnly => PokemonGender::Male,
    _ => if (pid & 0xFF) < gender_ratio as u32 {
      PokemonGender::Female
    } else {
      PokemonGender::Male
    }
  }
}

pub fn convert_from_char(symbol: char) -> PokemonGender {
  match symbol {
    '♀' | 'F' => PokemonGender::Female,
    '♂' | 'M' => PokemonGender::Male,
    _ => PokemonGender::Genderless,
  }
}
