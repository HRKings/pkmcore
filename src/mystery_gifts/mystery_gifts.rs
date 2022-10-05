#[derive(Debug, Default)]
pub struct MysteryGift {}

#[derive(Debug, Default)]
pub struct MysteryGiftAlbum {
    pub seed: Option<u32>,
    pub gifts: Vec<MysteryGift>,
    pub flags: Vec<bool>,
}
