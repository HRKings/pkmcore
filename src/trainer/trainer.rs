use crate::game::enums::generation::Generation;

#[derive(Debug, Clone)]
pub struct TrainerInfo {
    pub generation: Generation,

    pub gender: u8,

    pub public_id: u16,
    pub secret_id: u16,
    pub name: String,

    /// Generation 7+ Public ID
    pub new_id: u32,
    /// Generation 7+ Secret ID
    pub new_sid: u32,
}

impl Default for TrainerInfo {
    fn default() -> Self {
        Self {
            generation: Generation::None,
            gender: 0,
            public_id: 0,
            secret_id: 0,
            name: Default::default(),
            new_id: 0,
            new_sid: 0,
        }
    }
}

impl TrainerInfo {
    /// Trainer public ID for Generation 7+
    pub fn trainer_public_id_new(&self) -> u16 {
        (((self.public_id as u32) | ((self.secret_id as u32) << 16)) % 1_000_000) as u16
    }

    /// Trainer secret ID for Generation 7+
    pub fn trainer_secret_id_new(&self) -> u16 {
        (((self.public_id as u32) | ((self.secret_id as u32) << 16)) / 1_000_000) as u16
    }

    fn set_id_new(&mut self, sid_new: u16, tid_new: u16) {
        let new_id = (((sid_new as u32) * 1_000_000) + ((tid_new as u32) % 1_000_000)) as u32;
        self.public_id = new_id as u16;
        self.secret_id = (new_id >> 16) as u16;
    }

    /// Trainer public ID for Generation 7+
    pub fn set_trainer_public_id_new(&mut self, tid: u16) {
        self.set_id_new(tid, self.trainer_secret_id_new())
    }

    /// Trainer secret ID for Generation 7+
    pub fn set_trainer_secret_id_new(&mut self, sid: u16) {
        self.set_id_new(self.trainer_public_id_new(), sid)
    }

    pub fn trainer_public_id_to_display(&self) -> u32 {
        if self.generation >= Generation::G7 {
            self.new_id
        } else {
            self.public_id.into()
        }
    }

    pub fn trainer_secret_id_to_display(&self) -> u32 {
        if self.generation >= Generation::G7 {
            self.new_sid
        } else {
            self.secret_id.into()
        }
    }
}
