pub fn is_pokemon_present_gba(data: &[u8]) -> bool {
  // Ignore egg flag
  (data[0x13] & 0xFB) == 2
}
