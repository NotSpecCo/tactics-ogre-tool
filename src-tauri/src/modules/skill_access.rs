use crate::modules::types::*;

fn opt_skill_names() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Fists".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Daggers".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Swords(H)".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Swords(H)".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Axes".to_string(),
        },
        FieldOption {
            value: 6,
            label: "None".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Spears".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Hammers".to_string(),
        },
        FieldOption {
            value: 9,
            label: "None".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Katana (H)".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Katana (H)".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Cudgels".to_string(),
        },
        FieldOption {
            value: 13,
            label: "None".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Whips".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Spellbooks".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Instruments".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Blowguns".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Bows".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Crossbows".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Fusils".to_string(),
        },
        FieldOption {
            value: 21,
            label: "None".to_string(),
        },
        FieldOption {
            value: 22,
            label: "None".to_string(),
        },
        FieldOption {
            value: 23,
            label: "None".to_string(),
        },
        FieldOption {
            value: 24,
            label: "None".to_string(),
        },
        FieldOption {
            value: 25,
            label: "None".to_string(),
        },
        FieldOption {
            value: 26,
            label: "None".to_string(),
        },
        FieldOption {
            value: 27,
            label: "None".to_string(),
        },
        FieldOption {
            value: 28,
            label: "None".to_string(),
        },
        FieldOption {
            value: 29,
            label: "None".to_string(),
        },
        FieldOption {
            value: 30,
            label: "None".to_string(),
        },
        FieldOption {
            value: 31,
            label: "None".to_string(),
        },
        FieldOption {
            value: 32,
            label: "None".to_string(),
        },
        FieldOption {
            value: 33,
            label: "None".to_string(),
        },
        FieldOption {
            value: 34,
            label: "None".to_string(),
        },
        FieldOption {
            value: 35,
            label: "None".to_string(),
        },
        FieldOption {
            value: 36,
            label: "None".to_string(),
        },
        FieldOption {
            value: 37,
            label: "None".to_string(),
        },
        FieldOption {
            value: 38,
            label: "None".to_string(),
        },
        FieldOption {
            value: 39,
            label: "None".to_string(),
        },
        FieldOption {
            value: 40,
            label: "None".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Overpower".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Resist Petrify".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Resist Stun".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Resist Sleep".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Resist Charm".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Resist Poison".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Resist Silence".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Resist Slow".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Resist Bind".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Resist Shackle".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Resist Stop".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Resist Leaden".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Resist Fear".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Resist Venom".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Resist Curse".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 57,
            label: "None".to_string(),
        },
        FieldOption {
            value: 58,
            label: "None".to_string(),
        },
        FieldOption {
            value: 59,
            label: "None".to_string(),
        },
        FieldOption {
            value: 60,
            label: "None".to_string(),
        },
        FieldOption {
            value: 61,
            label: "None".to_string(),
        },
        FieldOption {
            value: 62,
            label: "None".to_string(),
        },
        FieldOption {
            value: 63,
            label: "None".to_string(),
        },
        FieldOption {
            value: 64,
            label: "None".to_string(),
        },
        FieldOption {
            value: 65,
            label: "None".to_string(),
        },
        FieldOption {
            value: 66,
            label: "None".to_string(),
        },
        FieldOption {
            value: 67,
            label: "None".to_string(),
        },
        FieldOption {
            value: 68,
            label: "None".to_string(),
        },
        FieldOption {
            value: 69,
            label: "None".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 71,
            label: "None".to_string(),
        },
        FieldOption {
            value: 72,
            label: "None".to_string(),
        },
        FieldOption {
            value: 73,
            label: "None".to_string(),
        },
        FieldOption {
            value: 74,
            label: "None".to_string(),
        },
        FieldOption {
            value: 75,
            label: "None".to_string(),
        },
        FieldOption {
            value: 76,
            label: "None".to_string(),
        },
        FieldOption {
            value: 77,
            label: "None".to_string(),
        },
        FieldOption {
            value: 78,
            label: "None".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Rampart Aura I".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Rampart Aura II".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Rampart Aura III".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Rampart Aura IV".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Rampart Aura V".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Counterattack I".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Counterattack II".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Counterattack III".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Counterattack IV".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 91,
            label: "None".to_string(),
        },
        FieldOption {
            value: 92,
            label: "None".to_string(),
        },
        FieldOption {
            value: 93,
            label: "None".to_string(),
        },
        FieldOption {
            value: 94,
            label: "None".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 96,
            label: "None".to_string(),
        },
        FieldOption {
            value: 97,
            label: "None".to_string(),
        },
        FieldOption {
            value: 98,
            label: "None".to_string(),
        },
        FieldOption {
            value: 99,
            label: "None".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 101,
            label: "None".to_string(),
        },
        FieldOption {
            value: 102,
            label: "None".to_string(),
        },
        FieldOption {
            value: 103,
            label: "None".to_string(),
        },
        FieldOption {
            value: 104,
            label: "None".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 106,
            label: "None".to_string(),
        },
        FieldOption {
            value: 107,
            label: "None".to_string(),
        },
        FieldOption {
            value: 108,
            label: "None".to_string(),
        },
        FieldOption {
            value: 109,
            label: "None".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 111,
            label: "None".to_string(),
        },
        FieldOption {
            value: 112,
            label: "None".to_string(),
        },
        FieldOption {
            value: 113,
            label: "None".to_string(),
        },
        FieldOption {
            value: 114,
            label: "None".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 116,
            label: "None".to_string(),
        },
        FieldOption {
            value: 117,
            label: "None".to_string(),
        },
        FieldOption {
            value: 118,
            label: "None".to_string(),
        },
        FieldOption {
            value: 119,
            label: "None".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 121,
            label: "None".to_string(),
        },
        FieldOption {
            value: 122,
            label: "None".to_string(),
        },
        FieldOption {
            value: 123,
            label: "None".to_string(),
        },
        FieldOption {
            value: 124,
            label: "None".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 126,
            label: "None".to_string(),
        },
        FieldOption {
            value: 127,
            label: "None".to_string(),
        },
        FieldOption {
            value: 128,
            label: "None".to_string(),
        },
        FieldOption {
            value: 129,
            label: "None".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 131,
            label: "None".to_string(),
        },
        FieldOption {
            value: 132,
            label: "None".to_string(),
        },
        FieldOption {
            value: 133,
            label: "None".to_string(),
        },
        FieldOption {
            value: 134,
            label: "None".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 136,
            label: "None".to_string(),
        },
        FieldOption {
            value: 137,
            label: "None".to_string(),
        },
        FieldOption {
            value: 138,
            label: "None".to_string(),
        },
        FieldOption {
            value: 139,
            label: "None".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 141,
            label: "None".to_string(),
        },
        FieldOption {
            value: 142,
            label: "None".to_string(),
        },
        FieldOption {
            value: 143,
            label: "None".to_string(),
        },
        FieldOption {
            value: 144,
            label: "None".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Constitution I".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Constitution II".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Constitution III".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Constitution IV".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Insight I".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Insight II".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Insight III".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Insight IV".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 156,
            label: "None".to_string(),
        },
        FieldOption {
            value: 157,
            label: "None".to_string(),
        },
        FieldOption {
            value: 158,
            label: "None".to_string(),
        },
        FieldOption {
            value: 159,
            label: "None".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 161,
            label: "None".to_string(),
        },
        FieldOption {
            value: 162,
            label: "None".to_string(),
        },
        FieldOption {
            value: 163,
            label: "None".to_string(),
        },
        FieldOption {
            value: 164,
            label: "None".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Stoneproof".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Stunproof".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Sleepproof".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Charmproof".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Foolproof".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Poisonproof".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Silenceproof".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Slowproof".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Stopproof".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Leadproof".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Fearproof".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Venomproof".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Curseproof".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Deathproof".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Swiftfoot I".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Swiftfoot II".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Jump I".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Jump II".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Wade I".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Wade II".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Sanctuary I".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Sanctuary II".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Invisibility".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Steadfast".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Double Attack".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Trajectory".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Siege".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Invincible".to_string(),
        },
        FieldOption {
            value: 196,
            label: "None".to_string(),
        },
        FieldOption {
            value: 197,
            label: "None".to_string(),
        },
        FieldOption {
            value: 198,
            label: "None".to_string(),
        },
        FieldOption {
            value: 199,
            label: "None".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Eagle Eye II".to_string(),
        },
        FieldOption {
            value: 201,
            label: "None".to_string(),
        },
        FieldOption {
            value: 202,
            label: "None".to_string(),
        },
        FieldOption {
            value: 203,
            label: "None".to_string(),
        },
        FieldOption {
            value: 204,
            label: "None".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 206,
            label: "None".to_string(),
        },
        FieldOption {
            value: 207,
            label: "None".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 209,
            label: "None".to_string(),
        },
        FieldOption {
            value: 210,
            label: "None".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 212,
            label: "None".to_string(),
        },
        FieldOption {
            value: 213,
            label: "None".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 215,
            label: "None".to_string(),
        },
        FieldOption {
            value: 216,
            label: "None".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 218,
            label: "None".to_string(),
        },
        FieldOption {
            value: 219,
            label: "None".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 221,
            label: "None".to_string(),
        },
        FieldOption {
            value: 222,
            label: "None".to_string(),
        },
        FieldOption {
            value: 223,
            label: "None".to_string(),
        },
        FieldOption {
            value: 224,
            label: "None".to_string(),
        },
        FieldOption {
            value: 225,
            label: "None".to_string(),
        },
        FieldOption {
            value: 226,
            label: "None".to_string(),
        },
        FieldOption {
            value: 227,
            label: "None".to_string(),
        },
        FieldOption {
            value: 228,
            label: "None".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Pumpkin Lure".to_string(),
        },
        FieldOption {
            value: 230,
            label: "None".to_string(),
        },
        FieldOption {
            value: 231,
            label: "None".to_string(),
        },
        FieldOption {
            value: 232,
            label: "None".to_string(),
        },
        FieldOption {
            value: 233,
            label: "None".to_string(),
        },
        FieldOption {
            value: 234,
            label: "None".to_string(),
        },
        FieldOption {
            value: 235,
            label: "None".to_string(),
        },
        FieldOption {
            value: 236,
            label: "None".to_string(),
        },
        FieldOption {
            value: 237,
            label: "None".to_string(),
        },
        FieldOption {
            value: 238,
            label: "None".to_string(),
        },
        FieldOption {
            value: 239,
            label: "None".to_string(),
        },
        FieldOption {
            value: 240,
            label: "None".to_string(),
        },
        FieldOption {
            value: 241,
            label: "None".to_string(),
        },
        FieldOption {
            value: 242,
            label: "None".to_string(),
        },
        FieldOption {
            value: 243,
            label: "None".to_string(),
        },
        FieldOption {
            value: 244,
            label: "None".to_string(),
        },
        FieldOption {
            value: 245,
            label: "None".to_string(),
        },
        FieldOption {
            value: 246,
            label: "None".to_string(),
        },
        FieldOption {
            value: 247,
            label: "None".to_string(),
        },
        FieldOption {
            value: 248,
            label: "None".to_string(),
        },
        FieldOption {
            value: 249,
            label: "None".to_string(),
        },
        FieldOption {
            value: 250,
            label: "None".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Recruit".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Subdue".to_string(),
        },
        FieldOption {
            value: 253,
            label: "Coax".to_string(),
        },
        FieldOption {
            value: 254,
            label: "Tame".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Seraph's Pact".to_string(),
        },
        FieldOption {
            value: 256,
            label: "Demon's Pact".to_string(),
        },
        FieldOption {
            value: 257,
            label: "Fey Pact".to_string(),
        },
        FieldOption {
            value: 258,
            label: "Master Undead".to_string(),
        },
        FieldOption {
            value: 259,
            label: "Control Golem".to_string(),
        },
        FieldOption {
            value: 260,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 261,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 262,
            label: "Gordian Key".to_string(),
        },
        FieldOption {
            value: 263,
            label: "None".to_string(),
        },
        FieldOption {
            value: 264,
            label: "Aqua Veil".to_string(),
        },
        FieldOption {
            value: 265,
            label: "Preempt".to_string(),
        },
        FieldOption {
            value: 266,
            label: "Animate Dead".to_string(),
        },
        FieldOption {
            value: 267,
            label: "Absolution".to_string(),
        },
        FieldOption {
            value: 268,
            label: "Eagle Eye".to_string(),
        },
        FieldOption {
            value: 269,
            label: "Evil's Bane".to_string(),
        },
        FieldOption {
            value: 270,
            label: "Mighty Strike".to_string(),
        },
        FieldOption {
            value: 271,
            label: "Evade".to_string(),
        },
        FieldOption {
            value: 272,
            label: "None".to_string(),
        },
        FieldOption {
            value: 273,
            label: "Evanescece".to_string(),
        },
        FieldOption {
            value: 274,
            label: "None".to_string(),
        },
        FieldOption {
            value: 275,
            label: "None".to_string(),
        },
        FieldOption {
            value: 276,
            label: "None".to_string(),
        },
        FieldOption {
            value: 277,
            label: "Evil Deeds".to_string(),
        },
        FieldOption {
            value: 278,
            label: "Magic Time!".to_string(),
        },
        FieldOption {
            value: 279,
            label: "Guardian Force".to_string(),
        },
        FieldOption {
            value: 280,
            label: "El Colas Winds".to_string(),
        },
        FieldOption {
            value: 281,
            label: "Intercession".to_string(),
        },
        FieldOption {
            value: 282,
            label: "Course Correction".to_string(),
        },
        FieldOption {
            value: 283,
            label: "None".to_string(),
        },
        FieldOption {
            value: 284,
            label: "Golem's Bane".to_string(),
        },
        FieldOption {
            value: 285,
            label: "Conserve MP".to_string(),
        },
        FieldOption {
            value: 286,
            label: "Conserve RT".to_string(),
        },
        FieldOption {
            value: 287,
            label: "Consecrate Edge".to_string(),
        },
        FieldOption {
            value: 288,
            label: "None".to_string(),
        },
        FieldOption {
            value: 289,
            label: "None".to_string(),
        },
        FieldOption {
            value: 290,
            label: "Sublime Sacrifice".to_string(),
        },
        FieldOption {
            value: 291,
            label: "Salvation".to_string(),
        },
        FieldOption {
            value: 292,
            label: "Sanctuary Shadow".to_string(),
        },
        FieldOption {
            value: 293,
            label: "Jack-o'-Lantern".to_string(),
        },
        FieldOption {
            value: 294,
            label: "Shadowbreak".to_string(),
        },
        FieldOption {
            value: 295,
            label: "None".to_string(),
        },
        FieldOption {
            value: 296,
            label: "Stardust Grace".to_string(),
        },
        FieldOption {
            value: 297,
            label: "Speedstar".to_string(),
        },
        FieldOption {
            value: 298,
            label: "Concentration (Ninjutsu)".to_string(),
        },
        FieldOption {
            value: 299,
            label: "Holy Water".to_string(),
        },
        FieldOption {
            value: 300,
            label: "Mother's Mercy".to_string(),
        },
        FieldOption {
            value: 301,
            label: "Mother's Blessing".to_string(),
        },
        FieldOption {
            value: 302,
            label: "Nature's Whisper".to_string(),
        },
        FieldOption {
            value: 303,
            label: "Nature's Touch".to_string(),
        },
        FieldOption {
            value: 304,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 305,
            label: "Echoing Voice".to_string(),
        },
        FieldOption {
            value: 306,
            label: "Sharpshoot".to_string(),
        },
        FieldOption {
            value: 307,
            label: "Double Impact".to_string(),
        },
        FieldOption {
            value: 308,
            label: "Double Shot".to_string(),
        },
        FieldOption {
            value: 309,
            label: "Resounding Voice".to_string(),
        },
        FieldOption {
            value: 310,
            label: "Fearful Impact".to_string(),
        },
        FieldOption {
            value: 311,
            label: "None".to_string(),
        },
        FieldOption {
            value: 312,
            label: "Dragonslayer".to_string(),
        },
        FieldOption {
            value: 313,
            label: "Dragon's Eye".to_string(),
        },
        FieldOption {
            value: 314,
            label: "None".to_string(),
        },
        FieldOption {
            value: 315,
            label: "Dragon's Wound".to_string(),
        },
        FieldOption {
            value: 316,
            label: "Dragonsbane".to_string(),
        },
        FieldOption {
            value: 317,
            label: "Tremendous Shot".to_string(),
        },
        FieldOption {
            value: 318,
            label: "Berserk".to_string(),
        },
        FieldOption {
            value: 319,
            label: "Back Attack".to_string(),
        },
        FieldOption {
            value: 320,
            label: "Paralysis Blade".to_string(),
        },
        FieldOption {
            value: 321,
            label: "None".to_string(),
        },
        FieldOption {
            value: 322,
            label: "None".to_string(),
        },
        FieldOption {
            value: 323,
            label: "Beastslayer".to_string(),
        },
        FieldOption {
            value: 324,
            label: "Beastbane".to_string(),
        },
        FieldOption {
            value: 325,
            label: "Vigorous Attack".to_string(),
        },
        FieldOption {
            value: 326,
            label: "None".to_string(),
        },
        FieldOption {
            value: 327,
            label: "None".to_string(),
        },
        FieldOption {
            value: 328,
            label: "None".to_string(),
        },
        FieldOption {
            value: 329,
            label: "Consecrate Dead".to_string(),
        },
        FieldOption {
            value: 330,
            label: "Condemn".to_string(),
        },
        FieldOption {
            value: 331,
            label: "Sanguine Assault".to_string(),
        },
        FieldOption {
            value: 332,
            label: "Break Curse".to_string(),
        },
        FieldOption {
            value: 333,
            label: "Broaden Force".to_string(),
        },
        FieldOption {
            value: 334,
            label: "Velocity Shift".to_string(),
        },
        FieldOption {
            value: 335,
            label: "Mighty Impact".to_string(),
        },
        FieldOption {
            value: 336,
            label: "None".to_string(),
        },
        FieldOption {
            value: 337,
            label: "None".to_string(),
        },
        FieldOption {
            value: 338,
            label: "Rampart Shadow".to_string(),
        },
        FieldOption {
            value: 339,
            label: "Risk Management".to_string(),
        },
        FieldOption {
            value: 340,
            label: "Reflection".to_string(),
        },
        FieldOption {
            value: 341,
            label: "None".to_string(),
        },
        FieldOption {
            value: 342,
            label: "None".to_string(),
        },
        FieldOption {
            value: 343,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 344,
            label: "None".to_string(),
        },
        FieldOption {
            value: 345,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 346,
            label: "None".to_string(),
        },
        FieldOption {
            value: 347,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 348,
            label: "Aqua Bubble".to_string(),
        },
        FieldOption {
            value: 349,
            label: "Agonal Scream".to_string(),
        },
        FieldOption {
            value: 350,
            label: "Acid Breath".to_string(),
        },
        FieldOption {
            value: 351,
            label: "Acid Breath II".to_string(),
        },
        FieldOption {
            value: 352,
            label: "Sweaty Palms".to_string(),
        },
        FieldOption {
            value: 353,
            label: "Wind Shot".to_string(),
        },
        FieldOption {
            value: 354,
            label: "Virtuous Dance".to_string(),
        },
        FieldOption {
            value: 355,
            label: "Cruelist Cut".to_string(),
        },
        FieldOption {
            value: 356,
            label: "Granite Fist".to_string(),
        },
        FieldOption {
            value: 357,
            label: "Crystal Pumpkin".to_string(),
        },
        FieldOption {
            value: 358,
            label: "Reeking Meetballs".to_string(),
        },
        FieldOption {
            value: 359,
            label: "Coquettish Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 360,
            label: "Coquettish Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 361,
            label: "Silent Song".to_string(),
        },
        FieldOption {
            value: 362,
            label: "Pirate Breath".to_string(),
        },
        FieldOption {
            value: 363,
            label: "Requiem".to_string(),
        },
        FieldOption {
            value: 364,
            label: "Thunder Breath".to_string(),
        },
        FieldOption {
            value: 365,
            label: "Thunder Breath II".to_string(),
        },
        FieldOption {
            value: 366,
            label: "Stun Breath".to_string(),
        },
        FieldOption {
            value: 367,
            label: "Stun Breath II".to_string(),
        },
        FieldOption {
            value: 368,
            label: "Sparagmos".to_string(),
        },
        FieldOption {
            value: 369,
            label: "Sand Breath".to_string(),
        },
        FieldOption {
            value: 370,
            label: "Sand Breath II".to_string(),
        },
        FieldOption {
            value: 371,
            label: "Lingering Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 372,
            label: "Lingering Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 373,
            label: "Day of Reckoning".to_string(),
        },
        FieldOption {
            value: 374,
            label: "Divine Breath".to_string(),
        },
        FieldOption {
            value: 375,
            label: "Divine Breath II".to_string(),
        },
        FieldOption {
            value: 376,
            label: "Tail Lash".to_string(),
        },
        FieldOption {
            value: 377,
            label: "Toxic Breath".to_string(),
        },
        FieldOption {
            value: 378,
            label: "Toxic Breath II".to_string(),
        },
        FieldOption {
            value: 379,
            label: "Vortex Breath".to_string(),
        },
        FieldOption {
            value: 380,
            label: "Vortex Breath II".to_string(),
        },
        FieldOption {
            value: 381,
            label: "Numbing Hook".to_string(),
        },
        FieldOption {
            value: 382,
            label: "Stinky Feet".to_string(),
        },
        FieldOption {
            value: 383,
            label: "Pumpkin Strike".to_string(),
        },
        FieldOption {
            value: 384,
            label: "Pumpkin Pie".to_string(),
        },
        FieldOption {
            value: 385,
            label: "Pumpkin Bomb".to_string(),
        },
        FieldOption {
            value: 386,
            label: "Flame Breath".to_string(),
        },
        FieldOption {
            value: 387,
            label: "Flame Breath II".to_string(),
        },
        FieldOption {
            value: 388,
            label: "Blood Syphon".to_string(),
        },
        FieldOption {
            value: 389,
            label: "Selfless Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 390,
            label: "Selfless Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 391,
            label: "Blue Spiral".to_string(),
        },
        FieldOption {
            value: 392,
            label: "Frost Breath".to_string(),
        },
        FieldOption {
            value: 393,
            label: "Frost Breath II".to_string(),
        },
        FieldOption {
            value: 394,
            label: "Heaven's Tear".to_string(),
        },
        FieldOption {
            value: 395,
            label: "Petro Breath".to_string(),
        },
        FieldOption {
            value: 396,
            label: "Petro Breath II".to_string(),
        },
        FieldOption {
            value: 397,
            label: "Poison Rain".to_string(),
        },
        FieldOption {
            value: 398,
            label: "Poison Breath".to_string(),
        },
        FieldOption {
            value: 399,
            label: "Poison Breath II".to_string(),
        },
        FieldOption {
            value: 400,
            label: "Poignant Melody".to_string(),
        },
        FieldOption {
            value: 401,
            label: "Maelstrom".to_string(),
        },
        FieldOption {
            value: 402,
            label: "Stirring Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 403,
            label: "Stirring Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 404,
            label: "Raven Eye".to_string(),
        },
        FieldOption {
            value: 405,
            label: "Corpse Breath".to_string(),
        },
        FieldOption {
            value: 406,
            label: "Corpse Breath II".to_string(),
        },
        FieldOption {
            value: 407,
            label: "Evil Eye".to_string(),
        },
        FieldOption {
            value: 408,
            label: "Celestial Song".to_string(),
        },
        FieldOption {
            value: 409,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 410,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 411,
            label: "Exhalito (Void)".to_string(),
        },
        FieldOption {
            value: 412,
            label: "Procella (Wind)".to_string(),
        },
        FieldOption {
            value: 413,
            label: "Pondus (Earth)".to_string(),
        },
        FieldOption {
            value: 414,
            label: "Tornitrus (Lightning)".to_string(),
        },
        FieldOption {
            value: 415,
            label: "Coctura (Water)".to_string(),
        },
        FieldOption {
            value: 416,
            label: "Diruptio (Fire)".to_string(),
        },
        FieldOption {
            value: 417,
            label: "Congelatio (Ice)".to_string(),
        },
        FieldOption {
            value: 418,
            label: "Radius (Light)".to_string(),
        },
        FieldOption {
            value: 419,
            label: "Umbra (Dark)".to_string(),
        },
        FieldOption {
            value: 420,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 421,
            label: "Quickdraw".to_string(),
        },
        FieldOption {
            value: 422,
            label: "Dummy".to_string(),
        },
        FieldOption {
            value: 423,
            label: "Parry I".to_string(),
        },
        FieldOption {
            value: 424,
            label: "Parry II".to_string(),
        },
        FieldOption {
            value: 425,
            label: "Parry III".to_string(),
        },
        FieldOption {
            value: 426,
            label: "Parry IV".to_string(),
        },
        FieldOption {
            value: 427,
            label: "Empower Golem I".to_string(),
        },
        FieldOption {
            value: 428,
            label: "Empower Golem II".to_string(),
        },
        FieldOption {
            value: 429,
            label: "Empower Golem III".to_string(),
        },
        FieldOption {
            value: 430,
            label: "Empower Golem IV".to_string(),
        },
        FieldOption {
            value: 431,
            label: "Empower Dragon I".to_string(),
        },
        FieldOption {
            value: 432,
            label: "Empower Dragon II".to_string(),
        },
        FieldOption {
            value: 433,
            label: "Empower Dragon III".to_string(),
        },
        FieldOption {
            value: 434,
            label: "Empower Dragon IV".to_string(),
        },
        FieldOption {
            value: 435,
            label: "Empower Beast".to_string(),
        },
        FieldOption {
            value: 436,
            label: "Empower Beast II".to_string(),
        },
        FieldOption {
            value: 437,
            label: "Empower Beast III".to_string(),
        },
        FieldOption {
            value: 438,
            label: "Empower Beast IV".to_string(),
        },
        FieldOption {
            value: 439,
            label: "Lucky Star I".to_string(),
        },
        FieldOption {
            value: 440,
            label: "Lucky Star II".to_string(),
        },
        FieldOption {
            value: 441,
            label: "Lucky Star III".to_string(),
        },
        FieldOption {
            value: 442,
            label: "Lucky Star IV".to_string(),
        },
        FieldOption {
            value: 443,
            label: "First Aid I (Stone)".to_string(),
        },
        FieldOption {
            value: 444,
            label: "First Aid II (Stone)".to_string(),
        },
        FieldOption {
            value: 445,
            label: "First Aid III (Stone)".to_string(),
        },
        FieldOption {
            value: 446,
            label: "First Aid IV (Stone)".to_string(),
        },
        FieldOption {
            value: 447,
            label: "First Aid I (Health)".to_string(),
        },
        FieldOption {
            value: 448,
            label: "First Aid II (Health)".to_string(),
        },
        FieldOption {
            value: 449,
            label: "First Aid III (Health)".to_string(),
        },
        FieldOption {
            value: 450,
            label: "First Aid IV (Health)".to_string(),
        },
        FieldOption {
            value: 451,
            label: "Meditate I".to_string(),
        },
        FieldOption {
            value: 452,
            label: "Meditate II".to_string(),
        },
        FieldOption {
            value: 453,
            label: "Meditate III".to_string(),
        },
        FieldOption {
            value: 454,
            label: "Meditate IV".to_string(),
        },
        FieldOption {
            value: 455,
            label: "Iron Maiden I".to_string(),
        },
        FieldOption {
            value: 456,
            label: "Iron Maiden II".to_string(),
        },
        FieldOption {
            value: 457,
            label: "Iron Maiden III".to_string(),
        },
        FieldOption {
            value: 458,
            label: "Iron Maiden IV".to_string(),
        },
        FieldOption {
            value: 459,
            label: "Featherstep I".to_string(),
        },
        FieldOption {
            value: 460,
            label: "Featherstep II".to_string(),
        },
        FieldOption {
            value: 461,
            label: "Featherstep III".to_string(),
        },
        FieldOption {
            value: 462,
            label: "Featherstep IV".to_string(),
        },
        FieldOption {
            value: 463,
            label: "Howl I".to_string(),
        },
        FieldOption {
            value: 464,
            label: "Howl II".to_string(),
        },
        FieldOption {
            value: 465,
            label: "Howl III".to_string(),
        },
        FieldOption {
            value: 466,
            label: "Howl IV".to_string(),
        },
        FieldOption {
            value: 467,
            label: "Huapango Winds I".to_string(),
        },
        FieldOption {
            value: 468,
            label: "Huapango Winds II".to_string(),
        },
        FieldOption {
            value: 469,
            label: "Huapango Winds III".to_string(),
        },
        FieldOption {
            value: 470,
            label: "Huapango Winds IV".to_string(),
        },
        FieldOption {
            value: 471,
            label: "Glare I".to_string(),
        },
        FieldOption {
            value: 472,
            label: "Glare II".to_string(),
        },
        FieldOption {
            value: 473,
            label: "Glare III".to_string(),
        },
        FieldOption {
            value: 474,
            label: "Glare IV".to_string(),
        },
        FieldOption {
            value: 475,
            label: "Threaten I".to_string(),
        },
        FieldOption {
            value: 476,
            label: "Threaten II".to_string(),
        },
        FieldOption {
            value: 477,
            label: "Threaten III".to_string(),
        },
        FieldOption {
            value: 478,
            label: "Threaten IV".to_string(),
        },
        FieldOption {
            value: 479,
            label: "Check I".to_string(),
        },
        FieldOption {
            value: 480,
            label: "Check II".to_string(),
        },
        FieldOption {
            value: 481,
            label: "Check III".to_string(),
        },
        FieldOption {
            value: 482,
            label: "Check IV".to_string(),
        },
        FieldOption {
            value: 483,
            label: "Black Mucus I".to_string(),
        },
        FieldOption {
            value: 484,
            label: "Black Mucus II".to_string(),
        },
        FieldOption {
            value: 485,
            label: "Black Mucus III".to_string(),
        },
        FieldOption {
            value: 486,
            label: "Black Mucus IV".to_string(),
        },
        FieldOption {
            value: 487,
            label: "Bloody Gag I".to_string(),
        },
        FieldOption {
            value: 488,
            label: "Bloody Gag II".to_string(),
        },
        FieldOption {
            value: 489,
            label: "Bloody Gag III".to_string(),
        },
        FieldOption {
            value: 490,
            label: "Bloody Gag IV".to_string(),
        },
        FieldOption {
            value: 491,
            label: "Witch's Smile I".to_string(),
        },
        FieldOption {
            value: 492,
            label: "Witch's Smile II".to_string(),
        },
        FieldOption {
            value: 493,
            label: "Witch's Smile III".to_string(),
        },
        FieldOption {
            value: 494,
            label: "Witch's Smile IV".to_string(),
        },
        FieldOption {
            value: 495,
            label: "Irresistable Beauty I".to_string(),
        },
        FieldOption {
            value: 496,
            label: "Irresistable Beauty II".to_string(),
        },
        FieldOption {
            value: 497,
            label: "Irresistable Beauty III".to_string(),
        },
        FieldOption {
            value: 498,
            label: "Irresistable Beauty IV".to_string(),
        },
        FieldOption {
            value: 499,
            label: "Lament of the Dead I".to_string(),
        },
        FieldOption {
            value: 500,
            label: "Lament of the Dead II".to_string(),
        },
        FieldOption {
            value: 501,
            label: "Lament of the Dead III".to_string(),
        },
        FieldOption {
            value: 502,
            label: "Lament of the Dead IV".to_string(),
        },
        FieldOption {
            value: 503,
            label: "Rapier Glance I".to_string(),
        },
        FieldOption {
            value: 504,
            label: "Rapier Glance II".to_string(),
        },
        FieldOption {
            value: 505,
            label: "Rapier Glance III".to_string(),
        },
        FieldOption {
            value: 506,
            label: "Rapier Glance IV".to_string(),
        },
        FieldOption {
            value: 507,
            label: "Intimidate I".to_string(),
        },
        FieldOption {
            value: 508,
            label: "Intimidate II".to_string(),
        },
        FieldOption {
            value: 509,
            label: "Intimidate III".to_string(),
        },
        FieldOption {
            value: 510,
            label: "Intimidate IV".to_string(),
        },
        FieldOption {
            value: 511,
            label: "Aerial Resonance I".to_string(),
        },
        FieldOption {
            value: 512,
            label: "Aerial Resonance II".to_string(),
        },
        FieldOption {
            value: 513,
            label: "Aerial Resonance III".to_string(),
        },
        FieldOption {
            value: 514,
            label: "Aerial Resonance IV".to_string(),
        },
        FieldOption {
            value: 515,
            label: "Telluric Resonance I".to_string(),
        },
        FieldOption {
            value: 516,
            label: "Telluric Resonance II".to_string(),
        },
        FieldOption {
            value: 517,
            label: "Telluric Resonance III".to_string(),
        },
        FieldOption {
            value: 518,
            label: "Telluric Resonance IV".to_string(),
        },
        FieldOption {
            value: 519,
            label: "Charged Resonance I".to_string(),
        },
        FieldOption {
            value: 520,
            label: "Charged Resonance II".to_string(),
        },
        FieldOption {
            value: 521,
            label: "Charged Resonance III".to_string(),
        },
        FieldOption {
            value: 522,
            label: "Charged Resonance IV".to_string(),
        },
        FieldOption {
            value: 523,
            label: "Aquatic Resonance I".to_string(),
        },
        FieldOption {
            value: 524,
            label: "Aquatic Resonance II".to_string(),
        },
        FieldOption {
            value: 525,
            label: "Aquatic Resonance III".to_string(),
        },
        FieldOption {
            value: 526,
            label: "Aquatic Resonance IV".to_string(),
        },
        FieldOption {
            value: 527,
            label: "Blazing Resonance I".to_string(),
        },
        FieldOption {
            value: 528,
            label: "Blazing Resonance II".to_string(),
        },
        FieldOption {
            value: 529,
            label: "Blazing Resonance III".to_string(),
        },
        FieldOption {
            value: 530,
            label: "Blazing Resonance IV".to_string(),
        },
        FieldOption {
            value: 531,
            label: "Icy Resonance I".to_string(),
        },
        FieldOption {
            value: 532,
            label: "Icy Resonance II".to_string(),
        },
        FieldOption {
            value: 533,
            label: "Icy Resonance III".to_string(),
        },
        FieldOption {
            value: 534,
            label: "Icy Resonance IV".to_string(),
        },
        FieldOption {
            value: 535,
            label: "Luminous Resonance I".to_string(),
        },
        FieldOption {
            value: 536,
            label: "Luminous Resonance II".to_string(),
        },
        FieldOption {
            value: 537,
            label: "Luminous Resonance III".to_string(),
        },
        FieldOption {
            value: 538,
            label: "Luminous Resonance IV".to_string(),
        },
        FieldOption {
            value: 539,
            label: "Shadow Resonance I".to_string(),
        },
        FieldOption {
            value: 540,
            label: "Shadow Resonance II".to_string(),
        },
        FieldOption {
            value: 541,
            label: "Shadow Resonance III".to_string(),
        },
        FieldOption {
            value: 542,
            label: "Shadow Resonance IV".to_string(),
        },
        FieldOption {
            value: 543,
            label: "None".to_string(),
        },
        FieldOption {
            value: 544,
            label: "None".to_string(),
        },
        FieldOption {
            value: 545,
            label: "None".to_string(),
        },
        FieldOption {
            value: 546,
            label: "None".to_string(),
        },
        FieldOption {
            value: 547,
            label: "None".to_string(),
        },
        FieldOption {
            value: 548,
            label: "None".to_string(),
        },
        FieldOption {
            value: 549,
            label: "None".to_string(),
        },
        FieldOption {
            value: 550,
            label: "None".to_string(),
        },
        FieldOption {
            value: 551,
            label: "None".to_string(),
        },
        FieldOption {
            value: 552,
            label: "None".to_string(),
        },
        FieldOption {
            value: 553,
            label: "None".to_string(),
        },
        FieldOption {
            value: 554,
            label: "None".to_string(),
        },
        FieldOption {
            value: 555,
            label: "None".to_string(),
        },
        FieldOption {
            value: 556,
            label: "None".to_string(),
        },
        FieldOption {
            value: 557,
            label: "None".to_string(),
        },
        FieldOption {
            value: 558,
            label: "None".to_string(),
        },
        FieldOption {
            value: 559,
            label: "None".to_string(),
        },
        FieldOption {
            value: 560,
            label: "None".to_string(),
        },
        FieldOption {
            value: 561,
            label: "None".to_string(),
        },
        FieldOption {
            value: 562,
            label: "None".to_string(),
        },
        FieldOption {
            value: 563,
            label: "None".to_string(),
        },
        FieldOption {
            value: 564,
            label: "None".to_string(),
        },
        FieldOption {
            value: 565,
            label: "None".to_string(),
        },
        FieldOption {
            value: 566,
            label: "None".to_string(),
        },
        FieldOption {
            value: 567,
            label: "Gordian Lock I".to_string(),
        },
        FieldOption {
            value: 568,
            label: "Gordian Lock II".to_string(),
        },
        FieldOption {
            value: 569,
            label: "Gordian Lock III".to_string(),
        },
        FieldOption {
            value: 570,
            label: "Gordian Lock IV".to_string(),
        },
        FieldOption {
            value: 571,
            label: "Engulf I".to_string(),
        },
        FieldOption {
            value: 572,
            label: "Engulf II".to_string(),
        },
        FieldOption {
            value: 573,
            label: "None".to_string(),
        },
        FieldOption {
            value: 574,
            label: "None".to_string(),
        },
        FieldOption {
            value: 575,
            label: "Princess Whim I".to_string(),
        },
        FieldOption {
            value: 576,
            label: "Princess Whim II".to_string(),
        },
        FieldOption {
            value: 577,
            label: "Princess Whim III".to_string(),
        },
        FieldOption {
            value: 578,
            label: "Princess Whim IV".to_string(),
        },
        FieldOption {
            value: 579,
            label: "None".to_string(),
        },
        FieldOption {
            value: 580,
            label: "None".to_string(),
        },
        FieldOption {
            value: 581,
            label: "None".to_string(),
        },
        FieldOption {
            value: 582,
            label: "None".to_string(),
        },
        FieldOption {
            value: 583,
            label: "None".to_string(),
        },
        FieldOption {
            value: 584,
            label: "None".to_string(),
        },
        FieldOption {
            value: 585,
            label: "None".to_string(),
        },
        FieldOption {
            value: 586,
            label: "None".to_string(),
        },
        FieldOption {
            value: 587,
            label: "None".to_string(),
        },
        FieldOption {
            value: 588,
            label: "None".to_string(),
        },
        FieldOption {
            value: 589,
            label: "None".to_string(),
        },
        FieldOption {
            value: 590,
            label: "None".to_string(),
        },
        FieldOption {
            value: 591,
            label: "None".to_string(),
        },
        FieldOption {
            value: 592,
            label: "None".to_string(),
        },
        FieldOption {
            value: 593,
            label: "None".to_string(),
        },
        FieldOption {
            value: 594,
            label: "None".to_string(),
        },
        FieldOption {
            value: 595,
            label: "None".to_string(),
        },
        FieldOption {
            value: 596,
            label: "None".to_string(),
        },
        FieldOption {
            value: 597,
            label: "None".to_string(),
        },
        FieldOption {
            value: 598,
            label: "None".to_string(),
        },
        FieldOption {
            value: 599,
            label: "None".to_string(),
        },
        FieldOption {
            value: 600,
            label: "None".to_string(),
        },
        FieldOption {
            value: 601,
            label: "None".to_string(),
        },
        FieldOption {
            value: 602,
            label: "None".to_string(),
        },
        FieldOption {
            value: 603,
            label: "None".to_string(),
        },
        FieldOption {
            value: 604,
            label: "None".to_string(),
        },
        FieldOption {
            value: 605,
            label: "None".to_string(),
        },
        FieldOption {
            value: 606,
            label: "None".to_string(),
        },
        FieldOption {
            value: 607,
            label: "None".to_string(),
        },
        FieldOption {
            value: 608,
            label: "None".to_string(),
        },
        FieldOption {
            value: 609,
            label: "None".to_string(),
        },
        FieldOption {
            value: 610,
            label: "None".to_string(),
        },
        FieldOption {
            value: 611,
            label: "None".to_string(),
        },
        FieldOption {
            value: 612,
            label: "None".to_string(),
        },
        FieldOption {
            value: 613,
            label: "None".to_string(),
        },
        FieldOption {
            value: 614,
            label: "None".to_string(),
        },
        FieldOption {
            value: 615,
            label: "None".to_string(),
        },
        FieldOption {
            value: 616,
            label: "None".to_string(),
        },
        FieldOption {
            value: 617,
            label: "None".to_string(),
        },
        FieldOption {
            value: 618,
            label: "None".to_string(),
        },
        FieldOption {
            value: 619,
            label: "Mind's Eye I".to_string(),
        },
        FieldOption {
            value: 620,
            label: "Mind's Eye II".to_string(),
        },
        FieldOption {
            value: 621,
            label: "None".to_string(),
        },
        FieldOption {
            value: 622,
            label: "None".to_string(),
        },
        FieldOption {
            value: 623,
            label: "None".to_string(),
        },
        FieldOption {
            value: 624,
            label: "None".to_string(),
        },
        FieldOption {
            value: 625,
            label: "None".to_string(),
        },
        FieldOption {
            value: 626,
            label: "None".to_string(),
        },
        FieldOption {
            value: 627,
            label: "None".to_string(),
        },
        FieldOption {
            value: 628,
            label: "None".to_string(),
        },
        FieldOption {
            value: 629,
            label: "None".to_string(),
        },
        FieldOption {
            value: 630,
            label: "None".to_string(),
        },
        FieldOption {
            value: 631,
            label: "None".to_string(),
        },
        FieldOption {
            value: 632,
            label: "None".to_string(),
        },
        FieldOption {
            value: 633,
            label: "None".to_string(),
        },
        FieldOption {
            value: 634,
            label: "None".to_string(),
        },
        FieldOption {
            value: 635,
            label: "None".to_string(),
        },
        FieldOption {
            value: 636,
            label: "None".to_string(),
        },
        FieldOption {
            value: 637,
            label: "None".to_string(),
        },
        FieldOption {
            value: 638,
            label: "None".to_string(),
        },
        FieldOption {
            value: 639,
            label: "None".to_string(),
        },
        FieldOption {
            value: 640,
            label: "None".to_string(),
        },
        FieldOption {
            value: 641,
            label: "None".to_string(),
        },
        FieldOption {
            value: 642,
            label: "None".to_string(),
        },
        FieldOption {
            value: 643,
            label: "None".to_string(),
        },
        FieldOption {
            value: 644,
            label: "None".to_string(),
        },
        FieldOption {
            value: 645,
            label: "None".to_string(),
        },
        FieldOption {
            value: 646,
            label: "None".to_string(),
        },
        FieldOption {
            value: 647,
            label: "None".to_string(),
        },
        FieldOption {
            value: 648,
            label: "None".to_string(),
        },
        FieldOption {
            value: 649,
            label: "None".to_string(),
        },
        FieldOption {
            value: 650,
            label: "None".to_string(),
        },
        FieldOption {
            value: 651,
            label: "None".to_string(),
        },
        FieldOption {
            value: 652,
            label: "None".to_string(),
        },
        FieldOption {
            value: 653,
            label: "None".to_string(),
        },
        FieldOption {
            value: 654,
            label: "None".to_string(),
        },
        FieldOption {
            value: 655,
            label: "None".to_string(),
        },
        FieldOption {
            value: 656,
            label: "None".to_string(),
        },
        FieldOption {
            value: 657,
            label: "None".to_string(),
        },
        FieldOption {
            value: 658,
            label: "None".to_string(),
        },
        FieldOption {
            value: 659,
            label: "None".to_string(),
        },
        FieldOption {
            value: 660,
            label: "None".to_string(),
        },
        FieldOption {
            value: 661,
            label: "None".to_string(),
        },
        FieldOption {
            value: 662,
            label: "None".to_string(),
        },
        FieldOption {
            value: 663,
            label: "None".to_string(),
        },
        FieldOption {
            value: 664,
            label: "None".to_string(),
        },
        FieldOption {
            value: 665,
            label: "None".to_string(),
        },
        FieldOption {
            value: 666,
            label: "None".to_string(),
        },
        FieldOption {
            value: 667,
            label: "None".to_string(),
        },
        FieldOption {
            value: 668,
            label: "None".to_string(),
        },
        FieldOption {
            value: 669,
            label: "None".to_string(),
        },
        FieldOption {
            value: 670,
            label: "None".to_string(),
        },
        FieldOption {
            value: 671,
            label: "Dash I".to_string(),
        },
        FieldOption {
            value: 672,
            label: "Dash II".to_string(),
        },
        FieldOption {
            value: 673,
            label: "None".to_string(),
        },
        FieldOption {
            value: 674,
            label: "None".to_string(),
        },
        FieldOption {
            value: 675,
            label: "None".to_string(),
        },
        FieldOption {
            value: 676,
            label: "None".to_string(),
        },
        FieldOption {
            value: 677,
            label: "None".to_string(),
        },
        FieldOption {
            value: 678,
            label: "None".to_string(),
        },
        FieldOption {
            value: 679,
            label: "None".to_string(),
        },
        FieldOption {
            value: 680,
            label: "None".to_string(),
        },
        FieldOption {
            value: 681,
            label: "None".to_string(),
        },
        FieldOption {
            value: 682,
            label: "None".to_string(),
        },
        FieldOption {
            value: 683,
            label: "Dragon's Scale I".to_string(),
        },
        FieldOption {
            value: 684,
            label: "Dragon's Scale II".to_string(),
        },
        FieldOption {
            value: 685,
            label: "Dragon's Scale III".to_string(),
        },
        FieldOption {
            value: 686,
            label: "Dragon's Scale IV".to_string(),
        },
        FieldOption {
            value: 687,
            label: "None".to_string(),
        },
        FieldOption {
            value: 688,
            label: "None".to_string(),
        },
        FieldOption {
            value: 689,
            label: "None".to_string(),
        },
        FieldOption {
            value: 690,
            label: "None".to_string(),
        },
        FieldOption {
            value: 691,
            label: "None".to_string(),
        },
        FieldOption {
            value: 692,
            label: "None".to_string(),
        },
        FieldOption {
            value: 693,
            label: "None".to_string(),
        },
        FieldOption {
            value: 694,
            label: "None".to_string(),
        },
        FieldOption {
            value: 695,
            label: "None".to_string(),
        },
        FieldOption {
            value: 696,
            label: "None".to_string(),
        },
        FieldOption {
            value: 697,
            label: "None".to_string(),
        },
        FieldOption {
            value: 698,
            label: "None".to_string(),
        },
        FieldOption {
            value: 699,
            label: "None".to_string(),
        },
        FieldOption {
            value: 700,
            label: "None".to_string(),
        },
        FieldOption {
            value: 701,
            label: "None".to_string(),
        },
        FieldOption {
            value: 702,
            label: "None".to_string(),
        },
        FieldOption {
            value: 703,
            label: "None".to_string(),
        },
        FieldOption {
            value: 704,
            label: "None".to_string(),
        },
        FieldOption {
            value: 705,
            label: "None".to_string(),
        },
        FieldOption {
            value: 706,
            label: "None".to_string(),
        },
        FieldOption {
            value: 707,
            label: "None".to_string(),
        },
        FieldOption {
            value: 708,
            label: "None".to_string(),
        },
        FieldOption {
            value: 709,
            label: "None".to_string(),
        },
        FieldOption {
            value: 710,
            label: "None".to_string(),
        },
        FieldOption {
            value: 711,
            label: "None".to_string(),
        },
        FieldOption {
            value: 712,
            label: "None".to_string(),
        },
        FieldOption {
            value: 713,
            label: "None".to_string(),
        },
        FieldOption {
            value: 714,
            label: "None".to_string(),
        },
        FieldOption {
            value: 715,
            label: "None".to_string(),
        },
        FieldOption {
            value: 716,
            label: "None".to_string(),
        },
        FieldOption {
            value: 717,
            label: "None".to_string(),
        },
        FieldOption {
            value: 718,
            label: "None".to_string(),
        },
        FieldOption {
            value: 719,
            label: "Phalanx I".to_string(),
        },
        FieldOption {
            value: 720,
            label: "Phalanx II".to_string(),
        },
        FieldOption {
            value: 721,
            label: "Phalanx III".to_string(),
        },
        FieldOption {
            value: 722,
            label: "Phalanx IV".to_string(),
        },
        FieldOption {
            value: 723,
            label: "Steelstance I".to_string(),
        },
        FieldOption {
            value: 724,
            label: "Steelstance II".to_string(),
        },
        FieldOption {
            value: 725,
            label: "Steelstance III".to_string(),
        },
        FieldOption {
            value: 726,
            label: "Steelstance IV".to_string(),
        },
        FieldOption {
            value: 727,
            label: "None".to_string(),
        },
        FieldOption {
            value: 728,
            label: "None".to_string(),
        },
        FieldOption {
            value: 729,
            label: "None".to_string(),
        },
        FieldOption {
            value: 730,
            label: "None".to_string(),
        },
        FieldOption {
            value: 731,
            label: "None".to_string(),
        },
        FieldOption {
            value: 732,
            label: "None".to_string(),
        },
        FieldOption {
            value: 733,
            label: "None".to_string(),
        },
        FieldOption {
            value: 734,
            label: "None".to_string(),
        },
        FieldOption {
            value: 735,
            label: "None".to_string(),
        },
        FieldOption {
            value: 736,
            label: "None".to_string(),
        },
        FieldOption {
            value: 737,
            label: "None".to_string(),
        },
        FieldOption {
            value: 738,
            label: "None".to_string(),
        },
        FieldOption {
            value: 739,
            label: "None".to_string(),
        },
        FieldOption {
            value: 740,
            label: "None".to_string(),
        },
        FieldOption {
            value: 741,
            label: "None".to_string(),
        },
        FieldOption {
            value: 742,
            label: "None".to_string(),
        },
        FieldOption {
            value: 743,
            label: "None".to_string(),
        },
        FieldOption {
            value: 744,
            label: "None".to_string(),
        },
        FieldOption {
            value: 745,
            label: "None".to_string(),
        },
        FieldOption {
            value: 746,
            label: "None".to_string(),
        },
        FieldOption {
            value: 747,
            label: "Apostate I".to_string(),
        },
        FieldOption {
            value: 748,
            label: "Apostate II".to_string(),
        },
        FieldOption {
            value: 749,
            label: "None".to_string(),
        },
        FieldOption {
            value: 750,
            label: "None".to_string(),
        },
        FieldOption {
            value: 751,
            label: "None".to_string(),
        },
        FieldOption {
            value: 752,
            label: "None".to_string(),
        },
        FieldOption {
            value: 753,
            label: "None".to_string(),
        },
        FieldOption {
            value: 754,
            label: "None".to_string(),
        },
        FieldOption {
            value: 755,
            label: "None".to_string(),
        },
        FieldOption {
            value: 756,
            label: "None".to_string(),
        },
        FieldOption {
            value: 757,
            label: "None".to_string(),
        },
        FieldOption {
            value: 758,
            label: "None".to_string(),
        },
        FieldOption {
            value: 759,
            label: "Repel Dragon I".to_string(),
        },
        FieldOption {
            value: 760,
            label: "Repel Dragon II".to_string(),
        },
        FieldOption {
            value: 761,
            label: "None".to_string(),
        },
        FieldOption {
            value: 762,
            label: "None".to_string(),
        },
        FieldOption {
            value: 763,
            label: "Repel Beast I".to_string(),
        },
        FieldOption {
            value: 764,
            label: "Repel Beast II".to_string(),
        },
        FieldOption {
            value: 765,
            label: "None".to_string(),
        },
        FieldOption {
            value: 766,
            label: "None".to_string(),
        },
        FieldOption {
            value: 767,
            label: "Ivory Tower I".to_string(),
        },
        FieldOption {
            value: 768,
            label: "Ivory Tower II".to_string(),
        },
        FieldOption {
            value: 769,
            label: "None".to_string(),
        },
        FieldOption {
            value: 770,
            label: "None".to_string(),
        },
        FieldOption {
            value: 771,
            label: "Concentration (Magic)".to_string(),
        },
        FieldOption {
            value: 772,
            label: "None".to_string(),
        },
        FieldOption {
            value: 773,
            label: "None".to_string(),
        },
        FieldOption {
            value: 774,
            label: "None".to_string(),
        },
        FieldOption {
            value: 775,
            label: "Smoke Screen I".to_string(),
        },
        FieldOption {
            value: 776,
            label: "Smoke Screen II".to_string(),
        },
        FieldOption {
            value: 777,
            label: "Smoke Screen III".to_string(),
        },
        FieldOption {
            value: 778,
            label: "Smoke Screen IV".to_string(),
        },
        FieldOption {
            value: 779,
            label: "Lobber I".to_string(),
        },
        FieldOption {
            value: 780,
            label: "Lobber II".to_string(),
        },
        FieldOption {
            value: 781,
            label: "Lobber III".to_string(),
        },
        FieldOption {
            value: 782,
            label: "Lobber IV".to_string(),
        },
        FieldOption {
            value: 783,
            label: "Falling Blade".to_string(),
        },
        FieldOption {
            value: 784,
            label: "None".to_string(),
        },
        FieldOption {
            value: 785,
            label: "None".to_string(),
        },
        FieldOption {
            value: 786,
            label: "None".to_string(),
        },
        FieldOption {
            value: 787,
            label: "Pincer Attack".to_string(),
        },
        FieldOption {
            value: 788,
            label: "Pincer Attack II".to_string(),
        },
        FieldOption {
            value: 789,
            label: "Pincer Attack III".to_string(),
        },
        FieldOption {
            value: 790,
            label: "Pincer Attack IV".to_string(),
        },
        FieldOption {
            value: 791,
            label: "None".to_string(),
        },
        FieldOption {
            value: 792,
            label: "None".to_string(),
        },
        FieldOption {
            value: 793,
            label: "None".to_string(),
        },
        FieldOption {
            value: 794,
            label: "None".to_string(),
        },
        FieldOption {
            value: 795,
            label: "None".to_string(),
        },
        FieldOption {
            value: 796,
            label: "None".to_string(),
        },
        FieldOption {
            value: 797,
            label: "None".to_string(),
        },
        FieldOption {
            value: 798,
            label: "None".to_string(),
        },
        FieldOption {
            value: 799,
            label: "None".to_string(),
        },
    ]
}

