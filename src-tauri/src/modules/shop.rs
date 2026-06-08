use crate::modules::types::*;

fn opt_shop_items() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Leather Caestus".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Leather Caestus +1".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Bronze Knuckles".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Bronze Knuckles +1".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Iron Claws".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Iron Claws +1".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Cat Bagh Nakh".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Cat Bagh Nakh +1".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Baldur Claws".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Baldur Claws +1".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Katara".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Katara +1".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Tiger Bagh Nakh".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Tiger Bagh Nakh +1".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Damascus Claws".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Damascus Claws +1".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Dragon Claws".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Dragon Blades".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Jamadhar".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Vishnu's Katara".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Hellbound Claws".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Kerberos Claws".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Daedalus Claws".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Vaisravana (Relic)".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Trueno's Scales (Special)".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Vainateya's Talons".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Huitzilopochtli's Rays (Relic)".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Sticker".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Sticker +1".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Battle Knife".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Battle Knife +1".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Dirk".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Dirk +1".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Butcher Knife".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Butcher Knife +1".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Baldur Dagger".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Baldur Dagger +1".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Kris".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Kris +1".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Kidney Spike".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Kidney Spike +1".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Damascus Dagger".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Damascus Dagger +1".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Valiant's Dagger".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Brilliant Dagger".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Marauder Knife".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Predator Knife".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Xolotl's Canine".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Yama (Relic)".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Dragon Fang".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Pinion Blade (Relic)".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Assassin's Blade (Relic)".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Short Sword".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Short Sword +1".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Gladius".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Gladius +1".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Rapier".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Rapier +1".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Shamshir".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Shamshir +1".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Baldur Sword".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Baldur Sword +1".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Cutlass".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Cutlass +1".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Khora".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Khora +1".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Damascus Sword".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Damascus Sword +1".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Walloon Sword".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Lightning Sword".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Ice Blade".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Isberg".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Kukri".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Fandango".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Nifrit Sword (Special)".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Lombardia (Prologue)".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Kumbhira (Relic)".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Odiferous Waster (Special)".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Ogre Blade (Special)".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Ambicion (Special)".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Brynhildr (Special)".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Daedalus Blade".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Oracion (Relic)".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Fafnir's Heart (Relic)".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Leksar's Beloved (Relic)".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Broadsword".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Broadsword +1".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Viking Sword".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Viking Sword +1".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Zweihander".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Zweihander +1".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Baldur Blade".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Baldur Blade +1".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Bastard Sword".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Bastard Sword +1".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Claymore".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Claymore +1".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Falx".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Falx +1".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Damascus Blade".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Damascus Blade +1".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Desert Blade".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Gaia Blade".to_string(),
        },
        FieldOption {
            value: 104,
            label: "The Headsman".to_string(),
        },
        FieldOption {
            value: 105,
            label: "The Dark Headsman".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Rhomphaia".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Grasshewer Blade".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Notos (Relic)".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Balmung (Special)".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Ishana (Relic)".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Durandal (Relic)".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Moon Blade (Relic)".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Hand Axe".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Hand Axe +1".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Battle Axe".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Battle Axe +1".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Heavy Axe".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Heavy Axe +1".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Baldur Axe".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Baldur Axe +1".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Tabar Zin".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Tabar Zin +1".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Chakmak".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Chakmak +1".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Guisarme".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Guisarme +1".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Damascus Axe".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Damascus Axe +1".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Balbriggan".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Trovaon".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Dragon Axe".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Terre Axe".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Glamrock (Special)".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Stardust".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Prox (Relic)".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Boreas (Relic)".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Kshuparaka (Relic)".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Shaytan's Bulova (Relic)".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Rune Axe (Relic)".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Bronze Spear".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Bronze Spear +1".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Xyston".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Xyston +1".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Voulge".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Voulge +1".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Baldur Spear".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Baldur Spear +1".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Scorpion".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Scorpion +1".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Trident".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Trident +1".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Bardiche".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Bardiche +1".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Damascus Spear".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Damascus Spear +1".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Poleaxe".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Hache".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Corne Licorne".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Holy Lance".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Volcaetus (Special)".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Ignis".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Zephyros (Relic)".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Bentisca (Relic)".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Tlaloc's Bolt".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Pavana (Relic)".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Dark Spear (Relic)".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Halt Hammer".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Halt Hammer +1".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Caldia".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Iron Fan".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Morning Star".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Morning Star +1".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Baldur Hammer".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Baldur Hammer +1".to_string(),
        },
        FieldOption {
            value: 175,
            label: "War Hammer".to_string(),
        },
        FieldOption {
            value: 176,
            label: "War Hammer +1".to_string(),
        },
        FieldOption {
            value: 177,
            label: "War Maul".to_string(),
        },
        FieldOption {
            value: 178,
            label: "War Maul +1".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Spiked Flail".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Spiked Flail +1".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Damascus Hammer".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Damascus Hammer +1".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Dragon Hammer".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Sanguine Hammer".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Yggdrasil Gnarl".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Glacies".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Aqua Hammer (Relic)".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Vajra (Relic)".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Sanscion (Special)".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Flame Flail (Relic)".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Euros (Relic)".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Dagda's Hammer (Relic)".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Veritas (Relic)".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Hisyu".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Superior Hisyu".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Wakizashi".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Superior Wakizashi".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Jitte".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Superior Jitte".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Spiritblade".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Superior Spiritblade".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Ninja Sword".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Superior Ninja Sword".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Moon Sickle".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Superior Moon Sickle".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Sai".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Superior Sai".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Muso Blade".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Superior Muso Blade".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Tigerblade".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Superior Tigerblade".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Ghostblade".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Superior Ghostblade".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Brahma".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Superior Brahma".to_string(),
        },
        FieldOption {
            value: 216,
            label: "The Awakener (Relic)".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Thunderfire (Relic)".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Golok (Relic)".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Bakasura (Relic)".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Shimmer Sword (Relic)".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Tachi".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Superior Tachi".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Siege Sword".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Sawblade".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Nodachi".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Superior Nodachi".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Mageblade".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Superior Mageblade".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Cane Blade".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Superior Cane Blade".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Dechevalier".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Superior Dechevalier".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Blacksteel Blade".to_string(),
        },
        FieldOption {
            value: 234,
            label: "Laquersteel Blade".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Helm Halver".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Superior Helm Halver".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Oakblade".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Nene Bane".to_string(),
        },
        FieldOption {
            value: 239,
            label: "Whispertouch Blade".to_string(),
        },
        FieldOption {
            value: 240,
            label: "Firefly".to_string(),
        },
        FieldOption {
            value: 241,
            label: "Macuafuitl".to_string(),
        },
        FieldOption {
            value: 242,
            label: "Bringer of Light".to_string(),
        },
        FieldOption {
            value: 243,
            label: "Asura (Relic)".to_string(),
        },
        FieldOption {
            value: 244,
            label: "Crescent Sword (Relic)".to_string(),
        },
        FieldOption {
            value: 245,
            label: "Beadbound Blade (Relic)".to_string(),
        },
        FieldOption {
            value: 246,
            label: "Ogrebane (Relic)".to_string(),
        },
        FieldOption {
            value: 247,
            label: "Sweepblade (Relic)".to_string(),
        },
        FieldOption {
            value: 248,
            label: "Sibyl's Staff".to_string(),
        },
        FieldOption {
            value: 249,
            label: "Sibyl's Staff +1".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Mage Staff".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Mage Staff +1".to_string(),
        },
        FieldOption {
            value: 252,
            label: "Baldur Mace".to_string(),
        },
        FieldOption {
            value: 253,
            label: "Baldur Mace +1".to_string(),
        },
        FieldOption {
            value: 254,
            label: "Exarch's Staff".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Exarch's Staff +1".to_string(),
        },
        FieldOption {
            value: 256,
            label: "Magus Staff".to_string(),
        },
        FieldOption {
            value: 257,
            label: "Magus Staff +1".to_string(),
        },
        FieldOption {
            value: 258,
            label: "Damascus Mace".to_string(),
        },
        FieldOption {
            value: 259,
            label: "Damascus Mace +1".to_string(),
        },
        FieldOption {
            value: 260,
            label: "Staff of Restoration".to_string(),
        },
        FieldOption {
            value: 261,
            label: "Staff of Purification".to_string(),
        },
        FieldOption {
            value: 262,
            label: "Malitza's Staff (Relic)".to_string(),
        },
        FieldOption {
            value: 263,
            label: "Wand of Air".to_string(),
        },
        FieldOption {
            value: 264,
            label: "Wand of Earth".to_string(),
        },
        FieldOption {
            value: 265,
            label: "Wand of Lightning".to_string(),
        },
        FieldOption {
            value: 266,
            label: "Wand of Water".to_string(),
        },
        FieldOption {
            value: 267,
            label: "Wand of Fire".to_string(),
        },
        FieldOption {
            value: 268,
            label: "Wand of Ice".to_string(),
        },
        FieldOption {
            value: 269,
            label: "Ripple's Rod (Relic)".to_string(),
        },
        FieldOption {
            value: 270,
            label: "Sagara (Relic)".to_string(),
        },
        FieldOption {
            value: 271,
            label: "Sage's Staff (Relic)".to_string(),
        },
        FieldOption {
            value: 272,
            label: "Wiseman's Staff (Relic)".to_string(),
        },
        FieldOption {
            value: 273,
            label: "Bullwhip".to_string(),
        },
        FieldOption {
            value: 274,
            label: "Bullwhip +1".to_string(),
        },
        FieldOption {
            value: 275,
            label: "Spiked Laurel".to_string(),
        },
        FieldOption {
            value: 276,
            label: "Spiked Laurel +1".to_string(),
        },
        FieldOption {
            value: 277,
            label: "Rose Whip (Special)".to_string(),
        },
        FieldOption {
            value: 278,
            label: "Clearcrack Whip".to_string(),
        },
        FieldOption {
            value: 279,
            label: "Holy Comet".to_string(),
        },
        FieldOption {
            value: 280,
            label: "Blood Whip".to_string(),
        },
        FieldOption {
            value: 281,
            label: "Supple Whip (Relic)".to_string(),
        },
        FieldOption {
            value: 282,
            label: "Cat o' Nine Tails (Relic)".to_string(),
        },
        FieldOption {
            value: 283,
            label: "Biblion Anatomiae".to_string(),
        },
        FieldOption {
            value: 284,
            label: "Biblion Teratos".to_string(),
        },
        FieldOption {
            value: 285,
            label: "Biblion Herpetou".to_string(),
        },
        FieldOption {
            value: 286,
            label: "Biblion Drakontos".to_string(),
        },
        FieldOption {
            value: 287,
            label: "Biblion Sacri".to_string(),
        },
        FieldOption {
            value: 288,
            label: "Biblion Daemonis".to_string(),
        },
        FieldOption {
            value: 289,
            label: "Biblion Spiritus".to_string(),
        },
        FieldOption {
            value: 290,
            label: "Biblion Thanatos".to_string(),
        },
        FieldOption {
            value: 291,
            label: "Biblion Pupparis".to_string(),
        },
        FieldOption {
            value: 292,
            label: "Gran Grimoire (Relic)".to_string(),
        },
        FieldOption {
            value: 293,
            label: "Pandeiro".to_string(),
        },
        FieldOption {
            value: 294,
            label: "Pandeiro +1".to_string(),
        },
        FieldOption {
            value: 295,
            label: "Bolon".to_string(),
        },
        FieldOption {
            value: 296,
            label: "Bolon +1".to_string(),
        },
        FieldOption {
            value: 297,
            label: "Cavaquinho".to_string(),
        },
        FieldOption {
            value: 298,
            label: "Cavaquinho +1".to_string(),
        },
        FieldOption {
            value: 299,
            label: "Gerza's Atabaque (Relic)".to_string(),
        },
        FieldOption {
            value: 300,
            label: "Rabana's Kemenche (Relic)".to_string(),
        },
        FieldOption {
            value: 301,
            label: "Rabana's Tanbur (Relic)".to_string(),
        },
        FieldOption {
            value: 302,
            label: "Livela's Harp (Relic)".to_string(),
        },
        FieldOption {
            value: 303,
            label: "Gerges Blowgun".to_string(),
        },
        FieldOption {
            value: 304,
            label: "Stundart Blowgun".to_string(),
        },
        FieldOption {
            value: 305,
            label: "Wortdart Blowgun".to_string(),
        },
        FieldOption {
            value: 306,
            label: "Frogdart Blowgun".to_string(),
        },
        FieldOption {
            value: 307,
            label: "Mutedart Blowgun".to_string(),
        },
        FieldOption {
            value: 308,
            label: "Petridart Blowgun".to_string(),
        },
        FieldOption {
            value: 309,
            label: "Baldur Blowgun".to_string(),
        },
        FieldOption {
            value: 310,
            label: "Damascus Blowgun".to_string(),
        },
        FieldOption {
            value: 311,
            label: "Femakk's Blowgun (Relic)".to_string(),
        },
        FieldOption {
            value: 312,
            label: "Rahula (Relic)".to_string(),
        },
        FieldOption {
            value: 313,
            label: "Shortbow".to_string(),
        },
        FieldOption {
            value: 314,
            label: "Shortbow +1".to_string(),
        },
        FieldOption {
            value: 315,
            label: "Great Bow".to_string(),
        },
        FieldOption {
            value: 316,
            label: "Great Bow +1".to_string(),
        },
        FieldOption {
            value: 317,
            label: "Longbow".to_string(),
        },
        FieldOption {
            value: 318,
            label: "Longbow +1".to_string(),
        },
        FieldOption {
            value: 319,
            label: "Baldur Bow".to_string(),
        },
        FieldOption {
            value: 320,
            label: "Baldur Bow +1".to_string(),
        },
        FieldOption {
            value: 321,
            label: "Composite Bow".to_string(),
        },
        FieldOption {
            value: 322,
            label: "Composite Bow +1".to_string(),
        },
        FieldOption {
            value: 323,
            label: "Siege Bow".to_string(),
        },
        FieldOption {
            value: 324,
            label: "Siege Bow +1".to_string(),
        },
        FieldOption {
            value: 325,
            label: "Damascus Bow".to_string(),
        },
        FieldOption {
            value: 326,
            label: "Damascus Bow +1".to_string(),
        },
        FieldOption {
            value: 327,
            label: "Crescente".to_string(),
        },
        FieldOption {
            value: 328,
            label: "Cupido Bow".to_string(),
        },
        FieldOption {
            value: 329,
            label: "Permafrost Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 330,
            label: "Ixquimilli's Bow".to_string(),
        },
        FieldOption {
            value: 331,
            label: "Tempest Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 332,
            label: "Garuda Bow".to_string(),
        },
        FieldOption {
            value: 333,
            label: "Thunder Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 334,
            label: "Indra's Bow".to_string(),
        },
        FieldOption {
            value: 335,
            label: "Brimstone Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 336,
            label: "Sirocco Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 337,
            label: "Ji'ygla's Bow (Relic)".to_string(),
        },
        FieldOption {
            value: 338,
            label: "Pajra (Relic)".to_string(),
        },
        FieldOption {
            value: 339,
            label: "Centeotl's Rib (Relic)".to_string(),
        },
        FieldOption {
            value: 340,
            label: "Crossbow".to_string(),
        },
        FieldOption {
            value: 341,
            label: "Crossbow +1".to_string(),
        },
        FieldOption {
            value: 342,
            label: "Stonebow".to_string(),
        },
        FieldOption {
            value: 343,
            label: "Stonebow +1".to_string(),
        },
        FieldOption {
            value: 344,
            label: "Bowgun".to_string(),
        },
        FieldOption {
            value: 345,
            label: "Bowgun +1".to_string(),
        },
        FieldOption {
            value: 346,
            label: "Baldur Crossbow".to_string(),
        },
        FieldOption {
            value: 347,
            label: "Baldur Crossbow +1".to_string(),
        },
        FieldOption {
            value: 348,
            label: "Heavy Crossbow".to_string(),
        },
        FieldOption {
            value: 349,
            label: "Heavy Crossbow +1".to_string(),
        },
        FieldOption {
            value: 350,
            label: "Arbalest".to_string(),
        },
        FieldOption {
            value: 351,
            label: "Arbalest +1".to_string(),
        },
        FieldOption {
            value: 352,
            label: "Steelbow".to_string(),
        },
        FieldOption {
            value: 353,
            label: "Steelbow +1".to_string(),
        },
        FieldOption {
            value: 354,
            label: "Damascus Crossbow".to_string(),
        },
        FieldOption {
            value: 355,
            label: "Damascus Crossbow +1".to_string(),
        },
        FieldOption {
            value: 356,
            label: "Roodbow (Relic)".to_string(),
        },
        FieldOption {
            value: 357,
            label: "Al-iklil".to_string(),
        },
        FieldOption {
            value: 358,
            label: "Keening Bowgun (Relic)".to_string(),
        },
        FieldOption {
            value: 359,
            label: "Daedalus Bowgun".to_string(),
        },
        FieldOption {
            value: 360,
            label: "Asmak (Relic)".to_string(),
        },
        FieldOption {
            value: 361,
            label: "Leilah (Relic)".to_string(),
        },
        FieldOption {
            value: 362,
            label: "Shams (Relic)".to_string(),
        },
        FieldOption {
            value: 363,
            label: "Khalmid (Relic)".to_string(),
        },
        FieldOption {
            value: 364,
            label: "Ysaar (Relic)".to_string(),
        },
        FieldOption {
            value: 365,
            label: "Barad (Relic)".to_string(),
        },
        FieldOption {
            value: 366,
            label: "Raed (Relic)".to_string(),
        },
        FieldOption {
            value: 367,
            label: "Rimfire".to_string(),
        },
        FieldOption {
            value: 368,
            label: "Rimfire +1".to_string(),
        },
        FieldOption {
            value: 369,
            label: "Commander's Gun".to_string(),
        },
        FieldOption {
            value: 370,
            label: "Commander's Gun +1".to_string(),
        },
        FieldOption {
            value: 371,
            label: "Musket".to_string(),
        },
        FieldOption {
            value: 372,
            label: "Musket +1".to_string(),
        },
        FieldOption {
            value: 373,
            label: "Petronel (Relic)".to_string(),
        },
        FieldOption {
            value: 374,
            label: "Banduq-i-chaqmaqi (Relic)".to_string(),
        },
        FieldOption {
            value: 375,
            label: "Snub Fusil (Relic)".to_string(),
        },
        FieldOption {
            value: 376,
            label: "Longgun (Relic)".to_string(),
        },
        FieldOption {
            value: 377,
            label: "None".to_string(),
        },
        FieldOption {
            value: 378,
            label: "None".to_string(),
        },
        FieldOption {
            value: 379,
            label: "None".to_string(),
        },
        FieldOption {
            value: 380,
            label: "None".to_string(),
        },
        FieldOption {
            value: 381,
            label: "None".to_string(),
        },
        FieldOption {
            value: 382,
            label: "None".to_string(),
        },
        FieldOption {
            value: 383,
            label: "None".to_string(),
        },
        FieldOption {
            value: 384,
            label: "Lombardia (Real)".to_string(),
        },
        FieldOption {
            value: 385,
            label: "Short Sword (Broken)".to_string(),
        },
        FieldOption {
            value: 386,
            label: "Sybil's Staff (Broken)".to_string(),
        },
        FieldOption {
            value: 387,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 388,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 392,
            label: "None".to_string(),
        },
        FieldOption {
            value: 393,
            label: "None".to_string(),
        },
        FieldOption {
            value: 394,
            label: "Shortbow (Aloser 1)".to_string(),
        },
        FieldOption {
            value: 395,
            label: "Alluring Corset".to_string(),
        },
        FieldOption {
            value: 396,
            label: "Alluring Boots".to_string(),
        },
        FieldOption {
            value: 397,
            label: "Buckler".to_string(),
        },
        FieldOption {
            value: 398,
            label: "Buckler +1".to_string(),
        },
        FieldOption {
            value: 399,
            label: "Pelta".to_string(),
        },
        FieldOption {
            value: 400,
            label: "Pelta +1".to_string(),
        },
        FieldOption {
            value: 401,
            label: "Aspis".to_string(),
        },
        FieldOption {
            value: 402,
            label: "Aspis +1".to_string(),
        },
        FieldOption {
            value: 403,
            label: "Tower Shield".to_string(),
        },
        FieldOption {
            value: 404,
            label: "Tower Shield +1".to_string(),
        },
        FieldOption {
            value: 405,
            label: "Spiked Shield".to_string(),
        },
        FieldOption {
            value: 406,
            label: "Spiked Shield +1".to_string(),
        },
        FieldOption {
            value: 407,
            label: "Baldur Shield".to_string(),
        },
        FieldOption {
            value: 408,
            label: "Baldur Shield +1".to_string(),
        },
        FieldOption {
            value: 409,
            label: "Heater Shield".to_string(),
        },
        FieldOption {
            value: 410,
            label: "Heater Shield +1".to_string(),
        },
        FieldOption {
            value: 411,
            label: "Damascus Shield".to_string(),
        },
        FieldOption {
            value: 412,
            label: "Damascus Shield +1".to_string(),
        },
        FieldOption {
            value: 413,
            label: "Dragon Scale (Relic)".to_string(),
        },
        FieldOption {
            value: 414,
            label: "Ancient Dragon Scale".to_string(),
        },
        FieldOption {
            value: 415,
            label: "Rozenzi Shield".to_string(),
        },
        FieldOption {
            value: 416,
            label: "Dread Shield".to_string(),
        },
        FieldOption {
            value: 417,
            label: "Shield of the Winds (Relic)".to_string(),
        },
        FieldOption {
            value: 418,
            label: "Shield of the Loam (Relic)".to_string(),
        },
        FieldOption {
            value: 419,
            label: "Shield of the Storm (Relic)".to_string(),
        },
        FieldOption {
            value: 420,
            label: "Shield of the Waves (Relic)".to_string(),
        },
        FieldOption {
            value: 421,
            label: "Shield of the Flames (Relic)".to_string(),
        },
        FieldOption {
            value: 422,
            label: "Shield of the Tundra (Relic)".to_string(),
        },
        FieldOption {
            value: 423,
            label: "Shield of Sages (Relic)".to_string(),
        },
        FieldOption {
            value: 424,
            label: "Ogre Shield (Special)".to_string(),
        },
        FieldOption {
            value: 425,
            label: "Aegis (Relic)".to_string(),
        },
        FieldOption {
            value: 426,
            label: "Medusa Shield".to_string(),
        },
        FieldOption {
            value: 427,
            label: "Circlet".to_string(),
        },
        FieldOption {
            value: 428,
            label: "Circlet +1".to_string(),
        },
        FieldOption {
            value: 429,
            label: "Bronze Helm".to_string(),
        },
        FieldOption {
            value: 430,
            label: "Bronze Helm +1".to_string(),
        },
        FieldOption {
            value: 431,
            label: "Baldur Helm".to_string(),
        },
        FieldOption {
            value: 432,
            label: "Baldur Helm +1".to_string(),
        },
        FieldOption {
            value: 433,
            label: "Wizard's Hat".to_string(),
        },
        FieldOption {
            value: 434,
            label: "Wizard's Hat +1".to_string(),
        },
        FieldOption {
            value: 435,
            label: "Damascus Helm".to_string(),
        },
        FieldOption {
            value: 436,
            label: "Damascus Helm +1".to_string(),
        },
        FieldOption {
            value: 437,
            label: "Holy Crown (Relic)".to_string(),
        },
        FieldOption {
            value: 438,
            label: "Wyrmscale Helm".to_string(),
        },
        FieldOption {
            value: 439,
            label: "Glistening Helm (Special)".to_string(),
        },
        FieldOption {
            value: 440,
            label: "Ogre Helm (Special)".to_string(),
        },
        FieldOption {
            value: 441,
            label: "Skull Mask (Relic)".to_string(),
        },
        FieldOption {
            value: 442,
            label: "Fruede Helm (Relic)".to_string(),
        },
        FieldOption {
            value: 443,
            label: "Robe".to_string(),
        },
        FieldOption {
            value: 444,
            label: "Robe +1".to_string(),
        },
        FieldOption {
            value: 445,
            label: "Leather Armor".to_string(),
        },
        FieldOption {
            value: 446,
            label: "Leather Armor +1".to_string(),
        },
        FieldOption {
            value: 447,
            label: "Chainmail".to_string(),
        },
        FieldOption {
            value: 448,
            label: "Chainmail +1".to_string(),
        },
        FieldOption {
            value: 449,
            label: "Magus Robe".to_string(),
        },
        FieldOption {
            value: 450,
            label: "Magus Robe +1".to_string(),
        },
        FieldOption {
            value: 451,
            label: "Baldur Armor".to_string(),
        },
        FieldOption {
            value: 452,
            label: "Baldur Armor +1".to_string(),
        },
        FieldOption {
            value: 453,
            label: "Brigandine".to_string(),
        },
        FieldOption {
            value: 454,
            label: "Brigandine +1".to_string(),
        },
        FieldOption {
            value: 455,
            label: "Sorcerer's Robe".to_string(),
        },
        FieldOption {
            value: 456,
            label: "Sorcerer's Robe +1".to_string(),
        },
        FieldOption {
            value: 457,
            label: "Damascus Mail".to_string(),
        },
        FieldOption {
            value: 458,
            label: "Damascus Mail +1".to_string(),
        },
        FieldOption {
            value: 459,
            label: "Wyrmscale Armor".to_string(),
        },
        FieldOption {
            value: 460,
            label: "Reeking Armor (Special)".to_string(),
        },
        FieldOption {
            value: 461,
            label: "Robes of the Gale".to_string(),
        },
        FieldOption {
            value: 462,
            label: "Robes of the Dust".to_string(),
        },
        FieldOption {
            value: 463,
            label: "Robes of the Storm".to_string(),
        },
        FieldOption {
            value: 464,
            label: "Robes of the Cataract".to_string(),
        },
        FieldOption {
            value: 465,
            label: "Robes of the Inferno".to_string(),
        },
        FieldOption {
            value: 466,
            label: "Robes of Black Ice".to_string(),
        },
        FieldOption {
            value: 467,
            label: "Robes of Radiance".to_string(),
        },
        FieldOption {
            value: 468,
            label: "Robes of Gloom".to_string(),
        },
        FieldOption {
            value: 469,
            label: "Falcon Feathercoat (Relic)".to_string(),
        },
        FieldOption {
            value: 470,
            label: "Nathalork Rockcoat (Relic)".to_string(),
        },
        FieldOption {
            value: 471,
            label: "Viraat's Thundercoat (Relic)".to_string(),
        },
        FieldOption {
            value: 472,
            label: "Whale Whiskercoat (Relic)".to_string(),
        },
        FieldOption {
            value: 473,
            label: "Phoenix Flamecoat (Relic)".to_string(),
        },
        FieldOption {
            value: 474,
            label: "Vikrant Icecoat (Relic)".to_string(),
        },
        FieldOption {
            value: 475,
            label: "Aganista Lightcoat (Relic)".to_string(),
        },
        FieldOption {
            value: 476,
            label: "Ji'ygla's Darkcoat (Relic)".to_string(),
        },
        FieldOption {
            value: 477,
            label: "Falcon Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 478,
            label: "Nathalork Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 479,
            label: "Viraat's Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 480,
            label: "Ur-Whale Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 481,
            label: "Phoenix Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 482,
            label: "Vikrant Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 483,
            label: "Titania Mail (Relic)".to_string(),
        },
        FieldOption {
            value: 484,
            label: "Thanantos Armor (Relic)".to_string(),
        },
        FieldOption {
            value: 485,
            label: "Garb of the Sages (Relic)".to_string(),
        },
        FieldOption {
            value: 486,
            label: "Ogre Armor (Special)".to_string(),
        },
        FieldOption {
            value: 487,
            label: "Alluring Dress".to_string(),
        },
        FieldOption {
            value: 488,
            label: "Leather Gloves".to_string(),
        },
        FieldOption {
            value: 489,
            label: "Leather Gloves +1".to_string(),
        },
        FieldOption {
            value: 490,
            label: "Leather Sleeves".to_string(),
        },
        FieldOption {
            value: 491,
            label: "Leather Sleeves +1".to_string(),
        },
        FieldOption {
            value: 492,
            label: "Gauntlets".to_string(),
        },
        FieldOption {
            value: 493,
            label: "Gauntlets +1".to_string(),
        },
        FieldOption {
            value: 494,
            label: "Baldur Gauntlets".to_string(),
        },
        FieldOption {
            value: 495,
            label: "Baldur Gauntlets +1".to_string(),
        },
        FieldOption {
            value: 496,
            label: "Nomad Bracers".to_string(),
        },
        FieldOption {
            value: 497,
            label: "Nomad Bracers +1".to_string(),
        },
        FieldOption {
            value: 498,
            label: "Overguards".to_string(),
        },
        FieldOption {
            value: 499,
            label: "Overguards +1".to_string(),
        },
        FieldOption {
            value: 500,
            label: "Damascus Mitts".to_string(),
        },
        FieldOption {
            value: 501,
            label: "Damascus Mitts +1".to_string(),
        },
        FieldOption {
            value: 502,
            label: "Arkhiatros Mitts".to_string(),
        },
        FieldOption {
            value: 503,
            label: "Mage's Mitts".to_string(),
        },
        FieldOption {
            value: 504,
            label: "Wyrmscale Sleeves".to_string(),
        },
        FieldOption {
            value: 505,
            label: "Musty Gauntlets (Special)".to_string(),
        },
        FieldOption {
            value: 506,
            label: "Ji'ygla's Bracers (Relic)".to_string(),
        },
        FieldOption {
            value: 507,
            label: "Lightning Gauntlets (Relic)".to_string(),
        },
        FieldOption {
            value: 508,
            label: "Fire Gauntlets (Relic)".to_string(),
        },
        FieldOption {
            value: 509,
            label: "Luminant Gauntlets (Relic)".to_string(),
        },
        FieldOption {
            value: 510,
            label: "Daedalus Gauntlets".to_string(),
        },
        FieldOption {
            value: 511,
            label: "Snipe Bracers (Relic)".to_string(),
        },
        FieldOption {
            value: 512,
            label: "Linen Slops".to_string(),
        },
        FieldOption {
            value: 513,
            label: "Linen Slops +1".to_string(),
        },
        FieldOption {
            value: 514,
            label: "Leather Leggings".to_string(),
        },
        FieldOption {
            value: 515,
            label: "Leather Leggings +1".to_string(),
        },
        FieldOption {
            value: 516,
            label: "Chain Leggings".to_string(),
        },
        FieldOption {
            value: 517,
            label: "Chain Leggings +1".to_string(),
        },
        FieldOption {
            value: 518,
            label: "Baldur Leggings".to_string(),
        },
        FieldOption {
            value: 519,
            label: "Baldur Leggings +1".to_string(),
        },
        FieldOption {
            value: 520,
            label: "Damascus Leggings".to_string(),
        },
        FieldOption {
            value: 521,
            label: "Damascus Leggings +1".to_string(),
        },
        FieldOption {
            value: 522,
            label: "Arkhiatros Trousers".to_string(),
        },
        FieldOption {
            value: 523,
            label: "Mage Trousers".to_string(),
        },
        FieldOption {
            value: 524,
            label: "Cloud Shoes".to_string(),
        },
        FieldOption {
            value: 525,
            label: "Winged Boots".to_string(),
        },
        FieldOption {
            value: 526,
            label: "Sidhe Shoes".to_string(),
        },
        FieldOption {
            value: 527,
            label: "Sparkguard Boots".to_string(),
        },
        FieldOption {
            value: 528,
            label: "Greased Boots".to_string(),
        },
        FieldOption {
            value: 529,
            label: "Earthen Greaves (Relic)".to_string(),
        },
        FieldOption {
            value: 530,
            label: "Watery Greaves (Relic)".to_string(),
        },
        FieldOption {
            value: 531,
            label: "Hoarfrost Greaves (Relic)".to_string(),
        },
        FieldOption {
            value: 532,
            label: "Shadowed Greaves (Relic)".to_string(),
        },
        FieldOption {
            value: 533,
            label: "Alluring Highboots".to_string(),
        },
        FieldOption {
            value: 534,
            label: "Snipe Gators (Relic)".to_string(),
        },
        FieldOption {
            value: 535,
            label: "Crimson Necklace (Special)".to_string(),
        },
        FieldOption {
            value: 536,
            label: "Azure Necklace (Special)".to_string(),
        },
        FieldOption {
            value: 537,
            label: "Warrior's Ring".to_string(),
        },
        FieldOption {
            value: 538,
            label: "Warrior's Ring +1".to_string(),
        },
        FieldOption {
            value: 539,
            label: "Defender's Ring".to_string(),
        },
        FieldOption {
            value: 540,
            label: "Defender's Ring +1".to_string(),
        },
        FieldOption {
            value: 541,
            label: "Ring of the Horde".to_string(),
        },
        FieldOption {
            value: 542,
            label: "Ring of the Horde +1".to_string(),
        },
        FieldOption {
            value: 543,
            label: "Ring of Vitality".to_string(),
        },
        FieldOption {
            value: 544,
            label: "Ring of Vitality +1".to_string(),
        },
        FieldOption {
            value: 545,
            label: "Ring of Deftness".to_string(),
        },
        FieldOption {
            value: 546,
            label: "Ring of Deftness +1".to_string(),
        },
        FieldOption {
            value: 547,
            label: "Ring of Alacrity".to_string(),
        },
        FieldOption {
            value: 548,
            label: "Ring of Alacrity +1".to_string(),
        },
        FieldOption {
            value: 549,
            label: "Ring of Evasion".to_string(),
        },
        FieldOption {
            value: 550,
            label: "Ring of Evasion +1".to_string(),
        },
        FieldOption {
            value: 551,
            label: "Ring of Intellect".to_string(),
        },
        FieldOption {
            value: 552,
            label: "Ring of Intellect +1".to_string(),
        },
        FieldOption {
            value: 553,
            label: "Ring of the Mind".to_string(),
        },
        FieldOption {
            value: 554,
            label: "Ring of the Mind +1".to_string(),
        },
        FieldOption {
            value: 555,
            label: "Magebane Band".to_string(),
        },
        FieldOption {
            value: 556,
            label: "Magebane Band +1".to_string(),
        },
        FieldOption {
            value: 557,
            label: "Band of Fortune".to_string(),
        },
        FieldOption {
            value: 558,
            label: "Ring of Evasion +1".to_string(),
        },
        FieldOption {
            value: 559,
            label: "Beast's Earring".to_string(),
        },
        FieldOption {
            value: 560,
            label: "Wasp's Earring".to_string(),
        },
        FieldOption {
            value: 561,
            label: "Guardsman's Earring".to_string(),
        },
        FieldOption {
            value: 562,
            label: "Swordsman's Earring".to_string(),
        },
        FieldOption {
            value: 563,
            label: "Barbarian's Earring".to_string(),
        },
        FieldOption {
            value: 564,
            label: "Spearman's Earring".to_string(),
        },
        FieldOption {
            value: 565,
            label: "Temblor Earring".to_string(),
        },
        FieldOption {
            value: 566,
            label: "Crescent Earring".to_string(),
        },
        FieldOption {
            value: 567,
            label: "Sunfire Earring".to_string(),
        },
        FieldOption {
            value: 568,
            label: "Saint's Earring".to_string(),
        },
        FieldOption {
            value: 569,
            label: "Earring of the Snake".to_string(),
        },
        FieldOption {
            value: 570,
            label: "Scrivener's Earring".to_string(),
        },
        FieldOption {
            value: 571,
            label: "Canso Earring".to_string(),
        },
        FieldOption {
            value: 572,
            label: "Earring of Stillness".to_string(),
        },
        FieldOption {
            value: 573,
            label: "Stalker's Earring".to_string(),
        },
        FieldOption {
            value: 574,
            label: "Archer's Earring".to_string(),
        },
        FieldOption {
            value: 575,
            label: "Farseer's Earring".to_string(),
        },
        FieldOption {
            value: 576,
            label: "Gale Choker".to_string(),
        },
        FieldOption {
            value: 577,
            label: "Dust Choker".to_string(),
        },
        FieldOption {
            value: 578,
            label: "Storm Choker".to_string(),
        },
        FieldOption {
            value: 579,
            label: "Cataract Choker".to_string(),
        },
        FieldOption {
            value: 580,
            label: "Firewyrm Choker".to_string(),
        },
        FieldOption {
            value: 581,
            label: "Black Ice Choker".to_string(),
        },
        FieldOption {
            value: 582,
            label: "Saint King's Choker".to_string(),
        },
        FieldOption {
            value: 583,
            label: "Ghast's Choker".to_string(),
        },
        FieldOption {
            value: 584,
            label: "Ring of Clouds (Relic)".to_string(),
        },
        FieldOption {
            value: 585,
            label: "Winged Ring (Relic)".to_string(),
        },
        FieldOption {
            value: 586,
            label: "Sidhe Ring (Relic)".to_string(),
        },
        FieldOption {
            value: 587,
            label: "Sparkguard Ring (Relic)".to_string(),
        },
        FieldOption {
            value: 588,
            label: "Greased Ring (Relic)".to_string(),
        },
        FieldOption {
            value: 589,
            label: "Reaver Ring".to_string(),
        },
        FieldOption {
            value: 590,
            label: "Crest of Fire (Special)".to_string(),
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
            label: "Cursed Weapon Fist".to_string(),
        },
        FieldOption {
            value: 611,
            label: "Cursed Weapon Dagger".to_string(),
        },
        FieldOption {
            value: 612,
            label: "Cursed Weapon 1H Sword".to_string(),
        },
        FieldOption {
            value: 613,
            label: "Cursed Weapon 2H Sword".to_string(),
        },
        FieldOption {
            value: 614,
            label: "Cursed Weapon Axe".to_string(),
        },
        FieldOption {
            value: 615,
            label: "Cursed Weapon Spear".to_string(),
        },
        FieldOption {
            value: 616,
            label: "Cursed Weapon Hammer".to_string(),
        },
        FieldOption {
            value: 617,
            label: "Cursed Weapon 1H Katana".to_string(),
        },
        FieldOption {
            value: 618,
            label: "Cursed Weapon 2H Katana".to_string(),
        },
        FieldOption {
            value: 619,
            label: "Cursed Weapon Cudgel".to_string(),
        },
        FieldOption {
            value: 620,
            label: "Cursed Weapon Whip".to_string(),
        },
        FieldOption {
            value: 621,
            label: "Cursed Weapon Spellbook".to_string(),
        },
        FieldOption {
            value: 622,
            label: "Cursed Weapon Instrument".to_string(),
        },
        FieldOption {
            value: 623,
            label: "Cursed Weapon Blowgun".to_string(),
        },
        FieldOption {
            value: 624,
            label: "Cursed Weapon Bow".to_string(),
        },
        FieldOption {
            value: 625,
            label: "Cursed Weapon Crossbow".to_string(),
        },
        FieldOption {
            value: 626,
            label: "Cursed Weapon Fusil".to_string(),
        },
        FieldOption {
            value: 627,
            label: "Cursed Weapon Fist".to_string(),
        },
        FieldOption {
            value: 628,
            label: "Cursed Weapon Dagger".to_string(),
        },
        FieldOption {
            value: 629,
            label: "Cursed Weapon 1H Sword".to_string(),
        },
        FieldOption {
            value: 630,
            label: "Cursed Weapon 2H Sword".to_string(),
        },
        FieldOption {
            value: 631,
            label: "Cursed Weapon Axe".to_string(),
        },
        FieldOption {
            value: 632,
            label: "Cursed Weapon Spear".to_string(),
        },
        FieldOption {
            value: 633,
            label: "Cursed Weapon Hammer".to_string(),
        },
        FieldOption {
            value: 634,
            label: "Cursed Weapon 1H Katana".to_string(),
        },
        FieldOption {
            value: 635,
            label: "Cursed Weapon 2H Katana".to_string(),
        },
        FieldOption {
            value: 636,
            label: "Cursed Weapon Cudgel".to_string(),
        },
        FieldOption {
            value: 637,
            label: "Cursed Weapon Whip".to_string(),
        },
        FieldOption {
            value: 638,
            label: "Cursed Weapon Spellbook".to_string(),
        },
        FieldOption {
            value: 639,
            label: "Cursed Weapon Instrument".to_string(),
        },
        FieldOption {
            value: 640,
            label: "Cursed Weapon Blowgun".to_string(),
        },
        FieldOption {
            value: 641,
            label: "Cursed Weapon Bow".to_string(),
        },
        FieldOption {
            value: 642,
            label: "Cursed Weapon Crossbow".to_string(),
        },
        FieldOption {
            value: 643,
            label: "Cursed Weapon Fusil".to_string(),
        },
        FieldOption {
            value: 644,
            label: "Cursed Weapon Fist".to_string(),
        },
        FieldOption {
            value: 645,
            label: "Cursed Weapon Dagger".to_string(),
        },
        FieldOption {
            value: 646,
            label: "Cursed Weapon 1H Sword".to_string(),
        },
        FieldOption {
            value: 647,
            label: "Cursed Weapon 2H Sword".to_string(),
        },
        FieldOption {
            value: 648,
            label: "Cursed Weapon Axe".to_string(),
        },
        FieldOption {
            value: 649,
            label: "Cursed Weapon Spear".to_string(),
        },
        FieldOption {
            value: 650,
            label: "Cursed Weapon Hammer".to_string(),
        },
        FieldOption {
            value: 651,
            label: "Cursed Weapon 1H Katana".to_string(),
        },
        FieldOption {
            value: 652,
            label: "Cursed Weapon 2H Katana".to_string(),
        },
        FieldOption {
            value: 653,
            label: "Cursed Weapon Cudgel".to_string(),
        },
        FieldOption {
            value: 654,
            label: "Cursed Weapon Whip".to_string(),
        },
        FieldOption {
            value: 655,
            label: "Cursed Weapon Spellbook".to_string(),
        },
        FieldOption {
            value: 656,
            label: "Cursed Weapon Instrument".to_string(),
        },
        FieldOption {
            value: 657,
            label: "Cursed Weapon Blowgun".to_string(),
        },
        FieldOption {
            value: 658,
            label: "Cursed Weapon Bow".to_string(),
        },
        FieldOption {
            value: 659,
            label: "Cursed Weapon Crossbow".to_string(),
        },
        FieldOption {
            value: 660,
            label: "Cursed Weapon Fusil".to_string(),
        },
        FieldOption {
            value: 661,
            label: "Trueno's Scales (Relic)".to_string(),
        },
        FieldOption {
            value: 662,
            label: "Nifrit Sword (Relic)".to_string(),
        },
        FieldOption {
            value: 663,
            label: "Ambicion (Relic)".to_string(),
        },
        FieldOption {
            value: 664,
            label: "Brynhildr (Relic)".to_string(),
        },
        FieldOption {
            value: 665,
            label: "Balmung (Relic)".to_string(),
        },
        FieldOption {
            value: 666,
            label: "Glamrock (Relic)".to_string(),
        },
        FieldOption {
            value: 667,
            label: "Volcaetus (Relic)".to_string(),
        },
        FieldOption {
            value: 668,
            label: "Sanscion (Relic)".to_string(),
        },
        FieldOption {
            value: 669,
            label: "Rose Whip (Relic)".to_string(),
        },
        FieldOption {
            value: 670,
            label: "Glamrock (Boss 1)".to_string(),
        },
        FieldOption {
            value: 671,
            label: "Glamrock (Boss 2)".to_string(),
        },
        FieldOption {
            value: 672,
            label: "Glamrock (Boss 3)".to_string(),
        },
        FieldOption {
            value: 673,
            label: "Volcaetus (Boss)".to_string(),
        },
        FieldOption {
            value: 674,
            label: "Rose Whip (Boss 1)".to_string(),
        },
        FieldOption {
            value: 675,
            label: "Rose Whip (Boss 2)".to_string(),
        },
        FieldOption {
            value: 676,
            label: "Trueno's Scales (Boss 1)".to_string(),
        },
        FieldOption {
            value: 677,
            label: "Trueno's Scales (Boss 2)".to_string(),
        },
        FieldOption {
            value: 678,
            label: "Nifrit Sword (Boss)".to_string(),
        },
        FieldOption {
            value: 679,
            label: "Ambicion (Boss)".to_string(),
        },
        FieldOption {
            value: 680,
            label: "Balmung (Boss)".to_string(),
        },
        FieldOption {
            value: 681,
            label: "Sanscion (Boss 1)".to_string(),
        },
        FieldOption {
            value: 682,
            label: "Sanscion (Boss 2)".to_string(),
        },
        FieldOption {
            value: 683,
            label: "None".to_string(),
        },
        FieldOption {
            value: 684,
            label: "None".to_string(),
        },
        FieldOption {
            value: 685,
            label: "Shortbow (Aloser 2)".to_string(),
        },
        FieldOption {
            value: 1000,
            label: "Mend Leaf".to_string(),
        },
        FieldOption {
            value: 1001,
            label: "Mend Leaf +1".to_string(),
        },
        FieldOption {
            value: 1002,
            label: "Mend Leaf +2".to_string(),
        },
        FieldOption {
            value: 1003,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1004,
            label: "Mending Seed".to_string(),
        },
        FieldOption {
            value: 1005,
            label: "Mending Seed +1".to_string(),
        },
        FieldOption {
            value: 1006,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1007,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1008,
            label: "Mending Salve".to_string(),
        },
        FieldOption {
            value: 1009,
            label: "Mending Salve +1".to_string(),
        },
        FieldOption {
            value: 1010,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1011,
            label: "Mending Essence".to_string(),
        },
        FieldOption {
            value: 1012,
            label: "Magic Leaf".to_string(),
        },
        FieldOption {
            value: 1013,
            label: "Magic Leaf +1".to_string(),
        },
        FieldOption {
            value: 1014,
            label: "Magic Leaf +2".to_string(),
        },
        FieldOption {
            value: 1015,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1016,
            label: "Magic Seed".to_string(),
        },
        FieldOption {
            value: 1017,
            label: "Magic Seed +1".to_string(),
        },
        FieldOption {
            value: 1018,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1019,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1020,
            label: "Magic Salve".to_string(),
        },
        FieldOption {
            value: 1021,
            label: "Magic Salve +1".to_string(),
        },
        FieldOption {
            value: 1022,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1023,
            label: "Magic Essence".to_string(),
        },
        FieldOption {
            value: 1024,
            label: "Fruit of the Adept".to_string(),
        },
        FieldOption {
            value: 1025,
            label: "Fruit of the Adept +1".to_string(),
        },
        FieldOption {
            value: 1026,
            label: "Fruit of the Sage".to_string(),
        },
        FieldOption {
            value: 1027,
            label: "Fruit of the Sage +1".to_string(),
        },
        FieldOption {
            value: 1028,
            label: "Fruit of the Seraph".to_string(),
        },
        FieldOption {
            value: 1029,
            label: "Overripe Fruit".to_string(),
        },
        FieldOption {
            value: 1030,
            label: "Zolia Draught".to_string(),
        },
        FieldOption {
            value: 1031,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1032,
            label: "Zena Wine".to_string(),
        },
        FieldOption {
            value: 1033,
            label: "Illumina Nectar".to_string(),
        },
        FieldOption {
            value: 1034,
            label: "Gerun Powder".to_string(),
        },
        FieldOption {
            value: 1035,
            label: "Feyrn Bolus".to_string(),
        },
        FieldOption {
            value: 1036,
            label: "Maca Antidote".to_string(),
        },
        FieldOption {
            value: 1037,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1038,
            label: "Jaarn's Poultice".to_string(),
        },
        FieldOption {
            value: 1039,
            label: "Ishtar's Ambrosia".to_string(),
        },
        FieldOption {
            value: 1040,
            label: "Ashmedai's Grog".to_string(),
        },
        FieldOption {
            value: 1041,
            label: "Blessing Stone".to_string(),
        },
        FieldOption {
            value: 1042,
            label: "Hallowing Stone".to_string(),
        },
        FieldOption {
            value: 1043,
            label: "Areion Plume".to_string(),
        },
        FieldOption {
            value: 1044,
            label: "Basin of Time".to_string(),
        },
        FieldOption {
            value: 1045,
            label: "Spiritstone of the Stars".to_string(),
        },
        FieldOption {
            value: 1046,
            label: "Faeriescale Powder".to_string(),
        },
        FieldOption {
            value: 1047,
            label: "Crystallized Flame".to_string(),
        },
        FieldOption {
            value: 1048,
            label: "Mercurial Phial".to_string(),
        },
        FieldOption {
            value: 1049,
            label: "Jewel of the Avatar".to_string(),
        },
        FieldOption {
            value: 1050,
            label: "Hair of the Unicorn".to_string(),
        },
        FieldOption {
            value: 1051,
            label: "Philtre of Ashes".to_string(),
        },
        FieldOption {
            value: 1052,
            label: "Black Lizard Powder".to_string(),
        },
        FieldOption {
            value: 1053,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1054,
            label: "Dragon Steak".to_string(),
        },
        FieldOption {
            value: 1055,
            label: "Braised Skewer".to_string(),
        },
        FieldOption {
            value: 1056,
            label: "Steamed Mollusk".to_string(),
        },
        FieldOption {
            value: 1057,
            label: "Minced Patty".to_string(),
        },
        FieldOption {
            value: 1058,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1059,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1060,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1061,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1062,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1063,
            label: "Brand of the Sacrifice".to_string(),
        },
        FieldOption {
            value: 1064,
            label: "Dynast-King's Mead".to_string(),
        },
        FieldOption {
            value: 1065,
            label: "Echo Stone".to_string(),
        },
        FieldOption {
            value: 1066,
            label: "Blackwing Leg".to_string(),
        },
        FieldOption {
            value: 1067,
            label: "Rood Upright".to_string(),
        },
        FieldOption {
            value: 1068,
            label: "Haunt's Tome".to_string(),
        },
        FieldOption {
            value: 1069,
            label: "Darkscale Tome".to_string(),
        },
        FieldOption {
            value: 1070,
            label: "Cursed Unicorn Blood".to_string(),
        },
        FieldOption {
            value: 1071,
            label: "Skulldust Nostrum".to_string(),
        },
        FieldOption {
            value: 1072,
            label: "Magedrain Gland".to_string(),
        },
        FieldOption {
            value: 1073,
            label: "Shiftstone".to_string(),
        },
        FieldOption {
            value: 1074,
            label: "Horn of the Savage".to_string(),
        },
        FieldOption {
            value: 1075,
            label: "Coral Harp".to_string(),
        },
        FieldOption {
            value: 1076,
            label: "Whirlwind Shot".to_string(),
        },
        FieldOption {
            value: 1077,
            label: "Duststorm Shot".to_string(),
        },
        FieldOption {
            value: 1078,
            label: "Thunder Shot".to_string(),
        },
        FieldOption {
            value: 1079,
            label: "Torrent Shot".to_string(),
        },
        FieldOption {
            value: 1080,
            label: "Conflagration Shot".to_string(),
        },
        FieldOption {
            value: 1081,
            label: "Firnice Shot".to_string(),
        },
        FieldOption {
            value: 1082,
            label: "Coruscate Shot".to_string(),
        },
        FieldOption {
            value: 1083,
            label: "Murk Shot".to_string(),
        },
        FieldOption {
            value: 1084,
            label: "Book of the Dead".to_string(),
        },
        FieldOption {
            value: 1085,
            label: "Ring of the Dead".to_string(),
        },
        FieldOption {
            value: 1086,
            label: "Ensanguined Rood".to_string(),
        },
        FieldOption {
            value: 1087,
            label: "Seal of Rebirth".to_string(),
        },
        FieldOption {
            value: 1088,
            label: "Void Orb".to_string(),
        },
        FieldOption {
            value: 1089,
            label: "Gale Orb".to_string(),
        },
        FieldOption {
            value: 1090,
            label: "Dust Orb".to_string(),
        },
        FieldOption {
            value: 1091,
            label: "Storm Orb".to_string(),
        },
        FieldOption {
            value: 1092,
            label: "Cataract Orb".to_string(),
        },
        FieldOption {
            value: 1093,
            label: "Inferno Orb".to_string(),
        },
        FieldOption {
            value: 1094,
            label: "Black Ice Orb".to_string(),
        },
        FieldOption {
            value: 1095,
            label: "Radiant Orb".to_string(),
        },
        FieldOption {
            value: 1096,
            label: "Gloom Orb".to_string(),
        },
        FieldOption {
            value: 1097,
            label: "Elixir".to_string(),
        },
        FieldOption {
            value: 1098,
            label: "Charm of Remission".to_string(),
        },
        FieldOption {
            value: 1099,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1100,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1101,
            label: "Intelligence Card".to_string(),
        },
        FieldOption {
            value: 1102,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1103,
            label: "MP Card".to_string(),
        },
        FieldOption {
            value: 1104,
            label: "HP Card".to_string(),
        },
        FieldOption {
            value: 1105,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1106,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1107,
            label: "Strength Card".to_string(),
        },
        FieldOption {
            value: 1108,
            label: "Strength Card (2)".to_string(),
        },
        FieldOption {
            value: 1109,
            label: "Intelligence Card (2)".to_string(),
        },
        FieldOption {
            value: 1110,
            label: "Avoidance Card".to_string(),
        },
        FieldOption {
            value: 1111,
            label: "Avoidance Card (2)".to_string(),
        },
        FieldOption {
            value: 1112,
            label: "Vitality Card".to_string(),
        },
        FieldOption {
            value: 1113,
            label: "Luck Card".to_string(),
        },
        FieldOption {
            value: 1114,
            label: "Resistance Card".to_string(),
        },
        FieldOption {
            value: 1115,
            label: "Luck Card (2)".to_string(),
        },
        FieldOption {
            value: 1116,
            label: "Vitality Card (2)".to_string(),
        },
        FieldOption {
            value: 1117,
            label: "Dexterity Card".to_string(),
        },
        FieldOption {
            value: 1118,
            label: "Agility Card".to_string(),
        },
        FieldOption {
            value: 1119,
            label: "Agility Card (2)".to_string(),
        },
        FieldOption {
            value: 1120,
            label: "Dexterity Card (2)".to_string(),
        },
        FieldOption {
            value: 1121,
            label: "Resistance Card (2)".to_string(),
        },
        FieldOption {
            value: 1122,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1123,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1124,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1125,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1126,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1127,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1128,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1129,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1130,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1131,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1132,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1133,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1134,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1135,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1136,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1137,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1138,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1139,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1140,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1141,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1142,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1143,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1144,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1145,
            label: "Copper Oberynth".to_string(),
        },
        FieldOption {
            value: 1146,
            label: "Bronze Oberynth".to_string(),
        },
        FieldOption {
            value: 1147,
            label: "Silver Oberynth".to_string(),
        },
        FieldOption {
            value: 1148,
            label: "Gold Oberynth".to_string(),
        },
        FieldOption {
            value: 1149,
            label: "Platinum Oberynth".to_string(),
        },
        FieldOption {
            value: 1150,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1151,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 1152,
            label: "Deadshot II".to_string(),
        },
        FieldOption {
            value: 1153,
            label: "Deadshot III".to_string(),
        },
        FieldOption {
            value: 1154,
            label: "Deadshot IV".to_string(),
        },
        FieldOption {
            value: 1155,
            label: "Tornado".to_string(),
        },
        FieldOption {
            value: 1156,
            label: "Tornado II".to_string(),
        },
        FieldOption {
            value: 1157,
            label: "Tornado III".to_string(),
        },
        FieldOption {
            value: 1158,
            label: "Tornado IV".to_string(),
        },
        FieldOption {
            value: 1159,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 1160,
            label: "Sylphide II".to_string(),
        },
        FieldOption {
            value: 1161,
            label: "Aeroflux".to_string(),
        },
        FieldOption {
            value: 1162,
            label: "Aeroflux II".to_string(),
        },
        FieldOption {
            value: 1163,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 1164,
            label: "Aeroguard".to_string(),
        },
        FieldOption {
            value: 1165,
            label: "Whirlwind".to_string(),
        },
        FieldOption {
            value: 1166,
            label: "Guarding Gale".to_string(),
        },
        FieldOption {
            value: 1167,
            label: "Balmy Breeze".to_string(),
        },
        FieldOption {
            value: 1168,
            label: "Black Williwaw".to_string(),
        },
        FieldOption {
            value: 1169,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 1170,
            label: "Vulcan Lance II".to_string(),
        },
        FieldOption {
            value: 1171,
            label: "Vulcan Lance III".to_string(),
        },
        FieldOption {
            value: 1172,
            label: "Vulcan Lance IV".to_string(),
        },
        FieldOption {
            value: 1173,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 1174,
            label: "Cragfall II".to_string(),
        },
        FieldOption {
            value: 1175,
            label: "Cragfall III".to_string(),
        },
        FieldOption {
            value: 1176,
            label: "Cragfall IV".to_string(),
        },
        FieldOption {
            value: 1177,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 1178,
            label: "Gnome II".to_string(),
        },
        FieldOption {
            value: 1179,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 1180,
            label: "Earthquake II".to_string(),
        },
        FieldOption {
            value: 1181,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 1182,
            label: "Petroguard".to_string(),
        },
        FieldOption {
            value: 1183,
            label: "Protect".to_string(),
        },
        FieldOption {
            value: 1184,
            label: "Blade Ward".to_string(),
        },
        FieldOption {
            value: 1185,
            label: "Duststrorm".to_string(),
        },
        FieldOption {
            value: 1186,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 1187,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 1188,
            label: "Lightning Bow II".to_string(),
        },
        FieldOption {
            value: 1189,
            label: "Lightning Bow III".to_string(),
        },
        FieldOption {
            value: 1190,
            label: "Lightning Bow IV".to_string(),
        },
        FieldOption {
            value: 1191,
            label: "Thunderflare".to_string(),
        },
        FieldOption {
            value: 1192,
            label: "Thunderflare II".to_string(),
        },
        FieldOption {
            value: 1193,
            label: "Thunderflare III".to_string(),
        },
        FieldOption {
            value: 1194,
            label: "Thunderflare IV".to_string(),
        },
        FieldOption {
            value: 1195,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 1196,
            label: "Thunderbird II".to_string(),
        },
        FieldOption {
            value: 1197,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 1198,
            label: "Thunderburst II".to_string(),
        },
        FieldOption {
            value: 1199,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 1200,
            label: "Electricgaurd".to_string(),
        },
        FieldOption {
            value: 1201,
            label: "Galvanize".to_string(),
        },
        FieldOption {
            value: 1202,
            label: "Stormspark".to_string(),
        },
        FieldOption {
            value: 1203,
            label: "Stunbomb".to_string(),
        },
        FieldOption {
            value: 1204,
            label: "Stunslay".to_string(),
        },
        FieldOption {
            value: 1205,
            label: "Aquablast".to_string(),
        },
        FieldOption {
            value: 1206,
            label: "Aquablast II".to_string(),
        },
        FieldOption {
            value: 1207,
            label: "Aquablast III".to_string(),
        },
        FieldOption {
            value: 1208,
            label: "Aquablast IV".to_string(),
        },
        FieldOption {
            value: 1209,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 1210,
            label: "Acid Rain II".to_string(),
        },
        FieldOption {
            value: 1211,
            label: "Acid Rain III".to_string(),
        },
        FieldOption {
            value: 1212,
            label: "Acid Rain IV".to_string(),
        },
        FieldOption {
            value: 1213,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 1214,
            label: "Undine II".to_string(),
        },
        FieldOption {
            value: 1215,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 1216,
            label: "Dread Vapor II".to_string(),
        },
        FieldOption {
            value: 1217,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 1218,
            label: "Aqaugaurd".to_string(),
        },
        FieldOption {
            value: 1219,
            label: "Quench".to_string(),
        },
        FieldOption {
            value: 1220,
            label: "Stagnate".to_string(),
        },
        FieldOption {
            value: 1221,
            label: "Poison Mist".to_string(),
        },
        FieldOption {
            value: 1222,
            label: "Sludgebind".to_string(),
        },
        FieldOption {
            value: 1223,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 1224,
            label: "Sparksphere II".to_string(),
        },
        FieldOption {
            value: 1225,
            label: "Sparksphere III".to_string(),
        },
        FieldOption {
            value: 1226,
            label: "Sparksphere IV".to_string(),
        },
        FieldOption {
            value: 1227,
            label: "Firestorm".to_string(),
        },
        FieldOption {
            value: 1228,
            label: "Firestorm II".to_string(),
        },
        FieldOption {
            value: 1229,
            label: "Firestorm III".to_string(),
        },
        FieldOption {
            value: 1230,
            label: "Firestorm IV".to_string(),
        },
        FieldOption {
            value: 1231,
            label: "Salamander".to_string(),
        },
        FieldOption {
            value: 1232,
            label: "Salamander II".to_string(),
        },
        FieldOption {
            value: 1233,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 1234,
            label: "Supernova II".to_string(),
        },
        FieldOption {
            value: 1235,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 1236,
            label: "Pryogaurd".to_string(),
        },
        FieldOption {
            value: 1237,
            label: "Flame Fusion".to_string(),
        },
        FieldOption {
            value: 1238,
            label: "Pyrocrlastic Flow".to_string(),
        },
        FieldOption {
            value: 1239,
            label: "Misery".to_string(),
        },
        FieldOption {
            value: 1240,
            label: "Brimstone".to_string(),
        },
        FieldOption {
            value: 1241,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 1242,
            label: "Iceblast II".to_string(),
        },
        FieldOption {
            value: 1243,
            label: "Iceblast III".to_string(),
        },
        FieldOption {
            value: 1244,
            label: "Iceblast IV".to_string(),
        },
        FieldOption {
            value: 1245,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 1246,
            label: "Avalanche II".to_string(),
        },
        FieldOption {
            value: 1247,
            label: "Avalanche III".to_string(),
        },
        FieldOption {
            value: 1248,
            label: "Avalanche IV".to_string(),
        },
        FieldOption {
            value: 1249,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 1250,
            label: "Wendigo II".to_string(),
        },
        FieldOption {
            value: 1251,
            label: "Ice Requiem".to_string(),
        },
        FieldOption {
            value: 1252,
            label: "Ice Requiem II".to_string(),
        },
        FieldOption {
            value: 1253,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 1254,
            label: "Frost Gaurd".to_string(),
        },
        FieldOption {
            value: 1255,
            label: "Icy Focus".to_string(),
        },
        FieldOption {
            value: 1256,
            label: "Indomitable Will".to_string(),
        },
        FieldOption {
            value: 1257,
            label: "Numbing Cold".to_string(),
        },
        FieldOption {
            value: 1258,
            label: "Freezing Gust".to_string(),
        },
        FieldOption {
            value: 1259,
            label: "Spiritsurge".to_string(),
        },
        FieldOption {
            value: 1260,
            label: "Spiritsurge II".to_string(),
        },
        FieldOption {
            value: 1261,
            label: "Spiritsurge III".to_string(),
        },
        FieldOption {
            value: 1262,
            label: "Spiritsurge IV".to_string(),
        },
        FieldOption {
            value: 1263,
            label: "Judgement".to_string(),
        },
        FieldOption {
            value: 1264,
            label: "Judgement II".to_string(),
        },
        FieldOption {
            value: 1265,
            label: "Judgement III".to_string(),
        },
        FieldOption {
            value: 1266,
            label: "Judgement IV".to_string(),
        },
        FieldOption {
            value: 1267,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 1268,
            label: "Wisplight II".to_string(),
        },
        FieldOption {
            value: 1269,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 1270,
            label: "Heavenly Judge II".to_string(),
        },
        FieldOption {
            value: 1271,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 1272,
            label: "Exorcism II".to_string(),
        },
        FieldOption {
            value: 1273,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 1274,
            label: "Light guard".to_string(),
        },
        FieldOption {
            value: 1275,
            label: "Silent Light".to_string(),
        },
        FieldOption {
            value: 1276,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 1277,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 1278,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 1279,
            label: "Awaken II".to_string(),
        },
        FieldOption {
            value: 1280,
            label: "Innervate".to_string(),
        },
        FieldOption {
            value: 1281,
            label: "Singing Light".to_string(),
        },
        FieldOption {
            value: 1282,
            label: "Awaken Stone".to_string(),
        },
        FieldOption {
            value: 1283,
            label: "Liberate".to_string(),
        },
        FieldOption {
            value: 1284,
            label: "Cleanse".to_string(),
        },
        FieldOption {
            value: 1285,
            label: "Cleanse II".to_string(),
        },
        FieldOption {
            value: 1286,
            label: "Unburden".to_string(),
        },
        FieldOption {
            value: 1287,
            label: "Decurse".to_string(),
        },
        FieldOption {
            value: 1288,
            label: "Hearten".to_string(),
        },
        FieldOption {
            value: 1289,
            label: "Ease".to_string(),
        },
        FieldOption {
            value: 1290,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 1291,
            label: "Heal II".to_string(),
        },
        FieldOption {
            value: 1292,
            label: "Heal III".to_string(),
        },
        FieldOption {
            value: 1293,
            label: "Heal IV".to_string(),
        },
        FieldOption {
            value: 1294,
            label: "Major Heal".to_string(),
        },
        FieldOption {
            value: 1295,
            label: "Major Heal II".to_string(),
        },
        FieldOption {
            value: 1296,
            label: "Major Heal III".to_string(),
        },
        FieldOption {
            value: 1297,
            label: "Resurrect".to_string(),
        },
        FieldOption {
            value: 1298,
            label: "Resurrect II".to_string(),
        },
        FieldOption {
            value: 1299,
            label: "Word of Pain".to_string(),
        },
        FieldOption {
            value: 1300,
            label: "Word of Pain II".to_string(),
        },
        FieldOption {
            value: 1301,
            label: "Word of Pain III".to_string(),
        },
        FieldOption {
            value: 1302,
            label: "Word of Pain IV".to_string(),
        },
        FieldOption {
            value: 1303,
            label: "Meteor Strike".to_string(),
        },
        FieldOption {
            value: 1304,
            label: "Meteor Strike II".to_string(),
        },
        FieldOption {
            value: 1305,
            label: "Meteor Strike III".to_string(),
        },
        FieldOption {
            value: 1306,
            label: "Meteor Strike IV".to_string(),
        },
        FieldOption {
            value: 1307,
            label: "Hellbound".to_string(),
        },
        FieldOption {
            value: 1308,
            label: "Hellbound II".to_string(),
        },
        FieldOption {
            value: 1309,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 1310,
            label: "Abyss II".to_string(),
        },
        FieldOption {
            value: 1311,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 1312,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 1313,
            label: "Drain Power".to_string(),
        },
        FieldOption {
            value: 1314,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 1315,
            label: "Shadow Gaurd".to_string(),
        },
        FieldOption {
            value: 1316,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 1317,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 1318,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 1319,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 1320,
            label: "Paralytic Wave".to_string(),
        },
        FieldOption {
            value: 1321,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 1322,
            label: "Deadly Poison".to_string(),
        },
        FieldOption {
            value: 1323,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 1324,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 1325,
            label: "Dominate".to_string(),
        },
        FieldOption {
            value: 1326,
            label: "Shackle".to_string(),
        },
        FieldOption {
            value: 1327,
            label: "Fixate".to_string(),
        },
        FieldOption {
            value: 1328,
            label: "Gravity Flux".to_string(),
        },
        FieldOption {
            value: 1329,
            label: "Deadscream".to_string(),
        },
        FieldOption {
            value: 1330,
            label: "Dead Mans Ivy".to_string(),
        },
        FieldOption {
            value: 1331,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 1332,
            label: "Tempest II".to_string(),
        },
        FieldOption {
            value: 1333,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 1334,
            label: "Gaia Strike II".to_string(),
        },
        FieldOption {
            value: 1335,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 1336,
            label: "Vortex II".to_string(),
        },
        FieldOption {
            value: 1337,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 1338,
            label: "Deluge II".to_string(),
        },
        FieldOption {
            value: 1339,
            label: "Annihilation".to_string(),
        },
        FieldOption {
            value: 1340,
            label: "Annihilation II".to_string(),
        },
        FieldOption {
            value: 1341,
            label: "Iceover".to_string(),
        },
        FieldOption {
            value: 1342,
            label: "Iceover II".to_string(),
        },
        FieldOption {
            value: 1343,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 1344,
            label: "Starfall II".to_string(),
        },
        FieldOption {
            value: 1345,
            label: "Diablo's Spite".to_string(),
        },
        FieldOption {
            value: 1346,
            label: "Diablo's Spite II".to_string(),
        },
        FieldOption {
            value: 1347,
            label: "Palace Guide Book".to_string(),
        },
        FieldOption {
            value: 1348,
            label: "Detect".to_string(),
        },
        FieldOption {
            value: 1349,
            label: "Springboard".to_string(),
        },
        FieldOption {
            value: 1350,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 1351,
            label: "Palace Guide Book II".to_string(),
        },
        FieldOption {
            value: 1352,
            label: "Gift of Restoration".to_string(),
        },
        FieldOption {
            value: 1353,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 1354,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 1355,
            label: "Negate Spell".to_string(),
        },
        FieldOption {
            value: 1356,
            label: "Dodge Blades".to_string(),
        },
        FieldOption {
            value: 1357,
            label: "Ballistics".to_string(),
        },
        FieldOption {
            value: 1358,
            label: "Enlighten".to_string(),
        },
        FieldOption {
            value: 1359,
            label: "Phantom Shell".to_string(),
        },
        FieldOption {
            value: 1360,
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 1361,
            label: "Sacrifice".to_string(),
        },
        FieldOption {
            value: 1362,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1363,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 1364,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 1365,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 1366,
            label: "Curse II".to_string(),
        },
        FieldOption {
            value: 1367,
            label: "Curse III".to_string(),
        },
        FieldOption {
            value: 1368,
            label: "Tainted Love".to_string(),
        },
        FieldOption {
            value: 1369,
            label: "Prodigize".to_string(),
        },
        FieldOption {
            value: 1370,
            label: "Breed Suspicion".to_string(),
        },
        FieldOption {
            value: 1371,
            label: "Phantom Pain".to_string(),
        },
        FieldOption {
            value: 1372,
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 1373,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 1374,
            label: "Putrify II".to_string(),
        },
        FieldOption {
            value: 1375,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1376,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 1377,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 1378,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 1379,
            label: "Wind Dervish".to_string(),
        },
        FieldOption {
            value: 1380,
            label: "Wind Dervish II".to_string(),
        },
        FieldOption {
            value: 1381,
            label: "Sand Spider".to_string(),
        },
        FieldOption {
            value: 1382,
            label: "Sand Spider II".to_string(),
        },
        FieldOption {
            value: 1383,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 1384,
            label: "Chimaera II".to_string(),
        },
        FieldOption {
            value: 1385,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 1386,
            label: "Water Tiger II".to_string(),
        },
        FieldOption {
            value: 1387,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 1388,
            label: "Fire Snake II".to_string(),
        },
        FieldOption {
            value: 1389,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 1390,
            label: "Rime Raven II".to_string(),
        },
        FieldOption {
            value: 1391,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 1392,
            label: "Palace Guide Book III".to_string(),
        },
        FieldOption {
            value: 1393,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 1394,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 1395,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 1396,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 1397,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 1398,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 1399,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 1400,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 1401,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 1402,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 1403,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 1404,
            label: "Bedeviling Dance".to_string(),
        },
        FieldOption {
            value: 1405,
            label: "Invirogating Dance".to_string(),
        },
        FieldOption {
            value: 1406,
            label: "Demonpetal Dance".to_string(),
        },
        FieldOption {
            value: 1407,
            label: "Ardent Conga".to_string(),
        },
        FieldOption {
            value: 1408,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 1409,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 1410,
            label: "Stiring Folclore".to_string(),
        },
        FieldOption {
            value: 1411,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 1412,
            label: "Escalating Sanat".to_string(),
        },
        FieldOption {
            value: 1413,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 1414,
            label: "Glass Pumpkin".to_string(),
        },
        FieldOption {
            value: 1415,
            label: "Heaven's Fork".to_string(),
        },
        FieldOption {
            value: 1416,
            label: "Warrior's Mark".to_string(),
        },
        FieldOption {
            value: 1417,
            label: "Archer's Mark".to_string(),
        },
        FieldOption {
            value: 1418,
            label: "Mage's Mark".to_string(),
        },
        FieldOption {
            value: 1419,
            label: "Sibyl's Mark".to_string(),
        },
        FieldOption {
            value: 1420,
            label: "Mage-Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1421,
            label: "Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1422,
            label: "Dreadknight's Mark".to_string(),
        },
        FieldOption {
            value: 1423,
            label: "Berserker's Mark".to_string(),
        },
        FieldOption {
            value: 1424,
            label: "Swordman's Mark".to_string(),
        },
        FieldOption {
            value: 1425,
            label: "Dragoon's Mark".to_string(),
        },
        FieldOption {
            value: 1426,
            label: "Ninja's Mark".to_string(),
        },
        FieldOption {
            value: 1427,
            label: "Bandit's Mark".to_string(),
        },
        FieldOption {
            value: 1428,
            label: "Fusilier's Mark".to_string(),
        },
        FieldOption {
            value: 1429,
            label: "Beastmaster's Mark".to_string(),
        },
        FieldOption {
            value: 1430,
            label: "Magus's Mark".to_string(),
        },
        FieldOption {
            value: 1431,
            label: "Necroprentice's Mark".to_string(),
        },
        FieldOption {
            value: 1432,
            label: "Footsoldier's Mark".to_string(),
        },
        FieldOption {
            value: 1433,
            label: "Juggernaut's Mark".to_string(),
        },
        FieldOption {
            value: 1434,
            label: "Chief's Mark".to_string(),
        },
        FieldOption {
            value: 1435,
            label: "Familiar's Mark".to_string(),
        },
        FieldOption {
            value: 1436,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1437,
            label: "Windwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1438,
            label: "Cragwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1439,
            label: "Stormwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1440,
            label: "Waterwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1441,
            label: "Firewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1442,
            label: "Icewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1443,
            label: "Gleamwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1444,
            label: "Gloomwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1445,
            label: "Sandstone's Mark".to_string(),
        },
        FieldOption {
            value: 1446,
            label: "Granite's Mark".to_string(),
        },
        FieldOption {
            value: 1447,
            label: "Black Iron's Mark".to_string(),
        },
        FieldOption {
            value: 1448,
            label: "Magesteel's Mark".to_string(),
        },
        FieldOption {
            value: 1449,
            label: "Sovereign's Mark".to_string(),
        },
        FieldOption {
            value: 1450,
            label: "Brave's Mark".to_string(),
        },
        FieldOption {
            value: 1451,
            label: "Abuna's Mark".to_string(),
        },
        FieldOption {
            value: 1452,
            label: "Heretic's Mark".to_string(),
        },
        FieldOption {
            value: 1453,
            label: "Princess's Mark".to_string(),
        },
        FieldOption {
            value: 1454,
            label: "Holy Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1455,
            label: "Star Seer's Mark".to_string(),
        },
        FieldOption {
            value: 1456,
            label: "Peregrine's Mark".to_string(),
        },
        FieldOption {
            value: 1457,
            label: "White Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1458,
            label: "Oracle's Mark".to_string(),
        },
        FieldOption {
            value: 1459,
            label: "Wicce's Mark".to_string(),
        },
        FieldOption {
            value: 1460,
            label: "Songstress's Mark".to_string(),
        },
        FieldOption {
            value: 1461,
            label: "Hagiaknight's Mark".to_string(),
        },
        FieldOption {
            value: 1462,
            label: "Pirate's Mark".to_string(),
        },
        FieldOption {
            value: 1463,
            label: "Inferior Ore".to_string(),
        },
        FieldOption {
            value: 1464,
            label: "Iron Sand".to_string(),
        },
        FieldOption {
            value: 1465,
            label: "Copper Ore".to_string(),
        },
        FieldOption {
            value: 1466,
            label: "Tin Ore".to_string(),
        },
        FieldOption {
            value: 1467,
            label: "Graphite".to_string(),
        },
        FieldOption {
            value: 1468,
            label: "Iron Ore".to_string(),
        },
        FieldOption {
            value: 1469,
            label: "Silver Ore".to_string(),
        },
        FieldOption {
            value: 1470,
            label: "Baldur Ore".to_string(),
        },
        FieldOption {
            value: 1471,
            label: "Gold Ore".to_string(),
        },
        FieldOption {
            value: 1472,
            label: "Platinum Ore".to_string(),
        },
        FieldOption {
            value: 1473,
            label: "Saltpeter".to_string(),
        },
        FieldOption {
            value: 1474,
            label: "Sulfur".to_string(),
        },
        FieldOption {
            value: 1475,
            label: "Limestone".to_string(),
        },
        FieldOption {
            value: 1476,
            label: "Skyiron".to_string(),
        },
        FieldOption {
            value: 1477,
            label: "Gemstones".to_string(),
        },
        FieldOption {
            value: 1478,
            label: "Krystallos Ore".to_string(),
        },
        FieldOption {
            value: 1479,
            label: "Bronze Ingot".to_string(),
        },
        FieldOption {
            value: 1480,
            label: "Iron Ingot".to_string(),
        },
        FieldOption {
            value: 1481,
            label: "Silver Ingot".to_string(),
        },
        FieldOption {
            value: 1482,
            label: "Baldur Ingot".to_string(),
        },
        FieldOption {
            value: 1483,
            label: "Steel Ingot".to_string(),
        },
        FieldOption {
            value: 1484,
            label: "Hagane Steel".to_string(),
        },
        FieldOption {
            value: 1485,
            label: "Wootz Steel".to_string(),
        },
        FieldOption {
            value: 1486,
            label: "Golden Ingot".to_string(),
        },
        FieldOption {
            value: 1487,
            label: "Platinum Ingot".to_string(),
        },
        FieldOption {
            value: 1488,
            label: "Fiery Gems".to_string(),
        },
        FieldOption {
            value: 1489,
            label: "Verdant Gems".to_string(),
        },
        FieldOption {
            value: 1490,
            label: "Regal Gems".to_string(),
        },
        FieldOption {
            value: 1491,
            label: "White Gems".to_string(),
        },
        FieldOption {
            value: 1492,
            label: "Black Gems".to_string(),
        },
        FieldOption {
            value: 1493,
            label: "Air Krystallos".to_string(),
        },
        FieldOption {
            value: 1494,
            label: "Earth Krystallos".to_string(),
        },
        FieldOption {
            value: 1495,
            label: "Lightning Krystallos".to_string(),
        },
        FieldOption {
            value: 1496,
            label: "Water Krystallos".to_string(),
        },
        FieldOption {
            value: 1497,
            label: "Fire Krystallos".to_string(),
        },
        FieldOption {
            value: 1498,
            label: "Ice Krystallos".to_string(),
        },
        FieldOption {
            value: 1499,
            label: "Light Krystallos".to_string(),
        },
        FieldOption {
            value: 1500,
            label: "Dark Krystallos".to_string(),
        },
        FieldOption {
            value: 1501,
            label: "Toneriwood".to_string(),
        },
        FieldOption {
            value: 1502,
            label: "Birnewood".to_string(),
        },
        FieldOption {
            value: 1503,
            label: "Ananawood".to_string(),
        },
        FieldOption {
            value: 1504,
            label: "Baobawood".to_string(),
        },
        FieldOption {
            value: 1505,
            label: "Beasthide".to_string(),
        },
        FieldOption {
            value: 1506,
            label: "Tannin".to_string(),
        },
        FieldOption {
            value: 1507,
            label: "Leather".to_string(),
        },
        FieldOption {
            value: 1508,
            label: "Parchment".to_string(),
        },
        FieldOption {
            value: 1509,
            label: "Ink".to_string(),
        },
        FieldOption {
            value: 1510,
            label: "Gold Leaf".to_string(),
        },
        FieldOption {
            value: 1511,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 1512,
            label: "Log".to_string(),
        },
        FieldOption {
            value: 1513,
            label: "Bundle of Herbs".to_string(),
        },
        FieldOption {
            value: 1514,
            label: "Herbal Extract".to_string(),
        },
        FieldOption {
            value: 1515,
            label: "Nightshade".to_string(),
        },
        FieldOption {
            value: 1516,
            label: "Nightshade Extract".to_string(),
        },
        FieldOption {
            value: 1517,
            label: "Fruit".to_string(),
        },
        FieldOption {
            value: 1518,
            label: "Spirits".to_string(),
        },
        FieldOption {
            value: 1519,
            label: "Hempen Thread".to_string(),
        },
        FieldOption {
            value: 1520,
            label: "Woolen Thread".to_string(),
        },
        FieldOption {
            value: 1521,
            label: "Cotton Thread".to_string(),
        },
        FieldOption {
            value: 1522,
            label: "Silken Thread".to_string(),
        },
        FieldOption {
            value: 1523,
            label: "Silver Thread".to_string(),
        },
        FieldOption {
            value: 1524,
            label: "Golden Thread".to_string(),
        },
        FieldOption {
            value: 1525,
            label: "Linen".to_string(),
        },
        FieldOption {
            value: 1526,
            label: "Pincord".to_string(),
        },
        FieldOption {
            value: 1527,
            label: "Flannel".to_string(),
        },
        FieldOption {
            value: 1528,
            label: "Velvet".to_string(),
        },
        FieldOption {
            value: 1529,
            label: "Satin".to_string(),
        },
        FieldOption {
            value: 1530,
            label: "Blackpowder".to_string(),
        },
        FieldOption {
            value: 1531,
            label: "Beast Horn".to_string(),
        },
        FieldOption {
            value: 1532,
            label: "Beast Fang".to_string(),
        },
        FieldOption {
            value: 1533,
            label: "Beast Claw".to_string(),
        },
        FieldOption {
            value: 1534,
            label: "Wyrm Fang".to_string(),
        },
        FieldOption {
            value: 1535,
            label: "Wyrm Claw".to_string(),
        },
        FieldOption {
            value: 1536,
            label: "Wyrm Scale".to_string(),
        },
        FieldOption {
            value: 1537,
            label: "Wyrm Horn".to_string(),
        },
        FieldOption {
            value: 1538,
            label: "Wyrm Whisker".to_string(),
        },
        FieldOption {
            value: 1539,
            label: "Wyrm Thighbone".to_string(),
        },
        FieldOption {
            value: 1540,
            label: "Tooth & Claw".to_string(),
        },
        FieldOption {
            value: 1541,
            label: "Unicorn Horn".to_string(),
        },
        FieldOption {
            value: 1542,
            label: "Enchanted Feather".to_string(),
        },
        FieldOption {
            value: 1543,
            label: "Ancient Wood".to_string(),
        },
        FieldOption {
            value: 1544,
            label: "Ancient Bone".to_string(),
        },
        FieldOption {
            value: 1545,
            label: "Orichalcum".to_string(),
        },
        FieldOption {
            value: 1546,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1547,
            label: "Daedalus Pinion".to_string(),
        },
        FieldOption {
            value: 1548,
            label: "Daedalus Rack".to_string(),
        },
        FieldOption {
            value: 1549,
            label: "Melee Weapons I".to_string(),
        },
        FieldOption {
            value: 1550,
            label: "Melee Weapons II".to_string(),
        },
        FieldOption {
            value: 1551,
            label: "The Fist".to_string(),
        },
        FieldOption {
            value: 1552,
            label: "Fist Enchiridion".to_string(),
        },
        FieldOption {
            value: 1553,
            label: "The Blade".to_string(),
        },
        FieldOption {
            value: 1554,
            label: "Dagger Enchiridion".to_string(),
        },
        FieldOption {
            value: 1555,
            label: "Sword Enchidirion".to_string(),
        },
        FieldOption {
            value: 1556,
            label: "2-H Sword Enchiridion".to_string(),
        },
        FieldOption {
            value: 1557,
            label: "Axe Spear & Hammer".to_string(),
        },
        FieldOption {
            value: 1558,
            label: "Axe Enchiridion".to_string(),
        },
        FieldOption {
            value: 1559,
            label: "Spear Enchiridion".to_string(),
        },
        FieldOption {
            value: 1560,
            label: "Hammer Enchiridion".to_string(),
        },
        FieldOption {
            value: 1561,
            label: "The Katana".to_string(),
        },
        FieldOption {
            value: 1562,
            label: "Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1563,
            label: "2-H Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1564,
            label: "Cudgel & Whip".to_string(),
        },
        FieldOption {
            value: 1565,
            label: "Cudgel Enchiridion".to_string(),
        },
        FieldOption {
            value: 1566,
            label: "Whip Enchiridion".to_string(),
        },
        FieldOption {
            value: 1567,
            label: "Transcription".to_string(),
        },
        FieldOption {
            value: 1568,
            label: "Musical Instruments I".to_string(),
        },
        FieldOption {
            value: 1569,
            label: "Musical Instruments II".to_string(),
        },
        FieldOption {
            value: 1570,
            label: "Ranged Weapons I".to_string(),
        },
        FieldOption {
            value: 1571,
            label: "Ranged Weapons II".to_string(),
        },
        FieldOption {
            value: 1572,
            label: "Ways of the Gerges".to_string(),
        },
        FieldOption {
            value: 1573,
            label: "The Bow".to_string(),
        },
        FieldOption {
            value: 1574,
            label: "Bow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1575,
            label: "The Crossbow".to_string(),
        },
        FieldOption {
            value: 1576,
            label: "Crossbow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1577,
            label: "The Fusil".to_string(),
        },
        FieldOption {
            value: 1578,
            label: "Fusil Enchiridion".to_string(),
        },
        FieldOption {
            value: 1579,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1580,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1581,
            label: "Smithing Armor I".to_string(),
        },
        FieldOption {
            value: 1582,
            label: "Smithing Armor II".to_string(),
        },
        FieldOption {
            value: 1583,
            label: "Armorcraft".to_string(),
        },
        FieldOption {
            value: 1584,
            label: "Shieldcraft".to_string(),
        },
        FieldOption {
            value: 1585,
            label: "Shield Enchiridion".to_string(),
        },
        FieldOption {
            value: 1586,
            label: "Helm Enchiridion".to_string(),
        },
        FieldOption {
            value: 1587,
            label: "Body Armor Enchiridion".to_string(),
        },
        FieldOption {
            value: 1588,
            label: "Armguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1589,
            label: "Legguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1590,
            label: "Codex of Jewelry I".to_string(),
        },
        FieldOption {
            value: 1591,
            label: "Codex of Jewelry II".to_string(),
        },
        FieldOption {
            value: 1592,
            label: "Codex of Jewelry III".to_string(),
        },
        FieldOption {
            value: 1593,
            label: "Codex of Jewelry IV".to_string(),
        },
        FieldOption {
            value: 1594,
            label: "Codex of Ores".to_string(),
        },
        FieldOption {
            value: 1595,
            label: "Codex of Gems".to_string(),
        },
        FieldOption {
            value: 1596,
            label: "Codex of Timber".to_string(),
        },
        FieldOption {
            value: 1597,
            label: "Codex of Textiles".to_string(),
        },
        FieldOption {
            value: 1598,
            label: "On Medicine I".to_string(),
        },
        FieldOption {
            value: 1599,
            label: "On Medicine II".to_string(),
        },
        FieldOption {
            value: 1600,
            label: "Secrets of the Master".to_string(),
        },
        FieldOption {
            value: 1601,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1602,
            label: "Ease II".to_string(),
        },
        FieldOption {
            value: 1603,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1604,
            label: "STR Charm".to_string(),
        },
        FieldOption {
            value: 1605,
            label: "VIT Charm".to_string(),
        },
        FieldOption {
            value: 1606,
            label: "DEX Charm".to_string(),
        },
        FieldOption {
            value: 1607,
            label: "AGI Charm".to_string(),
        },
        FieldOption {
            value: 1608,
            label: "AVD Charm".to_string(),
        },
        FieldOption {
            value: 1609,
            label: "INT Charm".to_string(),
        },
        FieldOption {
            value: 1610,
            label: "MND Charm".to_string(),
        },
        FieldOption {
            value: 1611,
            label: "RES Charm".to_string(),
        },
        FieldOption {
            value: 1612,
            label: "LUK Charm".to_string(),
        },
        FieldOption {
            value: 1613,
            label: "Air Charm".to_string(),
        },
        FieldOption {
            value: 1614,
            label: "Earth Charm".to_string(),
        },
        FieldOption {
            value: 1615,
            label: "Lightening Charm".to_string(),
        },
        FieldOption {
            value: 1616,
            label: "Water Charm".to_string(),
        },
        FieldOption {
            value: 1617,
            label: "Fire Charm".to_string(),
        },
        FieldOption {
            value: 1618,
            label: "Ice Charm".to_string(),
        },
        FieldOption {
            value: 1619,
            label: "Light Charm".to_string(),
        },
        FieldOption {
            value: 1620,
            label: "Dark Charm".to_string(),
        },
        FieldOption {
            value: 1621,
            label: "Experience Charm".to_string(),
        },
        FieldOption {
            value: 1622,
            label: "Experience Charm II".to_string(),
        },
        FieldOption {
            value: 1623,
            label: "Experience Charm III".to_string(),
        },
        FieldOption {
            value: 1624,
            label: "Experience Charm IV".to_string(),
        },
        FieldOption {
            value: 1625,
            label: "Experience Charm V".to_string(),
        },
        FieldOption {
            value: 1626,
            label: "Level Up Charm".to_string(),
        },
        FieldOption {
            value: 1627,
            label: "Grimoire Exorcisme".to_string(),
        },
    ]
}

