use crate::modules::types::*;

fn opt_finisher_type() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Debuff".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Fists".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Daggers".to_string(),
        },
        FieldOption {
            value: 24,
            label: "1H Swords".to_string(),
        },
        FieldOption {
            value: 25,
            label: "2H Swords".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Axe".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Spear".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Hammer".to_string(),
        },
        FieldOption {
            value: 31,
            label: "1H Katana".to_string(),
        },
        FieldOption {
            value: 32,
            label: "2H Katana".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Cudgel".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Whip".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Spellbook".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Instrument".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Blowgun".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Bow".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Crossbow".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Gun".to_string(),
        },
    ]
}

fn opt_spell_description() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Tornado".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Aeroflux".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Aerogaurd".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Whirlwind".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Gaurding Gale".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Balmy Breeze".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Black Williwaw".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Petrogaurd".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Protect".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Blade Ward".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Duststorm".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Thunderflare".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Electricgaurd".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Galvanize".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Stormspark".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Stunbomb".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Stunslay".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Aquablast".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Aqaugaurd".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Quench".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Stagnate".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Poison Mist".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Sludgebind".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Firestorm".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Salamander".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Pryogaurd".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Flame Fusion".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Pyrocrlastic Flow".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Misery".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Brimstone".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Ice Requiem".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Frost Gaurd".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Icy Focus".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Indomitable Will".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Numbing Cold".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Freezing Gust".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Spiritsurge".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Judgement".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Light guard".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Silent Light".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Innervate".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Singing Light".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Awaken Stone".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Liberate".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Cleanse".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Unburden".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Decurse".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Hearten".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Ease".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Major Heal".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Resurrect".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Word of Pain".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Meteor Strike".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Hellbound".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Drain Power".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Shadow Gaurd".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Paralytic Wave".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Deadly Poison".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Dominate".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Shackle".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Fixate".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Gravity Flux".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Deadscream".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Dead Mans Ivy".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Annihilation".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Iceover".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Diablo's Spite".to_string(),
        },
        FieldOption {
            value: 116,
            label: "None".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Detect".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Springboard".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Nonee".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Gift of Restoration".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Negate Spell".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Dodge Blades".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Ballistics".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Enlighten".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Phantom Shell".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Sacrifice".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Tainted Love".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Prodigize".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Breed Suspicion".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Phantom Pain".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Brain Rot".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Wind Dervish".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Sand Spider".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 151,
            label: "None".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Bedeviling Dance".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Invirogating Dance".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Demonpetal Dance".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Ardent Conga".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Stiring Folclore".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Escalating Sanat".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Flaming Fists".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Rapid Strike".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Howling Rage".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Retribution".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Heart Crusher".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Shadowpin".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Double Fang".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Overwhelm".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Rending Gale".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Vie Wound".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Cherry Ronde".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Papllion Reel".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Sonic Blade".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Lightning Strike".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Cyclone Saber".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Grand Cross".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Mistral Edge".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Ice Prison".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Mantis Strike".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Infinity".to_string(),
        },
        FieldOption {
            value: 193,
            label: "None".to_string(),
        },
        FieldOption {
            value: 194,
            label: "None".to_string(),
        },
        FieldOption {
            value: 195,
            label: "None".to_string(),
        },
        FieldOption {
            value: 196,
            label: "None".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Ruination".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Schthe Wind".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Giga Tempest".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Spiral Scourge".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Tyrant's Mace".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Gaia Sunder".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Crimson Reach".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Dancing Sprite".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Dark Blade".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Thunderwave".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Swallow Slash".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Advent Sign".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Stonebloom".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Skyrend".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Ghostwail".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Sunblossom".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Wrathful Strike".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Raining Blows".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Pressure Whirl".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Trinity Pulse".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Flood Lash".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Wrenching Coil".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Swift Thrash".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Armageddon".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Raging Pummel".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Disembrain".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Eviscerate".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Devastate".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Aggressive Rendition".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Harmonic Blast".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Torrential Rhapsody".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Forced Fermata".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Frigid Blast".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Scorpion Shot".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Heaven's Scorn".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Venom Sting".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Dark Weight".to_string(),
        },
        FieldOption {
            value: 234,
            label: "Slumber Shot".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Flaming Blast".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Empyreal Shot".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Brimstone Hail".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Dullbind".to_string(),
        },
        FieldOption {
            value: 239,
            label: "Deathwail".to_string(),
        },
        FieldOption {
            value: 240,
            label: "Sanctus Flare".to_string(),
        },
        FieldOption {
            value: 241,
            label: "Mirage Strike".to_string(),
        },
        FieldOption {
            value: 242,
            label: "Rapid Blast".to_string(),
        },
        FieldOption {
            value: 243,
            label: "Scatter Shot".to_string(),
        },
        FieldOption {
            value: 244,
            label: "Atonement".to_string(),
        },
        FieldOption {
            value: 245,
            label: "Fiery Death".to_string(),
        },
        FieldOption {
            value: 246,
            label: "Angel of Death".to_string(),
        },
        FieldOption {
            value: 247,
            label: "Venomous Strike".to_string(),
        },
        FieldOption {
            value: 248,
            label: "Crushing Blow".to_string(),
        },
        FieldOption {
            value: 249,
            label: "Tempest Blade".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Demon Rose".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Dark Prison".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Apocalypse".to_string(),
        },
    ]
}

