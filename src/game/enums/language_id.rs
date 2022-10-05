#[derive(Debug, Clone, Default)]
pub enum LanguageID {
    /// Undefined Language ID, usually indicative of a value not being set.
    /// Obs.: Generation 5 Japanese In-game Trades happen to not have their Language value set, and express Language=0
    #[default]
    Hacked = 0,

    /// Japanese (日本語)
    Japanese = 1,

    /// English (US/UK/AU)
    English = 2,

    /// French (Français)
    French = 3,

    /// Italian (Italiano)
    Italian = 4,

    /// German (Deutsch)
    German = 5,

    /// Unused Language ID
    /// Obs.: Was reserved for Korean in Generation 3 but never utilized
    UnusedKoreanGen3 = 6,

    /// Spanish (Español)
    Spanish = 7,

    /// Korean (한국어)
    Korean = 8,

    /// Chinese Simplified (简体中文)
    ChineseSimplified = 9,

    /// Chinese Traditional (繁體中文)
    ChineseTraditional = 10,
}

impl LanguageID {
    pub fn to_gamecube_id(&self) -> u8 {
        match self {
            LanguageID::Hacked => 0,
            LanguageID::Japanese => 1,
            LanguageID::English => 2,
            LanguageID::German => 3,
            LanguageID::French => 4,
            LanguageID::Italian => 5,
            LanguageID::Spanish => 6,
            LanguageID::UnusedKoreanGen3 => 7,
            _ => 2,
        }
    }

    pub fn from_gamecube_id(id: u8) -> Self {
        match id {
            0 => LanguageID::Hacked,
            1 => LanguageID::Japanese,
            2 => LanguageID::English,
            3 => LanguageID::German,
            4 => LanguageID::French,
            5 => LanguageID::Italian,
            6 => LanguageID::Spanish,
            7 => LanguageID::UnusedKoreanGen3,
            _ => LanguageID::English,
        }
    }
}
