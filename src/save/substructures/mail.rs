pub const SIZE_GENERATION_3: usize = 0x24;

pub struct MailDetailBase {
  data: Vec<u8>,
  offset: usize,
}

pub trait MailDetailTrait {

}

pub struct Generation3Mail {
  pub base: MailDetailBase,
}

impl Generation3Mail {
  pub fn new(data: &[u8], offset: usize, is_japanese: bool) -> Self {
    Self {
        base: todo!(),
    }
  }
}

impl MailDetailTrait for Generation3Mail {

}
