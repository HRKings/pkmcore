use crate::utils::data_manipulation::SliceUtils;

/// Calculates the 32bit checksum over an input byte array. Used in GBA save files.
///
/// # Arguments
/// `data` - Input byte array
/// `initial` - Initial value for checksum
pub fn checksum_32bit(data: &[u8], initial: u32) -> u16 {
    let mut result = initial;
    // for (int i = 0; i < data.Length; i += 4)
    (0..(data.len())).step_by(4).for_each(|i| {
      result += data.get_u32_le_offset(i).unwrap();
    });

    (result + result.checked_shr(16).unwrap_or(0)) as u16
}

#[cfg(test)]
mod tests {
    #[test]
    fn fuck() {
        let a = u32::MAX;
        let b = a as u16;

        assert_eq!(b, u16::MAX);
    }
}
