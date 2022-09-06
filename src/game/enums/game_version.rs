#[derive(Debug, Clone)]
pub enum GameVersion {
    Invalid = -2,
    Any = -1,
    Unknown = 0,

    // region: Stored within Pokemon Data

    // region: Generation 3
    Sapphire = 1,
    Ruby = 2,
    Emerald = 3,

    FireRed = 4,
    LeafGreen = 5,

    ColosseumXD = 15,
    // endregion

    // region: Generation 4
    Diamond = 10,
    Pearl = 11,
    Platinum = 12,
    HeartGold = 7,
    SoulSilver = 8,
    // endregion

    // region: Generation 5
    White = 20,
    Black = 21,
    White2 = 22,
    Black2 = 23,
    // endregion

    // region: Generation 6
    X = 24,
    Y = 25,

    AlphaSapphire = 26,
    OmegaRuby = 27,
    // endregion

    // region: Generation 7
    Sun = 30,
    Moon = 31,
    UltraSun = 32,
    UltraMoon = 33,

    /// For GO -> Let's Go/Home Transfers
    Go = 34,

    // Virtual Console 3DS Generation 1
    VirtualRed = 35,

    /// Green for Japan | Blue for International
    VirtualGreenJPBlueInt = 36,

    /// Blue for Japan
    VirtualBlueJP = 37,

    VirtualYellow = 38,
    // endregion

    // region: Virtual Console 3DS Generation 2
    VirtualGold = 39,
    VirtualSilver = 40,
    VirtualCrystal = 41,
    // endregion

    // region: Generation 8/Switch
    LetsGoPikachu = 42,
    LetsGoEevee = 43,

    Sword = 44,
    Shield = 45,

    Home = 46,

    LegendsArceus = 47,

    BrilliantDiamond = 48,
    ShiningPearl = 49,
    // endregion

    // endregion

    // region: The following values are not actually stored values in pk* data

    // These values are assigned within PKHeX as properties for various logic branching.
    Stadium,
    Stadium2,
    StadiumJapanese,

    // region: Grouping
    GroupRedBlueGreen,
    GroupRedBlueYellow,

    GroupGoldSilver,
    GroupGoldSilverCrystal,

    GroupRubySapphire,
    GroupRubySapphireEmerald,

    GroupFireRedLeafGreen,

    GroupBoxRubySapphire,

    /// Also used to mark Colosseum-only origin data as this game shares a version ID with XD
    GroupColosseum,
    /// Also used to mark XD-only origin data as this game shares a version ID with Colosseum
    GroupXD,

    GroupDiamondPearl,
    GroupDiamondPearlPlatinum,

    GroupHeartGoldSoulSilver,

    GroupBattleRevolution,

    GroupBlackWhite,
    GroupBlack2White2,

    GroupXY,

    GroupOmegaRubyAlphaSapphire,
    GroupOmegaRubyAlphaSapphireDemo,

    GroupSunMoon,
    GroupUltraSunUltraMoon,

    GroupLetsGo,

    GroupSwordShield,

    GroupBrilliantDiamondShiningPearl,
    // endregion

    // region: Grouping by Generation
    CollectionGeneration1,
    CollectionGeneration2,
    CollectionGeneration3,
    CollectionGeneration4,
    CollectionGeneration5,
    CollectionGeneration6,
    /// Generation 7 games on the 3DS
    CollectionGeneration7,
    /// Generation 7 games on the Switch
    CollectionGeneration7Switch,
    CollectionGeneration8,
    // endregion

    // endregion
}

impl GameVersion {
    /// Colosseum/XD analogues used instead of the main-series values.
    pub fn to_colosseumxd_id(&self) -> u8 {
        match self {
            GameVersion::FireRed => 1,
            GameVersion::LeafGreen => 2,
            GameVersion::Sapphire => 8,
            GameVersion::Ruby => 9,
            GameVersion::Emerald => 10,
            GameVersion::ColosseumXD => 11,
            _ => 0,
        }
    }

    /// GameVersion analogues used by Colosseum/XD instead of the main-series values.
    pub fn from_colosseumxd_id(item: u8) -> Self {
        match item {
            1 => GameVersion::FireRed,
            2 => GameVersion::LeafGreen,
            8 => GameVersion::Sapphire,
            9 => GameVersion::Ruby,
            10 => GameVersion::Emerald,
            11 => GameVersion::ColosseumXD,
            _ => GameVersion::Unknown,
        }
    }
}