fn opt_skill_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Fists".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Daggers".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Swords(1H)".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Swords(2H)".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Axes".to_string(),
        },
        FieldOption {
            value: 6,
            label: "None".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Spears".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Hammers".to_string(),
        },
        FieldOption {
            value: 9,
            label: "None".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Katana (1H)".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Katana (2H)".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Cudgels".to_string(),
        },
        FieldOption {
            value: 13,
            label: "None".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Whips".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Spellbooks".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Instruments".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Blowguns".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Bows".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Crossbows".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Fusils".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Thrown".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Anatomy".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Teratology".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Herpetology".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Draconology".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Sacrology".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Daemonology".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Aurology".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Thanatology".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Golemy".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Augment Air".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Augment Earth".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Augment Lightning".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Augment Water".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Augment Fire".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Augment Ice".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Augment Light".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Augment Darkness".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Parry".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Deflect".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Overpower".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Resist Petrify".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Resist Stun".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Resist Sleep".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Resist Charm".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Resist Poison".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Resist Silence".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Resist Slow".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Resist Bind".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Resist Shackle".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Resist Stop".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Resist Leaden".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Resist Fear".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Resist Venom".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Resist Curse".to_string(),
        },
        FieldOption {
            value: 56,
            label: "None".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Air Magic".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Earth Magic".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Lightning Magic".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Water Magic".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Fire Magic".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Ice Magic".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Divine Magic".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Dark Magic".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Draconic Magic".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Necromancy".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Ninjutsu".to_string(),
        },
        FieldOption {
            value: 68,
            label: "War Dances".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Songs".to_string(),
        },
        FieldOption {
            value: 70,
            label: "None".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Attenuate Air".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Attenuate Earth".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Attenuate Lightning".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Attenuate Water".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Attenuate Fire".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Attenuate Ice".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Attenuate Light".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Attenuate Dark".to_string(),
        },
        FieldOption {
            value: 79,
            label: "None".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Rampart Aura I".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Rampart Aura II".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Rampart Aura III".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Rampart Aura IV".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Rampart Aura V".to_string(),
        },
        FieldOption {
            value: 85,
            label: "None".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Counterattack I".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Counterattack II".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Counterattack III".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Counterattack IV".to_string(),
        },
        FieldOption {
            value: 90,
            label: "None".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Knockback I".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Knockback II".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Knockback III".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Knockback IV".to_string(),
        },
        FieldOption {
            value: 95,
            label: "None".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Strengthen I".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Strengthen II".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Strengthen III".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Strengthen IV".to_string(),
        },
        FieldOption {
            value: 100,
            label: "None".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Fortify I".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Fortify II".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Fortify III".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Fortify IV".to_string(),
        },
        FieldOption {
            value: 105,
            label: "None".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Spellcraft I".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Spellcraft II".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Spellcraft III".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Spellcraft IV".to_string(),
        },
        FieldOption {
            value: 110,
            label: "None".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Resistance I".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Resistance II".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Resistance III".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Resistance IV".to_string(),
        },
        FieldOption {
            value: 115,
            label: "None".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Truestrike I".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Truestrike II".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Truestrike III".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Truestrike IV".to_string(),
        },
        FieldOption {
            value: 120,
            label: "None".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Trueflight I".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Trueflight II".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Trueflight III".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Trueflight IV".to_string(),
        },
        FieldOption {
            value: 125,
            label: "None".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Spellstrike I".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Spellstrike II".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Spellstrike III".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Spellstrike IV".to_string(),
        },
        FieldOption {
            value: 130,
            label: "None".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Dodge I".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Dodge II".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Dodge III".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Dodge IV".to_string(),
        },
        FieldOption {
            value: 135,
            label: "None".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Sidestep I".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Sidestep II".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Sidestep III".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Sidestep IV".to_string(),
        },
        FieldOption {
            value: 140,
            label: "None".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Spell Ward I".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Spell Ward II".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Spell Ward III".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Spell Ward IV".to_string(),
        },
        FieldOption {
            value: 145,
            label: "None".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Constitution I".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Constitution II".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Constitution III".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Constitution IV".to_string(),
        },
        FieldOption {
            value: 150,
            label: "None".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Insight I".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Insight II".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Insight III".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Insight IV".to_string(),
        },
        FieldOption {
            value: 155,
            label: "None".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Expand Mind I".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Expand Mind II".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Expand Mind III".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Expand Mind IV".to_string(),
        },
        FieldOption {
            value: 160,
            label: "None".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Channeling I".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Channeling II".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Channeling III".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Channeling IV".to_string(),
        },
        FieldOption {
            value: 165,
            label: "None".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Stoneproof".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Stunproof".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Sleepproof".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Charmproof".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Foolproof".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Poisonproof".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Silenceproof".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Slowproof".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Stopproof".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Leadproof".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Fearproof".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Venomproof".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Curseproof".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Deathproof".to_string(),
        },
        FieldOption {
            value: 180,
            label: "None".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Swiftfoot I".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Swiftfoot II".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Jump I".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Jump II".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Wade I".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Wade II".to_string(),
        },
        FieldOption {
            value: 187,
            label: "None".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Sanctuary I".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Sanctuary II".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Invisibility".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Steadfast".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Double Attack".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Trajectory".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Siege".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Invincible".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Field Alchemy I".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Field Alchemy II".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Field Alchemy III".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Field Alchemy IV".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Eagle Eye II".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Max TP I".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Max TP II".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Max TP III".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Max TP IV".to_string(),
        },
        FieldOption {
            value: 205,
            label: "None".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Treasure Hunt I".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Treasure Hunt II".to_string(),
        },
        FieldOption {
            value: 208,
            label: "None".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Tactician I".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Tactician II".to_string(),
        },
        FieldOption {
            value: 211,
            label: "None".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Reflect Damage I".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Reflect Damage II".to_string(),
        },
        FieldOption {
            value: 214,
            label: "None".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Reflect Magic I".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Reflect Magic II".to_string(),
        },
        FieldOption {
            value: 217,
            label: "None".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Absorb MP I".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Absorb MP II".to_string(),
        },
        FieldOption {
            value: 220,
            label: "None".to_string(),
        },
        FieldOption {
            value: 221,
            label: "None".to_string(),
        },
        FieldOption {
            value: 222,
            label: "None".to_string(),
        },
        FieldOption {
            value: 223,
            label: "None".to_string(),
        },
        FieldOption {
            value: 224,
            label: "None".to_string(),
        },
        FieldOption {
            value: 225,
            label: "None".to_string(),
        },
        FieldOption {
            value: 226,
            label: "None".to_string(),
        },
        FieldOption {
            value: 227,
            label: "None".to_string(),
        },
        FieldOption {
            value: 228,
            label: "None".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Pumpkin Lure".to_string(),
        },
        FieldOption {
            value: 230,
            label: "None".to_string(),
        },
        FieldOption {
            value: 231,
            label: "None".to_string(),
        },
        FieldOption {
            value: 232,
            label: "None".to_string(),
        },
        FieldOption {
            value: 233,
            label: "None".to_string(),
        },
        FieldOption {
            value: 234,
            label: "None".to_string(),
        },
        FieldOption {
            value: 235,
            label: "None".to_string(),
        },
        FieldOption {
            value: 236,
            label: "None".to_string(),
        },
        FieldOption {
            value: 237,
            label: "None".to_string(),
        },
        FieldOption {
            value: 238,
            label: "None".to_string(),
        },
        FieldOption {
            value: 239,
            label: "None".to_string(),
        },
        FieldOption {
            value: 240,
            label: "None".to_string(),
        },
        FieldOption {
            value: 241,
            label: "None".to_string(),
        },
        FieldOption {
            value: 242,
            label: "None".to_string(),
        },
        FieldOption {
            value: 243,
            label: "None".to_string(),
        },
        FieldOption {
            value: 244,
            label: "None".to_string(),
        },
        FieldOption {
            value: 245,
            label: "None".to_string(),
        },
        FieldOption {
            value: 246,
            label: "None".to_string(),
        },
        FieldOption {
            value: 247,
            label: "None".to_string(),
        },
        FieldOption {
            value: 248,
            label: "None".to_string(),
        },
        FieldOption {
            value: 249,
            label: "None".to_string(),
        },
        FieldOption {
            value: 250,
            label: "None".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Recruit".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Subdue".to_string(),
        },
        FieldOption {
            value: 253,
            label: "Coax".to_string(),
        },
        FieldOption {
            value: 254,
            label: "Tame".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Seraph's Pact".to_string(),
        },
        FieldOption {
            value: 256,
            label: "Demon's Pact".to_string(),
        },
        FieldOption {
            value: 257,
            label: "Fey Pact".to_string(),
        },
        FieldOption {
            value: 258,
            label: "Master Undead".to_string(),
        },
        FieldOption {
            value: 259,
            label: "Control Golem".to_string(),
        },
        FieldOption {
            value: 260,
            label: "None".to_string(),
        },
        FieldOption {
            value: 261,
            label: "None".to_string(),
        },
        FieldOption {
            value: 262,
            label: "Gordian Key".to_string(),
        },
        FieldOption {
            value: 263,
            label: "None".to_string(),
        },
        FieldOption {
            value: 264,
            label: "None".to_string(),
        },
        FieldOption {
            value: 265,
            label: "None".to_string(),
        },
        FieldOption {
            value: 266,
            label: "Animate Dead".to_string(),
        },
        FieldOption {
            value: 267,
            label: "Absolution".to_string(),
        },
        FieldOption {
            value: 268,
            label: "None".to_string(),
        },
        FieldOption {
            value: 269,
            label: "None".to_string(),
        },
        FieldOption {
            value: 270,
            label: "None".to_string(),
        },
        FieldOption {
            value: 271,
            label: "None".to_string(),
        },
        FieldOption {
            value: 272,
            label: "None".to_string(),
        },
        FieldOption {
            value: 273,
            label: "Evanescece".to_string(),
        },
        FieldOption {
            value: 274,
            label: "None".to_string(),
        },
        FieldOption {
            value: 275,
            label: "None".to_string(),
        },
        FieldOption {
            value: 276,
            label: "None".to_string(),
        },
        FieldOption {
            value: 277,
            label: "Evil Deeds".to_string(),
        },
        FieldOption {
            value: 278,
            label: "None".to_string(),
        },
        FieldOption {
            value: 279,
            label: "None".to_string(),
        },
        FieldOption {
            value: 280,
            label: "None".to_string(),
        },
        FieldOption {
            value: 281,
            label: "None".to_string(),
        },
        FieldOption {
            value: 282,
            label: "None".to_string(),
        },
        FieldOption {
            value: 283,
            label: "None".to_string(),
        },
        FieldOption {
            value: 284,
            label: "None".to_string(),
        },
        FieldOption {
            value: 285,
            label: "None".to_string(),
        },
        FieldOption {
            value: 286,
            label: "None".to_string(),
        },
        FieldOption {
            value: 287,
            label: "None".to_string(),
        },
        FieldOption {
            value: 288,
            label: "None".to_string(),
        },
        FieldOption {
            value: 289,
            label: "None".to_string(),
        },
        FieldOption {
            value: 290,
            label: "Sublime Sacrifice".to_string(),
        },
        FieldOption {
            value: 291,
            label: "None".to_string(),
        },
        FieldOption {
            value: 292,
            label: "Sanctuary Shadow".to_string(),
        },
        FieldOption {
            value: 293,
            label: "Jack-o'-Lantern".to_string(),
        },
        FieldOption {
            value: 294,
            label: "Shadowbreak".to_string(),
        },
        FieldOption {
            value: 295,
            label: "None".to_string(),
        },
        FieldOption {
            value: 296,
            label: "None".to_string(),
        },
        FieldOption {
            value: 297,
            label: "None".to_string(),
        },
        FieldOption {
            value: 298,
            label: "None".to_string(),
        },
        FieldOption {
            value: 299,
            label: "Holy Water".to_string(),
        },
        FieldOption {
            value: 300,
            label: "None".to_string(),
        },
        FieldOption {
            value: 301,
            label: "None".to_string(),
        },
        FieldOption {
            value: 302,
            label: "Nature's Whisper".to_string(),
        },
        FieldOption {
            value: 303,
            label: "None".to_string(),
        },
        FieldOption {
            value: 304,
            label: "None".to_string(),
        },
        FieldOption {
            value: 305,
            label: "None".to_string(),
        },
        FieldOption {
            value: 306,
            label: "None".to_string(),
        },
        FieldOption {
            value: 307,
            label: "None".to_string(),
        },
        FieldOption {
            value: 308,
            label: "None".to_string(),
        },
        FieldOption {
            value: 309,
            label: "None".to_string(),
        },
        FieldOption {
            value: 310,
            label: "None".to_string(),
        },
        FieldOption {
            value: 311,
            label: "None".to_string(),
        },
        FieldOption {
            value: 312,
            label: "None".to_string(),
        },
        FieldOption {
            value: 313,
            label: "None".to_string(),
        },
        FieldOption {
            value: 314,
            label: "None".to_string(),
        },
        FieldOption {
            value: 315,
            label: "Dragon's Wound".to_string(),
        },
        FieldOption {
            value: 316,
            label: "None".to_string(),
        },
        FieldOption {
            value: 317,
            label: "None".to_string(),
        },
        FieldOption {
            value: 318,
            label: "None".to_string(),
        },
        FieldOption {
            value: 319,
            label: "None".to_string(),
        },
        FieldOption {
            value: 320,
            label: "None".to_string(),
        },
        FieldOption {
            value: 321,
            label: "None".to_string(),
        },
        FieldOption {
            value: 322,
            label: "None".to_string(),
        },
        FieldOption {
            value: 323,
            label: "None".to_string(),
        },
        FieldOption {
            value: 324,
            label: "None".to_string(),
        },
        FieldOption {
            value: 325,
            label: "None".to_string(),
        },
        FieldOption {
            value: 326,
            label: "None".to_string(),
        },
        FieldOption {
            value: 327,
            label: "None".to_string(),
        },
        FieldOption {
            value: 328,
            label: "None".to_string(),
        },
        FieldOption {
            value: 329,
            label: "Consecrate Dead".to_string(),
        },
        FieldOption {
            value: 330,
            label: "Condemn".to_string(),
        },
        FieldOption {
            value: 331,
            label: "None".to_string(),
        },
        FieldOption {
            value: 332,
            label: "None".to_string(),
        },
        FieldOption {
            value: 333,
            label: "None".to_string(),
        },
        FieldOption {
            value: 334,
            label: "None".to_string(),
        },
        FieldOption {
            value: 335,
            label: "None".to_string(),
        },
        FieldOption {
            value: 336,
            label: "None".to_string(),
        },
        FieldOption {
            value: 337,
            label: "None".to_string(),
        },
        FieldOption {
            value: 338,
            label: "Rampart Shadow".to_string(),
        },
        FieldOption {
            value: 339,
            label: "None".to_string(),
        },
        FieldOption {
            value: 340,
            label: "None".to_string(),
        },
        FieldOption {
            value: 341,
            label: "None".to_string(),
        },
        FieldOption {
            value: 342,
            label: "None".to_string(),
        },
        FieldOption {
            value: 343,
            label: "None".to_string(),
        },
        FieldOption {
            value: 344,
            label: "None".to_string(),
        },
        FieldOption {
            value: 345,
            label: "None".to_string(),
        },
        FieldOption {
            value: 346,
            label: "None".to_string(),
        },
        FieldOption {
            value: 347,
            label: "None".to_string(),
        },
        FieldOption {
            value: 348,
            label: "Aqua Bubble".to_string(),
        },
        FieldOption {
            value: 349,
            label: "Agonal Scream".to_string(),
        },
        FieldOption {
            value: 350,
            label: "Acid Breath".to_string(),
        },
        FieldOption {
            value: 351,
            label: "Acid Breath II".to_string(),
        },
        FieldOption {
            value: 352,
            label: "Sweaty Palms".to_string(),
        },
        FieldOption {
            value: 353,
            label: "Wind Shot".to_string(),
        },
        FieldOption {
            value: 354,
            label: "Virtuous Dance".to_string(),
        },
        FieldOption {
            value: 355,
            label: "Cruelist Cut".to_string(),
        },
        FieldOption {
            value: 356,
            label: "Granite Fist".to_string(),
        },
        FieldOption {
            value: 357,
            label: "Crystal Pumpkin".to_string(),
        },
        FieldOption {
            value: 358,
            label: "Reeking Meetballs".to_string(),
        },
        FieldOption {
            value: 359,
            label: "Coquettish Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 360,
            label: "Coquettish Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 361,
            label: "Silent Song".to_string(),
        },
        FieldOption {
            value: 362,
            label: "Pirate Breath".to_string(),
        },
        FieldOption {
            value: 363,
            label: "Requiem".to_string(),
        },
        FieldOption {
            value: 364,
            label: "Thunder Breath".to_string(),
        },
        FieldOption {
            value: 365,
            label: "Thunder Breath II".to_string(),
        },
        FieldOption {
            value: 366,
            label: "Stun Breath".to_string(),
        },
        FieldOption {
            value: 367,
            label: "Stun Breath II".to_string(),
        },
        FieldOption {
            value: 368,
            label: "Sparagmos".to_string(),
        },
        FieldOption {
            value: 369,
            label: "Sand Breath".to_string(),
        },
        FieldOption {
            value: 370,
            label: "Sand Breath II".to_string(),
        },
        FieldOption {
            value: 371,
            label: "Lingering Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 372,
            label: "Lingering Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 373,
            label: "Day of Reckoning".to_string(),
        },
        FieldOption {
            value: 374,
            label: "Divine Breath".to_string(),
        },
        FieldOption {
            value: 375,
            label: "Divine Breath II".to_string(),
        },
        FieldOption {
            value: 376,
            label: "Tail Lash".to_string(),
        },
        FieldOption {
            value: 377,
            label: "Toxic Breath".to_string(),
        },
        FieldOption {
            value: 378,
            label: "Toxic Breath II".to_string(),
        },
        FieldOption {
            value: 379,
            label: "Vortex Breath".to_string(),
        },
        FieldOption {
            value: 380,
            label: "Vortex Breath II".to_string(),
        },
        FieldOption {
            value: 381,
            label: "Numbing Hook".to_string(),
        },
        FieldOption {
            value: 382,
            label: "Stinky Feet".to_string(),
        },
        FieldOption {
            value: 383,
            label: "Pumpkin Strike".to_string(),
        },
        FieldOption {
            value: 384,
            label: "Pumpkin Pie".to_string(),
        },
        FieldOption {
            value: 385,
            label: "Pumpkin Bomb".to_string(),
        },
        FieldOption {
            value: 386,
            label: "Flame Breath".to_string(),
        },
        FieldOption {
            value: 387,
            label: "Flame Breath II".to_string(),
        },
        FieldOption {
            value: 388,
            label: "Blood Syphon".to_string(),
        },
        FieldOption {
            value: 389,
            label: "Selfless Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 390,
            label: "Selfless Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 391,
            label: "Blue Spiral".to_string(),
        },
        FieldOption {
            value: 392,
            label: "Frost Breath".to_string(),
        },
        FieldOption {
            value: 393,
            label: "Frost Breath II".to_string(),
        },
        FieldOption {
            value: 394,
            label: "Heaven's Tear".to_string(),
        },
        FieldOption {
            value: 395,
            label: "Petro Breath".to_string(),
        },
        FieldOption {
            value: 396,
            label: "Petro Breath II".to_string(),
        },
        FieldOption {
            value: 397,
            label: "Poison Rain".to_string(),
        },
        FieldOption {
            value: 398,
            label: "Poison Breath".to_string(),
        },
        FieldOption {
            value: 399,
            label: "Poison Breath II".to_string(),
        },
        FieldOption {
            value: 400,
            label: "Poignant Melody".to_string(),
        },
        FieldOption {
            value: 401,
            label: "Maelstrom".to_string(),
        },
        FieldOption {
            value: 402,
            label: "Stirring Kiss (Faerie)".to_string(),
        },
        FieldOption {
            value: 403,
            label: "Stirring Kiss (Gremlin)".to_string(),
        },
        FieldOption {
            value: 404,
            label: "Raven Eye".to_string(),
        },
        FieldOption {
            value: 405,
            label: "Corpse Breath".to_string(),
        },
        FieldOption {
            value: 406,
            label: "Corpse Breath II".to_string(),
        },
        FieldOption {
            value: 407,
            label: "Evil Eye".to_string(),
        },
        FieldOption {
            value: 408,
            label: "Celestial Song".to_string(),
        },
        FieldOption {
            value: 409,
            label: "None".to_string(),
        },
        FieldOption {
            value: 410,
            label: "Doppelganger".to_string(),
        },
        FieldOption {
            value: 411,
            label: "None".to_string(),
        },
        FieldOption {
            value: 412,
            label: "None".to_string(),
        },
        FieldOption {
            value: 413,
            label: "None".to_string(),
        },
        FieldOption {
            value: 414,
            label: "None".to_string(),
        },
        FieldOption {
            value: 415,
            label: "None".to_string(),
        },
        FieldOption {
            value: 416,
            label: "None".to_string(),
        },
        FieldOption {
            value: 417,
            label: "None".to_string(),
        },
        FieldOption {
            value: 418,
            label: "None".to_string(),
        },
        FieldOption {
            value: 419,
            label: "None".to_string(),
        },
        FieldOption {
            value: 420,
            label: "None".to_string(),
        },
        FieldOption {
            value: 421,
            label: "Quickdraw".to_string(),
        },
        FieldOption {
            value: 422,
            label: "None".to_string(),
        },
        FieldOption {
            value: 423,
            label: "Parry I".to_string(),
        },
        FieldOption {
            value: 424,
            label: "Parry II".to_string(),
        },
        FieldOption {
            value: 425,
            label: "Parry III".to_string(),
        },
        FieldOption {
            value: 426,
            label: "Parry IV".to_string(),
        },
        FieldOption {
            value: 427,
            label: "Empower Golem I".to_string(),
        },
        FieldOption {
            value: 428,
            label: "Empower Golem II".to_string(),
        },
        FieldOption {
            value: 429,
            label: "Empower Golem III".to_string(),
        },
        FieldOption {
            value: 430,
            label: "Empower Golem IV".to_string(),
        },
        FieldOption {
            value: 431,
            label: "Empower Dragon I".to_string(),
        },
        FieldOption {
            value: 432,
            label: "Empower Dragon II".to_string(),
        },
        FieldOption {
            value: 433,
            label: "Empower Dragon III".to_string(),
        },
        FieldOption {
            value: 434,
            label: "Empower Dragon IV".to_string(),
        },
        FieldOption {
            value: 435,
            label: "Empower Beast".to_string(),
        },
        FieldOption {
            value: 436,
            label: "Empower Beast II".to_string(),
        },
        FieldOption {
            value: 437,
            label: "Empower Beast III".to_string(),
        },
        FieldOption {
            value: 438,
            label: "Empower Beast IV".to_string(),
        },
        FieldOption {
            value: 439,
            label: "Lucky Star I".to_string(),
        },
        FieldOption {
            value: 440,
            label: "Lucky Star II".to_string(),
        },
        FieldOption {
            value: 441,
            label: "Lucky Star III".to_string(),
        },
        FieldOption {
            value: 442,
            label: "Lucky Star IV".to_string(),
        },
        FieldOption {
            value: 443,
            label: "First Aid I (Stone)".to_string(),
        },
        FieldOption {
            value: 444,
            label: "First Aid II (Stone)".to_string(),
        },
        FieldOption {
            value: 445,
            label: "First Aid III (Stone)".to_string(),
        },
        FieldOption {
            value: 446,
            label: "First Aid IV (Stone)".to_string(),
        },
        FieldOption {
            value: 447,
            label: "First Aid I (Health)".to_string(),
        },
        FieldOption {
            value: 448,
            label: "First Aid II (Health)".to_string(),
        },
        FieldOption {
            value: 449,
            label: "First Aid III (Health)".to_string(),
        },
        FieldOption {
            value: 450,
            label: "First Aid IV (Health)".to_string(),
        },
        FieldOption {
            value: 451,
            label: "Meditate I".to_string(),
        },
        FieldOption {
            value: 452,
            label: "Meditate II".to_string(),
        },
        FieldOption {
            value: 453,
            label: "Meditate III".to_string(),
        },
        FieldOption {
            value: 454,
            label: "Meditate IV".to_string(),
        },
        FieldOption {
            value: 455,
            label: "Iron Maiden I".to_string(),
        },
        FieldOption {
            value: 456,
            label: "Iron Maiden II".to_string(),
        },
        FieldOption {
            value: 457,
            label: "Iron Maiden III".to_string(),
        },
        FieldOption {
            value: 458,
            label: "Iron Maiden IV".to_string(),
        },
        FieldOption {
            value: 459,
            label: "Featherstep I".to_string(),
        },
        FieldOption {
            value: 460,
            label: "Featherstep II".to_string(),
        },
        FieldOption {
            value: 461,
            label: "Featherstep III".to_string(),
        },
        FieldOption {
            value: 462,
            label: "Featherstep IV".to_string(),
        },
        FieldOption {
            value: 463,
            label: "Howl I".to_string(),
        },
        FieldOption {
            value: 464,
            label: "Howl II".to_string(),
        },
        FieldOption {
            value: 465,
            label: "Howl III".to_string(),
        },
        FieldOption {
            value: 466,
            label: "Howl IV".to_string(),
        },
        FieldOption {
            value: 467,
            label: "Huapango Winds I".to_string(),
        },
        FieldOption {
            value: 468,
            label: "Huapango Winds II".to_string(),
        },
        FieldOption {
            value: 469,
            label: "Huapango Winds III".to_string(),
        },
        FieldOption {
            value: 470,
            label: "Huapango Winds IV".to_string(),
        },
        FieldOption {
            value: 471,
            label: "Glare I".to_string(),
        },
        FieldOption {
            value: 472,
            label: "Glare II".to_string(),
        },
        FieldOption {
            value: 473,
            label: "Glare III".to_string(),
        },
        FieldOption {
            value: 474,
            label: "Glare IV".to_string(),
        },
        FieldOption {
            value: 475,
            label: "Threaten I".to_string(),
        },
        FieldOption {
            value: 476,
            label: "Threaten II".to_string(),
        },
        FieldOption {
            value: 477,
            label: "Threaten III".to_string(),
        },
        FieldOption {
            value: 478,
            label: "Threaten IV".to_string(),
        },
        FieldOption {
            value: 479,
            label: "Check I".to_string(),
        },
        FieldOption {
            value: 480,
            label: "Check II".to_string(),
        },
        FieldOption {
            value: 481,
            label: "Check III".to_string(),
        },
        FieldOption {
            value: 482,
            label: "Check IV".to_string(),
        },
        FieldOption {
            value: 483,
            label: "Black Mucus I".to_string(),
        },
        FieldOption {
            value: 484,
            label: "Black Mucus II".to_string(),
        },
        FieldOption {
            value: 485,
            label: "Black Mucus III".to_string(),
        },
        FieldOption {
            value: 486,
            label: "Black Mucus IV".to_string(),
        },
        FieldOption {
            value: 487,
            label: "Bloody Gag I".to_string(),
        },
        FieldOption {
            value: 488,
            label: "Bloody Gag II".to_string(),
        },
        FieldOption {
            value: 489,
            label: "Bloody Gag III".to_string(),
        },
        FieldOption {
            value: 490,
            label: "Bloody Gag IV".to_string(),
        },
        FieldOption {
            value: 491,
            label: "Witch's Smile I".to_string(),
        },
        FieldOption {
            value: 492,
            label: "Witch's Smile II".to_string(),
        },
        FieldOption {
            value: 493,
            label: "Witch's Smile III".to_string(),
        },
        FieldOption {
            value: 494,
            label: "Witch's Smile IV".to_string(),
        },
        FieldOption {
            value: 495,
            label: "Irresistable Beauty I".to_string(),
        },
        FieldOption {
            value: 496,
            label: "Irresistable Beauty II".to_string(),
        },
        FieldOption {
            value: 497,
            label: "Irresistable Beauty III".to_string(),
        },
        FieldOption {
            value: 498,
            label: "Irresistable Beauty IV".to_string(),
        },
        FieldOption {
            value: 499,
            label: "Lament of the Dead I".to_string(),
        },
        FieldOption {
            value: 500,
            label: "Lament of the Dead II".to_string(),
        },
        FieldOption {
            value: 501,
            label: "Lament of the Dead III".to_string(),
        },
        FieldOption {
            value: 502,
            label: "Lament of the Dead IV".to_string(),
        },
        FieldOption {
            value: 503,
            label: "Rapier Glance I".to_string(),
        },
        FieldOption {
            value: 504,
            label: "Rapier Glance II".to_string(),
        },
        FieldOption {
            value: 505,
            label: "Rapier Glance III".to_string(),
        },
        FieldOption {
            value: 506,
            label: "Rapier Glance IV".to_string(),
        },
        FieldOption {
            value: 507,
            label: "Intimidate I".to_string(),
        },
        FieldOption {
            value: 508,
            label: "Intimidate II".to_string(),
        },
        FieldOption {
            value: 509,
            label: "Intimidate III".to_string(),
        },
        FieldOption {
            value: 510,
            label: "Intimidate IV".to_string(),
        },
        FieldOption {
            value: 511,
            label: "Aerial Resonance I".to_string(),
        },
        FieldOption {
            value: 512,
            label: "Aerial Resonance II".to_string(),
        },
        FieldOption {
            value: 513,
            label: "Aerial Resonance III".to_string(),
        },
        FieldOption {
            value: 514,
            label: "Aerial Resonance IV".to_string(),
        },
        FieldOption {
            value: 515,
            label: "Telluric Resonance I".to_string(),
        },
        FieldOption {
            value: 516,
            label: "Telluric Resonance II".to_string(),
        },
        FieldOption {
            value: 517,
            label: "Telluric Resonance III".to_string(),
        },
        FieldOption {
            value: 518,
            label: "Telluric Resonance IV".to_string(),
        },
        FieldOption {
            value: 519,
            label: "Charged Resonance I".to_string(),
        },
        FieldOption {
            value: 520,
            label: "Charged Resonance II".to_string(),
        },
        FieldOption {
            value: 521,
            label: "Charged Resonance III".to_string(),
        },
        FieldOption {
            value: 522,
            label: "Charged Resonance IV".to_string(),
        },
        FieldOption {
            value: 523,
            label: "Aquatic Resonance I".to_string(),
        },
        FieldOption {
            value: 524,
            label: "Aquatic Resonance II".to_string(),
        },
        FieldOption {
            value: 525,
            label: "Aquatic Resonance III".to_string(),
        },
        FieldOption {
            value: 526,
            label: "Aquatic Resonance IV".to_string(),
        },
        FieldOption {
            value: 527,
            label: "Blazing Resonance I".to_string(),
        },
        FieldOption {
            value: 528,
            label: "Blazing Resonance II".to_string(),
        },
        FieldOption {
            value: 529,
            label: "Blazing Resonance III".to_string(),
        },
        FieldOption {
            value: 530,
            label: "Blazing Resonance IV".to_string(),
        },
        FieldOption {
            value: 531,
            label: "Icy Resonance I".to_string(),
        },
        FieldOption {
            value: 532,
            label: "Icy Resonance II".to_string(),
        },
        FieldOption {
            value: 533,
            label: "Icy Resonance III".to_string(),
        },
        FieldOption {
            value: 534,
            label: "Icy Resonance IV".to_string(),
        },
        FieldOption {
            value: 535,
            label: "Luminous Resonance I".to_string(),
        },
        FieldOption {
            value: 536,
            label: "Luminous Resonance II".to_string(),
        },
        FieldOption {
            value: 537,
            label: "Luminous Resonance III".to_string(),
        },
        FieldOption {
            value: 538,
            label: "Luminous Resonance IV".to_string(),
        },
        FieldOption {
            value: 539,
            label: "Shadow Resonance I".to_string(),
        },
        FieldOption {
            value: 540,
            label: "Shadow Resonance II".to_string(),
        },
        FieldOption {
            value: 541,
            label: "Shadow Resonance III".to_string(),
        },
        FieldOption {
            value: 542,
            label: "Shadow Resonance IV".to_string(),
        },
        FieldOption {
            value: 543,
            label: "Aqua Veil".to_string(),
        },
        FieldOption {
            value: 544,
            label: "None".to_string(),
        },
        FieldOption {
            value: 545,
            label: "None".to_string(),
        },
        FieldOption {
            value: 546,
            label: "None".to_string(),
        },
        FieldOption {
            value: 547,
            label: "Preempt".to_string(),
        },
        FieldOption {
            value: 548,
            label: "None".to_string(),
        },
        FieldOption {
            value: 549,
            label: "None".to_string(),
        },
        FieldOption {
            value: 550,
            label: "None".to_string(),
        },
        FieldOption {
            value: 551,
            label: "Eagle Eye".to_string(),
        },
        FieldOption {
            value: 552,
            label: "None".to_string(),
        },
        FieldOption {
            value: 553,
            label: "None".to_string(),
        },
        FieldOption {
            value: 554,
            label: "None".to_string(),
        },
        FieldOption {
            value: 555,
            label: "Evil's Bane".to_string(),
        },
        FieldOption {
            value: 556,
            label: "None".to_string(),
        },
        FieldOption {
            value: 557,
            label: "None".to_string(),
        },
        FieldOption {
            value: 558,
            label: "None".to_string(),
        },
        FieldOption {
            value: 559,
            label: "Mighty Strike".to_string(),
        },
        FieldOption {
            value: 560,
            label: "None".to_string(),
        },
        FieldOption {
            value: 561,
            label: "None".to_string(),
        },
        FieldOption {
            value: 562,
            label: "None".to_string(),
        },
        FieldOption {
            value: 563,
            label: "Evade".to_string(),
        },
        FieldOption {
            value: 564,
            label: "None".to_string(),
        },
        FieldOption {
            value: 565,
            label: "None".to_string(),
        },
        FieldOption {
            value: 566,
            label: "None".to_string(),
        },
        FieldOption {
            value: 567,
            label: "Gordian Lock I".to_string(),
        },
        FieldOption {
            value: 568,
            label: "Gordian Lock II".to_string(),
        },
        FieldOption {
            value: 569,
            label: "Gordian Lock III".to_string(),
        },
        FieldOption {
            value: 570,
            label: "Gordian Lock IV".to_string(),
        },
        FieldOption {
            value: 571,
            label: "Engulf I".to_string(),
        },
        FieldOption {
            value: 572,
            label: "Engulf II".to_string(),
        },
        FieldOption {
            value: 573,
            label: "None".to_string(),
        },
        FieldOption {
            value: 574,
            label: "None".to_string(),
        },
        FieldOption {
            value: 575,
            label: "Princess Whim I".to_string(),
        },
        FieldOption {
            value: 576,
            label: "Princess Whim II".to_string(),
        },
        FieldOption {
            value: 577,
            label: "Princess Whim III".to_string(),
        },
        FieldOption {
            value: 578,
            label: "Princess Whim IV".to_string(),
        },
        FieldOption {
            value: 579,
            label: "Magic Time!".to_string(),
        },
        FieldOption {
            value: 580,
            label: "None".to_string(),
        },
        FieldOption {
            value: 581,
            label: "None".to_string(),
        },
        FieldOption {
            value: 582,
            label: "None".to_string(),
        },
        FieldOption {
            value: 583,
            label: "Guardian Force".to_string(),
        },
        FieldOption {
            value: 584,
            label: "None".to_string(),
        },
        FieldOption {
            value: 585,
            label: "None".to_string(),
        },
        FieldOption {
            value: 586,
            label: "None".to_string(),
        },
        FieldOption {
            value: 587,
            label: "El Colas Winds".to_string(),
        },
        FieldOption {
            value: 588,
            label: "None".to_string(),
        },
        FieldOption {
            value: 589,
            label: "None".to_string(),
        },
        FieldOption {
            value: 590,
            label: "None".to_string(),
        },
        FieldOption {
            value: 591,
            label: "Intercession".to_string(),
        },
        FieldOption {
            value: 592,
            label: "None".to_string(),
        },
        FieldOption {
            value: 593,
            label: "None".to_string(),
        },
        FieldOption {
            value: 594,
            label: "None".to_string(),
        },
        FieldOption {
            value: 595,
            label: "Course Correction".to_string(),
        },
        FieldOption {
            value: 596,
            label: "None".to_string(),
        },
        FieldOption {
            value: 597,
            label: "None".to_string(),
        },
        FieldOption {
            value: 598,
            label: "None".to_string(),
        },
        FieldOption {
            value: 599,
            label: "Golem's Bane".to_string(),
        },
        FieldOption {
            value: 600,
            label: "None".to_string(),
        },
        FieldOption {
            value: 601,
            label: "None".to_string(),
        },
        FieldOption {
            value: 602,
            label: "None".to_string(),
        },
        FieldOption {
            value: 603,
            label: "Conserve MP".to_string(),
        },
        FieldOption {
            value: 604,
            label: "None".to_string(),
        },
        FieldOption {
            value: 605,
            label: "None".to_string(),
        },
        FieldOption {
            value: 606,
            label: "None".to_string(),
        },
        FieldOption {
            value: 607,
            label: "Conserve RT".to_string(),
        },
        FieldOption {
            value: 608,
            label: "None".to_string(),
        },
        FieldOption {
            value: 609,
            label: "None".to_string(),
        },
        FieldOption {
            value: 610,
            label: "None".to_string(),
        },
        FieldOption {
            value: 611,
            label: "Consecrate Edge".to_string(),
        },
        FieldOption {
            value: 612,
            label: "None".to_string(),
        },
        FieldOption {
            value: 613,
            label: "None".to_string(),
        },
        FieldOption {
            value: 614,
            label: "None".to_string(),
        },
        FieldOption {
            value: 615,
            label: "Salvation".to_string(),
        },
        FieldOption {
            value: 616,
            label: "None".to_string(),
        },
        FieldOption {
            value: 617,
            label: "None".to_string(),
        },
        FieldOption {
            value: 618,
            label: "None".to_string(),
        },
        FieldOption {
            value: 619,
            label: "Mind's Eye I".to_string(),
        },
        FieldOption {
            value: 620,
            label: "Mind's Eye II".to_string(),
        },
        FieldOption {
            value: 621,
            label: "None".to_string(),
        },
        FieldOption {
            value: 622,
            label: "None".to_string(),
        },
        FieldOption {
            value: 623,
            label: "Stardust Grace".to_string(),
        },
        FieldOption {
            value: 624,
            label: "None".to_string(),
        },
        FieldOption {
            value: 625,
            label: "None".to_string(),
        },
        FieldOption {
            value: 626,
            label: "None".to_string(),
        },
        FieldOption {
            value: 627,
            label: "Speedstar".to_string(),
        },
        FieldOption {
            value: 628,
            label: "None".to_string(),
        },
        FieldOption {
            value: 629,
            label: "None".to_string(),
        },
        FieldOption {
            value: 630,
            label: "None".to_string(),
        },
        FieldOption {
            value: 631,
            label: "Concentration (Ninjutsu)".to_string(),
        },
        FieldOption {
            value: 632,
            label: "None".to_string(),
        },
        FieldOption {
            value: 633,
            label: "None".to_string(),
        },
        FieldOption {
            value: 634,
            label: "None".to_string(),
        },
        FieldOption {
            value: 635,
            label: "Mother's Mercy".to_string(),
        },
        FieldOption {
            value: 636,
            label: "None".to_string(),
        },
        FieldOption {
            value: 637,
            label: "None".to_string(),
        },
        FieldOption {
            value: 638,
            label: "None".to_string(),
        },
        FieldOption {
            value: 639,
            label: "Mother's Blessing".to_string(),
        },
        FieldOption {
            value: 640,
            label: "None".to_string(),
        },
        FieldOption {
            value: 641,
            label: "None".to_string(),
        },
        FieldOption {
            value: 642,
            label: "None".to_string(),
        },
        FieldOption {
            value: 643,
            label: "Nature's Touch".to_string(),
        },
        FieldOption {
            value: 644,
            label: "None".to_string(),
        },
        FieldOption {
            value: 645,
            label: "None".to_string(),
        },
        FieldOption {
            value: 646,
            label: "None".to_string(),
        },
        FieldOption {
            value: 647,
            label: "Echoing Voice".to_string(),
        },
        FieldOption {
            value: 648,
            label: "None".to_string(),
        },
        FieldOption {
            value: 649,
            label: "None".to_string(),
        },
        FieldOption {
            value: 650,
            label: "None".to_string(),
        },
        FieldOption {
            value: 651,
            label: "Sharpshoot".to_string(),
        },
        FieldOption {
            value: 652,
            label: "None".to_string(),
        },
        FieldOption {
            value: 653,
            label: "None".to_string(),
        },
        FieldOption {
            value: 654,
            label: "None".to_string(),
        },
        FieldOption {
            value: 655,
            label: "Double Impact".to_string(),
        },
        FieldOption {
            value: 656,
            label: "None".to_string(),
        },
        FieldOption {
            value: 657,
            label: "None".to_string(),
        },
        FieldOption {
            value: 658,
            label: "None".to_string(),
        },
        FieldOption {
            value: 659,
            label: "Double Shot".to_string(),
        },
        FieldOption {
            value: 660,
            label: "None".to_string(),
        },
        FieldOption {
            value: 661,
            label: "None".to_string(),
        },
        FieldOption {
            value: 662,
            label: "None".to_string(),
        },
        FieldOption {
            value: 663,
            label: "Resounding Voice".to_string(),
        },
        FieldOption {
            value: 664,
            label: "None".to_string(),
        },
        FieldOption {
            value: 665,
            label: "None".to_string(),
        },
        FieldOption {
            value: 666,
            label: "None".to_string(),
        },
        FieldOption {
            value: 667,
            label: "Fearful Impact".to_string(),
        },
        FieldOption {
            value: 668,
            label: "None".to_string(),
        },
        FieldOption {
            value: 669,
            label: "None".to_string(),
        },
        FieldOption {
            value: 670,
            label: "None".to_string(),
        },
        FieldOption {
            value: 671,
            label: "Dash I".to_string(),
        },
        FieldOption {
            value: 672,
            label: "Dash II".to_string(),
        },
        FieldOption {
            value: 673,
            label: "None".to_string(),
        },
        FieldOption {
            value: 674,
            label: "None".to_string(),
        },
        FieldOption {
            value: 675,
            label: "Dragonslayer".to_string(),
        },
        FieldOption {
            value: 676,
            label: "None".to_string(),
        },
        FieldOption {
            value: 677,
            label: "None".to_string(),
        },
        FieldOption {
            value: 678,
            label: "None".to_string(),
        },
        FieldOption {
            value: 679,
            label: "Dragon's Eye".to_string(),
        },
        FieldOption {
            value: 680,
            label: "None".to_string(),
        },
        FieldOption {
            value: 681,
            label: "None".to_string(),
        },
        FieldOption {
            value: 682,
            label: "None".to_string(),
        },
        FieldOption {
            value: 683,
            label: "Dragon's Scale I".to_string(),
        },
        FieldOption {
            value: 684,
            label: "Dragon's Scale II".to_string(),
        },
        FieldOption {
            value: 685,
            label: "Dragon's Scale III".to_string(),
        },
        FieldOption {
            value: 686,
            label: "Dragon's Scale IV".to_string(),
        },
        FieldOption {
            value: 687,
            label: "Dragonsbane".to_string(),
        },
        FieldOption {
            value: 688,
            label: "None".to_string(),
        },
        FieldOption {
            value: 689,
            label: "None".to_string(),
        },
        FieldOption {
            value: 690,
            label: "None".to_string(),
        },
        FieldOption {
            value: 691,
            label: "Tremendous Shot".to_string(),
        },
        FieldOption {
            value: 692,
            label: "None".to_string(),
        },
        FieldOption {
            value: 693,
            label: "None".to_string(),
        },
        FieldOption {
            value: 694,
            label: "None".to_string(),
        },
        FieldOption {
            value: 695,
            label: "Berserk".to_string(),
        },
        FieldOption {
            value: 696,
            label: "None".to_string(),
        },
        FieldOption {
            value: 697,
            label: "None".to_string(),
        },
        FieldOption {
            value: 698,
            label: "None".to_string(),
        },
        FieldOption {
            value: 699,
            label: "Back Attack".to_string(),
        },
        FieldOption {
            value: 700,
            label: "None".to_string(),
        },
        FieldOption {
            value: 701,
            label: "None".to_string(),
        },
        FieldOption {
            value: 702,
            label: "None".to_string(),
        },
        FieldOption {
            value: 703,
            label: "Paralysis Blade".to_string(),
        },
        FieldOption {
            value: 704,
            label: "None".to_string(),
        },
        FieldOption {
            value: 705,
            label: "None".to_string(),
        },
        FieldOption {
            value: 706,
            label: "None".to_string(),
        },
        FieldOption {
            value: 707,
            label: "Beastslayer".to_string(),
        },
        FieldOption {
            value: 708,
            label: "None".to_string(),
        },
        FieldOption {
            value: 709,
            label: "None".to_string(),
        },
        FieldOption {
            value: 710,
            label: "None".to_string(),
        },
        FieldOption {
            value: 711,
            label: "Beastbane".to_string(),
        },
        FieldOption {
            value: 712,
            label: "None".to_string(),
        },
        FieldOption {
            value: 713,
            label: "None".to_string(),
        },
        FieldOption {
            value: 714,
            label: "None".to_string(),
        },
        FieldOption {
            value: 715,
            label: "Vigorous Attack".to_string(),
        },
        FieldOption {
            value: 716,
            label: "None".to_string(),
        },
        FieldOption {
            value: 717,
            label: "None".to_string(),
        },
        FieldOption {
            value: 718,
            label: "None".to_string(),
        },
        FieldOption {
            value: 719,
            label: "Phalanx I".to_string(),
        },
        FieldOption {
            value: 720,
            label: "Phalanx II".to_string(),
        },
        FieldOption {
            value: 721,
            label: "Phalanx III".to_string(),
        },
        FieldOption {
            value: 722,
            label: "Phalanx IV".to_string(),
        },
        FieldOption {
            value: 723,
            label: "Steelstance I".to_string(),
        },
        FieldOption {
            value: 724,
            label: "Steelstance II".to_string(),
        },
        FieldOption {
            value: 725,
            label: "Steelstance III".to_string(),
        },
        FieldOption {
            value: 726,
            label: "Steelstance IV".to_string(),
        },
        FieldOption {
            value: 727,
            label: "Sanguine Assault".to_string(),
        },
        FieldOption {
            value: 728,
            label: "None".to_string(),
        },
        FieldOption {
            value: 729,
            label: "None".to_string(),
        },
        FieldOption {
            value: 730,
            label: "None".to_string(),
        },
        FieldOption {
            value: 731,
            label: "Break Curse".to_string(),
        },
        FieldOption {
            value: 732,
            label: "None".to_string(),
        },
        FieldOption {
            value: 733,
            label: "None".to_string(),
        },
        FieldOption {
            value: 734,
            label: "None".to_string(),
        },
        FieldOption {
            value: 735,
            label: "Broaden Force".to_string(),
        },
        FieldOption {
            value: 736,
            label: "None".to_string(),
        },
        FieldOption {
            value: 737,
            label: "None".to_string(),
        },
        FieldOption {
            value: 738,
            label: "None".to_string(),
        },
        FieldOption {
            value: 739,
            label: "Velocity Shift".to_string(),
        },
        FieldOption {
            value: 740,
            label: "None".to_string(),
        },
        FieldOption {
            value: 741,
            label: "None".to_string(),
        },
        FieldOption {
            value: 742,
            label: "None".to_string(),
        },
        FieldOption {
            value: 743,
            label: "Mighty Impact".to_string(),
        },
        FieldOption {
            value: 744,
            label: "None".to_string(),
        },
        FieldOption {
            value: 745,
            label: "None".to_string(),
        },
        FieldOption {
            value: 746,
            label: "None".to_string(),
        },
        FieldOption {
            value: 747,
            label: "Apostate I".to_string(),
        },
        FieldOption {
            value: 748,
            label: "Apostate II".to_string(),
        },
        FieldOption {
            value: 749,
            label: "None".to_string(),
        },
        FieldOption {
            value: 750,
            label: "None".to_string(),
        },
        FieldOption {
            value: 751,
            label: "Risk Management".to_string(),
        },
        FieldOption {
            value: 752,
            label: "None".to_string(),
        },
        FieldOption {
            value: 753,
            label: "None".to_string(),
        },
        FieldOption {
            value: 754,
            label: "None".to_string(),
        },
        FieldOption {
            value: 755,
            label: "Reflection".to_string(),
        },
        FieldOption {
            value: 756,
            label: "None".to_string(),
        },
        FieldOption {
            value: 757,
            label: "None".to_string(),
        },
        FieldOption {
            value: 758,
            label: "None".to_string(),
        },
        FieldOption {
            value: 759,
            label: "Repel Dragon I".to_string(),
        },
        FieldOption {
            value: 760,
            label: "Repel Dragon II".to_string(),
        },
        FieldOption {
            value: 761,
            label: "None".to_string(),
        },
        FieldOption {
            value: 762,
            label: "None".to_string(),
        },
        FieldOption {
            value: 763,
            label: "Repel Beast I".to_string(),
        },
        FieldOption {
            value: 764,
            label: "Repel Beast II".to_string(),
        },
        FieldOption {
            value: 765,
            label: "None".to_string(),
        },
        FieldOption {
            value: 766,
            label: "None".to_string(),
        },
        FieldOption {
            value: 767,
            label: "Ivory Tower I".to_string(),
        },
        FieldOption {
            value: 768,
            label: "Ivory Tower II".to_string(),
        },
        FieldOption {
            value: 769,
            label: "None".to_string(),
        },
        FieldOption {
            value: 770,
            label: "None".to_string(),
        },
        FieldOption {
            value: 771,
            label: "Concentration (Magic)".to_string(),
        },
        FieldOption {
            value: 772,
            label: "None".to_string(),
        },
        FieldOption {
            value: 773,
            label: "None".to_string(),
        },
        FieldOption {
            value: 774,
            label: "None".to_string(),
        },
        FieldOption {
            value: 775,
            label: "Smoke Screen I".to_string(),
        },
        FieldOption {
            value: 776,
            label: "Smoke Screen II".to_string(),
        },
        FieldOption {
            value: 777,
            label: "Smoke Screen III".to_string(),
        },
        FieldOption {
            value: 778,
            label: "Smoke Screen IV".to_string(),
        },
        FieldOption {
            value: 779,
            label: "Lobber I".to_string(),
        },
        FieldOption {
            value: 780,
            label: "Lobber II".to_string(),
        },
        FieldOption {
            value: 781,
            label: "Lobber III".to_string(),
        },
        FieldOption {
            value: 782,
            label: "Lobber IV".to_string(),
        },
        FieldOption {
            value: 783,
            label: "Falling Blade".to_string(),
        },
        FieldOption {
            value: 784,
            label: "None".to_string(),
        },
        FieldOption {
            value: 785,
            label: "None".to_string(),
        },
        FieldOption {
            value: 786,
            label: "None".to_string(),
        },
        FieldOption {
            value: 787,
            label: "Pincer Attack".to_string(),
        },
        FieldOption {
            value: 788,
            label: "Pincer Attack II".to_string(),
        },
        FieldOption {
            value: 789,
            label: "Pincer Attack III".to_string(),
        },
        FieldOption {
            value: 790,
            label: "Pincer Attack IV".to_string(),
        },
        FieldOption {
            value: 791,
            label: "Exhalito (Void)".to_string(),
        },
        FieldOption {
            value: 792,
            label: "Procella (Wind)".to_string(),
        },
        FieldOption {
            value: 793,
            label: "Pondus (Earth)".to_string(),
        },
        FieldOption {
            value: 794,
            label: "Tonitrus (Lightning)".to_string(),
        },
        FieldOption {
            value: 795,
            label: "Coctura (Water)".to_string(),
        },
        FieldOption {
            value: 796,
            label: "Diruptio (Fire)".to_string(),
        },
        FieldOption {
            value: 797,
            label: "Congelatio (Ice)".to_string(),
        },
        FieldOption {
            value: 798,
            label: "Radius (Light)".to_string(),
        },
        FieldOption {
            value: 799,
            label: "Umbra (Dark)".to_string(),
        },
    ]
}

