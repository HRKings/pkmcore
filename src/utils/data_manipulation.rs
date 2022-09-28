use std::array::TryFromSliceError;

pub trait SliceUtils {
    fn write_into(&mut self, data: &[u8], offset: usize);

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8];

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

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
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

    fn get_offset(&self, offset: usize, byte_quantity: usize) -> &[u8] {
        &self[offset..offset + byte_quantity]
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
