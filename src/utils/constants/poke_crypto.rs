use crate::utils::data_manipulation::SliceUtils;

pub const SIZE_GENERATION_1_INTERNATIONAL_LIST: usize = 69;
pub const SIZE_GENERATION_1_JAPANESE_LIST: usize = 59;
pub const SIZE_GENERATION_1_PARTY: usize = 44;
pub const SIZE_GENERATION_1_STORED: usize = 33;

pub const SIZE_GENERATION_3_COLOSSEUM_STORED: usize = 312;
pub const SIZE_GENERATION_3_XD_STORED: usize = 196;
pub const SIZE_GENERATION_3_PARTY: u16 = 100;
pub const SIZE_GENERATION_3_STORED: u16 = 80;
pub const SIZE_GENERATION_3_HEADER: usize = 32;
pub const SIZE_GENERATION_3_BLOCK: usize = 12;


/// Positions for shuffling.
const BLOCK_SHUFFLE_POSITION: [u8; 128] =
[
    0, 1, 2, 3,
    0, 1, 3, 2,
    0, 2, 1, 3,
    0, 3, 1, 2,
    0, 2, 3, 1,
    0, 3, 2, 1,
    1, 0, 2, 3,
    1, 0, 3, 2,
    2, 0, 1, 3,
    3, 0, 1, 2,
    2, 0, 3, 1,
    3, 0, 2, 1,
    1, 2, 0, 3,
    1, 3, 0, 2,
    2, 1, 0, 3,
    3, 1, 0, 2,
    2, 3, 0, 1,
    3, 2, 0, 1,
    1, 2, 3, 0,
    1, 3, 2, 0,
    2, 1, 3, 0,
    3, 1, 2, 0,
    2, 3, 1, 0,
    3, 2, 1, 0,

    // duplicates of 0-7 to eliminate modulus
    0, 1, 2, 3,
    0, 1, 3, 2,
    0, 2, 1, 3,
    0, 3, 1, 2,
    0, 2, 3, 1,
    0, 3, 2, 1,
    1, 0, 2, 3,
    1, 0, 3, 2,
];

/// Shuffles an 80 byte format Generation 3 Pokémon byte array.
///
/// # Arguments
/// * `data` - Un-shuffled data
/// * `shuffle_value` - Block order shuffle value
fn shuffle_generation3_array(data: &[u8], shuffle_value: usize) -> Vec<u8> {
  let mut shuffled_data = data.to_vec();
  let index = shuffle_value * 4;

  (0..4).for_each(|block| {
    let offset = BLOCK_SHUFFLE_POSITION[index + block] as usize;

    let source_block = data.get_offset(SIZE_GENERATION_3_HEADER + (SIZE_GENERATION_3_BLOCK * offset), SIZE_GENERATION_3_BLOCK);
    let destination_block = shuffled_data.get_mutable_offset(SIZE_GENERATION_3_HEADER + (SIZE_GENERATION_3_BLOCK * block), SIZE_GENERATION_3_BLOCK);

    destination_block.write_into(source_block, 0);
  });

  shuffled_data
}

/// Decrypts an 80 byte format Generation 3 Pokémon byte array.
pub fn decrypt_generation3_array(data: &[u8]) -> Vec<u8> {
  let pid = data.get_u32_le().unwrap();
  let oid = data.get_u32_le_offset(4).unwrap();

  let seed = pid ^ oid;

  let mut data_clone = data.to_vec();
  let to_decrypt = &mut data_clone[SIZE_GENERATION_3_HEADER..SIZE_GENERATION_3_STORED as usize];

  (0..(data.len())).for_each(|i| {
    let block = to_decrypt.get_mutable_offset(i, 4);

    let chunk = block.get_u32_le().unwrap();
    let update = chunk ^ seed;

    block.write_into(&update.to_le_bytes(), 0);
  });

  shuffle_generation3_array(&data_clone, (pid as usize) % 24)
}

/// Gets the checksum of a Generation 3 byte array.
///
/// # Arguments
/// `data` - Decrypted Pokémon data.
pub fn get_generatation3_checksum(data: &[u8]) -> u16 {
    let mut checksum = 0;
    let span = &data[0x20..(SIZE_GENERATION_3_STORED as usize)];

    // for (int i = 0; i < span.Length; i += 2)
    (0..(span.len())).step_by(2).for_each(|i| {
      checksum += span.get_u16_le_offset(i).unwrap();
    });

    checksum
}

/// Decrypts the input data into a new array if it is encrypted, and updates the reference.
///
/// # Remarks
/// Generation 3 Format encryption check which verifies the checksum
pub fn decrypt_generatation3_array_if_encrypted(mut data: Vec<u8>)
{
    let checksum = get_generatation3_checksum(&data);

    if checksum != data.get_u16_le_offset(0x1C).unwrap() {
      let temp = decrypt_generation3_array(&data);
      data = temp;
    }
}
