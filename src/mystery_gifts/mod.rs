pub struct MysteryGift {

}

pub struct MysteryGiftAlbum {
  seed: Option<u32>,
  gifts: Vec<MysteryGift>,
  flags: Vec<bool>,
}
