use std::array::TryFromSliceError;

pub trait SliceUtils {
    fn write_into(&mut self, data: &[u8], offset: usize);

    fn get_mutable_slice(&mut self, start: usize, end: usize) -> &mut [u8];

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];
    fn get_mutable_offset(&mut self, offset: usize, byte_quantity: usize) -> &mut [u8];

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError>;
    fn get_u32_le(&self) -> Result<u32, TryFromSliceError>;

    fn get_u16_le_offset(&self, offset: usize) -> Result<u16, TryFromSliceError> {
        self.get_offset(offset, 2).get_u16_le()
    }

    fn get_u32_le_offset(&self, offset: usize) -> Result<u32, TryFromSliceError> {
        self.get_offset(offset, 4).get_u32_le()
    }
}

impl SliceUtils for [u8] {
    fn write_into(&mut self, data: &[u8], offset: usize) {
        self[offset..(data.len())].copy_from_slice(data);
    }

    fn get_mutable_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self[start..=end]
    }

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }

    fn get_mutable_offset(&mut self, offset: usize, byte_quantity: usize) -> &mut [u8] {
        &mut self[offset..offset + byte_quantity]
    }

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {
        Ok(u16::from_le_bytes(self[..2].try_into()?))
    }

    fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {
        Ok(u32::from_le_bytes(self[..4].try_into()?))
    }
}

impl SliceUtils for Vec<u8> {
    fn write_into(&mut self, data: &[u8], offset: usize) {
        self[offset..(data.len())].copy_from_slice(data);
    }

    fn get_mutable_slice(&mut self, start: usize, end: usize) -> &mut [u8] {
        &mut self[start..=end]
    }

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
    }

    fn get_mutable_offset(&mut self, offset: usize, byte_quantity: usize) -> &mut [u8] {
        &mut self[offset..offset + byte_quantity]
    }

    fn get_u16_le(&self) -> Result<u16, TryFromSliceError> {
        Ok(u16::from_le_bytes(self[..2].try_into()?))
    }

    fn get_u32_le(&self) -> Result<u32, TryFromSliceError> {
        Ok(u32::from_le_bytes(self[..4].try_into()?))
    }
}

pub trait FlagTrait {
    fn get_flag(&self, offset: usize, bit_index: u8) -> bool;
    fn set_flag(&mut self, offset: usize, bit_index: u8, value: bool);
}

impl FlagTrait for [u8] {
    fn get_flag(&self, offset: usize, mut bit_index: u8) -> bool {
        // Ensure bit access is 0-7
        bit_index &= 7;

        self[offset] >> bit_index != 0
    }

    fn set_flag(&mut self, offset: usize, mut bit_index: u8, value: bool) {
        // Ensure bit access is 0-7
        bit_index &= 7;

        let current_value = self[offset] & !(1 << bit_index);
        let new_value = current_value | ((value as u8) << bit_index);

        self[offset] = new_value;
    }
}

impl FlagTrait for Vec<u8> {
    fn get_flag(&self, offset: usize, mut bit_index: u8) -> bool {
        // Ensure bit access is between 0 and 7
        bit_index &= 7;

        self[offset] >> bit_index != 0
    }

    fn set_flag(&mut self, offset: usize, mut bit_index: u8, value: bool) {
        // Ensure bit access is between 0 and 7
        bit_index &= 7;

        let current_value = self[offset] & !(1 << bit_index);
        let new_value = current_value | ((value as u8) << bit_index);

        self[offset] = new_value;
    }
}

pub const fn concat_array<T, const FINAL_SIZE: usize>(
    collection: &[&[T]],
    fill: T,
) -> [T; FINAL_SIZE]
where
    T: Sized + Copy,
{
    let mut result: [T; FINAL_SIZE] = [fill; FINAL_SIZE];

    let mut index = 0;
    let mut i = 0;
    while i < collection.len() {
        let mut j = 0;
        let inner = collection[i];

        while j < inner.len() {
            result[index] = inner[j];

            j += 1;
            index += 1;
        }

        i += 1;
    }

    result
}

#[cfg(test)]
mod tests {
    use crate::utils::data_manipulation::concat_array;

    #[test]
    fn concat_two_arrays() {
        const ARRAY_1: [u16; 2] = [1, 2];
        const ARRAY_2: [u16; 2] = [3, 4];

        const RESULT: [u16; 4] = concat_array(&[&ARRAY_1, &ARRAY_2], 0);
        assert_eq!(RESULT, [1, 2, 3, 4]);
    }

    #[test]
    fn concat_four_arrays() {
        const ARRAY_1: [u16; 2] = [1, 2];
        const ARRAY_2: [u16; 2] = [3, 4];
        const ARRAY_3: [u16; 2] = [5, 6];
        const ARRAY_4: [u16; 2] = [7, 8];

        const RESULT: [u16; 8] = concat_array(&[&ARRAY_1, &ARRAY_2, &ARRAY_3, &ARRAY_4], 0);
        assert_eq!(RESULT, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn fuck() {
        let mut array_1: Vec<u8> = vec![0, 1, 2, 3];
        let array_2 = &mut array_1[1..=2];

        array_2[0] = 9;
        array_2[1] = 9;

        assert_eq!(array_1, [0, 9, 9, 3]);
    }
}
