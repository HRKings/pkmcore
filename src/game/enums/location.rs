pub enum Locations {
  LinkTrade4 = 2002,
  LinkTrade5 = 30003,
  LinkTrade6 = 30002,

  Daycare4 = 2000,
  Daycare5 = 60002,
  Daycare8b = 60010,

  LinkTrade2NPC = 126,
  LinkTrade3NPC = 254,
  LinkTrade4NPC = 2001,
  LinkTrade5NPC = 30002,
  LinkTrade6NPC = 30001,

  Breeder5 = 60003,
  Breeder6 = 60004,

  PokeWalker4 = 233,
  Ranger4 = 3001,
  Faraway4 = 3002,

  /// Goldenrod City in Crystal
  HatchLocation_GoldenrodCity_Crystal = 16,

  /// Route 117 in Ruby/Sapphire/Emerald
  HatchLocation_Route117_RubySapphireEmerald = 32,

  /// Four Island in FireRed/LeafGreen
  HatchLocation_FourIsland_FireRedLeafGreen = 146,

  /// Solaceon Town in Diamond/Pearl/Platinum
  HatchLocation_SolaceonTown_DiamondPearlPlatinum = 4,

  /// Route 34 in HeartGold/SoulSilver
  HatchLocation_Route34_HeartGold_SoulSilver = 182,

  /// Skyarrow Bridge in Black/White/Black2/White2
  HatchLocation_SkyarrowBridge_BlackWhite_Black2White2 = 64,

  /// Route 7 in X/Y
  HatchLocation_Route7_XY = 38,

  /// Battle Resort in OmegaRuby/AlphaSapphire
  HatchLocation_BattleResort_OmegaRubyAlphaSapphire = 318,

  /// Paniola Ranch in Sun/Moon/UltraSun/UltraMoon
  HatchLocation_PaniolaRanch_SunMoon_UltraSunUltraMoon = 78,

  /// Route 5 in Sword/Shield
  HatchLocation_Route5_SwordShield = 40,

  /// Solaceon Town in BrilliantDiamond/ShiningPearl
  HatchLocation_SolaceonTown_BrilliantDiamondShiningPearl = 446,

  /// Generation 1 -> Generation 7 Transfer Location (Kanto)
  Transfer_Generation1_Generation7 = 30013,

  /// Generation 2 -> Generation 7 Transfer Location (Johto)
  Transfer_Generation2_Generation7 = 30017,

  /// Generation 3 -> Generation 4 Transfer Location (Pal Park)
  Transfer_PalPark_Generation3_Generation4 = 0x37,

  /// Generation 4 -> Generation 5 Transfer Location (Poké Transporter)
  Transfer_PokeTransporter_Generation4_Generation5 = 30001,

  /// Generation 4 -> Generation 5 Transfer Location (Crown Celebi - Event not activated in Gen 5)
  Transfer_CelebiUnused_Generation4_Generation5 = 30010,

  /// Generation 4 -> Generation 5 Transfer Location (Crown Celebi - Event activated in Gen 5)
  Transfer_CelebiUsed_Generation4_Generation5 = 30011,

  /// Generation 4 -> Generation 5 Transfer Location (Crown Beast - Event not activated in Gen 5)
  Transfer_CrownUnused_Generation4_Generation5 = 30012,

  /// Generation 4 -> Generation 5 Transfer Location (Crown Beast - Event activated in Gen 5)
  Transfer_CrownUsed_Generation4_Generation5 = 30013,

  /// Generation 6 Gift from Pokémon Link
  LinkGift_Generation6 = 30011,

  /// Generation 7 Poké Pelago
  PokePelago_Generation7 = 30016,

  /// Generation 7 Transfer from GO to Pokémon LGP/E's GO Park
  Transfer_Go_LetsGoEeveeLetsGoPikachu = 50,

  /// Generation 8 Transfer from GO to Pokémon HOME
  Transfer_Go_Home = 30012,

  /// Generation 8 Gift from Pokémon HOME
  Gift_Home_Generation8 = 30018,

  BugCatchingContest_Generation4 = 207,

  /// SP traded to (SW)SH
  HOME_ShiningPearl_SwordShield = 59998,
  /// BD traded to SW(SH)
  HOME_BrilliantDiamond_SwordShield = 59999,
  /// PLA traded to SW(SH)
  HOME_LegendsArceus_SwordShield = 60000,
  /// -2 = 8bNone-1..
  HOME_SWSHBDSPEgg = 65534,
  Default8bNone = 65535,
}