fn opt_use() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Yes".to_string(),
        },
        FieldOption {
            value: 1,
            label: "No".to_string(),
        },
    ]
}

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "skill_access".to_string(),
        name: "Skill Access Editor".to_string(),
        description: "Edit which classes can learn each skill".to_string(),
        base_offset: 0x42B982,
        entry_count: 800,
        entry_size: 102,
        entry_names: vec![
            "None".to_string(),
            "Fists".to_string(),
            "Daggers".to_string(),
            "Swords(1H)".to_string(),
            "Swords(2H)".to_string(),
            "Axes".to_string(),
            "None".to_string(),
            "Spears".to_string(),
            "Hammers".to_string(),
            "None".to_string(),
            "Katana (1H)".to_string(),
            "Katana (2H)".to_string(),
            "Cudgels".to_string(),
            "None".to_string(),
            "Whips".to_string(),
            "Spellbooks".to_string(),
            "Instruments".to_string(),
            "Blowguns".to_string(),
            "Bows".to_string(),
            "Crossbows".to_string(),
            "Fusils".to_string(),
            "Thrown".to_string(),
            "Anatomy".to_string(),
            "Teratology".to_string(),
            "Herpetology".to_string(),
            "Draconology".to_string(),
            "Sacrology".to_string(),
            "Daemonology".to_string(),
            "Aurology".to_string(),
            "Thanatology".to_string(),
            "Golemy".to_string(),
            "Augment Air".to_string(),
            "Augment Earth".to_string(),
            "Augment Lightning".to_string(),
            "Augment Water".to_string(),
            "Augment Fire".to_string(),
            "Augment Ice".to_string(),
            "Augment Light".to_string(),
            "Augment Darkness".to_string(),
            "Parry".to_string(),
            "Deflect".to_string(),
            "Overpower".to_string(),
            "Resist Petrify".to_string(),
            "Resist Stun".to_string(),
            "Resist Sleep".to_string(),
            "Resist Charm".to_string(),
            "Resist Poison".to_string(),
            "Resist Silence".to_string(),
            "Resist Slow".to_string(),
            "Resist Bind".to_string(),
            "Resist Shackle".to_string(),
            "Resist Stop".to_string(),
            "Resist Leaden".to_string(),
            "Resist Fear".to_string(),
            "Resist Venom".to_string(),
            "Resist Curse".to_string(),
            "None".to_string(),
            "Air Magic".to_string(),
            "Earth Magic".to_string(),
            "Lightning Magic".to_string(),
            "Water Magic".to_string(),
            "Fire Magic".to_string(),
            "Ice Magic".to_string(),
            "Divine Magic".to_string(),
            "Dark Magic".to_string(),
            "Draconic Magic".to_string(),
            "Necromancy".to_string(),
            "Ninjutsu".to_string(),
            "War Dances".to_string(),
            "Songs".to_string(),
            "None".to_string(),
            "Attenuate Air".to_string(),
            "Attenuate Earth".to_string(),
            "Attenuate Lightning".to_string(),
            "Attenuate Water".to_string(),
            "Attenuate Fire".to_string(),
            "Attenuate Ice".to_string(),
            "Attenuate Light".to_string(),
            "Attenuate Dark".to_string(),
            "None".to_string(),
            "Rampart Aura I".to_string(),
            "Rampart Aura II".to_string(),
            "Rampart Aura III".to_string(),
            "Rampart Aura IV".to_string(),
            "Rampart Aura V".to_string(),
            "None".to_string(),
            "Counterattack I".to_string(),
            "Counterattack II".to_string(),
            "Counterattack III".to_string(),
            "Counterattack IV".to_string(),
            "None".to_string(),
            "Knockback I".to_string(),
            "Knockback II".to_string(),
            "Knockback III".to_string(),
            "Knockback IV".to_string(),
            "None".to_string(),
            "Strengthen I".to_string(),
            "Strengthen II".to_string(),
            "Strengthen III".to_string(),
            "Strengthen IV".to_string(),
            "None".to_string(),
            "Fortify I".to_string(),
            "Fortify II".to_string(),
            "Fortify III".to_string(),
            "Fortify IV".to_string(),
            "None".to_string(),
            "Spellcraft I".to_string(),
            "Spellcraft II".to_string(),
            "Spellcraft III".to_string(),
            "Spellcraft IV".to_string(),
            "None".to_string(),
            "Resistance I".to_string(),
            "Resistance II".to_string(),
            "Resistance III".to_string(),
            "Resistance IV".to_string(),
            "None".to_string(),
            "Truestrike I".to_string(),
            "Truestrike II".to_string(),
            "Truestrike III".to_string(),
            "Truestrike IV".to_string(),
            "None".to_string(),
            "Trueflight I".to_string(),
            "Trueflight II".to_string(),
            "Trueflight III".to_string(),
            "Trueflight IV".to_string(),
            "None".to_string(),
            "Spellstrike I".to_string(),
            "Spellstrike II".to_string(),
            "Spellstrike III".to_string(),
            "Spellstrike IV".to_string(),
            "None".to_string(),
            "Dodge I".to_string(),
            "Dodge II".to_string(),
            "Dodge III".to_string(),
            "Dodge IV".to_string(),
            "None".to_string(),
            "Sidestep I".to_string(),
            "Sidestep II".to_string(),
            "Sidestep III".to_string(),
            "Sidestep IV".to_string(),
            "None".to_string(),
            "Spell Ward I".to_string(),
            "Spell Ward II".to_string(),
            "Spell Ward III".to_string(),
            "Spell Ward IV".to_string(),
            "None".to_string(),
            "Constitution I".to_string(),
            "Constitution II".to_string(),
            "Constitution III".to_string(),
            "Constitution IV".to_string(),
            "None".to_string(),
            "Insight I".to_string(),
            "Insight II".to_string(),
            "Insight III".to_string(),
            "Insight IV".to_string(),
            "None".to_string(),
            "Expand Mind I".to_string(),
            "Expand Mind II".to_string(),
            "Expand Mind III".to_string(),
            "Expand Mind IV".to_string(),
            "None".to_string(),
            "Channeling I".to_string(),
            "Channeling II".to_string(),
            "Channeling III".to_string(),
            "Channeling IV".to_string(),
            "None".to_string(),
            "Stoneproof".to_string(),
            "Stunproof".to_string(),
            "Sleepproof".to_string(),
            "Charmproof".to_string(),
            "Foolproof".to_string(),
            "Poisonproof".to_string(),
            "Silenceproof".to_string(),
            "Slowproof".to_string(),
            "Stopproof".to_string(),
            "Leadproof".to_string(),
            "Fearproof".to_string(),
            "Venomproof".to_string(),
            "Curseproof".to_string(),
            "Deathproof".to_string(),
            "None".to_string(),
            "Swiftfoot I".to_string(),
            "Swiftfoot II".to_string(),
            "Jump I".to_string(),
            "Jump II".to_string(),
            "Wade I".to_string(),
            "Wade II".to_string(),
            "None".to_string(),
            "Sanctuary I".to_string(),
            "Sanctuary II".to_string(),
            "Invisibility".to_string(),
            "Steadfast".to_string(),
            "Double Attack".to_string(),
            "Trajectory".to_string(),
            "Siege".to_string(),
            "Invincible".to_string(),
            "Field Alchemy I".to_string(),
            "Field Alchemy II".to_string(),
            "Field Alchemy III".to_string(),
            "Field Alchemy IV".to_string(),
            "Eagle Eye II".to_string(),
            "Max TP I".to_string(),
            "Max TP II".to_string(),
            "Max TP III".to_string(),
            "Max TP IV".to_string(),
            "None".to_string(),
            "Treasure Hunt I".to_string(),
            "Treasure Hunt II".to_string(),
            "None".to_string(),
            "Tactician I".to_string(),
            "Tactician II".to_string(),
            "None".to_string(),
            "Reflect Damage I".to_string(),
            "Reflect Damage II".to_string(),
            "None".to_string(),
            "Reflect Magic I".to_string(),
            "Reflect Magic II".to_string(),
            "None".to_string(),
            "Absorb MP I".to_string(),
            "Absorb MP II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Pumpkin Lure".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Recruit".to_string(),
            "Subdue".to_string(),
            "Coax".to_string(),
            "Tame".to_string(),
            "Seraph's Pact".to_string(),
            "Demon's Pact".to_string(),
            "Fey Pact".to_string(),
            "Master Undead".to_string(),
            "Control Golem".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Gordian Key".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Animate Dead".to_string(),
            "Absolution".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evanescece".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evil Deeds".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Sublime Sacrifice".to_string(),
            "None".to_string(),
            "Sanctuary Shadow".to_string(),
            "Jack-o'-Lantern".to_string(),
            "Shadowbreak".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Holy Water".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Nature's Whisper".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dragon's Wound".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Consecrate Dead".to_string(),
            "Condemn".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Rampart Shadow".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Aqua Bubble".to_string(),
            "Agonal Scream".to_string(),
            "Acid Breath".to_string(),
            "Acid Breath II".to_string(),
            "Sweaty Palms".to_string(),
            "Wind Shot".to_string(),
            "Virtuous Dance".to_string(),
            "Cruelist Cut".to_string(),
            "Granite Fist".to_string(),
            "Crystal Pumpkin".to_string(),
            "Reeking Meetballs".to_string(),
            "Coquettish Kiss (Faerie)".to_string(),
            "Coquettish Kiss (Gremlin)".to_string(),
            "Silent Song".to_string(),
            "Pirate Breath".to_string(),
            "Requiem".to_string(),
            "Thunder Breath".to_string(),
            "Thunder Breath II".to_string(),
            "Stun Breath".to_string(),
            "Stun Breath II".to_string(),
            "Sparagmos".to_string(),
            "Sand Breath".to_string(),
            "Sand Breath II".to_string(),
            "Lingering Kiss (Faerie)".to_string(),
            "Lingering Kiss (Gremlin)".to_string(),
            "Day of Reckoning".to_string(),
            "Divine Breath".to_string(),
            "Divine Breath II".to_string(),
            "Tail Lash".to_string(),
            "Toxic Breath".to_string(),
            "Toxic Breath II".to_string(),
            "Vortex Breath".to_string(),
            "Vortex Breath II".to_string(),
            "Numbing Hook".to_string(),
            "Stinky Feet".to_string(),
            "Pumpkin Strike".to_string(),
            "Pumpkin Pie".to_string(),
            "Pumpkin Bomb".to_string(),
            "Flame Breath".to_string(),
            "Flame Breath II".to_string(),
            "Blood Syphon".to_string(),
            "Selfless Kiss (Faerie)".to_string(),
            "Selfless Kiss (Gremlin)".to_string(),
            "Blue Spiral".to_string(),
            "Frost Breath".to_string(),
            "Frost Breath II".to_string(),
            "Heaven's Tear".to_string(),
            "Petro Breath".to_string(),
            "Petro Breath II".to_string(),
            "Poison Rain".to_string(),
            "Poison Breath".to_string(),
            "Poison Breath II".to_string(),
            "Poignant Melody".to_string(),
            "Maelstrom".to_string(),
            "Stirring Kiss (Faerie)".to_string(),
            "Stirring Kiss (Gremlin)".to_string(),
            "Raven Eye".to_string(),
            "Corpse Breath".to_string(),
            "Corpse Breath II".to_string(),
            "Evil Eye".to_string(),
            "Celestial Song".to_string(),
            "None".to_string(),
            "Doppelganger".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Quickdraw".to_string(),
            "None".to_string(),
            "Parry I".to_string(),
            "Parry II".to_string(),
            "Parry III".to_string(),
            "Parry IV".to_string(),
            "Empower Golem I".to_string(),
            "Empower Golem II".to_string(),
            "Empower Golem III".to_string(),
            "Empower Golem IV".to_string(),
            "Empower Dragon I".to_string(),
            "Empower Dragon II".to_string(),
            "Empower Dragon III".to_string(),
            "Empower Dragon IV".to_string(),
            "Empower Beast".to_string(),
            "Empower Beast II".to_string(),
            "Empower Beast III".to_string(),
            "Empower Beast IV".to_string(),
            "Lucky Star I".to_string(),
            "Lucky Star II".to_string(),
            "Lucky Star III".to_string(),
            "Lucky Star IV".to_string(),
            "First Aid I (Stone)".to_string(),
            "First Aid II (Stone)".to_string(),
            "First Aid III (Stone)".to_string(),
            "First Aid IV (Stone)".to_string(),
            "First Aid I (Health)".to_string(),
            "First Aid II (Health)".to_string(),
            "First Aid III (Health)".to_string(),
            "First Aid IV (Health)".to_string(),
            "Meditate I".to_string(),
            "Meditate II".to_string(),
            "Meditate III".to_string(),
            "Meditate IV".to_string(),
            "Iron Maiden I".to_string(),
            "Iron Maiden II".to_string(),
            "Iron Maiden III".to_string(),
            "Iron Maiden IV".to_string(),
            "Featherstep I".to_string(),
            "Featherstep II".to_string(),
            "Featherstep III".to_string(),
            "Featherstep IV".to_string(),
            "Howl I".to_string(),
            "Howl II".to_string(),
            "Howl III".to_string(),
            "Howl IV".to_string(),
            "Huapango Winds I".to_string(),
            "Huapango Winds II".to_string(),
            "Huapango Winds III".to_string(),
            "Huapango Winds IV".to_string(),
            "Glare I".to_string(),
            "Glare II".to_string(),
            "Glare III".to_string(),
            "Glare IV".to_string(),
            "Threaten I".to_string(),
            "Threaten II".to_string(),
            "Threaten III".to_string(),
            "Threaten IV".to_string(),
            "Check I".to_string(),
            "Check II".to_string(),
            "Check III".to_string(),
            "Check IV".to_string(),
            "Black Mucus I".to_string(),
            "Black Mucus II".to_string(),
            "Black Mucus III".to_string(),
            "Black Mucus IV".to_string(),
            "Bloody Gag I".to_string(),
            "Bloody Gag II".to_string(),
            "Bloody Gag III".to_string(),
            "Bloody Gag IV".to_string(),
            "Witch's Smile I".to_string(),
            "Witch's Smile II".to_string(),
            "Witch's Smile III".to_string(),
            "Witch's Smile IV".to_string(),
            "Irresistable Beauty I".to_string(),
            "Irresistable Beauty II".to_string(),
            "Irresistable Beauty III".to_string(),
            "Irresistable Beauty IV".to_string(),
            "Lament of the Dead I".to_string(),
            "Lament of the Dead II".to_string(),
            "Lament of the Dead III".to_string(),
            "Lament of the Dead IV".to_string(),
            "Rapier Glance I".to_string(),
            "Rapier Glance II".to_string(),
            "Rapier Glance III".to_string(),
            "Rapier Glance IV".to_string(),
            "Intimidate I".to_string(),
            "Intimidate II".to_string(),
            "Intimidate III".to_string(),
            "Intimidate IV".to_string(),
            "Aerial Resonance I".to_string(),
            "Aerial Resonance II".to_string(),
            "Aerial Resonance III".to_string(),
            "Aerial Resonance IV".to_string(),
            "Telluric Resonance I".to_string(),
            "Telluric Resonance II".to_string(),
            "Telluric Resonance III".to_string(),
            "Telluric Resonance IV".to_string(),
            "Charged Resonance I".to_string(),
            "Charged Resonance II".to_string(),
            "Charged Resonance III".to_string(),
            "Charged Resonance IV".to_string(),
            "Aquatic Resonance I".to_string(),
            "Aquatic Resonance II".to_string(),
            "Aquatic Resonance III".to_string(),
            "Aquatic Resonance IV".to_string(),
            "Blazing Resonance I".to_string(),
            "Blazing Resonance II".to_string(),
            "Blazing Resonance III".to_string(),
            "Blazing Resonance IV".to_string(),
            "Icy Resonance I".to_string(),
            "Icy Resonance II".to_string(),
            "Icy Resonance III".to_string(),
            "Icy Resonance IV".to_string(),
            "Luminous Resonance I".to_string(),
            "Luminous Resonance II".to_string(),
            "Luminous Resonance III".to_string(),
            "Luminous Resonance IV".to_string(),
            "Shadow Resonance I".to_string(),
            "Shadow Resonance II".to_string(),
            "Shadow Resonance III".to_string(),
            "Shadow Resonance IV".to_string(),
            "Aqua Veil".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Preempt".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Eagle Eye".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evil's Bane".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mighty Strike".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Evade".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Gordian Lock I".to_string(),
            "Gordian Lock II".to_string(),
            "Gordian Lock III".to_string(),
            "Gordian Lock IV".to_string(),
            "Engulf I".to_string(),
            "Engulf II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Princess Whim I".to_string(),
            "Princess Whim II".to_string(),
            "Princess Whim III".to_string(),
            "Princess Whim IV".to_string(),
            "Magic Time!".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Guardian Force".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "El Colas Winds".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Intercession".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Course Correction".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Golem's Bane".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Conserve MP".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Conserve RT".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Consecrate Edge".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Salvation".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mind's Eye I".to_string(),
            "Mind's Eye II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Stardust Grace".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Speedstar".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Concentration (Ninjutsu)".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mother's Mercy".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mother's Blessing".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Nature's Touch".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Echoing Voice".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Sharpshoot".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Double Impact".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Double Shot".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Resounding Voice".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Fearful Impact".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dash I".to_string(),
            "Dash II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dragonslayer".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dragon's Eye".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Dragon's Scale I".to_string(),
            "Dragon's Scale II".to_string(),
            "Dragon's Scale III".to_string(),
            "Dragon's Scale IV".to_string(),
            "Dragonsbane".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Tremendous Shot".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Berserk".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Back Attack".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Paralysis Blade".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Beastslayer".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Beastbane".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Vigorous Attack".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Phalanx I".to_string(),
            "Phalanx II".to_string(),
            "Phalanx III".to_string(),
            "Phalanx IV".to_string(),
            "Steelstance I".to_string(),
            "Steelstance II".to_string(),
            "Steelstance III".to_string(),
            "Steelstance IV".to_string(),
            "Sanguine Assault".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Break Curse".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Broaden Force".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Velocity Shift".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mighty Impact".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Apostate I".to_string(),
            "Apostate II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Risk Management".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Reflection".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Repel Dragon I".to_string(),
            "Repel Dragon II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Repel Beast I".to_string(),
            "Repel Beast II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Ivory Tower I".to_string(),
            "Ivory Tower II".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Concentration (Magic)".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Smoke Screen I".to_string(),
            "Smoke Screen II".to_string(),
            "Smoke Screen III".to_string(),
            "Smoke Screen IV".to_string(),
            "Lobber I".to_string(),
            "Lobber II".to_string(),
            "Lobber III".to_string(),
            "Lobber IV".to_string(),
            "Falling Blade".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Pincer Attack".to_string(),
            "Pincer Attack II".to_string(),
            "Pincer Attack III".to_string(),
            "Pincer Attack IV".to_string(),
            "Exhalito (Void)".to_string(),
            "Procella (Wind)".to_string(),
            "Pondus (Earth)".to_string(),
            "Tonitrus (Lightning)".to_string(),
            "Coctura (Water)".to_string(),
            "Diruptio (Fire)".to_string(),
            "Congelatio (Ice)".to_string(),
            "Radius (Light)".to_string(),
            "Umbra (Dark)".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "Name".to_string(),
                offset: 88,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_names()),
            },
            FieldDefinition {
                name: "Description".to_string(),
                offset: 90,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_names()),
            },
            FieldDefinition {
                name: "Skill Type".to_string(),
                offset: 0,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Ability Effect".to_string(),
                offset: 2,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Ability Type".to_string(),
                offset: 4,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Rank Effect/Type".to_string(),
                offset: 5,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "SP Cost".to_string(),
                offset: 6,
                size: 2,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Prerequisite Skill".to_string(),
                offset: 8,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_names()),
            },
            FieldDefinition {
                name: "EXP Rate".to_string(),
                offset: 10,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Exclusion Group".to_string(),
                offset: 11,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Auto Activation %".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Enemy Skillset".to_string(),
                offset: 14,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Warrior".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Archer".to_string(),
                offset: 16,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Wizard".to_string(),
                offset: 17,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cleric".to_string(),
                offset: 18,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Rune Fencer".to_string(),
                offset: 19,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Knight".to_string(),
                offset: 20,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Terror Knight".to_string(),
                offset: 21,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Berserker".to_string(),
                offset: 22,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Swordmaster".to_string(),
                offset: 23,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Dragoon".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Ninja".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Rogue".to_string(),
                offset: 26,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Fusilier".to_string(),
                offset: 27,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Beast Tamer".to_string(),
                offset: 28,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Warlock".to_string(),
                offset: 29,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Necromancer".to_string(),
                offset: 30,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Lich".to_string(),
                offset: 31,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Divine Knight".to_string(),
                offset: 32,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Hoplite".to_string(),
                offset: 33,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Juggernaut".to_string(),
                offset: 34,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Patriarch".to_string(),
                offset: 35,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Familiar".to_string(),
                offset: 36,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Gremlin".to_string(),
                offset: 37,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Faerie".to_string(),
                offset: 38,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unused".to_string(),
                offset: 39,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Pumpkin".to_string(),
                offset: 40,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cloud Dragon".to_string(),
                offset: 41,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Earth Dragon".to_string(),
                offset: 42,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Thunder Dragon".to_string(),
                offset: 43,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Flood Dragon".to_string(),
                offset: 44,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Flame Dragon".to_string(),
                offset: 45,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Frost Dragon".to_string(),
                offset: 46,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Arc Dragon".to_string(),
                offset: 47,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Dark Dragon".to_string(),
                offset: 48,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Hydra".to_string(),
                offset: 49,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Clay Golem".to_string(),
                offset: 50,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Stone Golem".to_string(),
                offset: 51,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Iron Golem".to_string(),
                offset: 52,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Baldur Golem".to_string(),
                offset: 53,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Gryphon".to_string(),
                offset: 54,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cockatrice".to_string(),
                offset: 55,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Octopus".to_string(),
                offset: 56,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Cylops".to_string(),
                offset: 57,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Lord".to_string(),
                offset: 58,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Ranger".to_string(),
                offset: 59,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Priest".to_string(),
                offset: 60,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Dark Priest".to_string(),
                offset: 61,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Princess".to_string(),
                offset: 62,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Paladin".to_string(),
                offset: 63,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Astromancer".to_string(),
                offset: 64,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Vartan".to_string(),
                offset: 65,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "White Knight".to_string(),
                offset: 66,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Shaman".to_string(),
                offset: 67,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Wicce".to_string(),
                offset: 68,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Songstress".to_string(),
                offset: 69,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Buccaneer".to_string(),
                offset: 70,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Knight Commander".to_string(),
                offset: 71,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Scylla".to_string(),
                offset: 72,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Phoenix".to_string(),
                offset: 73,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Ruhk".to_string(),
                offset: 74,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unused".to_string(),
                offset: 75,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unused".to_string(),
                offset: 76,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unused".to_string(),
                offset: 77,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "List Order".to_string(),
                offset: 80,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Skill Catagory".to_string(),
                offset: 82,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Learnable".to_string(),
                offset: 83,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_use()),
            },
            FieldDefinition {
                name: "Rank Group".to_string(),
                offset: 86,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Learnable 2".to_string(),
                offset: 92,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_use()),
            },
            FieldDefinition {
                name: "Unknown 93".to_string(),
                offset: 93,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 94".to_string(),
                offset: 94,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 95".to_string(),
                offset: 95,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 96".to_string(),
                offset: 96,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 97".to_string(),
                offset: 97,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 98".to_string(),
                offset: 98,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 99".to_string(),
                offset: 99,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 100".to_string(),
                offset: 100,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 101".to_string(),
                offset: 101,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metadata() {
        let def = definition();
        assert_eq!(def.id, "skill_access");
        assert_eq!(def.base_offset, 0x42B982);
        assert_eq!(def.entry_count, 800);
        assert_eq!(def.entry_size, 102);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 89);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 800);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Name");
        assert_eq!(f.offset, 88);
        assert_eq!(f.size, 2);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[88];
        assert_eq!(f.name, "Unknown 101");
        assert_eq!(f.offset, 101);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn fields_within_entry_bounds() {
        let def = definition();
        for field in &def.fields {
            assert!(
                field.offset + field.size <= def.entry_size,
                "field '{}' at offset {} + size {} exceeds entry_size {}",
                field.name,
                field.offset,
                field.size,
                def.entry_size
            );
        }
    }
}
