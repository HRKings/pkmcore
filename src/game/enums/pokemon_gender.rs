#[derive(Debug, Clone, PartialEq)]
pub enum PokemonGender {
	Male = 0,
	Female = 1,
	Genderless = 2,
}

impl From<u8> for PokemonGender {
    fn from(item: u8) -> Self {
        match item {
			2 => PokemonGender::Genderless,
			1 => PokemonGender::Female,
			_ => PokemonGender::Male
		}
    }
}