fn opt_item_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1000,
            label: "Mend Leaf".to_string(),
        },
        FieldOption {
            value: 1001,
            label: "Mend Leaf +1".to_string(),
        },
        FieldOption {
            value: 1002,
            label: "Mend Leaf +2".to_string(),
        },
        FieldOption {
            value: 1003,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1004,
            label: "Mending Seed".to_string(),
        },
        FieldOption {
            value: 1005,
            label: "Mending Seed +1".to_string(),
        },
        FieldOption {
            value: 1006,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1007,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1008,
            label: "Mending Salve".to_string(),
        },
        FieldOption {
            value: 1009,
            label: "Mending Salve +1".to_string(),
        },
        FieldOption {
            value: 1010,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1011,
            label: "Mending Essence".to_string(),
        },
        FieldOption {
            value: 1012,
            label: "Magic Leaf".to_string(),
        },
        FieldOption {
            value: 1013,
            label: "Magic Leaf +1".to_string(),
        },
        FieldOption {
            value: 1014,
            label: "Magic Leaf +2".to_string(),
        },
        FieldOption {
            value: 1015,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1016,
            label: "Magic Seed".to_string(),
        },
        FieldOption {
            value: 1017,
            label: "Magic Seed +1".to_string(),
        },
        FieldOption {
            value: 1018,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1019,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1020,
            label: "Magic Salve".to_string(),
        },
        FieldOption {
            value: 1021,
            label: "Magic Salve +1".to_string(),
        },
        FieldOption {
            value: 1022,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1023,
            label: "Magic Essence".to_string(),
        },
        FieldOption {
            value: 1024,
            label: "Fruit of the Adept".to_string(),
        },
        FieldOption {
            value: 1025,
            label: "Fruit of the Adept +1".to_string(),
        },
        FieldOption {
            value: 1026,
            label: "Fruit of the Sage".to_string(),
        },
        FieldOption {
            value: 1027,
            label: "Fruit of the Sage +1".to_string(),
        },
        FieldOption {
            value: 1028,
            label: "Fruit of the Seraph".to_string(),
        },
        FieldOption {
            value: 1029,
            label: "Overripe Fruit".to_string(),
        },
        FieldOption {
            value: 1030,
            label: "Zolia Draught".to_string(),
        },
        FieldOption {
            value: 1031,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1032,
            label: "Zena Wine".to_string(),
        },
        FieldOption {
            value: 1033,
            label: "Illumina Nectar".to_string(),
        },
        FieldOption {
            value: 1034,
            label: "Gerun Powder".to_string(),
        },
        FieldOption {
            value: 1035,
            label: "Feyrn Bolus".to_string(),
        },
        FieldOption {
            value: 1036,
            label: "Maca Antidote".to_string(),
        },
        FieldOption {
            value: 1037,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1038,
            label: "Jaarn's Poultice".to_string(),
        },
        FieldOption {
            value: 1039,
            label: "Ishtar's Ambrosia".to_string(),
        },
        FieldOption {
            value: 1040,
            label: "Ashmedai's Grog".to_string(),
        },
        FieldOption {
            value: 1041,
            label: "Blessing Stone".to_string(),
        },
        FieldOption {
            value: 1042,
            label: "Hallowing Stone".to_string(),
        },
        FieldOption {
            value: 1043,
            label: "Areion Plume".to_string(),
        },
        FieldOption {
            value: 1044,
            label: "Basin of Time".to_string(),
        },
        FieldOption {
            value: 1045,
            label: "Spiritstone of the Stars".to_string(),
        },
        FieldOption {
            value: 1046,
            label: "Faeriescale Powder".to_string(),
        },
        FieldOption {
            value: 1047,
            label: "Crystallized Flame".to_string(),
        },
        FieldOption {
            value: 1048,
            label: "Mercurial Phial".to_string(),
        },
        FieldOption {
            value: 1049,
            label: "Jewel of the Avatar".to_string(),
        },
        FieldOption {
            value: 1050,
            label: "Hair of the Unicorn".to_string(),
        },
        FieldOption {
            value: 1051,
            label: "Philtre of Ashes".to_string(),
        },
        FieldOption {
            value: 1052,
            label: "Black Lizard Powder".to_string(),
        },
        FieldOption {
            value: 1053,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1054,
            label: "Dragon Steak".to_string(),
        },
        FieldOption {
            value: 1055,
            label: "Braised Skewer".to_string(),
        },
        FieldOption {
            value: 1056,
            label: "Steamed Mollusk".to_string(),
        },
        FieldOption {
            value: 1057,
            label: "Minced Patty".to_string(),
        },
        FieldOption {
            value: 1058,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1059,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1060,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1061,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1062,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1063,
            label: "Brand of the Sacrifice".to_string(),
        },
        FieldOption {
            value: 1064,
            label: "Dynast-King's Mead".to_string(),
        },
        FieldOption {
            value: 1065,
            label: "Echo Stone".to_string(),
        },
        FieldOption {
            value: 1066,
            label: "Blackwing Leg".to_string(),
        },
        FieldOption {
            value: 1067,
            label: "Rood Upright".to_string(),
        },
        FieldOption {
            value: 1068,
            label: "Haunt's Tome".to_string(),
        },
        FieldOption {
            value: 1069,
            label: "Darkscale Tome".to_string(),
        },
        FieldOption {
            value: 1070,
            label: "Cursed Unicorn Blood".to_string(),
        },
        FieldOption {
            value: 1071,
            label: "Skulldust Nostrum".to_string(),
        },
        FieldOption {
            value: 1072,
            label: "Magedrain Gland".to_string(),
        },
        FieldOption {
            value: 1073,
            label: "Shiftstone".to_string(),
        },
        FieldOption {
            value: 1074,
            label: "Horn of the Savage".to_string(),
        },
        FieldOption {
            value: 1075,
            label: "Coral Harp".to_string(),
        },
        FieldOption {
            value: 1076,
            label: "Whirlwind Shot".to_string(),
        },
        FieldOption {
            value: 1077,
            label: "Duststorm Shot".to_string(),
        },
        FieldOption {
            value: 1078,
            label: "Thunder Shot".to_string(),
        },
        FieldOption {
            value: 1079,
            label: "Torrent Shot".to_string(),
        },
        FieldOption {
            value: 1080,
            label: "Conflagration Shot".to_string(),
        },
        FieldOption {
            value: 1081,
            label: "Firnice Shot".to_string(),
        },
        FieldOption {
            value: 1082,
            label: "Coruscate Shot".to_string(),
        },
        FieldOption {
            value: 1083,
            label: "Murk Shot".to_string(),
        },
        FieldOption {
            value: 1084,
            label: "Book of the Dead".to_string(),
        },
        FieldOption {
            value: 1085,
            label: "Ring of the Dead".to_string(),
        },
        FieldOption {
            value: 1086,
            label: "Ensanguined Rood".to_string(),
        },
        FieldOption {
            value: 1087,
            label: "Seal of Rebirth".to_string(),
        },
        FieldOption {
            value: 1088,
            label: "Void Orb".to_string(),
        },
        FieldOption {
            value: 1089,
            label: "Gale Orb".to_string(),
        },
        FieldOption {
            value: 1090,
            label: "Dust Orb".to_string(),
        },
        FieldOption {
            value: 1091,
            label: "Storm Orb".to_string(),
        },
        FieldOption {
            value: 1092,
            label: "Cataract Orb".to_string(),
        },
        FieldOption {
            value: 1093,
            label: "Inferno Orb".to_string(),
        },
        FieldOption {
            value: 1094,
            label: "Black Ice Orb".to_string(),
        },
        FieldOption {
            value: 1095,
            label: "Radiant Orb".to_string(),
        },
        FieldOption {
            value: 1096,
            label: "Gloom Orb".to_string(),
        },
        FieldOption {
            value: 1097,
            label: "Elixir".to_string(),
        },
        FieldOption {
            value: 1098,
            label: "Charm of Remission".to_string(),
        },
        FieldOption {
            value: 1099,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1100,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1101,
            label: "Intelligence Card".to_string(),
        },
        FieldOption {
            value: 1102,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1103,
            label: "MP Card".to_string(),
        },
        FieldOption {
            value: 1104,
            label: "HP Card".to_string(),
        },
        FieldOption {
            value: 1105,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1106,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1107,
            label: "Strength Card".to_string(),
        },
        FieldOption {
            value: 1108,
            label: "Strength Card (2)".to_string(),
        },
        FieldOption {
            value: 1109,
            label: "Intelligence Card (2)".to_string(),
        },
        FieldOption {
            value: 1110,
            label: "Avoidance Card".to_string(),
        },
        FieldOption {
            value: 1111,
            label: "Avoidance Card (2)".to_string(),
        },
        FieldOption {
            value: 1112,
            label: "Vitality Card".to_string(),
        },
        FieldOption {
            value: 1113,
            label: "Luck Card".to_string(),
        },
        FieldOption {
            value: 1114,
            label: "Resistance Card".to_string(),
        },
        FieldOption {
            value: 1115,
            label: "Luck Card (2)".to_string(),
        },
        FieldOption {
            value: 1116,
            label: "Vitality Card (2)".to_string(),
        },
        FieldOption {
            value: 1117,
            label: "Dexterity Card".to_string(),
        },
        FieldOption {
            value: 1118,
            label: "Agility Card".to_string(),
        },
        FieldOption {
            value: 1119,
            label: "Agility Card (2)".to_string(),
        },
        FieldOption {
            value: 1120,
            label: "Dexterity Card (2)".to_string(),
        },
        FieldOption {
            value: 1121,
            label: "Resistance Card (2)".to_string(),
        },
        FieldOption {
            value: 1122,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1123,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1124,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1125,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1126,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1127,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1128,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1129,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1130,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1131,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1132,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1133,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1134,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1135,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1136,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1137,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1138,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1139,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1140,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1141,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1142,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1143,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1144,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1145,
            label: "Copper Oberynth".to_string(),
        },
        FieldOption {
            value: 1146,
            label: "Bronze Oberynth".to_string(),
        },
        FieldOption {
            value: 1147,
            label: "Silver Oberynth".to_string(),
        },
        FieldOption {
            value: 1148,
            label: "Gold Oberynth".to_string(),
        },
        FieldOption {
            value: 1149,
            label: "Platinum Oberynth".to_string(),
        },
        FieldOption {
            value: 1150,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1151,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 1152,
            label: "Deadshot II".to_string(),
        },
        FieldOption {
            value: 1153,
            label: "Deadshot III".to_string(),
        },
        FieldOption {
            value: 1154,
            label: "Deadshot IV".to_string(),
        },
        FieldOption {
            value: 1155,
            label: "Tornado".to_string(),
        },
        FieldOption {
            value: 1156,
            label: "Tornado II".to_string(),
        },
        FieldOption {
            value: 1157,
            label: "Tornado III".to_string(),
        },
        FieldOption {
            value: 1158,
            label: "Tornado IV".to_string(),
        },
        FieldOption {
            value: 1159,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 1160,
            label: "Sylphide II".to_string(),
        },
        FieldOption {
            value: 1161,
            label: "Aeroflux".to_string(),
        },
        FieldOption {
            value: 1162,
            label: "Aeroflux II".to_string(),
        },
        FieldOption {
            value: 1163,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 1164,
            label: "Aeroguard".to_string(),
        },
        FieldOption {
            value: 1165,
            label: "Whirlwind".to_string(),
        },
        FieldOption {
            value: 1166,
            label: "Guarding Gale".to_string(),
        },
        FieldOption {
            value: 1167,
            label: "Balmy Breeze".to_string(),
        },
        FieldOption {
            value: 1168,
            label: "Black Williwaw".to_string(),
        },
        FieldOption {
            value: 1169,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 1170,
            label: "Vulcan Lance II".to_string(),
        },
        FieldOption {
            value: 1171,
            label: "Vulcan Lance III".to_string(),
        },
        FieldOption {
            value: 1172,
            label: "Vulcan Lance IV".to_string(),
        },
        FieldOption {
            value: 1173,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 1174,
            label: "Cragfall II".to_string(),
        },
        FieldOption {
            value: 1175,
            label: "Cragfall III".to_string(),
        },
        FieldOption {
            value: 1176,
            label: "Cragfall IV".to_string(),
        },
        FieldOption {
            value: 1177,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 1178,
            label: "Gnome II".to_string(),
        },
        FieldOption {
            value: 1179,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 1180,
            label: "Earthquake II".to_string(),
        },
        FieldOption {
            value: 1181,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 1182,
            label: "Petroguard".to_string(),
        },
        FieldOption {
            value: 1183,
            label: "Protect".to_string(),
        },
        FieldOption {
            value: 1184,
            label: "Blade Ward".to_string(),
        },
        FieldOption {
            value: 1185,
            label: "Duststrorm".to_string(),
        },
        FieldOption {
            value: 1186,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 1187,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 1188,
            label: "Lightning Bow II".to_string(),
        },
        FieldOption {
            value: 1189,
            label: "Lightning Bow III".to_string(),
        },
        FieldOption {
            value: 1190,
            label: "Lightning Bow IV".to_string(),
        },
        FieldOption {
            value: 1191,
            label: "Thunderflare".to_string(),
        },
        FieldOption {
            value: 1192,
            label: "Thunderflare II".to_string(),
        },
        FieldOption {
            value: 1193,
            label: "Thunderflare III".to_string(),
        },
        FieldOption {
            value: 1194,
            label: "Thunderflare IV".to_string(),
        },
        FieldOption {
            value: 1195,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 1196,
            label: "Thunderbird II".to_string(),
        },
        FieldOption {
            value: 1197,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 1198,
            label: "Thunderburst II".to_string(),
        },
        FieldOption {
            value: 1199,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 1200,
            label: "Electricgaurd".to_string(),
        },
        FieldOption {
            value: 1201,
            label: "Galvanize".to_string(),
        },
        FieldOption {
            value: 1202,
            label: "Stormspark".to_string(),
        },
        FieldOption {
            value: 1203,
            label: "Stunbomb".to_string(),
        },
        FieldOption {
            value: 1204,
            label: "Stunslay".to_string(),
        },
        FieldOption {
            value: 1205,
            label: "Aquablast".to_string(),
        },
        FieldOption {
            value: 1206,
            label: "Aquablast II".to_string(),
        },
        FieldOption {
            value: 1207,
            label: "Aquablast III".to_string(),
        },
        FieldOption {
            value: 1208,
            label: "Aquablast IV".to_string(),
        },
        FieldOption {
            value: 1209,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 1210,
            label: "Acid Rain II".to_string(),
        },
        FieldOption {
            value: 1211,
            label: "Acid Rain III".to_string(),
        },
        FieldOption {
            value: 1212,
            label: "Acid Rain IV".to_string(),
        },
        FieldOption {
            value: 1213,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 1214,
            label: "Undine II".to_string(),
        },
        FieldOption {
            value: 1215,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 1216,
            label: "Dread Vapor II".to_string(),
        },
        FieldOption {
            value: 1217,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 1218,
            label: "Aqaugaurd".to_string(),
        },
        FieldOption {
            value: 1219,
            label: "Quench".to_string(),
        },
        FieldOption {
            value: 1220,
            label: "Stagnate".to_string(),
        },
        FieldOption {
            value: 1221,
            label: "Poison Mist".to_string(),
        },
        FieldOption {
            value: 1222,
            label: "Sludgebind".to_string(),
        },
        FieldOption {
            value: 1223,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 1224,
            label: "Sparksphere II".to_string(),
        },
        FieldOption {
            value: 1225,
            label: "Sparksphere III".to_string(),
        },
        FieldOption {
            value: 1226,
            label: "Sparksphere IV".to_string(),
        },
        FieldOption {
            value: 1227,
            label: "Firestorm".to_string(),
        },
        FieldOption {
            value: 1228,
            label: "Firestorm II".to_string(),
        },
        FieldOption {
            value: 1229,
            label: "Firestorm III".to_string(),
        },
        FieldOption {
            value: 1230,
            label: "Firestorm IV".to_string(),
        },
        FieldOption {
            value: 1231,
            label: "Salamander".to_string(),
        },
        FieldOption {
            value: 1232,
            label: "Salamander II".to_string(),
        },
        FieldOption {
            value: 1233,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 1234,
            label: "Supernova II".to_string(),
        },
        FieldOption {
            value: 1235,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 1236,
            label: "Pryogaurd".to_string(),
        },
        FieldOption {
            value: 1237,
            label: "Flame Fusion".to_string(),
        },
        FieldOption {
            value: 1238,
            label: "Pyrocrlastic Flow".to_string(),
        },
        FieldOption {
            value: 1239,
            label: "Misery".to_string(),
        },
        FieldOption {
            value: 1240,
            label: "Brimstone".to_string(),
        },
        FieldOption {
            value: 1241,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 1242,
            label: "Iceblast II".to_string(),
        },
        FieldOption {
            value: 1243,
            label: "Iceblast III".to_string(),
        },
        FieldOption {
            value: 1244,
            label: "Iceblast IV".to_string(),
        },
        FieldOption {
            value: 1245,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 1246,
            label: "Avalanche II".to_string(),
        },
        FieldOption {
            value: 1247,
            label: "Avalanche III".to_string(),
        },
        FieldOption {
            value: 1248,
            label: "Avalanche IV".to_string(),
        },
        FieldOption {
            value: 1249,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 1250,
            label: "Wendigo II".to_string(),
        },
        FieldOption {
            value: 1251,
            label: "Ice Requiem".to_string(),
        },
        FieldOption {
            value: 1252,
            label: "Ice Requiem II".to_string(),
        },
        FieldOption {
            value: 1253,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 1254,
            label: "Frost Gaurd".to_string(),
        },
        FieldOption {
            value: 1255,
            label: "Icy Focus".to_string(),
        },
        FieldOption {
            value: 1256,
            label: "Indomitable Will".to_string(),
        },
        FieldOption {
            value: 1257,
            label: "Numbing Cold".to_string(),
        },
        FieldOption {
            value: 1258,
            label: "Freezing Gust".to_string(),
        },
        FieldOption {
            value: 1259,
            label: "Spiritsurge".to_string(),
        },
        FieldOption {
            value: 1260,
            label: "Spiritsurge II".to_string(),
        },
        FieldOption {
            value: 1261,
            label: "Spiritsurge III".to_string(),
        },
        FieldOption {
            value: 1262,
            label: "Spiritsurge IV".to_string(),
        },
        FieldOption {
            value: 1263,
            label: "Judgement".to_string(),
        },
        FieldOption {
            value: 1264,
            label: "Judgement II".to_string(),
        },
        FieldOption {
            value: 1265,
            label: "Judgement III".to_string(),
        },
        FieldOption {
            value: 1266,
            label: "Judgement IV".to_string(),
        },
        FieldOption {
            value: 1267,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 1268,
            label: "Wisplight II".to_string(),
        },
        FieldOption {
            value: 1269,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 1270,
            label: "Heavenly Judge II".to_string(),
        },
        FieldOption {
            value: 1271,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 1272,
            label: "Exorcism II".to_string(),
        },
        FieldOption {
            value: 1273,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 1274,
            label: "Light guard".to_string(),
        },
        FieldOption {
            value: 1275,
            label: "Silent Light".to_string(),
        },
        FieldOption {
            value: 1276,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 1277,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 1278,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 1279,
            label: "Awaken II".to_string(),
        },
        FieldOption {
            value: 1280,
            label: "Innervate".to_string(),
        },
        FieldOption {
            value: 1281,
            label: "Singing Light".to_string(),
        },
        FieldOption {
            value: 1282,
            label: "Awaken Stone".to_string(),
        },
        FieldOption {
            value: 1283,
            label: "Liberate".to_string(),
        },
        FieldOption {
            value: 1284,
            label: "Cleanse".to_string(),
        },
        FieldOption {
            value: 1285,
            label: "Cleanse II".to_string(),
        },
        FieldOption {
            value: 1286,
            label: "Unburden".to_string(),
        },
        FieldOption {
            value: 1287,
            label: "Decurse".to_string(),
        },
        FieldOption {
            value: 1288,
            label: "Hearten".to_string(),
        },
        FieldOption {
            value: 1289,
            label: "Ease".to_string(),
        },
        FieldOption {
            value: 1290,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 1291,
            label: "Heal II".to_string(),
        },
        FieldOption {
            value: 1292,
            label: "Heal III".to_string(),
        },
        FieldOption {
            value: 1293,
            label: "Heal IV".to_string(),
        },
        FieldOption {
            value: 1294,
            label: "Major Heal".to_string(),
        },
        FieldOption {
            value: 1295,
            label: "Major Heal II".to_string(),
        },
        FieldOption {
            value: 1296,
            label: "Major Heal III".to_string(),
        },
        FieldOption {
            value: 1297,
            label: "Resurrect".to_string(),
        },
        FieldOption {
            value: 1298,
            label: "Resurrect II".to_string(),
        },
        FieldOption {
            value: 1299,
            label: "Word of Pain".to_string(),
        },
        FieldOption {
            value: 1300,
            label: "Word of Pain II".to_string(),
        },
        FieldOption {
            value: 1301,
            label: "Word of Pain III".to_string(),
        },
        FieldOption {
            value: 1302,
            label: "Word of Pain IV".to_string(),
        },
        FieldOption {
            value: 1303,
            label: "Meteor Strike".to_string(),
        },
        FieldOption {
            value: 1304,
            label: "Meteor Strike II".to_string(),
        },
        FieldOption {
            value: 1305,
            label: "Meteor Strike III".to_string(),
        },
        FieldOption {
            value: 1306,
            label: "Meteor Strike IV".to_string(),
        },
        FieldOption {
            value: 1307,
            label: "Hellbound".to_string(),
        },
        FieldOption {
            value: 1308,
            label: "Hellbound II".to_string(),
        },
        FieldOption {
            value: 1309,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 1310,
            label: "Abyss II".to_string(),
        },
        FieldOption {
            value: 1311,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 1312,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 1313,
            label: "Drain Power".to_string(),
        },
        FieldOption {
            value: 1314,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 1315,
            label: "Shadow Gaurd".to_string(),
        },
        FieldOption {
            value: 1316,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 1317,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 1318,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 1319,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 1320,
            label: "Paralytic Wave".to_string(),
        },
        FieldOption {
            value: 1321,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 1322,
            label: "Deadly Poison".to_string(),
        },
        FieldOption {
            value: 1323,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 1324,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 1325,
            label: "Dominate".to_string(),
        },
        FieldOption {
            value: 1326,
            label: "Shackle".to_string(),
        },
        FieldOption {
            value: 1327,
            label: "Fixate".to_string(),
        },
        FieldOption {
            value: 1328,
            label: "Gravity Flux".to_string(),
        },
        FieldOption {
            value: 1329,
            label: "Deadscream".to_string(),
        },
        FieldOption {
            value: 1330,
            label: "Dead Mans Ivy".to_string(),
        },
        FieldOption {
            value: 1331,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 1332,
            label: "Tempest II".to_string(),
        },
        FieldOption {
            value: 1333,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 1334,
            label: "Gaia Strike II".to_string(),
        },
        FieldOption {
            value: 1335,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 1336,
            label: "Vortex II".to_string(),
        },
        FieldOption {
            value: 1337,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 1338,
            label: "Deluge II".to_string(),
        },
        FieldOption {
            value: 1339,
            label: "Annihilation".to_string(),
        },
        FieldOption {
            value: 1340,
            label: "Annihilation II".to_string(),
        },
        FieldOption {
            value: 1341,
            label: "Iceover".to_string(),
        },
        FieldOption {
            value: 1342,
            label: "Iceover II".to_string(),
        },
        FieldOption {
            value: 1343,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 1344,
            label: "Starfall II".to_string(),
        },
        FieldOption {
            value: 1345,
            label: "Diablo's Spite".to_string(),
        },
        FieldOption {
            value: 1346,
            label: "Diablo's Spite II".to_string(),
        },
        FieldOption {
            value: 1347,
            label: "Palace Guide Book".to_string(),
        },
        FieldOption {
            value: 1348,
            label: "Detect".to_string(),
        },
        FieldOption {
            value: 1349,
            label: "Springboard".to_string(),
        },
        FieldOption {
            value: 1350,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 1351,
            label: "Palace Guide Book II".to_string(),
        },
        FieldOption {
            value: 1352,
            label: "Gift of Restoration".to_string(),
        },
        FieldOption {
            value: 1353,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 1354,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 1355,
            label: "Negate Spell".to_string(),
        },
        FieldOption {
            value: 1356,
            label: "Dodge Blades".to_string(),
        },
        FieldOption {
            value: 1357,
            label: "Ballistics".to_string(),
        },
        FieldOption {
            value: 1358,
            label: "Enlighten".to_string(),
        },
        FieldOption {
            value: 1359,
            label: "Phantom Shell".to_string(),
        },
        FieldOption {
            value: 1360,
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 1361,
            label: "Sacrifice".to_string(),
        },
        FieldOption {
            value: 1362,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1363,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 1364,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 1365,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 1366,
            label: "Curse II".to_string(),
        },
        FieldOption {
            value: 1367,
            label: "Curse III".to_string(),
        },
        FieldOption {
            value: 1368,
            label: "Tainted Love".to_string(),
        },
        FieldOption {
            value: 1369,
            label: "Prodigize".to_string(),
        },
        FieldOption {
            value: 1370,
            label: "Breed Suspicion".to_string(),
        },
        FieldOption {
            value: 1371,
            label: "Phantom Pain".to_string(),
        },
        FieldOption {
            value: 1372,
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 1373,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 1374,
            label: "Putrify II".to_string(),
        },
        FieldOption {
            value: 1375,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1376,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 1377,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 1378,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 1379,
            label: "Wind Dervish".to_string(),
        },
        FieldOption {
            value: 1380,
            label: "Wind Dervish II".to_string(),
        },
        FieldOption {
            value: 1381,
            label: "Sand Spider".to_string(),
        },
        FieldOption {
            value: 1382,
            label: "Sand Spider II".to_string(),
        },
        FieldOption {
            value: 1383,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 1384,
            label: "Chimaera II".to_string(),
        },
        FieldOption {
            value: 1385,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 1386,
            label: "Water Tiger II".to_string(),
        },
        FieldOption {
            value: 1387,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 1388,
            label: "Fire Snake II".to_string(),
        },
        FieldOption {
            value: 1389,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 1390,
            label: "Rime Raven II".to_string(),
        },
        FieldOption {
            value: 1391,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 1392,
            label: "Palace Guide Book III".to_string(),
        },
        FieldOption {
            value: 1393,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 1394,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 1395,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 1396,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 1397,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 1398,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 1399,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 1400,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 1401,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 1402,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 1403,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 1404,
            label: "Bedeviling Dance".to_string(),
        },
        FieldOption {
            value: 1405,
            label: "Invirogating Dance".to_string(),
        },
        FieldOption {
            value: 1406,
            label: "Demonpetal Dance".to_string(),
        },
        FieldOption {
            value: 1407,
            label: "Ardent Conga".to_string(),
        },
        FieldOption {
            value: 1408,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 1409,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 1410,
            label: "Stiring Folclore".to_string(),
        },
        FieldOption {
            value: 1411,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 1412,
            label: "Escalating Sanat".to_string(),
        },
        FieldOption {
            value: 1413,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 1414,
            label: "Glass Pumpkin".to_string(),
        },
        FieldOption {
            value: 1415,
            label: "Heaven's Fork".to_string(),
        },
        FieldOption {
            value: 1416,
            label: "Warrior's Mark".to_string(),
        },
        FieldOption {
            value: 1417,
            label: "Archer's Mark".to_string(),
        },
        FieldOption {
            value: 1418,
            label: "Mage's Mark".to_string(),
        },
        FieldOption {
            value: 1419,
            label: "Sibyl's Mark".to_string(),
        },
        FieldOption {
            value: 1420,
            label: "Mage-Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1421,
            label: "Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1422,
            label: "Dreadknight's Mark".to_string(),
        },
        FieldOption {
            value: 1423,
            label: "Berserker's Mark".to_string(),
        },
        FieldOption {
            value: 1424,
            label: "Swordman's Mark".to_string(),
        },
        FieldOption {
            value: 1425,
            label: "Dragoon's Mark".to_string(),
        },
        FieldOption {
            value: 1426,
            label: "Ninja's Mark".to_string(),
        },
        FieldOption {
            value: 1427,
            label: "Bandit's Mark".to_string(),
        },
        FieldOption {
            value: 1428,
            label: "Fusilier's Mark".to_string(),
        },
        FieldOption {
            value: 1429,
            label: "Beastmaster's Mark".to_string(),
        },
        FieldOption {
            value: 1430,
            label: "Magus's Mark".to_string(),
        },
        FieldOption {
            value: 1431,
            label: "Necroprentice's Mark".to_string(),
        },
        FieldOption {
            value: 1432,
            label: "Footsoldier's Mark".to_string(),
        },
        FieldOption {
            value: 1433,
            label: "Juggernaut's Mark".to_string(),
        },
        FieldOption {
            value: 1434,
            label: "Chief's Mark".to_string(),
        },
        FieldOption {
            value: 1435,
            label: "Familiar's Mark".to_string(),
        },
        FieldOption {
            value: 1436,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1437,
            label: "Windwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1438,
            label: "Cragwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1439,
            label: "Stormwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1440,
            label: "Waterwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1441,
            label: "Firewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1442,
            label: "Icewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1443,
            label: "Gleamwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1444,
            label: "Gloomwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1445,
            label: "Sandstone's Mark".to_string(),
        },
        FieldOption {
            value: 1446,
            label: "Granite's Mark".to_string(),
        },
        FieldOption {
            value: 1447,
            label: "Black Iron's Mark".to_string(),
        },
        FieldOption {
            value: 1448,
            label: "Magesteel's Mark".to_string(),
        },
        FieldOption {
            value: 1449,
            label: "Sovereign's Mark".to_string(),
        },
        FieldOption {
            value: 1450,
            label: "Brave's Mark".to_string(),
        },
        FieldOption {
            value: 1451,
            label: "Abuna's Mark".to_string(),
        },
        FieldOption {
            value: 1452,
            label: "Heretic's Mark".to_string(),
        },
        FieldOption {
            value: 1453,
            label: "Princess's Mark".to_string(),
        },
        FieldOption {
            value: 1454,
            label: "Holy Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1455,
            label: "Star Seer's Mark".to_string(),
        },
        FieldOption {
            value: 1456,
            label: "Peregrine's Mark".to_string(),
        },
        FieldOption {
            value: 1457,
            label: "White Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1458,
            label: "Oracle's Mark".to_string(),
        },
        FieldOption {
            value: 1459,
            label: "Wicce's Mark".to_string(),
        },
        FieldOption {
            value: 1460,
            label: "Songstress's Mark".to_string(),
        },
        FieldOption {
            value: 1461,
            label: "Hagiaknight's Mark".to_string(),
        },
        FieldOption {
            value: 1462,
            label: "Pirate's Mark".to_string(),
        },
        FieldOption {
            value: 1463,
            label: "Inferior Ore".to_string(),
        },
        FieldOption {
            value: 1464,
            label: "Iron Sand".to_string(),
        },
        FieldOption {
            value: 1465,
            label: "Copper Ore".to_string(),
        },
        FieldOption {
            value: 1466,
            label: "Tin Ore".to_string(),
        },
        FieldOption {
            value: 1467,
            label: "Graphite".to_string(),
        },
        FieldOption {
            value: 1468,
            label: "Iron Ore".to_string(),
        },
        FieldOption {
            value: 1469,
            label: "Silver Ore".to_string(),
        },
        FieldOption {
            value: 1470,
            label: "Baldur Ore".to_string(),
        },
        FieldOption {
            value: 1471,
            label: "Gold Ore".to_string(),
        },
        FieldOption {
            value: 1472,
            label: "Platinum Ore".to_string(),
        },
        FieldOption {
            value: 1473,
            label: "Saltpeter".to_string(),
        },
        FieldOption {
            value: 1474,
            label: "Sulfur".to_string(),
        },
        FieldOption {
            value: 1475,
            label: "Limestone".to_string(),
        },
        FieldOption {
            value: 1476,
            label: "Skyiron".to_string(),
        },
        FieldOption {
            value: 1477,
            label: "Gemstones".to_string(),
        },
        FieldOption {
            value: 1478,
            label: "Krystallos Ore".to_string(),
        },
        FieldOption {
            value: 1479,
            label: "Bronze Ingot".to_string(),
        },
        FieldOption {
            value: 1480,
            label: "Iron Ingot".to_string(),
        },
        FieldOption {
            value: 1481,
            label: "Silver Ingot".to_string(),
        },
        FieldOption {
            value: 1482,
            label: "Baldur Ingot".to_string(),
        },
        FieldOption {
            value: 1483,
            label: "Steel Ingot".to_string(),
        },
        FieldOption {
            value: 1484,
            label: "Hagane Steel".to_string(),
        },
        FieldOption {
            value: 1485,
            label: "Wootz Steel".to_string(),
        },
        FieldOption {
            value: 1486,
            label: "Golden Ingot".to_string(),
        },
        FieldOption {
            value: 1487,
            label: "Platinum Ingot".to_string(),
        },
        FieldOption {
            value: 1488,
            label: "Fiery Gems".to_string(),
        },
        FieldOption {
            value: 1489,
            label: "Verdant Gems".to_string(),
        },
        FieldOption {
            value: 1490,
            label: "Regal Gems".to_string(),
        },
        FieldOption {
            value: 1491,
            label: "White Gems".to_string(),
        },
        FieldOption {
            value: 1492,
            label: "Black Gems".to_string(),
        },
        FieldOption {
            value: 1493,
            label: "Air Krystallos".to_string(),
        },
        FieldOption {
            value: 1494,
            label: "Earth Krystallos".to_string(),
        },
        FieldOption {
            value: 1495,
            label: "Lightning Krystallos".to_string(),
        },
        FieldOption {
            value: 1496,
            label: "Water Krystallos".to_string(),
        },
        FieldOption {
            value: 1497,
            label: "Fire Krystallos".to_string(),
        },
        FieldOption {
            value: 1498,
            label: "Ice Krystallos".to_string(),
        },
        FieldOption {
            value: 1499,
            label: "Light Krystallos".to_string(),
        },
        FieldOption {
            value: 1500,
            label: "Dark Krystallos".to_string(),
        },
        FieldOption {
            value: 1501,
            label: "Toneriwood".to_string(),
        },
        FieldOption {
            value: 1502,
            label: "Birnewood".to_string(),
        },
        FieldOption {
            value: 1503,
            label: "Ananawood".to_string(),
        },
        FieldOption {
            value: 1504,
            label: "Baobawood".to_string(),
        },
        FieldOption {
            value: 1505,
            label: "Beasthide".to_string(),
        },
        FieldOption {
            value: 1506,
            label: "Tannin".to_string(),
        },
        FieldOption {
            value: 1507,
            label: "Leather".to_string(),
        },
        FieldOption {
            value: 1508,
            label: "Parchment".to_string(),
        },
        FieldOption {
            value: 1509,
            label: "Ink".to_string(),
        },
        FieldOption {
            value: 1510,
            label: "Gold Leaf".to_string(),
        },
        FieldOption {
            value: 1511,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 1512,
            label: "Log".to_string(),
        },
        FieldOption {
            value: 1513,
            label: "Bundle of Herbs".to_string(),
        },
        FieldOption {
            value: 1514,
            label: "Herbal Extract".to_string(),
        },
        FieldOption {
            value: 1515,
            label: "Nightshade".to_string(),
        },
        FieldOption {
            value: 1516,
            label: "Nightshade Extract".to_string(),
        },
        FieldOption {
            value: 1517,
            label: "Fruit".to_string(),
        },
        FieldOption {
            value: 1518,
            label: "Spirits".to_string(),
        },
        FieldOption {
            value: 1519,
            label: "Hempen Thread".to_string(),
        },
        FieldOption {
            value: 1520,
            label: "Woolen Thread".to_string(),
        },
        FieldOption {
            value: 1521,
            label: "Cotton Thread".to_string(),
        },
        FieldOption {
            value: 1522,
            label: "Silken Thread".to_string(),
        },
        FieldOption {
            value: 1523,
            label: "Silver Thread".to_string(),
        },
        FieldOption {
            value: 1524,
            label: "Golden Thread".to_string(),
        },
        FieldOption {
            value: 1525,
            label: "Linen".to_string(),
        },
        FieldOption {
            value: 1526,
            label: "Pincord".to_string(),
        },
        FieldOption {
            value: 1527,
            label: "Flannel".to_string(),
        },
        FieldOption {
            value: 1528,
            label: "Velvet".to_string(),
        },
        FieldOption {
            value: 1529,
            label: "Satin".to_string(),
        },
        FieldOption {
            value: 1530,
            label: "Blackpowder".to_string(),
        },
        FieldOption {
            value: 1531,
            label: "Beast Horn".to_string(),
        },
        FieldOption {
            value: 1532,
            label: "Beast Fang".to_string(),
        },
        FieldOption {
            value: 1533,
            label: "Beast Claw".to_string(),
        },
        FieldOption {
            value: 1534,
            label: "Wyrm Fang".to_string(),
        },
        FieldOption {
            value: 1535,
            label: "Wyrm Claw".to_string(),
        },
        FieldOption {
            value: 1536,
            label: "Wyrm Scale".to_string(),
        },
        FieldOption {
            value: 1537,
            label: "Wyrm Horn".to_string(),
        },
        FieldOption {
            value: 1538,
            label: "Wyrm Whisker".to_string(),
        },
        FieldOption {
            value: 1539,
            label: "Wyrm Thighbone".to_string(),
        },
        FieldOption {
            value: 1540,
            label: "Tooth & Claw".to_string(),
        },
        FieldOption {
            value: 1541,
            label: "Unicorn Horn".to_string(),
        },
        FieldOption {
            value: 1542,
            label: "Enchanted Feather".to_string(),
        },
        FieldOption {
            value: 1543,
            label: "Ancient Wood".to_string(),
        },
        FieldOption {
            value: 1544,
            label: "Ancient Bone".to_string(),
        },
        FieldOption {
            value: 1545,
            label: "Orichalcum".to_string(),
        },
        FieldOption {
            value: 1546,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1547,
            label: "Daedalus Pinion".to_string(),
        },
        FieldOption {
            value: 1548,
            label: "Daedalus Rack".to_string(),
        },
        FieldOption {
            value: 1549,
            label: "Melee Weapons I".to_string(),
        },
        FieldOption {
            value: 1550,
            label: "Melee Weapons II".to_string(),
        },
        FieldOption {
            value: 1551,
            label: "The Fist".to_string(),
        },
        FieldOption {
            value: 1552,
            label: "Fist Enchiridion".to_string(),
        },
        FieldOption {
            value: 1553,
            label: "The Blade".to_string(),
        },
        FieldOption {
            value: 1554,
            label: "Dagger Enchiridion".to_string(),
        },
        FieldOption {
            value: 1555,
            label: "Sword Enchidirion".to_string(),
        },
        FieldOption {
            value: 1556,
            label: "2-H Sword Enchiridion".to_string(),
        },
        FieldOption {
            value: 1557,
            label: "Axe Spear & Hammer".to_string(),
        },
        FieldOption {
            value: 1558,
            label: "Axe Enchiridion".to_string(),
        },
        FieldOption {
            value: 1559,
            label: "Spear Enchiridion".to_string(),
        },
        FieldOption {
            value: 1560,
            label: "Hammer Enchiridion".to_string(),
        },
        FieldOption {
            value: 1561,
            label: "The Katana".to_string(),
        },
        FieldOption {
            value: 1562,
            label: "Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1563,
            label: "2-H Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1564,
            label: "Cudgel & Whip".to_string(),
        },
        FieldOption {
            value: 1565,
            label: "Cudgel Enchiridion".to_string(),
        },
        FieldOption {
            value: 1566,
            label: "Whip Enchiridion".to_string(),
        },
        FieldOption {
            value: 1567,
            label: "Transcription".to_string(),
        },
        FieldOption {
            value: 1568,
            label: "Musical Instruments I".to_string(),
        },
        FieldOption {
            value: 1569,
            label: "Musical Instruments II".to_string(),
        },
        FieldOption {
            value: 1570,
            label: "Ranged Weapons I".to_string(),
        },
        FieldOption {
            value: 1571,
            label: "Ranged Weapons II".to_string(),
        },
        FieldOption {
            value: 1572,
            label: "Ways of the Gerges".to_string(),
        },
        FieldOption {
            value: 1573,
            label: "The Bow".to_string(),
        },
        FieldOption {
            value: 1574,
            label: "Bow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1575,
            label: "The Crossbow".to_string(),
        },
        FieldOption {
            value: 1576,
            label: "Crossbow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1577,
            label: "The Fusil".to_string(),
        },
        FieldOption {
            value: 1578,
            label: "Fusil Enchiridion".to_string(),
        },
        FieldOption {
            value: 1579,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1580,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1581,
            label: "Smithing Armor I".to_string(),
        },
        FieldOption {
            value: 1582,
            label: "Smithing Armor II".to_string(),
        },
        FieldOption {
            value: 1583,
            label: "Armorcraft".to_string(),
        },
        FieldOption {
            value: 1584,
            label: "Shieldcraft".to_string(),
        },
        FieldOption {
            value: 1585,
            label: "Shield Enchiridion".to_string(),
        },
        FieldOption {
            value: 1586,
            label: "Helm Enchiridion".to_string(),
        },
        FieldOption {
            value: 1587,
            label: "Body Armor Enchiridion".to_string(),
        },
        FieldOption {
            value: 1588,
            label: "Armguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1589,
            label: "Legguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1590,
            label: "Codex of Jewelry I".to_string(),
        },
        FieldOption {
            value: 1591,
            label: "Codex of Jewelry II".to_string(),
        },
        FieldOption {
            value: 1592,
            label: "Codex of Jewelry III".to_string(),
        },
        FieldOption {
            value: 1593,
            label: "Codex of Jewelry IV".to_string(),
        },
        FieldOption {
            value: 1594,
            label: "Codex of Ores".to_string(),
        },
        FieldOption {
            value: 1595,
            label: "Codex of Gems".to_string(),
        },
        FieldOption {
            value: 1596,
            label: "Codex of Timber".to_string(),
        },
        FieldOption {
            value: 1597,
            label: "Codex of Textiles".to_string(),
        },
        FieldOption {
            value: 1598,
            label: "On Medicine I".to_string(),
        },
        FieldOption {
            value: 1599,
            label: "On Medicine II".to_string(),
        },
        FieldOption {
            value: 1600,
            label: "Secrets of the Master".to_string(),
        },
        FieldOption {
            value: 1601,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1602,
            label: "Ease II".to_string(),
        },
        FieldOption {
            value: 1603,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1604,
            label: "STR Charm".to_string(),
        },
        FieldOption {
            value: 1605,
            label: "VIT Charm".to_string(),
        },
        FieldOption {
            value: 1606,
            label: "DEX Charm".to_string(),
        },
        FieldOption {
            value: 1607,
            label: "AGI Charm".to_string(),
        },
        FieldOption {
            value: 1608,
            label: "AVD Charm".to_string(),
        },
        FieldOption {
            value: 1609,
            label: "INT Charm".to_string(),
        },
        FieldOption {
            value: 1610,
            label: "MND Charm".to_string(),
        },
        FieldOption {
            value: 1611,
            label: "RES Charm".to_string(),
        },
        FieldOption {
            value: 1612,
            label: "LUK Charm".to_string(),
        },
        FieldOption {
            value: 1613,
            label: "Air Charm".to_string(),
        },
        FieldOption {
            value: 1614,
            label: "Earth Charm".to_string(),
        },
        FieldOption {
            value: 1615,
            label: "Lightening Charm".to_string(),
        },
        FieldOption {
            value: 1616,
            label: "Water Charm".to_string(),
        },
        FieldOption {
            value: 1617,
            label: "Fire Charm".to_string(),
        },
        FieldOption {
            value: 1618,
            label: "Ice Charm".to_string(),
        },
        FieldOption {
            value: 1619,
            label: "Light Charm".to_string(),
        },
        FieldOption {
            value: 1620,
            label: "Dark Charm".to_string(),
        },
        FieldOption {
            value: 1621,
            label: "Experience Charm".to_string(),
        },
        FieldOption {
            value: 1622,
            label: "Experience Charm II".to_string(),
        },
        FieldOption {
            value: 1623,
            label: "Experience Charm III".to_string(),
        },
        FieldOption {
            value: 1624,
            label: "Experience Charm IV".to_string(),
        },
        FieldOption {
            value: 1625,
            label: "Experience Charm V".to_string(),
        },
        FieldOption {
            value: 1626,
            label: "Level Up Charm".to_string(),
        },
        FieldOption {
            value: 1627,
            label: "Grimoire Exorcisme".to_string(),
        },
        FieldOption {
            value: 1628,
            label: "Max HP Charm".to_string(),
        },
        FieldOption {
            value: 1629,
            label: "Max MP Charm".to_string(),
        },
        FieldOption {
            value: 3364,
            label: "Unknown".to_string(),
        },
    ]
}

