use std::collections::HashSet;

use lazy_static::lazy_static;

use crate::utils::data_manipulation::concat_array;

// region: Constants
pub const MAX_SPECIES_INDEX: usize = 412;
pub const MAX_SPECIES_ID: u16 = 386;
pub const MAX_MOVE_ID: u32 = 354;
pub const MAX_ITEM_ID: u16 = 374;
pub const MAX_ITEM_ID_COLO: u16 = 547;
pub const MAX_ITEM_ID_XD: u16 = 593;
pub const MAX_ABILITY_ID: u16 = 77;
pub const MAX_POKEBALL_ID: u16 = 0xC;
/// GroupColosseumXD
pub const MAX_GAME_ID: u16 = 15;

lazy_static! {
    /// `064` - is an unused location for Meteor Falls
    ///
    /// `084` - is Inside of a truck, no possible pokemon can be hatched there
    ///
    /// `071` - is Mirage island, cannot be obtained as the player is technically still on Route 130's map.
    ///
    /// `075` - is an unused location for Fiery Path
    ///
    /// `077` - is an unused location for Jagged Pass
    pub static ref VALID_METLOCATION_RUBYSAPPHIRE: HashSet<u8> = HashSet::from(
    #[allow(clippy::zero_prefixed_literal)]
    [
        000, 001, 002, 003, 004, 005, 006, 007, 008, 009,
        010, 011, 012, 013, 014, 015, 016, 017, 018, 019,
        020, 021, 022, 023, 024, 025, 026, 027, 028, 029,
        030, 031, 032, 033, 034, 035, 036, 037, 038, 039,
        040, 041, 042, 043, 044, 045, 046, 047, 048, 049,
        050, 051, 052, 053, 054, 055, 056, 057, 058, 059,
        060, 061, 062, 063,      065, 066, 067, 068, 069,
        070, 072, 073,      074,      076,      078, 079,
        080, 081, 082, 083, 085, 086, 087,
    ]);

    /// `155 - 158` - Sevii Isle 6-9 Unused
    ///
    /// `171 - 173` - Sevii Isle 22-24 Unused
    pub static ref VALID_METLOCATION_FIRERED_LEAFGREEN: HashSet<u8> = HashSet::from(
    #[allow(clippy::zero_prefixed_literal)]
    [
                                           087, 088, 089,
        090, 091, 092, 093, 094, 095, 096, 097, 098, 099,
        100, 101, 102, 103, 104, 105, 106, 107, 108, 109,
        110, 111, 112, 113, 114, 115, 116, 117, 118, 119,
        120, 121, 122, 123, 124, 125, 126, 127, 128, 129,
        130, 131, 132, 133, 134, 135, 136, 137, 138, 139,
        140, 141, 142, 143, 144, 145, 146, 147, 148, 149,
        150, 151, 152, 153, 154,                     159,
        160, 161, 162, 163, 164, 165, 166, 167, 168, 169,
        170,                174, 175, 176, 177, 178, 179,
        180, 181, 182, 183, 184, 185, 186, 187, 188, 189,
        190, 191, 192, 193, 194, 195, 196,
    ]);

    pub static ref VALID_METLOCATION_EMERALD: HashSet<u8> = {
        let mut hashset = VALID_METLOCATION_RUBYSAPPHIRE.clone();
        hashset.extend([
                                          196, 197, 198, 199,
            200, 201, 202, 203, 204, 205, 206, 207, 208, 209,
            210, 211, 212,
        ].into_iter());

        hashset
    };
}
// endregion

// region: Ruby/Sapphire/Emerald/FireRed/LeafGreen
pub const POUCH_ITEMS_RUBYSAPPHIRE: [u16; 139] =
[
    13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 63, 64, 65, 66, 67, 68, 69, 70, 71, 73, 74, 75, 76, 77, 78, 79, 80, 81, 83, 84, 85, 86, 93, 94, 95, 96, 97, 98, 103, 104, 106, 107, 108, 109, 110, 111, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192, 193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211, 212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 254, 255, 256, 257, 258,
];

pub const POUCH_KEYITEMS_RUBYSAPPHIRE: [u16; 29] =
[
    259, 260, 261, 262, 263, 264, 265, 266, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288,
];

pub const POUCH_TM_RUBYSAPPHIRE: [u16; 50] =
[
    289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314, 315, 316, 317, 318, 319, 320, 321, 322, 323, 324, 325, 326, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338,
];

pub const POUCH_HM_RUBYSAPPHIRE: [u16; 8] =
[
    339, 340, 341, 342, 343, 344, 345, 346,
];

pub const POUCH_BERRIES_RUBYSAPPHIRE: [u16; 43] =
[
    133, 134, 135, 136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154, 155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173, 174, 175,
];

pub const POUCH_BALL_RUBYSAPPHIRE: [u16; 12] =
[
    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12,
];


pub const POUCH_KEY_FIREREDLEAFGREEN: [u16; 55] = concat_array(&[ &POUCH_KEYITEMS_RUBYSAPPHIRE,
    &[349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374]
],0);

pub const POUCH_KEY_EMERALD: [u16; 57] = concat_array(&[ &POUCH_KEY_FIREREDLEAFGREEN,
        &[375, 376]
], 0);

pub const POUCH_TMH_RUBYSAPPHIRE: [u16; 58] = concat_array(&[
    &POUCH_TM_RUBYSAPPHIRE, &POUCH_HM_RUBYSAPPHIRE
], 0);

pub const HELD_ITEMS_RUBYSAPPHIRE: [u16; 244] = concat_array(&[
    &POUCH_ITEMS_RUBYSAPPHIRE, &POUCH_BALL_RUBYSAPPHIRE, &POUCH_TM_RUBYSAPPHIRE, &POUCH_BERRIES_RUBYSAPPHIRE
], 0);

// endregion