fn opt_spell_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Deadshot II".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Deadshot III".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Deadshot IV".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Tornado I".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Tornado II".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Tornado III".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Tornado IV".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Sylphide II".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Aero Flux".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Aero Flux II".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Aerogaurd".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Whirlwind".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Gaurding Gale".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Balmy Breeze".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Black Williwaw".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Vulcan Lance II".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Vulcan Lance III".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Vulcan Lance IV".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Cragfall II".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Cragfall III".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Cragfall IV".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Gnome II".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Earthqauke II".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Petrogaurd".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Protect".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Blade Ward".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Duststorm".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Lightning Bow II".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Lightning Bow III".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Lightning Bow IV".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Thunderflare I".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Thunderflare II".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Thunderflare III".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Thunderflare IV".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Thunderbird II".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Thunderburst II".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Electricgaurd".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Galvanize".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Stormspark".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Stunbomb".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Stunslay".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Aqaublast".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Aqaublast II".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Aquablast III".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Aqaublast IV".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Acid Rain II".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Acid Rain III".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Acid Rain IV".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Undine II".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Dread Vapor II".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Aqaugaurd".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Quench".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Stagnate".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Poison Mist".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Sludgebind".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Sparksphere II".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Sparksphere III".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Sparksphere IV".to_string(),
        },
        FieldOption {
            value: 77,
            label: "FireStorm".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Firstorm II".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Firestorm III".to_string(),
        },
        FieldOption {
            value: 80,
            label: "FireStorm IV".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Salamnder".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Salamander II".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Supernova II".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Pryogaurd".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Flame Fusion".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Pyrocrlastic Flow".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Misery".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Brimstone".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Iceblast II".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Iceblast III".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Iceblast IV".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Avalanche II".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Avalanche III".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Avalanche IV".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Wendigo II".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Ice Reqium".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Ice Reqium II".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Frost Gaurd".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Icy Focus".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Indomitable Will".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Numbing Cold".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Freezing Gust".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Spirit Surge".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Spirit Surge II".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Spirit Surge III".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Spirit Surge IV".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Judgement I".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Judgement II".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Judgement III".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Judgement IV".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Wisplight II".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Heavenly Judge II".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Excorsim II".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Light guard".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Silent Light".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Awaken II".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Innervate".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Singing Light".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Awaken Stone".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Liberate".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Cleanse".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Cleanse II".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Unburden".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Decurse".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Hearten".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Ease".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Heal II".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Heal III".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Heal IV".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Major heal I".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Major Heal II".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Major Heal III".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Ressurct I".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Ressurect II".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Word of pain I".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Word of pain II".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Word of pain III".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Word of pain IV".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Meteor Strike I".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Meteor Strike II".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Meteor Strike III".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Meteor Strike IV".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Hellhound".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Hellhound II".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Abyss II".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Drain Power".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Shadow Gaurd".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Paralytics Wave".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Deadly Poison".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Dominate".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Shackle".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Fixate".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Gravity Flux".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Deadscream".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Dead Mans Ivy".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Tempest II".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Gaia Strike II".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Vortex II".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Deluge II".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Annihiliation".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Annihilation II".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Ice Over".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Ice Over II".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Starfall II".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Diablos Spite".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Diablos Spite II".to_string(),
        },
        FieldOption {
            value: 197,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Detect".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Spring Board".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 201,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Gift of Restoration".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Negate Spell".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Dodge Blades".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Ballistics".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Enlighten".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Phantom Shell".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Sacrafice".to_string(),
        },
        FieldOption {
            value: 212,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Curse II".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Curse III".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Tainted Love".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Prodigize".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Breed Suspicion".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Phantom Pain".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Putrify II".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Brain Rot".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Wind Dervish I".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Wind Dervish II".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Sand Spider I".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Sand Spider II".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 234,
            label: "Chimaera II".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Water Tiger II".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Fire Snake II".to_string(),
        },
        FieldOption {
            value: 239,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 240,
            label: "Rime Raven II".to_string(),
        },
        FieldOption {
            value: 241,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 242,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 243,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 244,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 245,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 246,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 247,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 248,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 249,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 253,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 254,
            label: "Bedvelling Dance".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Envigorating Dance".to_string(),
        },
        FieldOption {
            value: 256,
            label: "Demon Petal Dance".to_string(),
        },
        FieldOption {
            value: 257,
            label: "Argent Conga".to_string(),
        },
        FieldOption {
            value: 258,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 259,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 260,
            label: "Stirring Folclore".to_string(),
        },
        FieldOption {
            value: 261,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 262,
            label: "Escalating Sanata".to_string(),
        },
        FieldOption {
            value: 263,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 264,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 265,
            label: "HP Heal 75".to_string(),
        },
        FieldOption {
            value: 266,
            label: "HP Heal 150".to_string(),
        },
        FieldOption {
            value: 267,
            label: "HP Heal 225".to_string(),
        },
        FieldOption {
            value: 268,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 269,
            label: "HP Heal 300".to_string(),
        },
        FieldOption {
            value: 270,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 271,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 272,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 273,
            label: "HP Heal 50%".to_string(),
        },
        FieldOption {
            value: 274,
            label: "HP Heal 75%".to_string(),
        },
        FieldOption {
            value: 275,
            label: "HP Heal 75%".to_string(),
        },
        FieldOption {
            value: 276,
            label: "HP Heal 100%".to_string(),
        },
        FieldOption {
            value: 277,
            label: "MP Charge 20".to_string(),
        },
        FieldOption {
            value: 278,
            label: "MP Charge 40".to_string(),
        },
        FieldOption {
            value: 279,
            label: "MP Charge 60".to_string(),
        },
        FieldOption {
            value: 280,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 281,
            label: "MP Charge 80".to_string(),
        },
        FieldOption {
            value: 282,
            label: "MP Charge 100".to_string(),
        },
        FieldOption {
            value: 283,
            label: "MP Charge 175".to_string(),
        },
        FieldOption {
            value: 284,
            label: "MP Charge 200".to_string(),
        },
        FieldOption {
            value: 285,
            label: "MP Charge 50%".to_string(),
        },
        FieldOption {
            value: 286,
            label: "MP Charge 75%".to_string(),
        },
        FieldOption {
            value: 287,
            label: "MP Charge 75%".to_string(),
        },
        FieldOption {
            value: 288,
            label: "MP Charge 100%".to_string(),
        },
        FieldOption {
            value: 289,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 290,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 291,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 292,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 293,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 294,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 295,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 296,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 297,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 298,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 299,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 300,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 301,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 302,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 303,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 304,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 305,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 306,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 307,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 308,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 309,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 310,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 311,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 312,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 313,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 314,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 315,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 316,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 317,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 318,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 319,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 320,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 321,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 322,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 323,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 324,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 325,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 326,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 327,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 328,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 329,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 330,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 331,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 332,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 333,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 334,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 335,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 336,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 337,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 338,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 339,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 340,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 341,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 342,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 343,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 344,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 345,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 346,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 347,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 348,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 349,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 350,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 351,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 352,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 353,
            label: "Bodysnatch".to_string(),
        },
        FieldOption {
            value: 354,
            label: "Snapdragon".to_string(),
        },
        FieldOption {
            value: 355,
            label: "355".to_string(),
        },
        FieldOption {
            value: 356,
            label: "356".to_string(),
        },
        FieldOption {
            value: 357,
            label: "357".to_string(),
        },
        FieldOption {
            value: 358,
            label: "358".to_string(),
        },
        FieldOption {
            value: 359,
            label: "359".to_string(),
        },
        FieldOption {
            value: 360,
            label: "360".to_string(),
        },
        FieldOption {
            value: 361,
            label: "361".to_string(),
        },
        FieldOption {
            value: 362,
            label: "362".to_string(),
        },
        FieldOption {
            value: 363,
            label: "363".to_string(),
        },
        FieldOption {
            value: 364,
            label: "364".to_string(),
        },
        FieldOption {
            value: 365,
            label: "365".to_string(),
        },
        FieldOption {
            value: 366,
            label: "366".to_string(),
        },
        FieldOption {
            value: 367,
            label: "367".to_string(),
        },
        FieldOption {
            value: 368,
            label: "368".to_string(),
        },
        FieldOption {
            value: 369,
            label: "369".to_string(),
        },
        FieldOption {
            value: 370,
            label: "370".to_string(),
        },
        FieldOption {
            value: 371,
            label: "371".to_string(),
        },
        FieldOption {
            value: 372,
            label: "372".to_string(),
        },
        FieldOption {
            value: 373,
            label: "373".to_string(),
        },
        FieldOption {
            value: 374,
            label: "374".to_string(),
        },
        FieldOption {
            value: 375,
            label: "375".to_string(),
        },
        FieldOption {
            value: 376,
            label: "376".to_string(),
        },
        FieldOption {
            value: 377,
            label: "377".to_string(),
        },
        FieldOption {
            value: 378,
            label: "378".to_string(),
        },
        FieldOption {
            value: 379,
            label: "379".to_string(),
        },
        FieldOption {
            value: 380,
            label: "380".to_string(),
        },
        FieldOption {
            value: 381,
            label: "381".to_string(),
        },
        FieldOption {
            value: 382,
            label: "382".to_string(),
        },
        FieldOption {
            value: 383,
            label: "383".to_string(),
        },
        FieldOption {
            value: 384,
            label: "384".to_string(),
        },
        FieldOption {
            value: 385,
            label: "385".to_string(),
        },
        FieldOption {
            value: 386,
            label: "386".to_string(),
        },
        FieldOption {
            value: 387,
            label: "387".to_string(),
        },
        FieldOption {
            value: 388,
            label: "Envigorate".to_string(),
        },
        FieldOption {
            value: 389,
            label: "None".to_string(),
        },
        FieldOption {
            value: 390,
            label: "None".to_string(),
        },
        FieldOption {
            value: 391,
            label: "Flaming Fists".to_string(),
        },
        FieldOption {
            value: 392,
            label: "Rapid Strike".to_string(),
        },
        FieldOption {
            value: 393,
            label: "Howling Rage".to_string(),
        },
        FieldOption {
            value: 394,
            label: "Retribution".to_string(),
        },
        FieldOption {
            value: 395,
            label: "Envigorate".to_string(),
        },
        FieldOption {
            value: 396,
            label: "Heart Crusher".to_string(),
        },
        FieldOption {
            value: 397,
            label: "Shadowpin".to_string(),
        },
        FieldOption {
            value: 398,
            label: "Double Fang".to_string(),
        },
        FieldOption {
            value: 399,
            label: "Overwhelm".to_string(),
        },
        FieldOption {
            value: 400,
            label: "Tempest Blade".to_string(),
        },
        FieldOption {
            value: 401,
            label: "Rending Gale".to_string(),
        },
        FieldOption {
            value: 402,
            label: "Vie Wound".to_string(),
        },
        FieldOption {
            value: 403,
            label: "Cherry Ronde".to_string(),
        },
        FieldOption {
            value: 404,
            label: "Papllion Reel".to_string(),
        },
        FieldOption {
            value: 405,
            label: "Vanomous Strike".to_string(),
        },
        FieldOption {
            value: 406,
            label: "Sonic Blade".to_string(),
        },
        FieldOption {
            value: 407,
            label: "Lightning Strike".to_string(),
        },
        FieldOption {
            value: 408,
            label: "Cyclone Saber".to_string(),
        },
        FieldOption {
            value: 409,
            label: "Grand Cross".to_string(),
        },
        FieldOption {
            value: 410,
            label: "Crushing Blow".to_string(),
        },
        FieldOption {
            value: 411,
            label: "Mistral Edge".to_string(),
        },
        FieldOption {
            value: 412,
            label: "Ice Prison".to_string(),
        },
        FieldOption {
            value: 413,
            label: "Mantis Strike".to_string(),
        },
        FieldOption {
            value: 414,
            label: "Infinity".to_string(),
        },
        FieldOption {
            value: 415,
            label: "Dark Prison".to_string(),
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
            label: "Ruination".to_string(),
        },
        FieldOption {
            value: 422,
            label: "Schthe Wind".to_string(),
        },
        FieldOption {
            value: 423,
            label: "Giga Tempest".to_string(),
        },
        FieldOption {
            value: 424,
            label: "Spiral Scourge".to_string(),
        },
        FieldOption {
            value: 425,
            label: "Fiery Death".to_string(),
        },
        FieldOption {
            value: 426,
            label: "Tyrant's Mace".to_string(),
        },
        FieldOption {
            value: 427,
            label: "Gaia Sunder".to_string(),
        },
        FieldOption {
            value: 428,
            label: "Crimson Reach".to_string(),
        },
        FieldOption {
            value: 429,
            label: "Dancing Sprite".to_string(),
        },
        FieldOption {
            value: 430,
            label: "Angel of Death".to_string(),
        },
        FieldOption {
            value: 431,
            label: "Weaken".to_string(),
        },
        FieldOption {
            value: 432,
            label: "Breach".to_string(),
        },
        FieldOption {
            value: 433,
            label: "Spoilspell".to_string(),
        },
        FieldOption {
            value: 434,
            label: "Spoilheal".to_string(),
        },
        FieldOption {
            value: 435,
            label: "Enfeeble".to_string(),
        },
        FieldOption {
            value: 436,
            label: "Dark Blade".to_string(),
        },
        FieldOption {
            value: 437,
            label: "Thunderwave".to_string(),
        },
        FieldOption {
            value: 438,
            label: "Swallow Slash".to_string(),
        },
        FieldOption {
            value: 439,
            label: "Advent Sign".to_string(),
        },
        FieldOption {
            value: 440,
            label: "Apocalypse".to_string(),
        },
        FieldOption {
            value: 441,
            label: "Stonebloom".to_string(),
        },
        FieldOption {
            value: 442,
            label: "Skyrend".to_string(),
        },
        FieldOption {
            value: 443,
            label: "Ghostwail".to_string(),
        },
        FieldOption {
            value: 444,
            label: "Sunblossom".to_string(),
        },
        FieldOption {
            value: 445,
            label: "None".to_string(),
        },
        FieldOption {
            value: 446,
            label: "Wrathful Strike".to_string(),
        },
        FieldOption {
            value: 447,
            label: "Raining Blows".to_string(),
        },
        FieldOption {
            value: 448,
            label: "Pressure Whirl".to_string(),
        },
        FieldOption {
            value: 449,
            label: "Trinity Pulse".to_string(),
        },
        FieldOption {
            value: 450,
            label: "None".to_string(),
        },
        FieldOption {
            value: 451,
            label: "Falsestrike".to_string(),
        },
        FieldOption {
            value: 452,
            label: "Stagger".to_string(),
        },
        FieldOption {
            value: 453,
            label: "Falseflight".to_string(),
        },
        FieldOption {
            value: 454,
            label: "Misstep".to_string(),
        },
        FieldOption {
            value: 455,
            label: "Spellslip".to_string(),
        },
        FieldOption {
            value: 456,
            label: "Flood Lash".to_string(),
        },
        FieldOption {
            value: 457,
            label: "Wrenching Coil".to_string(),
        },
        FieldOption {
            value: 458,
            label: "Swift Thrash".to_string(),
        },
        FieldOption {
            value: 459,
            label: "Armageddon".to_string(),
        },
        FieldOption {
            value: 460,
            label: "Demon Rose".to_string(),
        },
        FieldOption {
            value: 461,
            label: "Raging Pummel".to_string(),
        },
        FieldOption {
            value: 462,
            label: "DisemBrain".to_string(),
        },
        FieldOption {
            value: 463,
            label: "Eviscerate".to_string(),
        },
        FieldOption {
            value: 464,
            label: "Devastate".to_string(),
        },
        FieldOption {
            value: 465,
            label: "None".to_string(),
        },
        FieldOption {
            value: 466,
            label: "Aggressive Rendition".to_string(),
        },
        FieldOption {
            value: 467,
            label: "Harmonic Blast".to_string(),
        },
        FieldOption {
            value: 468,
            label: "Torrential Rhapsody".to_string(),
        },
        FieldOption {
            value: 469,
            label: "Forced Fermata".to_string(),
        },
        FieldOption {
            value: 470,
            label: "None".to_string(),
        },
        FieldOption {
            value: 471,
            label: "Frigid Blast".to_string(),
        },
        FieldOption {
            value: 472,
            label: "Scorpion Shot".to_string(),
        },
        FieldOption {
            value: 473,
            label: "Venom Sting".to_string(),
        },
        FieldOption {
            value: 474,
            label: "Heaven's Scorn".to_string(),
        },
        FieldOption {
            value: 475,
            label: "None".to_string(),
        },
        FieldOption {
            value: 476,
            label: "Dark Weight".to_string(),
        },
        FieldOption {
            value: 477,
            label: "Slumber Shot".to_string(),
        },
        FieldOption {
            value: 478,
            label: "Flaming Blast".to_string(),
        },
        FieldOption {
            value: 479,
            label: "Empyreal Shot".to_string(),
        },
        FieldOption {
            value: 480,
            label: "None".to_string(),
        },
        FieldOption {
            value: 481,
            label: "Brimstone Hail".to_string(),
        },
        FieldOption {
            value: 482,
            label: "Dullbind".to_string(),
        },
        FieldOption {
            value: 483,
            label: "Deathwail".to_string(),
        },
        FieldOption {
            value: 484,
            label: "Sanctus Flare".to_string(),
        },
        FieldOption {
            value: 485,
            label: "None".to_string(),
        },
        FieldOption {
            value: 486,
            label: "Mirage Strike".to_string(),
        },
        FieldOption {
            value: 487,
            label: "Rapid Blast".to_string(),
        },
        FieldOption {
            value: 488,
            label: "Scatter Shot".to_string(),
        },
        FieldOption {
            value: 489,
            label: "Atonement".to_string(),
        },
        FieldOption {
            value: 490,
            label: "None".to_string(),
        },
        FieldOption {
            value: 491,
            label: "Fiery Death".to_string(),
        },
        FieldOption {
            value: 492,
            label: "Angel of Death".to_string(),
        },
        FieldOption {
            value: 493,
            label: "Venomous Strike".to_string(),
        },
        FieldOption {
            value: 494,
            label: "Crushing Blow".to_string(),
        },
        FieldOption {
            value: 495,
            label: "Tempest Blade".to_string(),
        },
        FieldOption {
            value: 496,
            label: "Demon Rose".to_string(),
        },
        FieldOption {
            value: 497,
            label: "Dark Prison".to_string(),
        },
        FieldOption {
            value: 498,
            label: "Apocalypse".to_string(),
        },
        FieldOption {
            value: 499,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 500,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 501,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 502,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 503,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 504,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 505,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 506,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 507,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 508,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 509,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 510,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 511,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 706,
            label: "Ease II".to_string(),
        },
    ]
}

