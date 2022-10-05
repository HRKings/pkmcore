pub mod generation_1_or_2;
pub mod generation_3;

#[derive(Debug, PartialEq, Eq)]
pub enum StringConverterOption {
  /// Does not do any operation on the buffer.
  None,

  /// Zeroes out the entire buffer.
  ClearZero,

  /// Fills the entire buffer with 0x50; used by Generation 1/2 string encoding.
  Clear50,

  /// Fills the entire buffer with 0x7F; used by Generation 1/2 Stadium to space over for the next line.
  Clear7F,

  /// Fills the entire buffer with 0xFF; used by Generation 3-5 which use 0xFF/0xFFFF as their terminator.
  ClearFF,
}
