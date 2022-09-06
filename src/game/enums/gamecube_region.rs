#[derive(Debug, Clone)]
pub enum GCRegion {
    NoRegion = 0,
    /// NTSC-J
    NTSCJapan = 1,
    /// NTSC-U
    NTSCNorthAmerica = 2,
    PAL = 3,
}
