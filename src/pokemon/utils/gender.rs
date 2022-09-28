use crate::{game::enums::pokemon_gender::PokemonGender, utils::constants::*};

pub fn generate_from_pid_and_ratio(pid: u32, gender_ratio: u8) -> PokemonGender{
  match gender_ratio {
    MAGIC_RATIO_GENDERLESS => PokemonGender::Genderless,
    MAGIC_RATIO_FEMALE => PokemonGender::Female,
    MAGIC_RATIO_MALE => PokemonGender::Male,
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

#[cfg(test)]
mod tests {
  use crate::{pokemon::utils::gender::generate_from_pid_and_ratio, game::enums::pokemon_gender::PokemonGender, utils::constants::*};

  #[test]
  fn generate_female_only() {
    let result = generate_from_pid_and_ratio(0, MAGIC_RATIO_FEMALE);
    assert_eq!(result, PokemonGender::Female);
  }

  #[test]
  fn generate_male_only() {
    let result = generate_from_pid_and_ratio(0, MAGIC_RATIO_MALE);
    assert_eq!(result, PokemonGender::Male);
  }

  #[test]
  fn generate_genderless_only() {
    let result = generate_from_pid_and_ratio(0, MAGIC_RATIO_GENDERLESS);
    assert_eq!(result, PokemonGender::Genderless);
  }

  #[test]
  fn generate_male_when_dualgender() {
    let result = generate_from_pid_and_ratio(255, 253);
    assert_eq!(result, PokemonGender::Male);
  }

  #[test]
  fn generate_female_when_dualgender() {
    let result = generate_from_pid_and_ratio(251, 253);
    assert_eq!(result, PokemonGender::Female);
  }
}