fn opt_character_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Denam Morne".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Vyce Bozeck".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Catiua Pavel".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Lanselot Hamilton".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Warren Omon".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Canopus Wolph".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Myrdin Walhorn".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Gildas W. Byrne".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Cerya Phoraena".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Sherri Phoraena".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Cistina Phoraena".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Olyvia Phoraena".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Deneb Rove".to_string(),
        },
        FieldOption {
            value: 14,
            label: "NPC Gilbald O'Brien".to_string(),
        },
        FieldOption {
            value: 15,
            label: "NPC Andoras Gaffryn".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Donnalto Presance".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Folcurt Reeda Lynde".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Bayin Rosen Orne".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Arycelle Dania".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Hobyrim V. Rahms".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Jeunan Avertif".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Ocionne Rabine".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Xapan Illudas".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Dievold Obdilord".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Oelias Obdilord".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Robert Rudlum".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Ganpp Vochstein".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Iuria Wolph".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Cressida Obdilord".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Ozma Moh Glacius".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Jonathon Torgeaux Lindl".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Leonar Reci Rimon".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Ravness Loxaerion".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Diego Galet Azelstan".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Agrikora".to_string(),
        },
        FieldOption {
            value: 36,
            label: "NPC Generic Male".to_string(),
        },
        FieldOption {
            value: 37,
            label: "NPC Generic Male".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Merrik Ehlrig".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Berda the Younger".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Obda the Younger".to_string(),
        },
        FieldOption {
            value: 41,
            label: "NPC Vyce Bozeck".to_string(),
        },
        FieldOption {
            value: 42,
            label: "NPC Catiua(Dark Priest)".to_string(),
        },
        FieldOption {
            value: 43,
            label: "NPC Gildas Zombie".to_string(),
        },
        FieldOption {
            value: 44,
            label: "NPC Sherri Phoraena".to_string(),
        },
        FieldOption {
            value: 45,
            label: "NPC Leonar Reci Rimon".to_string(),
        },
        FieldOption {
            value: 46,
            label: "NPC Leonar Reci Rimon Zombie".to_string(),
        },
        FieldOption {
            value: 47,
            label: "NPC Arycelle Dania".to_string(),
        },
        FieldOption {
            value: 48,
            label: "NPC Ocionne Rabine".to_string(),
        },
        FieldOption {
            value: 49,
            label: "NPC Xapan Illudas".to_string(),
        },
        FieldOption {
            value: 50,
            label: "NPC Jeunan Avertif".to_string(),
        },
        FieldOption {
            value: 51,
            label: "NPC Lanselot Tartaros".to_string(),
        },
        FieldOption {
            value: 52,
            label: "NPC Balxephon V. Rahms".to_string(),
        },
        FieldOption {
            value: 53,
            label: "NPC Volaq Windsalf".to_string(),
        },
        FieldOption {
            value: 54,
            label: "NPC Barbas Dahd Geuse".to_string(),
        },
        FieldOption {
            value: 55,
            label: "NPC Martym Noumous".to_string(),
        },
        FieldOption {
            value: 56,
            label: "NPC Oz Moh Glavius".to_string(),
        },
        FieldOption {
            value: 57,
            label: "NPC Ozma Moh Glaviusma".to_string(),
        },
        FieldOption {
            value: 58,
            label: "NPC Andoras Gaffryn".to_string(),
        },
        FieldOption {
            value: 59,
            label: "NPC Dorgalua Oberyth Valeria (Dark Lord I)".to_string(),
        },
        FieldOption {
            value: 60,
            label: "NPC Brantyn Morne".to_string(),
        },
        FieldOption {
            value: 61,
            label: "NPC Nybeth Obdilord".to_string(),
        },
        FieldOption {
            value: 62,
            label: "NPC Nybeth Obdilord Lich".to_string(),
        },
        FieldOption {
            value: 63,
            label: "NPC Xaebos Ronsenbach".to_string(),
        },
        FieldOption {
            value: 64,
            label: "NPC Xaebos Ronsenbach Zombie".to_string(),
        },
        FieldOption {
            value: 65,
            label: "NPC Ganpp Vochstein".to_string(),
        },
        FieldOption {
            value: 66,
            label: "NPC Rodrick Desmoria".to_string(),
        },
        FieldOption {
            value: 67,
            label: "NPC Cressida Obdilord".to_string(),
        },
        FieldOption {
            value: 68,
            label: "NPC Ravness Loxaerion".to_string(),
        },
        FieldOption {
            value: 69,
            label: "NPC Bayin Rosen Orne".to_string(),
        },
        FieldOption {
            value: 70,
            label: "NPC Vyce Bozeck".to_string(),
        },
        FieldOption {
            value: 71,
            label: "NPC Marino Arvine (Terror Knight)".to_string(),
        },
        FieldOption {
            value: 72,
            label: "NPC Agares Bazin (Knight)".to_string(),
        },
        FieldOption {
            value: 73,
            label: "NPC Johann Anaberg (Lich)".to_string(),
        },
        FieldOption {
            value: 74,
            label: "NPC Wynoa Canaletto (Cleric)".to_string(),
        },
        FieldOption {
            value: 75,
            label: "NPC Vepahl Dabran (Sorceress)".to_string(),
        },
        FieldOption {
            value: 76,
            label: "NPC Veldrei (Valkyrie)".to_string(),
        },
        FieldOption {
            value: 77,
            label: "NPC Vermado Johannsen (Knight)".to_string(),
        },
        FieldOption {
            value: 78,
            label: "NPC Uram Fenelon (Wizard)".to_string(),
        },
        FieldOption {
            value: 79,
            label: "NPC Aeshan (Swordmaster)".to_string(),
        },
        FieldOption {
            value: 80,
            label: "NPC Merrik Ehlrig".to_string(),
        },
        FieldOption {
            value: 81,
            label: "NPC Obda (Gryphon)".to_string(),
        },
        FieldOption {
            value: 82,
            label: "NPC Orgeau Manheim (Hawkman)".to_string(),
        },
        FieldOption {
            value: 83,
            label: "NPC Orba Brodel (Wizard)".to_string(),
        },
        FieldOption {
            value: 84,
            label: "NPC Bruno Kakrinoros (Wizard)".to_string(),
        },
        FieldOption {
            value: 85,
            label: "NPC Gousin Blum (Ninja)".to_string(),
        },
        FieldOption {
            value: 86,
            label: "NPC Farrel Ganache (Knight)".to_string(),
        },
        FieldOption {
            value: 87,
            label: "NPC Berruk Gannon (Berserker)".to_string(),
        },
        FieldOption {
            value: 88,
            label: "NPC Kamlott Roberval (Ninja)".to_string(),
        },
        FieldOption {
            value: 89,
            label: "NPC Garba Brondel (Mage)".to_string(),
        },
        FieldOption {
            value: 90,
            label: "NPC Dukas Windelband Gatialo (Terror Knight)".to_string(),
        },
        FieldOption {
            value: 91,
            label: "NPC Latimer Grandier (Knight)".to_string(),
        },
        FieldOption {
            value: 92,
            label: "NPC Genzo Ageja (Ninja)".to_string(),
        },
        FieldOption {
            value: 93,
            label: "NPC Xadoba (Lamia)".to_string(),
        },
        FieldOption {
            value: 94,
            label: "NPC Gildora Bastian (Witch)".to_string(),
        },
        FieldOption {
            value: 95,
            label: "NPC Swift Stanoska (Beast Tamer)".to_string(),
        },
        FieldOption {
            value: 96,
            label: "NPC Dagon Simmel (Wizard)".to_string(),
        },
        FieldOption {
            value: 97,
            label: "NPC Darza (Berserker)".to_string(),
        },
        FieldOption {
            value: 98,
            label: "NPC Hektor Didarro (Knight)".to_string(),
        },
        FieldOption {
            value: 99,
            label: "NPC Nadia Eginhard (Cleric)".to_string(),
        },
        FieldOption {
            value: 100,
            label: "NPC Hiram (Terror Knight)".to_string(),
        },
        FieldOption {
            value: 101,
            label: "NPC Bravan (Berserker)".to_string(),
        },
        FieldOption {
            value: 102,
            label: "NPC Juglar Pajeot (Dragoon)".to_string(),
        },
        FieldOption {
            value: 103,
            label: "NPC Pavan Bapal (Berserker)".to_string(),
        },
        FieldOption {
            value: 104,
            label: "NPC Halphas Hohenstaufen (Beast Tamer)".to_string(),
        },
        FieldOption {
            value: 105,
            label: "NPC Hanzo Rugen (Swordmaster)".to_string(),
        },
        FieldOption {
            value: 106,
            label: "NPC Crossa Bingham (Hawkman)".to_string(),
        },
        FieldOption {
            value: 107,
            label: "NPC Falfaday Geb Lesmoaria (Sorceress)".to_string(),
        },
        FieldOption {
            value: 108,
            label: "NPC Dilthey Pherenian (Fusiller)".to_string(),
        },
        FieldOption {
            value: 109,
            label: "NPC Felnatorre Savonarola (Knight)".to_string(),
        },
        FieldOption {
            value: 110,
            label: "NPC Blackmoor (Lich)".to_string(),
        },
        FieldOption {
            value: 111,
            label: "NPC Brutakos Kapote (Sorcerer)".to_string(),
        },
        FieldOption {
            value: 112,
            label: "NPC Tomasius Brezen (Berserker)".to_string(),
        },
        FieldOption {
            value: 113,
            label: "NPC Brondel Vestiarri (Enchantress)".to_string(),
        },
        FieldOption {
            value: 114,
            label: "NPC Beelzebuth".to_string(),
        },
        FieldOption {
            value: 115,
            label: "NPC Berda (Gryphon)".to_string(),
        },
        FieldOption {
            value: 116,
            label: "NPC Botis (Wizard)".to_string(),
        },
        FieldOption {
            value: 117,
            label: "NPC Schleiden Mercure (Swordmaster)".to_string(),
        },
        FieldOption {
            value: 118,
            label: "NPC Anelio Muntzer (Necromancer)".to_string(),
        },
        FieldOption {
            value: 119,
            label: "NPC Vailland Modiliani (Sorcerer)".to_string(),
        },
        FieldOption {
            value: 120,
            label: "NPC Moldova Obdilord (Necromancer)".to_string(),
        },
        FieldOption {
            value: 121,
            label: "NPC Derain Lowart (Terror Knight)".to_string(),
        },
        FieldOption {
            value: 122,
            label: "NPC Ramidos Mendoza (Sorceress)".to_string(),
        },
        FieldOption {
            value: 123,
            label: "NPC Bolis Rhumoth (Knight)".to_string(),
        },
        FieldOption {
            value: 124,
            label: "NPC Romulus LeRozza (Warlock)".to_string(),
        },
        FieldOption {
            value: 125,
            label: "NPC Grion Lexentale (Knight)".to_string(),
        },
        FieldOption {
            value: 126,
            label: "NPC Lobos (Rogue)".to_string(),
        },
        FieldOption {
            value: 127,
            label: "NPC Christos Vance (Rune Fencer)".to_string(),
        },
        FieldOption {
            value: 128,
            label: "NPC Banga (Cockatrice)".to_string(),
        },
        FieldOption {
            value: 129,
            label: "NPC Zanga (Cockatrice)".to_string(),
        },
        FieldOption {
            value: 130,
            label: "NPC Daesi Apollinaire (Knight)".to_string(),
        },
        FieldOption {
            value: 131,
            label: "NPC Cassandra Obdilord (Necromancer)".to_string(),
        },
        FieldOption {
            value: 132,
            label: "NPC Alessandro Zuloaga (Terror Knight)".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Sara Ostvald".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Voltare Montrose".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Felicia Malxion".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Chamos Zalman".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Phaesta Morandi".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Tamuz Fedorenko".to_string(),
        },
        FieldOption {
            value: 139,
            label: "NPC Dorgalua Oberyth Valeria (Dark Lord II)".to_string(),
        },
        FieldOption {
            value: 140,
            label: "NPC Mimose Prongniart (Valkyrie)".to_string(),
        },
        FieldOption {
            value: 141,
            label: "NPC Cielo Segur (Fusilier)".to_string(),
        },
        FieldOption {
            value: 142,
            label: "NPC Georges Sekendorff (Necromancer)".to_string(),
        },
        FieldOption {
            value: 143,
            label: "NPC Jilessa Krapelin (Cleric)".to_string(),
        },
        FieldOption {
            value: 144,
            label: "NPC Reymos Cavour (Necromancer)".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Unknown Gryphon".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Unknown Gryphon".to_string(),
        },
        FieldOption {
            value: 147,
            label: "NPC Ragnar (Berserker)".to_string(),
        },
        FieldOption {
            value: 148,
            label: "NPC Josephine Ysarc (Cleric)".to_string(),
        },
        FieldOption {
            value: 149,
            label: "NPC Leon Wilfred (Fusiler)".to_string(),
        },
        FieldOption {
            value: 150,
            label: "NPC Alfred Boulvart (Knight)".to_string(),
        },
        FieldOption {
            value: 151,
            label: "NPC Punkin".to_string(),
        },
        FieldOption {
            value: 152,
            label: "NPC Rackham".to_string(),
        },
        FieldOption {
            value: 153,
            label: "None".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Ch3 Neutral Male Warrior".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Ch3 Neutral Female Archer".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Ch3 Neutral Female Knight".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Ch3 Neutral Female Warrior".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Ch3 Neutral Male Archer".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Generic M(Recruit)".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Generic M(Recruit) II".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Generic M(Recruit) III".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Generic M(Recruit) IV".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Generic F(Recruit)".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Generic F(Recruit) II".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Generic F(Recruit) III".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Generic F(Recruit) IV".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Hawkman I".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Hawkman II".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Hawkman III".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Hawkman IV".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Lizardmen I".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Lizardmen II".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Lizardman III".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Lizardmen IV".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Lamia I".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Lamia II".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Lamia III".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Lamia IV".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Orc I".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Orc II".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Orc III".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Orc IV".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Skeleton M I".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Skeleton M II".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Skeleton M III".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Skeleton M IV".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Skeleton F I".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Skeleton F II".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Skeleton F III".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Skeleton F IV".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Phantom M I".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Phantom M II".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Phantom M III".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Phantom M IV".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Phantom F I".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Phantom F II".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Phantom F III".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Phantom F IV".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Faerie I".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Faerie II".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Faerie III".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Faerie IV".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Gremlin I".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Gremlin II".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Gremlin III".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Gremlin IV".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Pumpkin I".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Pumpkin II".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Pumpkin III".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Pumpkin IV".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Dragon I".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Dragon II".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Dragon III".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Dragon IV".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Hydra I".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Hydra II".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Hydra III".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Hydra IV".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Gryphon I".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Gryphon II".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Gryphon III".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Gryphon IV".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Cockatrice I".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Cockatrice II".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Cockatrice III".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Cockatrice IV".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Octopus I".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Octopus II".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Octopus III".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Octopus IV".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Cyclops I".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Cyclops II".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Cyclops III".to_string(),
        },
        FieldOption {
            value: 234,
            label: "Cyclops IV".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Golem I".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Golem II".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Golem III".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Golem IV".to_string(),
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
            label: "NPC Young Man (Vasque Survivor)".to_string(),
        },
        FieldOption {
            value: 242,
            label: "NPC Young Woman (Vasque Survivor)".to_string(),
        },
        FieldOption {
            value: 243,
            label: "NPC Middle Aged Man (Vasque Survivor)".to_string(),
        },
        FieldOption {
            value: 244,
            label: "NPC Middle Aged Woman(Vasque Survivor)".to_string(),
        },
        FieldOption {
            value: 245,
            label: "NPC Bald Old Man (Vasque Survivor)".to_string(),
        },
        FieldOption {
            value: 246,
            label: "NPC Old Woman (Vasque Survivor)".to_string(),
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
            label: "NPC Azelstan".to_string(),
        },
        FieldOption {
            value: 250,
            label: "NPC Dievold Obdilord".to_string(),
        },
        FieldOption {
            value: 251,
            label: "NPC Cassandra Obdilord Zombie".to_string(),
        },
        FieldOption {
            value: 252,
            label: "NPC Moldova Obdilord Zombie".to_string(),
        },
        FieldOption {
            value: 253,
            label: "NPC Hektor Diddaro Zombie".to_string(),
        },
        FieldOption {
            value: 254,
            label: "NPC Vyce Bozeck Zombie".to_string(),
        },
        FieldOption {
            value: 255,
            label: "None".to_string(),
        },
        FieldOption {
            value: 256,
            label: "PotD Gladiator Male".to_string(),
        },
        FieldOption {
            value: 257,
            label: "PotD Gladiator Male".to_string(),
        },
        FieldOption {
            value: 258,
            label: "PotD Gladiator Female".to_string(),
        },
        FieldOption {
            value: 259,
            label: "PotD Gladiator Female".to_string(),
        },
        FieldOption {
            value: 260,
            label: "PotD Blood Hunter Male".to_string(),
        },
        FieldOption {
            value: 261,
            label: "PotD Blood Hunter Male".to_string(),
        },
        FieldOption {
            value: 262,
            label: "PotD Blood Hunter Female".to_string(),
        },
        FieldOption {
            value: 263,
            label: "PotD Blood Hunter Female".to_string(),
        },
        FieldOption {
            value: 264,
            label: "PotD Death Eater Male".to_string(),
        },
        FieldOption {
            value: 265,
            label: "PotD Death Eater Male".to_string(),
        },
        FieldOption {
            value: 266,
            label: "PotD Death Eater Female".to_string(),
        },
        FieldOption {
            value: 267,
            label: "PotD Death Eater Female".to_string(),
        },
        FieldOption {
            value: 268,
            label: "PotD Cenobite Male".to_string(),
        },
        FieldOption {
            value: 269,
            label: "PotD Cenobite Male".to_string(),
        },
        FieldOption {
            value: 270,
            label: "PotD Cenobite Female".to_string(),
        },
        FieldOption {
            value: 271,
            label: "PotD Cenobite Female".to_string(),
        },
        FieldOption {
            value: 272,
            label: "PotD Cannibal Male".to_string(),
        },
        FieldOption {
            value: 273,
            label: "PotD Cannibal Male".to_string(),
        },
        FieldOption {
            value: 274,
            label: "PotD Cannibal Female".to_string(),
        },
        FieldOption {
            value: 275,
            label: "PotD Cannibal Female".to_string(),
        },
        FieldOption {
            value: 276,
            label: "PotD Shadow Knight Male".to_string(),
        },
        FieldOption {
            value: 277,
            label: "PotD Shadow Knight Male".to_string(),
        },
        FieldOption {
            value: 278,
            label: "PotD Shadow Knight Female".to_string(),
        },
        FieldOption {
            value: 279,
            label: "PotD Shadow Knight Female".to_string(),
        },
        FieldOption {
            value: 280,
            label: "PotD Dreadnought Male".to_string(),
        },
        FieldOption {
            value: 281,
            label: "PotD Dreadnought Male".to_string(),
        },
        FieldOption {
            value: 282,
            label: "PotD Dreadnought Female".to_string(),
        },
        FieldOption {
            value: 283,
            label: "PotD Dreadnought Female".to_string(),
        },
        FieldOption {
            value: 284,
            label: "PotD Executioner Male".to_string(),
        },
        FieldOption {
            value: 285,
            label: "PotD Executioner Male".to_string(),
        },
        FieldOption {
            value: 286,
            label: "PotD Executioner Female".to_string(),
        },
        FieldOption {
            value: 287,
            label: "PotD Executioner Female".to_string(),
        },
        FieldOption {
            value: 288,
            label: "PotD Kill Seeker Male".to_string(),
        },
        FieldOption {
            value: 289,
            label: "PotD Kill Seeker Male".to_string(),
        },
        FieldOption {
            value: 290,
            label: "PotD Kill Seeker Female".to_string(),
        },
        FieldOption {
            value: 291,
            label: "PotD Kill Seeker Female".to_string(),
        },
        FieldOption {
            value: 292,
            label: "PotD Crimson Uhlan Male".to_string(),
        },
        FieldOption {
            value: 293,
            label: "PotD Crimson Uhlan Male".to_string(),
        },
        FieldOption {
            value: 294,
            label: "PotD Crimson Uhlan Female".to_string(),
        },
        FieldOption {
            value: 295,
            label: "PotD Crimson Uhlan Female".to_string(),
        },
        FieldOption {
            value: 296,
            label: "PotD Dark Stalker Male".to_string(),
        },
        FieldOption {
            value: 297,
            label: "PotD Dark Stalker Male".to_string(),
        },
        FieldOption {
            value: 298,
            label: "PotD Dark Stalker Female".to_string(),
        },
        FieldOption {
            value: 299,
            label: "PotD Dark Stalker Female".to_string(),
        },
        FieldOption {
            value: 300,
            label: "PotD Grim Reaper Male".to_string(),
        },
        FieldOption {
            value: 301,
            label: "PotD Grim Reaper Male".to_string(),
        },
        FieldOption {
            value: 302,
            label: "PotD Grim Reaper Female".to_string(),
        },
        FieldOption {
            value: 303,
            label: "PotD Grim Reaper Female".to_string(),
        },
        FieldOption {
            value: 304,
            label: "PotD Sniper Male".to_string(),
        },
        FieldOption {
            value: 305,
            label: "PotD Sniper Male".to_string(),
        },
        FieldOption {
            value: 306,
            label: "PotD Sniper Female".to_string(),
        },
        FieldOption {
            value: 307,
            label: "PotD Sniper Female".to_string(),
        },
        FieldOption {
            value: 308,
            label: "PotD Iron Fist Male".to_string(),
        },
        FieldOption {
            value: 309,
            label: "PotD Iron Fist Male".to_string(),
        },
        FieldOption {
            value: 310,
            label: "PotD Iron Fist Female".to_string(),
        },
        FieldOption {
            value: 311,
            label: "PotD Iron Fist Female".to_string(),
        },
        FieldOption {
            value: 312,
            label: "PotD Loremaster Male".to_string(),
        },
        FieldOption {
            value: 313,
            label: "PotD Loremaster Male".to_string(),
        },
        FieldOption {
            value: 314,
            label: "PotD Loremaster Female".to_string(),
        },
        FieldOption {
            value: 315,
            label: "PotD Loremaster Female".to_string(),
        },
        FieldOption {
            value: 316,
            label: "PotD Witch King".to_string(),
        },
        FieldOption {
            value: 317,
            label: "PotD Witch King".to_string(),
        },
        FieldOption {
            value: 318,
            label: "PotD Witch Queen".to_string(),
        },
        FieldOption {
            value: 319,
            label: "PotD Witch Queen".to_string(),
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
            label: "NPC Ravenman".to_string(),
        },
        FieldOption {
            value: 329,
            label: "NPC Ravenman".to_string(),
        },
        FieldOption {
            value: 330,
            label: "NPC Ravenman".to_string(),
        },
        FieldOption {
            value: 331,
            label: "NPC Ravenman".to_string(),
        },
        FieldOption {
            value: 332,
            label: "NPC Lich King".to_string(),
        },
        FieldOption {
            value: 333,
            label: "NPC Lich King".to_string(),
        },
        FieldOption {
            value: 334,
            label: "NPC Lich Queen".to_string(),
        },
        FieldOption {
            value: 335,
            label: "NPC Lich Queen".to_string(),
        },
        FieldOption {
            value: 336,
            label: "NPC Etheral Vision Male".to_string(),
        },
        FieldOption {
            value: 337,
            label: "NPC Etheral Vision Male".to_string(),
        },
        FieldOption {
            value: 338,
            label: "NPC Etheral Vision Female".to_string(),
        },
        FieldOption {
            value: 339,
            label: "NPC Etheral Vision Female".to_string(),
        },
        FieldOption {
            value: 340,
            label: "NPC Night Crow".to_string(),
        },
        FieldOption {
            value: 341,
            label: "NPC Night Crow".to_string(),
        },
        FieldOption {
            value: 342,
            label: "NPC Night Crow".to_string(),
        },
        FieldOption {
            value: 343,
            label: "NPC Night Crow".to_string(),
        },
        FieldOption {
            value: 344,
            label: "NPC Blood Gavial".to_string(),
        },
        FieldOption {
            value: 345,
            label: "NPC Blood Gavial".to_string(),
        },
        FieldOption {
            value: 346,
            label: "NPC Blood Gavial".to_string(),
        },
        FieldOption {
            value: 347,
            label: "NPC Blood Gavial".to_string(),
        },
        FieldOption {
            value: 348,
            label: "NPC Gorgon".to_string(),
        },
        FieldOption {
            value: 349,
            label: "NPC Gorgon".to_string(),
        },
        FieldOption {
            value: 350,
            label: "NPC Gorgon".to_string(),
        },
        FieldOption {
            value: 351,
            label: "NPC Gorgon".to_string(),
        },
        FieldOption {
            value: 352,
            label: "NPC Uruk".to_string(),
        },
        FieldOption {
            value: 353,
            label: "NPC Uruk".to_string(),
        },
        FieldOption {
            value: 354,
            label: "NPC Uruk".to_string(),
        },
        FieldOption {
            value: 355,
            label: "NPC Uruk".to_string(),
        },
        FieldOption {
            value: 356,
            label: "NPC Wight Male".to_string(),
        },
        FieldOption {
            value: 357,
            label: "NPC Wight Male".to_string(),
        },
        FieldOption {
            value: 358,
            label: "NPC Wight Female".to_string(),
        },
        FieldOption {
            value: 359,
            label: "NPC Wight Female".to_string(),
        },
        FieldOption {
            value: 360,
            label: "NPC Wraith Male".to_string(),
        },
        FieldOption {
            value: 361,
            label: "NPC Wraith Male".to_string(),
        },
        FieldOption {
            value: 362,
            label: "NPC Wraith Female".to_string(),
        },
        FieldOption {
            value: 363,
            label: "NPC Wraith Female".to_string(),
        },
        FieldOption {
            value: 364,
            label: "NPC Banshee".to_string(),
        },
        FieldOption {
            value: 365,
            label: "NPC Banshee".to_string(),
        },
        FieldOption {
            value: 366,
            label: "NPC Banshee".to_string(),
        },
        FieldOption {
            value: 367,
            label: "NPC Banshee".to_string(),
        },
        FieldOption {
            value: 368,
            label: "NPC Incubus".to_string(),
        },
        FieldOption {
            value: 369,
            label: "NPC Incubus".to_string(),
        },
        FieldOption {
            value: 370,
            label: "NPC Incubus".to_string(),
        },
        FieldOption {
            value: 371,
            label: "NPC Incubus".to_string(),
        },
        FieldOption {
            value: 372,
            label: "NPC Boggart".to_string(),
        },
        FieldOption {
            value: 373,
            label: "NPC Boggart".to_string(),
        },
        FieldOption {
            value: 374,
            label: "NPC Boggart".to_string(),
        },
        FieldOption {
            value: 375,
            label: "NPC Boggart".to_string(),
        },
        FieldOption {
            value: 376,
            label: "NPC Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 377,
            label: "NPC Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 378,
            label: "NPC Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 379,
            label: "NPC Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 380,
            label: "NPC Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 381,
            label: "NPC Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 382,
            label: "NPC Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 383,
            label: "NPC Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 384,
            label: "NPC Scylla".to_string(),
        },
        FieldOption {
            value: 385,
            label: "NPC Scylla".to_string(),
        },
        FieldOption {
            value: 386,
            label: "NPC Scylla".to_string(),
        },
        FieldOption {
            value: 387,
            label: "NPC Scylla".to_string(),
        },
        FieldOption {
            value: 388,
            label: "NPC Naga".to_string(),
        },
        FieldOption {
            value: 389,
            label: "NPC Naga".to_string(),
        },
        FieldOption {
            value: 390,
            label: "NPC Naga".to_string(),
        },
        FieldOption {
            value: 391,
            label: "NPC Naga".to_string(),
        },
        FieldOption {
            value: 392,
            label: "NPC Damasc Golem".to_string(),
        },
        FieldOption {
            value: 393,
            label: "NPC Damasc Golem".to_string(),
        },
        FieldOption {
            value: 394,
            label: "NPC Damasc Golem".to_string(),
        },
        FieldOption {
            value: 395,
            label: "NPC Damasc Golem".to_string(),
        },
        FieldOption {
            value: 396,
            label: "NPC Flesh Golem".to_string(),
        },
        FieldOption {
            value: 397,
            label: "NPC Flesh Golem".to_string(),
        },
        FieldOption {
            value: 398,
            label: "NPC Flesh Golem".to_string(),
        },
        FieldOption {
            value: 399,
            label: "NPC Flesh Golem".to_string(),
        },
        FieldOption {
            value: 400,
            label: "NPC Hippogryph".to_string(),
        },
        FieldOption {
            value: 401,
            label: "NPC Hippogryph".to_string(),
        },
        FieldOption {
            value: 402,
            label: "NPC Hippogryph".to_string(),
        },
        FieldOption {
            value: 403,
            label: "NPC Hippogryph".to_string(),
        },
        FieldOption {
            value: 404,
            label: "NPC Phoenix".to_string(),
        },
        FieldOption {
            value: 405,
            label: "NPC Phoenix".to_string(),
        },
        FieldOption {
            value: 406,
            label: "NPC Phoenix".to_string(),
        },
        FieldOption {
            value: 407,
            label: "NPC Phoenix".to_string(),
        },
        FieldOption {
            value: 408,
            label: "NPC Ruhk".to_string(),
        },
        FieldOption {
            value: 409,
            label: "NPC Ruhk".to_string(),
        },
        FieldOption {
            value: 410,
            label: "NPC Ruhk".to_string(),
        },
        FieldOption {
            value: 411,
            label: "NPC Ruhk".to_string(),
        },
        FieldOption {
            value: 412,
            label: "NPC Basilisk".to_string(),
        },
        FieldOption {
            value: 413,
            label: "NPC Basilisk".to_string(),
        },
        FieldOption {
            value: 414,
            label: "NPC Basilisk".to_string(),
        },
        FieldOption {
            value: 415,
            label: "NPC Basilisk".to_string(),
        },
        FieldOption {
            value: 416,
            label: "NPC Kraken".to_string(),
        },
        FieldOption {
            value: 417,
            label: "NPC Kraken".to_string(),
        },
        FieldOption {
            value: 418,
            label: "NPC Kraken".to_string(),
        },
        FieldOption {
            value: 419,
            label: "NPC Kraken".to_string(),
        },
        FieldOption {
            value: 420,
            label: "NPC Dagon".to_string(),
        },
        FieldOption {
            value: 421,
            label: "NPC Dagon".to_string(),
        },
        FieldOption {
            value: 422,
            label: "NPC Dagon".to_string(),
        },
        FieldOption {
            value: 423,
            label: "NPC Dagon".to_string(),
        },
        FieldOption {
            value: 424,
            label: "NPC Spriggan".to_string(),
        },
        FieldOption {
            value: 425,
            label: "NPC Spriggan".to_string(),
        },
        FieldOption {
            value: 426,
            label: "NPC Spriggan".to_string(),
        },
        FieldOption {
            value: 427,
            label: "NPC Spriggan".to_string(),
        },
        FieldOption {
            value: 428,
            label: "NPC Titan".to_string(),
        },
        FieldOption {
            value: 429,
            label: "NPC Titan".to_string(),
        },
        FieldOption {
            value: 430,
            label: "NPC Titan".to_string(),
        },
        FieldOption {
            value: 431,
            label: "NPC Titan".to_string(),
        },
        FieldOption {
            value: 432,
            label: "NPC Aym".to_string(),
        },
        FieldOption {
            value: 433,
            label: "NPC Aloser".to_string(),
        },
        FieldOption {
            value: 434,
            label: "None".to_string(),
        },
        FieldOption {
            value: 435,
            label: "None".to_string(),
        },
        FieldOption {
            value: 436,
            label: "None".to_string(),
        },
        FieldOption {
            value: 437,
            label: "None".to_string(),
        },
        FieldOption {
            value: 438,
            label: "None".to_string(),
        },
        FieldOption {
            value: 439,
            label: "None".to_string(),
        },
        FieldOption {
            value: 440,
            label: "None".to_string(),
        },
        FieldOption {
            value: 441,
            label: "None".to_string(),
        },
        FieldOption {
            value: 442,
            label: "None".to_string(),
        },
        FieldOption {
            value: 443,
            label: "None".to_string(),
        },
        FieldOption {
            value: 444,
            label: "None".to_string(),
        },
        FieldOption {
            value: 445,
            label: "None".to_string(),
        },
        FieldOption {
            value: 446,
            label: "None".to_string(),
        },
        FieldOption {
            value: 447,
            label: "None".to_string(),
        },
        FieldOption {
            value: 448,
            label: "None".to_string(),
        },
        FieldOption {
            value: 449,
            label: "None".to_string(),
        },
        FieldOption {
            value: 450,
            label: "None".to_string(),
        },
        FieldOption {
            value: 451,
            label: "None".to_string(),
        },
        FieldOption {
            value: 452,
            label: "None".to_string(),
        },
        FieldOption {
            value: 453,
            label: "None".to_string(),
        },
        FieldOption {
            value: 454,
            label: "None".to_string(),
        },
        FieldOption {
            value: 455,
            label: "None".to_string(),
        },
        FieldOption {
            value: 456,
            label: "None".to_string(),
        },
        FieldOption {
            value: 457,
            label: "None".to_string(),
        },
        FieldOption {
            value: 458,
            label: "None".to_string(),
        },
        FieldOption {
            value: 459,
            label: "None".to_string(),
        },
        FieldOption {
            value: 460,
            label: "None".to_string(),
        },
        FieldOption {
            value: 461,
            label: "None".to_string(),
        },
        FieldOption {
            value: 462,
            label: "None".to_string(),
        },
        FieldOption {
            value: 463,
            label: "None".to_string(),
        },
        FieldOption {
            value: 464,
            label: "None".to_string(),
        },
        FieldOption {
            value: 465,
            label: "None".to_string(),
        },
        FieldOption {
            value: 466,
            label: "None".to_string(),
        },
        FieldOption {
            value: 467,
            label: "None".to_string(),
        },
        FieldOption {
            value: 468,
            label: "None".to_string(),
        },
        FieldOption {
            value: 469,
            label: "None".to_string(),
        },
        FieldOption {
            value: 470,
            label: "None".to_string(),
        },
        FieldOption {
            value: 471,
            label: "None".to_string(),
        },
        FieldOption {
            value: 472,
            label: "None".to_string(),
        },
        FieldOption {
            value: 473,
            label: "None".to_string(),
        },
        FieldOption {
            value: 474,
            label: "None".to_string(),
        },
        FieldOption {
            value: 475,
            label: "None".to_string(),
        },
        FieldOption {
            value: 476,
            label: "None".to_string(),
        },
        FieldOption {
            value: 477,
            label: "None".to_string(),
        },
        FieldOption {
            value: 478,
            label: "None".to_string(),
        },
        FieldOption {
            value: 479,
            label: "None".to_string(),
        },
        FieldOption {
            value: 480,
            label: "None".to_string(),
        },
        FieldOption {
            value: 481,
            label: "None".to_string(),
        },
        FieldOption {
            value: 482,
            label: "None".to_string(),
        },
        FieldOption {
            value: 483,
            label: "None".to_string(),
        },
        FieldOption {
            value: 484,
            label: "None".to_string(),
        },
        FieldOption {
            value: 485,
            label: "None".to_string(),
        },
        FieldOption {
            value: 486,
            label: "None".to_string(),
        },
        FieldOption {
            value: 487,
            label: "None".to_string(),
        },
        FieldOption {
            value: 488,
            label: "None".to_string(),
        },
        FieldOption {
            value: 489,
            label: "None".to_string(),
        },
        FieldOption {
            value: 490,
            label: "NPC Vija".to_string(),
        },
        FieldOption {
            value: 491,
            label: "NPC Enja".to_string(),
        },
        FieldOption {
            value: 492,
            label: "NPC Maitreya".to_string(),
        },
        FieldOption {
            value: 493,
            label: "NPC Ijana".to_string(),
        },
        FieldOption {
            value: 494,
            label: "NPC Chandra".to_string(),
        },
        FieldOption {
            value: 495,
            label: "NPC Vayu".to_string(),
        },
        FieldOption {
            value: 496,
            label: "NPC Indra".to_string(),
        },
        FieldOption {
            value: 497,
            label: "NPC Rakshas".to_string(),
        },
        FieldOption {
            value: 498,
            label: "NPC Ashurama".to_string(),
        },
        FieldOption {
            value: 499,
            label: "NPC Asurama".to_string(),
        },
        FieldOption {
            value: 500,
            label: "NPC Aditi".to_string(),
        },
        FieldOption {
            value: 501,
            label: "NPC Saranga".to_string(),
        },
        FieldOption {
            value: 502,
            label: "NPC Kandyce".to_string(),
        },
        FieldOption {
            value: 503,
            label: "None".to_string(),
        },
        FieldOption {
            value: 504,
            label: "NPC Sirene".to_string(),
        },
        FieldOption {
            value: 505,
            label: "NPC Vainateya".to_string(),
        },
        FieldOption {
            value: 506,
            label: "NPC Nathalork".to_string(),
        },
        FieldOption {
            value: 507,
            label: "NPC Xolotl".to_string(),
        },
        FieldOption {
            value: 508,
            label: "NPC Tlaloc".to_string(),
        },
        FieldOption {
            value: 509,
            label: "NPC Ifrit".to_string(),
        },
        FieldOption {
            value: 510,
            label: "NPC Lygenstzel".to_string(),
        },
    ]
}

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "shop".to_string(),
        name: "Shop Editor".to_string(),
        description: "Edit shop inventory and availability".to_string(),
        base_offset: 0x69376,
        entry_count: 514,
        entry_size: 38,
        entry_names: vec![
            "Record 0".to_string(),
            "Record 1".to_string(),
            "Record 2".to_string(),
            "Record 3".to_string(),
            "Record 4".to_string(),
            "Record 5".to_string(),
            "Record 6".to_string(),
            "Record 7".to_string(),
            "Record 8".to_string(),
            "Record 9".to_string(),
            "Record 10".to_string(),
            "Record 11".to_string(),
            "Record 12".to_string(),
            "Record 13".to_string(),
            "Record 14".to_string(),
            "Record 15".to_string(),
            "Record 16".to_string(),
            "Record 17".to_string(),
            "Record 18".to_string(),
            "Record 19".to_string(),
            "Record 20".to_string(),
            "Record 21".to_string(),
            "Record 22".to_string(),
            "Record 23".to_string(),
            "Record 24".to_string(),
            "Record 25".to_string(),
            "Record 26".to_string(),
            "Record 27".to_string(),
            "Record 28".to_string(),
            "Record 29".to_string(),
            "Record 30".to_string(),
            "Record 31".to_string(),
            "Record 32".to_string(),
            "Record 33".to_string(),
            "Record 34".to_string(),
            "Record 35".to_string(),
            "Record 36".to_string(),
            "Record 37".to_string(),
            "Record 38".to_string(),
            "Record 39".to_string(),
            "Record 40".to_string(),
            "Record 41".to_string(),
            "Record 42".to_string(),
            "Record 43".to_string(),
            "Record 44".to_string(),
            "Record 45".to_string(),
            "Record 46".to_string(),
            "Record 47".to_string(),
            "Record 48".to_string(),
            "Record 49".to_string(),
            "Record 50".to_string(),
            "Record 51".to_string(),
            "Record 52".to_string(),
            "Record 53".to_string(),
            "Record 54".to_string(),
            "Record 55".to_string(),
            "Record 56".to_string(),
            "Record 57".to_string(),
            "Record 58".to_string(),
            "Record 59".to_string(),
            "Record 60".to_string(),
            "Record 61".to_string(),
            "Record 62".to_string(),
            "Record 63".to_string(),
            "Record 64".to_string(),
            "Record 65".to_string(),
            "Record 66".to_string(),
            "Record 67".to_string(),
            "Record 68".to_string(),
            "Record 69".to_string(),
            "Record 70".to_string(),
            "Record 71".to_string(),
            "Record 72".to_string(),
            "Record 73".to_string(),
            "Record 74".to_string(),
            "Record 75".to_string(),
            "Record 76".to_string(),
            "Record 77".to_string(),
            "Record 78".to_string(),
            "Record 79".to_string(),
            "Record 80".to_string(),
            "Record 81".to_string(),
            "Record 82".to_string(),
            "Record 83".to_string(),
            "Record 84".to_string(),
            "Record 85".to_string(),
            "Record 86".to_string(),
            "Record 87".to_string(),
            "Record 88".to_string(),
            "Record 89".to_string(),
            "Record 90".to_string(),
            "Record 91".to_string(),
            "Record 92".to_string(),
            "Record 93".to_string(),
            "Record 94".to_string(),
            "Record 95".to_string(),
            "Record 96".to_string(),
            "Record 97".to_string(),
            "Record 98".to_string(),
            "Record 99".to_string(),
            "Record 100".to_string(),
            "Record 101".to_string(),
            "Record 102".to_string(),
            "Record 103".to_string(),
            "Record 104".to_string(),
            "Record 105".to_string(),
            "Record 106".to_string(),
            "Record 107".to_string(),
            "Record 108".to_string(),
            "Record 109".to_string(),
            "Record 110".to_string(),
            "Record 111".to_string(),
            "Record 112".to_string(),
            "Record 113".to_string(),
            "Record 114".to_string(),
            "Record 115".to_string(),
            "Record 116".to_string(),
            "Record 117".to_string(),
            "Record 118".to_string(),
            "Record 119".to_string(),
            "Record 120".to_string(),
            "Record 121".to_string(),
            "Record 122".to_string(),
            "Record 123".to_string(),
            "Record 124".to_string(),
            "Record 125".to_string(),
            "Record 126".to_string(),
            "Record 127".to_string(),
            "Record 128".to_string(),
            "Record 129".to_string(),
            "Record 130".to_string(),
            "Record 131".to_string(),
            "Record 132".to_string(),
            "Record 133".to_string(),
            "Record 134".to_string(),
            "Record 135".to_string(),
            "Record 136".to_string(),
            "Record 137".to_string(),
            "Record 138".to_string(),
            "Record 139".to_string(),
            "Record 140".to_string(),
            "Record 141".to_string(),
            "Record 142".to_string(),
            "Record 143".to_string(),
            "Record 144".to_string(),
            "Record 145".to_string(),
            "Record 146".to_string(),
            "Record 147".to_string(),
            "Record 148".to_string(),
            "Record 149".to_string(),
            "Record 150".to_string(),
            "Record 151".to_string(),
            "Record 152".to_string(),
            "Record 153".to_string(),
            "Record 154".to_string(),
            "Record 155".to_string(),
            "Record 156".to_string(),
            "Record 157".to_string(),
            "Record 158".to_string(),
            "Record 159".to_string(),
            "Record 160".to_string(),
            "Record 161".to_string(),
            "Record 162".to_string(),
            "Record 163".to_string(),
            "Record 164".to_string(),
            "Record 165".to_string(),
            "Record 166".to_string(),
            "Record 167".to_string(),
            "Record 168".to_string(),
            "Record 169".to_string(),
            "Record 170".to_string(),
            "Record 171".to_string(),
            "Record 172".to_string(),
            "Record 173".to_string(),
            "Record 174".to_string(),
            "Record 175".to_string(),
            "Record 176".to_string(),
            "Record 177".to_string(),
            "Record 178".to_string(),
            "Record 179".to_string(),
            "Record 180".to_string(),
            "Record 181".to_string(),
            "Record 182".to_string(),
            "Record 183".to_string(),
            "Record 184".to_string(),
            "Record 185".to_string(),
            "Record 186".to_string(),
            "Record 187".to_string(),
            "Record 188".to_string(),
            "Record 189".to_string(),
            "Record 190".to_string(),
            "Record 191".to_string(),
            "Record 192".to_string(),
            "Record 193".to_string(),
            "Record 194".to_string(),
            "Record 195".to_string(),
            "Record 196".to_string(),
            "Record 197".to_string(),
            "Record 198".to_string(),
            "Record 199".to_string(),
            "Record 200".to_string(),
            "Record 201".to_string(),
            "Record 202".to_string(),
            "Record 203".to_string(),
            "Record 204".to_string(),
            "Record 205".to_string(),
            "Record 206".to_string(),
            "Record 207".to_string(),
            "Record 208".to_string(),
            "Record 209".to_string(),
            "Record 210".to_string(),
            "Record 211".to_string(),
            "Record 212".to_string(),
            "Record 213".to_string(),
            "Record 214".to_string(),
            "Record 215".to_string(),
            "Record 216".to_string(),
            "Record 217".to_string(),
            "Record 218".to_string(),
            "Record 219".to_string(),
            "Record 220".to_string(),
            "Record 221".to_string(),
            "Record 222".to_string(),
            "Record 223".to_string(),
            "Record 224".to_string(),
            "Record 225".to_string(),
            "Record 226".to_string(),
            "Record 227".to_string(),
            "Record 228".to_string(),
            "Record 229".to_string(),
            "Record 230".to_string(),
            "Record 231".to_string(),
            "Record 232".to_string(),
            "Record 233".to_string(),
            "Record 234".to_string(),
            "Record 235".to_string(),
            "Record 236".to_string(),
            "Record 237".to_string(),
            "Record 238".to_string(),
            "Record 239".to_string(),
            "Record 240".to_string(),
            "Record 241".to_string(),
            "Record 242".to_string(),
            "Record 243".to_string(),
            "Record 244".to_string(),
            "Record 245".to_string(),
            "Record 246".to_string(),
            "Record 247".to_string(),
            "Record 248".to_string(),
            "Record 249".to_string(),
            "Record 250".to_string(),
            "Record 251".to_string(),
            "Record 252".to_string(),
            "Record 253".to_string(),
            "Record 254".to_string(),
            "Record 255".to_string(),
            "Record 256".to_string(),
            "Record 257".to_string(),
            "Record 258".to_string(),
            "Record 259".to_string(),
            "Record 260".to_string(),
            "Record 261".to_string(),
            "Record 262".to_string(),
            "Record 263".to_string(),
            "Record 264".to_string(),
            "Record 265".to_string(),
            "Record 266".to_string(),
            "Record 267".to_string(),
            "Record 268".to_string(),
            "Record 269".to_string(),
            "Record 270".to_string(),
            "Record 271".to_string(),
            "Record 272".to_string(),
            "Record 273".to_string(),
            "Record 274".to_string(),
            "Record 275".to_string(),
            "Record 276".to_string(),
            "Record 277".to_string(),
            "Record 278".to_string(),
            "Record 279".to_string(),
            "Record 280".to_string(),
            "Record 281".to_string(),
            "Record 282".to_string(),
            "Record 283".to_string(),
            "Record 284".to_string(),
            "Record 285".to_string(),
            "Record 286".to_string(),
            "Record 287".to_string(),
            "Record 288".to_string(),
            "Record 289".to_string(),
            "Record 290".to_string(),
            "Record 291".to_string(),
            "Record 292".to_string(),
            "Record 293".to_string(),
            "Record 294".to_string(),
            "Record 295".to_string(),
            "Record 296".to_string(),
            "Record 297".to_string(),
            "Record 298".to_string(),
            "Record 299".to_string(),
            "Record 300".to_string(),
            "Record 301".to_string(),
            "Record 302".to_string(),
            "Record 303".to_string(),
            "Record 304".to_string(),
            "Record 305".to_string(),
            "Record 306".to_string(),
            "Record 307".to_string(),
            "Record 308".to_string(),
            "Record 309".to_string(),
            "Record 310".to_string(),
            "Record 311".to_string(),
            "Record 312".to_string(),
            "Record 313".to_string(),
            "Record 314".to_string(),
            "Record 315".to_string(),
            "Record 316".to_string(),
            "Record 317".to_string(),
            "Record 318".to_string(),
            "Record 319".to_string(),
            "Record 320".to_string(),
            "Record 321".to_string(),
            "Record 322".to_string(),
            "Record 323".to_string(),
            "Record 324".to_string(),
            "Record 325".to_string(),
            "Record 326".to_string(),
            "Record 327".to_string(),
            "Record 328".to_string(),
            "Record 329".to_string(),
            "Record 330".to_string(),
            "Record 331".to_string(),
            "Record 332".to_string(),
            "Record 333".to_string(),
            "Record 334".to_string(),
            "Record 335".to_string(),
            "Record 336".to_string(),
            "Record 337".to_string(),
            "Record 338".to_string(),
            "Record 339".to_string(),
            "Record 340".to_string(),
            "Record 341".to_string(),
            "Record 342".to_string(),
            "Record 343".to_string(),
            "Record 344".to_string(),
            "Record 345".to_string(),
            "Record 346".to_string(),
            "Record 347".to_string(),
            "Record 348".to_string(),
            "Record 349".to_string(),
            "Record 350".to_string(),
            "Record 351".to_string(),
            "Record 352".to_string(),
            "Record 353".to_string(),
            "Record 354".to_string(),
            "Record 355".to_string(),
            "Record 356".to_string(),
            "Record 357".to_string(),
            "Record 358".to_string(),
            "Record 359".to_string(),
            "Record 360".to_string(),
            "Record 361".to_string(),
            "Record 362".to_string(),
            "Record 363".to_string(),
            "Record 364".to_string(),
            "Record 365".to_string(),
            "Record 366".to_string(),
            "Record 367".to_string(),
            "Record 368".to_string(),
            "Record 369".to_string(),
            "Record 370".to_string(),
            "Record 371".to_string(),
            "Record 372".to_string(),
            "Record 373".to_string(),
            "Record 374".to_string(),
            "Record 375".to_string(),
            "Record 376".to_string(),
            "Record 377".to_string(),
            "Record 378".to_string(),
            "Record 379".to_string(),
            "Record 380".to_string(),
            "Record 381".to_string(),
            "Record 382".to_string(),
            "Record 383".to_string(),
            "Record 384".to_string(),
            "Record 385".to_string(),
            "Record 386".to_string(),
            "Record 387".to_string(),
            "Record 388".to_string(),
            "Record 389".to_string(),
            "Record 390".to_string(),
            "Record 391".to_string(),
            "Record 392".to_string(),
            "Record 393".to_string(),
            "Record 394".to_string(),
            "Record 395".to_string(),
            "Record 396".to_string(),
            "Record 397".to_string(),
            "Record 398".to_string(),
            "Record 399".to_string(),
            "Record 400".to_string(),
            "Record 401".to_string(),
            "Record 402".to_string(),
            "Record 403".to_string(),
            "Record 404".to_string(),
            "Record 405".to_string(),
            "Record 406".to_string(),
            "Record 407".to_string(),
            "Record 408".to_string(),
            "Record 409".to_string(),
            "Record 410".to_string(),
            "Record 411".to_string(),
            "Record 412".to_string(),
            "Record 413".to_string(),
            "Record 414".to_string(),
            "Record 415".to_string(),
            "Record 416".to_string(),
            "Record 417".to_string(),
            "Record 418".to_string(),
            "Record 419".to_string(),
            "Record 420".to_string(),
            "Record 421".to_string(),
            "Record 422".to_string(),
            "Record 423".to_string(),
            "Record 424".to_string(),
            "Record 425".to_string(),
            "Record 426".to_string(),
            "Record 427".to_string(),
            "Record 428".to_string(),
            "Record 429".to_string(),
            "Record 430".to_string(),
            "Record 431".to_string(),
            "Record 432".to_string(),
            "Record 433".to_string(),
            "Record 434".to_string(),
            "Record 435".to_string(),
            "Record 436".to_string(),
            "Record 437".to_string(),
            "Record 438".to_string(),
            "Record 439".to_string(),
            "Record 440".to_string(),
            "Record 441".to_string(),
            "Record 442".to_string(),
            "Record 443".to_string(),
            "Record 444".to_string(),
            "Record 445".to_string(),
            "Record 446".to_string(),
            "Record 447".to_string(),
            "Record 448".to_string(),
            "Record 449".to_string(),
            "Record 450".to_string(),
            "Record 451".to_string(),
            "Record 452".to_string(),
            "Record 453".to_string(),
            "Record 454".to_string(),
            "Record 455".to_string(),
            "Record 456".to_string(),
            "Record 457".to_string(),
            "Record 458".to_string(),
            "Record 459".to_string(),
            "Record 460".to_string(),
            "Record 461".to_string(),
            "Record 462".to_string(),
            "Record 463".to_string(),
            "Record 464".to_string(),
            "Record 465".to_string(),
            "Record 466".to_string(),
            "Record 467".to_string(),
            "Record 468".to_string(),
            "Record 469".to_string(),
            "Record 470".to_string(),
            "Record 471".to_string(),
            "Record 472".to_string(),
            "Record 473".to_string(),
            "Record 474".to_string(),
            "Record 475".to_string(),
            "Record 476".to_string(),
            "Record 477".to_string(),
            "Record 478".to_string(),
            "Record 479".to_string(),
            "Record 480".to_string(),
            "Record 481".to_string(),
            "Record 482".to_string(),
            "Record 483".to_string(),
            "Record 484".to_string(),
            "Record 485".to_string(),
            "Record 486".to_string(),
            "Record 487".to_string(),
            "Record 488".to_string(),
            "Record 489".to_string(),
            "Record 490".to_string(),
            "Record 491".to_string(),
            "Record 492".to_string(),
            "Record 493".to_string(),
            "Record 494".to_string(),
            "Record 495".to_string(),
            "Record 496".to_string(),
            "Record 497".to_string(),
            "Record 498".to_string(),
            "Record 499".to_string(),
            "Record 500".to_string(),
            "Record 501".to_string(),
            "Record 502".to_string(),
            "Record 503".to_string(),
            "Record 504".to_string(),
            "Record 505".to_string(),
            "Record 506".to_string(),
            "Record 507".to_string(),
            "Record 508".to_string(),
            "Record 509".to_string(),
            "Record 510".to_string(),
            "Record 511".to_string(),
            "Record 512".to_string(),
            "Record 513".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "Item".to_string(),
                offset: 0,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_shop_items()),
            },
            FieldDefinition {
                name: "Crafting Flag".to_string(),
                offset: 2,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Character Flag".to_string(),
                offset: 4,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_character_list()),
            },
            FieldDefinition {
                name: "Unknown Flag".to_string(),
                offset: 5,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Buy Count Global Flag".to_string(),
                offset: 6,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Sell Count Global Flag".to_string(),
                offset: 8,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown Flag".to_string(),
                offset: 10,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Brigantys Shop".to_string(),
                offset: 19,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Port Asyton Shop".to_string(),
                offset: 20,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Coritanae Shop".to_string(),
                offset: 21,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Balmamusa Shop".to_string(),
                offset: 22,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Phidoch Castle Shop".to_string(),
                offset: 23,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Krysaro Shop".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Rhime Shop".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Almorica Castle Shop".to_string(),
                offset: 26,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Golyat Shop".to_string(),
                offset: 27,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Barnicia Castle Shop".to_string(),
                offset: 28,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Oeram Shop".to_string(),
                offset: 29,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Heim Shop".to_string(),
                offset: 30,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Grimsby Shop".to_string(),
                offset: 31,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Port Omish Shop".to_string(),
                offset: 32,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Deneb's Shop".to_string(),
                offset: 34,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "PotD Floor 24".to_string(),
                offset: 35,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "PotD Floor 65".to_string(),
                offset: 36,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "PotD Floor 103".to_string(),
                offset: 37,
                size: 1,
                field_type: FieldType::Uint,
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
        assert_eq!(def.id, "shop");
        assert_eq!(def.base_offset, 0x69376);
        assert_eq!(def.entry_count, 514);
        assert_eq!(def.entry_size, 38);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 25);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 514);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Item");
        assert_eq!(f.offset, 0);
        assert_eq!(f.size, 2);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[24];
        assert_eq!(f.name, "PotD Floor 103");
        assert_eq!(f.offset, 37);
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
