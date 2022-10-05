use std::mem;

use crate::utils::data_manipulation::SliceUtils;

#[derive(Debug, Default)]
pub struct MysteryDataBase {
    pub data: Vec<u8>,
    pub checksum: u32,
}

impl MysteryDataBase {
    pub fn new(data: Vec<u8>) -> Self {
        let checksum = data.get_u32_le().unwrap();
        Self { data, checksum }
    }

    pub fn generate_checksum(&self) -> u32 {
        let mut sum: u32 = 0;
        (4..(self.data.len())).for_each(|i| {
            sum += self.data[i] as u32;
        });

        sum
    }

    pub fn is_checksum_valid(&self) -> bool {
        self.generate_checksum() == self.checksum
    }

    pub fn set_checksum(&mut self) {
        self.checksum = self.generate_checksum()
    }
}

#[derive(Debug, Default)]
pub struct MysteryEvent {
    pub base: MysteryDataBase,

    pub magic: u8,
    pub map_group: u8,
    pub map_number: u8,
    pub object_id: u8
}

impl MysteryEvent {
    pub const SIZE: usize = mem::size_of::<u32>() + 1000; // Total of 0x3EC

    pub fn new(data: &[u8]) -> Self {
        assert_eq!(data.len(), Self::SIZE);

        Self {
            base: MysteryDataBase ::new(data.to_vec()),
            magic: data[4],
            map_group: data[5],
            map_number: data[6],
            object_id: data[7],
        }
    }

    pub fn write_to_data(&mut self) {
        self.base.data[4] = self.magic;
        self.base.data[5] = self.map_group;
        self.base.data[6] = self.map_number;
        self.base.data[7] = self.object_id;
    }
}