fn opt_status_effects() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Deceased".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Stilled".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Near Death".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Asleep".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Charmed".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Bewitched".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Spendthrift".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Paranoia".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Stunned".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Silenced".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Petrified".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Bound".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Shackled".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Stopped".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Poisoned".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Envenomed".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Hobbled".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Leadened".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Withered".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Addled".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Cursed".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Frightened".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Quickened".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Slowed".to_string(),
        },
        FieldOption {
            value: 25,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Nimble".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Waterwalk".to_string(),
        },
        FieldOption {
            value: 28,
            label: "NOT USED".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Lavawalk".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Blinkwalk".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Cloudwalk".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Windwalk".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Renewal".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Pain Aura".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Nullify".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Negate".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Sanctified".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Battering Ram".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Strengthened".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Weakened".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Spellcraft".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Spoilspell".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Healcraft".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Spoilheal".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Fortified".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Breached".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Resilient".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Enfeebled".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Truestrike".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Falsestrike".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Trueflight".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Falseflight".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Spellstrike".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Spellslip".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Dodge".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Stagger".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Sidestep".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Misstep".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Air Touched".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Earth Touched".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Lightning Touched".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Water Touched".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Fire Touched".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Ice Touched".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Light Touched".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Dark Touched".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Silence-bringer".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Stun-bringer".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Poison-bringer".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Air Attuned".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Earth Attuned".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Lightning Attuned".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Water Attuned".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Fire Attuned".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Ice Attuned".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Light Attuned".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Dark Attuned".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Air Averse".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Earth Averse".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Lightning Averse".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Water Averse".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Fire Averse".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Ice Averse".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Light Averse".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Dark Averse".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Mold".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Lava".to_string(),
        },
    ]
}

fn opt_damage_type() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Crushing".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Slashing".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Piercing".to_string(),
        },
    ]
}

fn opt_element() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Air".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Earth".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Lightning".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Fire".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Ice".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Light".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Dark".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Random".to_string(),
        },
    ]
}

fn opt_class_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Warrior".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Archer".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Wizard".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Cleric".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Rune Fencer".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Knight".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Terror Knight".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Berserker".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Swordmaster".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Dragoon".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Ninja".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Rogue".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Fusilier".to_string(),
        },
        FieldOption {
            value: 14,
            label: "BeastTamer".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Warlock".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Necromancer".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Lich".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Divine Knight".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Hoplite".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Juggernaut".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Patriarch".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Familiar".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Gremlin 1".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Gremlin 2".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Pumpkin".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Cloud Dragon".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Earth Dragon".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Thunder Dragon".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Flood Dragon".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Flame Dragon".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Frost Dragon".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Arc Dragon".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Dark Dragon".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Hydra".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Clay Golem".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Stone Golem".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Iron Golem".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Baldur Golem".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 44,
            label: "None".to_string(),
        },
        FieldOption {
            value: 45,
            label: "None".to_string(),
        },
        FieldOption {
            value: 46,
            label: "None".to_string(),
        },
        FieldOption {
            value: 47,
            label: "None".to_string(),
        },
        FieldOption {
            value: 48,
            label: "None".to_string(),
        },
        FieldOption {
            value: 49,
            label: "None".to_string(),
        },
        FieldOption {
            value: 50,
            label: "None".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Lord".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Ranger".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Priest".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Dark Priest".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Princess".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Paladin".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Astromancer".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Vartan".to_string(),
        },
        FieldOption {
            value: 59,
            label: "White Knight".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Shaman".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Wicce".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Songstress".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Buccaneer".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Knight Commander/Ozma".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Knight Commander/Balxephon".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Knight Commander/Volak".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Knight Commander/Barbas".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Knight Commander/Martym".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Knight Commander/Oz".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Knight Commander/Ozma".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Knight Commander/Andoras".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Death Templar".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Dark Bishop".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Assassin".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Death Knight".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Dark Lord 1".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Dark Lord 2".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Revenant".to_string(),
        },
        FieldOption {
            value: 79,
            label: "None".to_string(),
        },
        FieldOption {
            value: 80,
            label: "None".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Templar/Warrior".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Templar/Archer".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Templar/Wizard".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Templar/Cleric".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Templar/Rune Fencer".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Templar/Knight".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Templar/Terror Knight".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Templar/Berserker".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Templar/Swordmaster".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Templar/Dragoon".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Templar/Ninja".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Templar/Rogue".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Templar/Fusilier".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Templar/Beast Tamer".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Templar/Warlock".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Templar/Necromancer".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Sorcerer".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Fleeing Villager".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Fleeing Villager".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Fleeing Villager".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Vasque Survivor".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Hanged Man".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Spectre".to_string(),
        },
        FieldOption {
            value: 104,
            label: "None".to_string(),
        },
        FieldOption {
            value: 105,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Gladiator".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Blood Hunter".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Death Eater".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Cenobite".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Cannibal".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Shadow Knight".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Dreadnought".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Executioner".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Kill Seeker".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Crimson Uhlan".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Dark Stalker".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Grim Reaper".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Sniper".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Iron Fist".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Loremaster".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Witch King".to_string(),
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
            label: "Raven".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Lich King".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Ethereal Vision".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Night Crow".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Blood Gavial".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Gorgon".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Uruk".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Wight".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Wraith".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Duinshee".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Incubus".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Boggart".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 146,
            label: "None".to_string(),
        },
        FieldOption {
            value: 147,
            label: "None".to_string(),
        },
        FieldOption {
            value: 148,
            label: "None".to_string(),
        },
        FieldOption {
            value: 149,
            label: "None".to_string(),
        },
        FieldOption {
            value: 150,
            label: "None".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Scylla".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Naga".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Damasc Golem".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Flesh Golem".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Hippogryph".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Phoenix".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Rukh".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Basilisk".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Kraken".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Dagon".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Spriggan".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Titan".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Knight Commander/Ozma".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Knight Commander/Balxephon".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Knight Commander/Volaq".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Knight Commander/Barbas".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Knight Commander/Martym".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Knight Commander/Oz".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Knight Commander/Ozma".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Knight Commander/Andoras".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Death Templar".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Aym".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Aloser".to_string(),
        },
        FieldOption {
            value: 176,
            label: "None".to_string(),
        },
        FieldOption {
            value: 177,
            label: "None".to_string(),
        },
        FieldOption {
            value: 178,
            label: "None".to_string(),
        },
        FieldOption {
            value: 179,
            label: "None".to_string(),
        },
        FieldOption {
            value: 180,
            label: "None".to_string(),
        },
        FieldOption {
            value: 181,
            label: "None".to_string(),
        },
        FieldOption {
            value: 182,
            label: "None".to_string(),
        },
        FieldOption {
            value: 183,
            label: "None".to_string(),
        },
        FieldOption {
            value: 184,
            label: "None".to_string(),
        },
        FieldOption {
            value: 185,
            label: "None".to_string(),
        },
        FieldOption {
            value: 186,
            label: "None".to_string(),
        },
        FieldOption {
            value: 187,
            label: "None".to_string(),
        },
        FieldOption {
            value: 188,
            label: "None".to_string(),
        },
        FieldOption {
            value: 189,
            label: "None".to_string(),
        },
        FieldOption {
            value: 190,
            label: "None".to_string(),
        },
        FieldOption {
            value: 191,
            label: "None".to_string(),
        },
        FieldOption {
            value: 192,
            label: "None".to_string(),
        },
        FieldOption {
            value: 193,
            label: "None".to_string(),
        },
        FieldOption {
            value: 194,
            label: "None".to_string(),
        },
        FieldOption {
            value: 195,
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "None".to_string(),
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
            label: "Vija".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Enja".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Maitreya".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Ijana".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Chandra".to_string(),
        },
        FieldOption {
            value: 239,
            label: "Vayu".to_string(),
        },
        FieldOption {
            value: 240,
            label: "Indra".to_string(),
        },
        FieldOption {
            value: 241,
            label: "Rakshas".to_string(),
        },
        FieldOption {
            value: 242,
            label: "Ahurama".to_string(),
        },
        FieldOption {
            value: 243,
            label: "Asurama".to_string(),
        },
        FieldOption {
            value: 244,
            label: "Aditi".to_string(),
        },
        FieldOption {
            value: 245,
            label: "Saranga".to_string(),
        },
        FieldOption {
            value: 246,
            label: "Kandyce".to_string(),
        },
        FieldOption {
            value: 247,
            label: "Blackmoor".to_string(),
        },
        FieldOption {
            value: 248,
            label: "Sirene".to_string(),
        },
        FieldOption {
            value: 249,
            label: "Vainateya".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Nathalork".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Xolotl".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Tlaloc".to_string(),
        },
        FieldOption {
            value: 253,
            label: "Ifrit".to_string(),
        },
        FieldOption {
            value: 254,
            label: "Lygenstzel".to_string(),
        },
    ]
}

fn opt_spell_type() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Air".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Earth".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Lightning".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Fire".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Ice".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Divine".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Dark".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Dragonic".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Necromancy".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Ninjutsu".to_string(),
        },
        FieldOption {
            value: 12,
            label: "War Dances".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Songs".to_string(),
        },
    ]
}

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "finishers".to_string(),
        name: "Finisher Editor".to_string(),
        description: "Edit finisher move properties".to_string(),
        dat_file: DatFile::BattleData,
        base_offset: 0x3E0ECC,
        entry_count: 108,
        entry_size: 180,
        entry_names: vec![
            "Flaming Fists".to_string(),
            "Rapid Strike".to_string(),
            "Howling Rage".to_string(),
            "Retribution".to_string(),
            "Envigorate".to_string(),
            "Heart Crusher".to_string(),
            "Shadowpin".to_string(),
            "Double Fang".to_string(),
            "Overwhelm".to_string(),
            "Tempest Blade (Weapon)".to_string(),
            "Rending Gale".to_string(),
            "Vie Wound".to_string(),
            "Cherry Ronde".to_string(),
            "Papllion Reel".to_string(),
            "Venomous Strike (Weapon)".to_string(),
            "Sonic Blade".to_string(),
            "Lightning Strike".to_string(),
            "Cyclone Saber".to_string(),
            "Grand Cross".to_string(),
            "Crushing Blow (Weapon)".to_string(),
            "Mistral Edge".to_string(),
            "Ice Prison".to_string(),
            "Mantis Strike".to_string(),
            "Infinity".to_string(),
            "Dark Prison (Weapon)".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Ruination".to_string(),
            "Schthe Wind".to_string(),
            "Giga Tempest".to_string(),
            "Spiral Scourge".to_string(),
            "Fiery Death (Weapon)".to_string(),
            "Tyrant's Mace".to_string(),
            "Gaia Sunder".to_string(),
            "Crimson Reach".to_string(),
            "Dancing Sprite".to_string(),
            "Angel of Death (Weapon)".to_string(),
            "Weaken".to_string(),
            "Breach".to_string(),
            "Spoilspell".to_string(),
            "Spoilheal".to_string(),
            "Enfeeble".to_string(),
            "Dark Blade".to_string(),
            "Thunderwave".to_string(),
            "Swallow Slash".to_string(),
            "Advent Sign".to_string(),
            "Apocalypse (Weapon)".to_string(),
            "Stonebloom".to_string(),
            "Skyrend".to_string(),
            "Ghostwail".to_string(),
            "Sunblossom".to_string(),
            "None".to_string(),
            "Wrathful Strike".to_string(),
            "Raining Blows".to_string(),
            "Pressure Whirl".to_string(),
            "Trinity Pulse".to_string(),
            "None".to_string(),
            "Falsestrike".to_string(),
            "Stagger".to_string(),
            "Falseflight".to_string(),
            "Misstep".to_string(),
            "Spellslip".to_string(),
            "Flood Lash".to_string(),
            "Wrenching Coil".to_string(),
            "Swift Thrash".to_string(),
            "Armageddon".to_string(),
            "Demon Rose (Weapon)".to_string(),
            "Raging Pummel".to_string(),
            "Disembrain".to_string(),
            "Eviscerate".to_string(),
            "Devastate".to_string(),
            "None".to_string(),
            "Aggressive Rendition".to_string(),
            "Harmonic Blast".to_string(),
            "Torrential Rhapsody".to_string(),
            "Forced Fermata".to_string(),
            "None".to_string(),
            "Frigid Blast".to_string(),
            "Scorpion Shot".to_string(),
            "Heaven's Scorn".to_string(),
            "Venom Sting".to_string(),
            "None".to_string(),
            "Dark Weight".to_string(),
            "Slumber Shot".to_string(),
            "Flaming Blast".to_string(),
            "Empyreal Shot".to_string(),
            "None".to_string(),
            "Brimstone Hail".to_string(),
            "Dullbind".to_string(),
            "Deathwail".to_string(),
            "Sanctus Flare".to_string(),
            "None".to_string(),
            "Mirage Strike".to_string(),
            "Rapid Blast".to_string(),
            "Scatter Shot".to_string(),
            "Atonement".to_string(),
            "None".to_string(),
            "Fiery Death".to_string(),
            "Angel of Death".to_string(),
            "Venomous Strike".to_string(),
            "Crushing Blow".to_string(),
            "Tempest Blade".to_string(),
            "Demon Rose".to_string(),
            "Dark Prison".to_string(),
            "Apocalypse".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "Finisher Type".to_string(),
                offset: 2,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_finisher_type()),
            },
            FieldDefinition {
                name: "RT Penalty".to_string(),
                offset: 3,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 4".to_string(),
                offset: 4,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 5".to_string(),
                offset: 5,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 6".to_string(),
                offset: 6,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Cost Type".to_string(),
                offset: 7,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Cost".to_string(),
                offset: 8,
                size: 2,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Reagent Type Required".to_string(),
                offset: 10,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Reagent Type Required".to_string(),
                offset: 11,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Range Type".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Maximum Range -1".to_string(),
                offset: 14,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Minimum Range".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Area".to_string(),
                offset: 16,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Summon Related".to_string(),
                offset: 17,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Flags?".to_string(),
                offset: 18,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Null/0".to_string(),
                offset: 19,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Description".to_string(),
                offset: 20,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_description()),
            },
            FieldDefinition {
                name: "List Order".to_string(),
                offset: 22,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Unknown 24".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Flags".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 26".to_string(),
                offset: 26,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Unknown 29".to_string(),
                offset: 29,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 30".to_string(),
                offset: 30,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 31".to_string(),
                offset: 31,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 32".to_string(),
                offset: 32,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 33".to_string(),
                offset: 33,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 35".to_string(),
                offset: 35,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 36".to_string(),
                offset: 36,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 37".to_string(),
                offset: 37,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 38".to_string(),
                offset: 38,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 39".to_string(),
                offset: 39,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 40".to_string(),
                offset: 40,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 41".to_string(),
                offset: 41,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Modifier Effect 1".to_string(),
                offset: 42,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 43".to_string(),
                offset: 43,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "On Hit Effect 1".to_string(),
                offset: 44,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_status_effects()),
            },
            FieldDefinition {
                name: "Ressurection".to_string(),
                offset: 45,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Target Restrictions".to_string(),
                offset: 46,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Sets if next value used 0/2?".to_string(),
                offset: 47,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "00/42/64".to_string(),
                offset: 48,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "09/07/0B".to_string(),
                offset: 49,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Sets next lines".to_string(),
                offset: 50,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Bonus".to_string(),
                offset: 51,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 1".to_string(),
                offset: 52,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 2".to_string(),
                offset: 53,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 3".to_string(),
                offset: 54,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 4".to_string(),
                offset: 55,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 56".to_string(),
                offset: 56,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 57".to_string(),
                offset: 57,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 58".to_string(),
                offset: 58,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Affinity Modifier - Physical".to_string(),
                offset: 60,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_damage_type()),
            },
            FieldDefinition {
                name: "Affinity Modifier - Magical".to_string(),
                offset: 61,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_element()),
            },
            FieldDefinition {
                name: "Null/0".to_string(),
                offset: 62,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Sets Targeting Display".to_string(),
                offset: 63,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "0 for all but 32 for Steal".to_string(),
                offset: 64,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Modifier Effect 2".to_string(),
                offset: 68,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "On Hit Effect 2".to_string(),
                offset: 70,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_status_effects()),
            },
            FieldDefinition {
                name: "Unknown 71".to_string(),
                offset: 71,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Target Restrictions".to_string(),
                offset: 72,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Next value used 0/2?".to_string(),
                offset: 73,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Percentage if Set?00 /64?".to_string(),
                offset: 74,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "09/0D".to_string(),
                offset: 75,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Next Line Function".to_string(),
                offset: 76,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Bonus".to_string(),
                offset: 77,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling".to_string(),
                offset: 78,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 79".to_string(),
                offset: 79,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 2".to_string(),
                offset: 80,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 81".to_string(),
                offset: 81,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 3".to_string(),
                offset: 82,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 83".to_string(),
                offset: 83,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Damage Scaling 4".to_string(),
                offset: 84,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 87".to_string(),
                offset: 87,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 88".to_string(),
                offset: 88,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 89".to_string(),
                offset: 89,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always 14".to_string(),
                offset: 90,
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
                name: "Starfall/Heavenly Judge".to_string(),
                offset: 98,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Special Effect".to_string(),
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
                name: "Unknown 102".to_string(),
                offset: 102,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 104".to_string(),
                offset: 104,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 110".to_string(),
                offset: 110,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "07 Starfall".to_string(),
                offset: 113,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always 14".to_string(),
                offset: 116,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 120".to_string(),
                offset: 120,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 121".to_string(),
                offset: 121,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 122".to_string(),
                offset: 122,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 123".to_string(),
                offset: 123,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Class Lock".to_string(),
                offset: 124,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_class_list()),
            },
            FieldDefinition {
                name: "Unknown 125".to_string(),
                offset: 125,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 126".to_string(),
                offset: 126,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 127".to_string(),
                offset: 127,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 128".to_string(),
                offset: 128,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 129".to_string(),
                offset: 129,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Learn Level".to_string(),
                offset: 130,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Animation".to_string(),
                offset: 131,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Effect Animation".to_string(),
                offset: 132,
                size: 3,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always FF".to_string(),
                offset: 135,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always Sequential".to_string(),
                offset: 136,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "AoE's".to_string(),
                offset: 138,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "1E Auto Skills".to_string(),
                offset: 142,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always 10B".to_string(),
                offset: 144,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always 0A".to_string(),
                offset: 146,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always Sequential".to_string(),
                offset: 148,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Unknown 150".to_string(),
                offset: 150,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 152".to_string(),
                offset: 152,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Starfall/Heavely Judge".to_string(),
                offset: 154,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 158".to_string(),
                offset: 158,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "00/FF".to_string(),
                offset: 159,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Healing Spell".to_string(),
                offset: 160,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Spell Type 2?".to_string(),
                offset: 161,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_type()),
            },
            FieldDefinition {
                name: "Unknown 162".to_string(),
                offset: 162,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 164".to_string(),
                offset: 164,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 167".to_string(),
                offset: 167,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Summon Darkness".to_string(),
                offset: 168,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 169".to_string(),
                offset: 169,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always Sequential".to_string(),
                offset: 170,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Unknown 172".to_string(),
                offset: 172,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Effect Animation 2".to_string(),
                offset: 174,
                size: 3,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Always FF".to_string(),
                offset: 177,
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
        assert_eq!(def.id, "finishers");
        assert_eq!(def.dat_file, DatFile::BattleData);
        assert_eq!(def.base_offset, 0x3E0ECC);
        assert_eq!(def.entry_count, 108);
        assert_eq!(def.entry_size, 180);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 120);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 108);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Finisher Type");
        assert_eq!(f.offset, 2);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[119];
        assert_eq!(f.name, "Always FF");
        assert_eq!(f.offset, 177);
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
