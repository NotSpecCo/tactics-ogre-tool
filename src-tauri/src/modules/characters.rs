use crate::modules::types::*;

fn opt_race_list() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Human".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Beast".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Reptile".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Divine".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Umbra".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Phantom".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Golem".to_string(),
        },
    ]
}

fn opt_gender() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Male".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Female".to_string(),
        },
    ]
}

fn opt_yes_no() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "No".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Yes".to_string(),
        },
    ]
}

fn opt_class_groups() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Generic".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Hawkman".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Lizardman".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Lamia".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Orc".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Skeleton".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Phantom".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Gremlin".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Pumpkinhead".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Golem".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Oelias".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Inherent".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Cressidia".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Denam".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Vyce".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Catiua".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Lancelot".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Warren".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Canopus".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Mirdyn".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Gildas".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Cerya".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Sherri".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Cistina".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Olyvia".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Deneb".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Iuria".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Ozma".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Leonar".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Ravness".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Azelstan".to_string(),
        },
    ]
}

fn opt_race_list_2() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Human".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Hawkman".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Lizardman".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Lamia".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Orc".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Skeleton".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Phantom".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Gremlin".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Pumpkinhead".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Golem".to_string(),
        },
    ]
}

fn opt_sprites() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Class-Dependent".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Villager Woman".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Villager Old Man".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Villager Old Woman".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Villager Old Man 2".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Andoras".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Female Archer".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Male Archer".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Azelstan".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Balbatos".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Balxephon".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Balxephon Cloaked".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Barbas".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Female Beast Tamer".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Male Beast Tamer".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Female Berserker".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Male Berserker".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Villager Boy".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Villager Girl".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Villager Boy 2".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Brantyn".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Canopus".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Female Cleric".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Male Cleric".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Cressida".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Catiua (fake)".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Deneb".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Denam".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Denam (lord)".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Female Divine Knight".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Male Divine Knight".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Templar".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Dorgalua".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Final Boss".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Arc Dragon".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Cloud Dragon".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Dark Dragon".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Earth Dragon".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Flame Dragon".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Flood Dragon".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Frost Dragon".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Thunder Dragon".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Female Dragoon".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Male Dragoon".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Old Sorceress".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Ganpp".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Phantom".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Ganpp Pixelated".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Vilager Girl 2".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Clay Golem".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Iron Golem".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Stone Golem".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Baldur Golem".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Gremlin".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Gildas".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Female Fusilier".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Male Fusilier".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Hawkman".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Hobyrim".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Hydra".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Catiua".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Dark Catiua".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Princess Catiua".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Klaire".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Female Knight".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Male Knight".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Lamia".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Lanselot Hamilton".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Lanselot Tartaros".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Lanselot Tartaros Cloaked".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Leonar".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Female Lich".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Male Lich".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Lindl".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Lizardman".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Denam Pixelated".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Mannaflora".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Martym".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Nothing".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Running Villager?".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Mirdyn".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Old Sorcerer".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Mreuva".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Female Necromancer".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Male Necromancer".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Kunoichi".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Ninja".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Nybeth".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Olivya".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Orc".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Oelias".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Oz".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Ozma".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Prancet".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Pumpkinhead".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Ravness".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Rodrick".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Villager Boy 3".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Ronway".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Female Rogue".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Male Rogue".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Cerya".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Sherri".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Cistina".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Skeleton".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Witch".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Warlock".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Female Swordmaster".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Male Swordmaster".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Female Terror Knight".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Male Terror Knight".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Valkyrie".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Rune Fencer".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Vernotta".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Vyce".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Vyce's Father".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Volaq".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Volaq Cloaked".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Warren".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Female Warrior".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Male Warrior".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Enchantress".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Wizard".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Villager Lady".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Villager Lady 2".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Villager Man".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Iuria".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Xaebos".to_string(),
        },
    ]
}

fn opt_color_palette() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Player".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Bakram".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Galgastan".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Pirates".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Walister".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Outlaws".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Zombie".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Divine".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Bestial".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Palette 10".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Palette 11".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Dorgalua".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Umbra".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Palette 14".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Palette 15".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Petrified".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Palette 17".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Palette 18".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Faction-Dependent".to_string(),
        },
    ]
}

fn opt_portraits() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Class Dependant".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Villager Woman".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Villager Old Man".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Villager Old Woman".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Villager Old Man".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Arycelle".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Arycelle (Copy?)".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Andoras".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Female Archer (Player)".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Female Archer (Bakram)".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Female Archer (Galgastan)".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Female Archer (Pirates)".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Female Archer (Walister)".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Female Archer (Outlaw)".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Female Archer (Unknown)".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Female Archer (Divine)".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Female Archer (Bestial)".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Female Archer (Umbral)".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Male Archer (Player)".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Male Archer (Bakram)".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Male Archer (Galgastan)".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Male Archer (Pirates)".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Male Archer (Walister)".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Male Archer (Outlaw)".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Male Archer (Unknown)".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Male Archer (Divine)".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Male Archer (Bestial)".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Male Archer (Umbral)".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Azelstan Smiling".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Azelstan Frowning".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Balbatos".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Balxephon".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Barbas".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Female Beast Tamer (Player)".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Female Beast Tamer (Bakram)".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Female Beast Tamer (Galgastan)".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Female Beast Tamer (Pirates)".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Female Beast Tamer (Outlaw)".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Female Beast Tamer (Unknown)".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Female Beast Tamer (Unknown)".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Female Beast Tamer (Divine)".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Female Beast Tamer (Bestial)".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Female Beast Tamer (Umbral)".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Male Beast Tamer (Player)".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Male Beast Tamer (Bakram)".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Male Beast Tamer (Galgastan)".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Male Beast Tamer (Pirates)".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Male Beast Tamer (Walister)".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Male Beast Tamer (Outlaw)".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Male Beast Tamer (Unknown)".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Male Beast Tamer (Divine)".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Male Beast Tamer (Bestial)".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Male Beast Tamer (Umbral)".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Beelzebuth".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Female Berserker (Player)".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Female Berserker (Bakram)".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Female Berserker (Galgastan)".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Female Berserker (Pirates)".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Female Berserker (Walister)".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Female Berserker (Outlaw)".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Female Berserker (Unknown)".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Female Berserker (Divine)".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Female Berserker (Bestial)".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Female Berserker (Umbral)".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Male Berserker (Player)".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Male Berserker (Bakram)".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Male Berserker (Galgastan)".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Male Berserker (Pirates)".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Male Berserker (Walister)".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Male Berserker (Outlaw)".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Male Berserker (Unknown)".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Male Berserker (Divine)".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Male Berserker (Bestial)".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Male Berserker (Umbral)".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Rodrick Young Ghost".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Rodrick Young".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Brantyn".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Bayin".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Canopus".to_string(),
        },
        FieldOption {
            value: 79,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Female Cleric (Player)".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Female Cleric (Bakram)".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Female Cleric (Galgastan)".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Female Cleric (Pirates)".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Female Cleric (Walister)".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Female Cleric (Outlaw)".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Female Cleric (Unknown)".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Female Cleric (Divine)".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Female Cleric (Bestial)".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Female Cleric (Umbral)".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Male Cleric (Player)".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Male Cleric (Bakram)".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Male Cleric (Galgastan)".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Male Cleric (Pirates)".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Male Cleric (Walister)".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Male Cleric (Outlaw)".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Male Cleric (Unknown)".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Male Cleric (Divine)".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Male Cleric (Bestial)".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Male Cleric (Umbral)".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Basilisk".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Cockatrice".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Cressidia".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Cyclops".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Dievold".to_string(),
        },
        FieldOption {
            value: 115,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 116,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Deneb".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Punkin".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Denam".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Denam Yelling".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Denam old portrait".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Denam King".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Denam Lord".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Hektor".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Angel Kight Female (Player)".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Angel Kight Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Angel Kight Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Angel Kight Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Angel Kight Female (Walister)".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Angel Kight Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Angel Kight Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Angel Kight Female (Divine)".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Angel Kight Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Angel Kight Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Angel Kight Male (Player)".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Angel Kight Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Angel Kight Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Angel Kight Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Angel Kight Male (Walister)".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Angel Kight Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Angel Kight Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Angel Kight Male (Divine)".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Angel Kight Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Angel Kight Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Templar (Terror Knight/Necromancer)".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Templar (Warrior/Valkyrie/Knight/Berserker/Dragoon/Beast Tamer)".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Templar (Archer/Ninja/Rogue/Fusilier)".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Templar (Cleric)".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Templar (Swordmaster)".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Templar (Wizard/Warlock)".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Templar (Unknown)".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Templar (Umbral)".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Dorgalua".to_string(),
        },
        FieldOption {
            value: 154,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Ogre Dorgalua".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Arc Dragon".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Crystal Dragon".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Cloud Dragon".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Dark Dragon".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Onyx Dragon".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Earth Dragon".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Flame Dragon".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Flood Dragon".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Frost Dragon".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Thunder Dragon".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Dragoon Female (Player)".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Dragoon Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Dragoon Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Dragoon Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Dragoon Female (Walister)".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Dragoon Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Dragoon Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Dragoon Female (Divine)".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Dragoon Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Dragoon Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Dragoon Male (Player)".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Dragoon Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Dragoon Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Dragoon Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Dragoon Male (Walister)".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Dragoon Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Dragoon Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Dragoon Male (Divine)".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Dragoon Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Dragoon Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Faerie (Player)".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Faerie (Bakram)".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Faerie (Galgastan)".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Faerie (Pirates)".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Faerie (Walister)".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Faerie (Outlaw)".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Faerie (Unknown)".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Faerie (Divine)".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Faerie (Bestial)".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Faerie (Umbral)".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Falfaday".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Folcurt".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Ganpp".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Gatialo".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Ghost (Player)".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Ghost (Bakram)".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Ghost (Galgastan)".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Ghost (Pirates)".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Ghost (Walister)".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Ghost (Outlaw)".to_string(),
        },
        FieldOption {
            value: 234,
            label: "Ghost (Unknown)".to_string(),
        },
        FieldOption {
            value: 235,
            label: "Ghost (Divine)".to_string(),
        },
        FieldOption {
            value: 236,
            label: "Ghost (Bestial)".to_string(),
        },
        FieldOption {
            value: 237,
            label: "Ghost (Umbral)".to_string(),
        },
        FieldOption {
            value: 238,
            label: "Gilbald".to_string(),
        },
        FieldOption {
            value: 239,
            label: "Rodrick Young Ghost".to_string(),
        },
        FieldOption {
            value: 240,
            label: "Clay Golem".to_string(),
        },
        FieldOption {
            value: 241,
            label: "Clay Golem".to_string(),
        },
        FieldOption {
            value: 242,
            label: "Stone Golem".to_string(),
        },
        FieldOption {
            value: 243,
            label: "Baldur Golem".to_string(),
        },
        FieldOption {
            value: 244,
            label: "Blue Golem".to_string(),
        },
        FieldOption {
            value: 245,
            label: "Red Golem".to_string(),
        },
        FieldOption {
            value: 246,
            label: "Damasc Golem".to_string(),
        },
        FieldOption {
            value: 247,
            label: "Purple Golem".to_string(),
        },
        FieldOption {
            value: 248,
            label: "Golem (Divine)".to_string(),
        },
        FieldOption {
            value: 249,
            label: "Golem (Bestial)".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Golem (Umbral)".to_string(),
        },
        FieldOption {
            value: 251,
            label: "Iron Golem".to_string(),
        },
        FieldOption {
            value: 252,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 253,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 254,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 255,
            label: "Stone Golem".to_string(),
        },
        FieldOption {
            value: 256,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 257,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 258,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 259,
            label: "Baldur Golem".to_string(),
        },
        FieldOption {
            value: 260,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 261,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 262,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 263,
            label: "Gremlin (Player)".to_string(),
        },
        FieldOption {
            value: 264,
            label: "Gremlin (Bakram)".to_string(),
        },
        FieldOption {
            value: 265,
            label: "Gremlin (Galgastan)".to_string(),
        },
        FieldOption {
            value: 266,
            label: "Gremlin (Pirates)".to_string(),
        },
        FieldOption {
            value: 267,
            label: "Gremlin (Walister)".to_string(),
        },
        FieldOption {
            value: 268,
            label: "Gremlin (Outlaw)".to_string(),
        },
        FieldOption {
            value: 269,
            label: "Gremlin (Unknown)".to_string(),
        },
        FieldOption {
            value: 270,
            label: "Gremlin (Divine)".to_string(),
        },
        FieldOption {
            value: 271,
            label: "Gremlin (Bestial)".to_string(),
        },
        FieldOption {
            value: 272,
            label: "Gremlin (Umbral)".to_string(),
        },
        FieldOption {
            value: 273,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 274,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 275,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 276,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 277,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 278,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 279,
            label: "Gryphon".to_string(),
        },
        FieldOption {
            value: 280,
            label: "Gildas".to_string(),
        },
        FieldOption {
            value: 281,
            label: "Fusilier Female (Player)".to_string(),
        },
        FieldOption {
            value: 282,
            label: "Fusilier Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 283,
            label: "Fusilier Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 284,
            label: "Fusilier Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 285,
            label: "Fusilier Female (Walister)".to_string(),
        },
        FieldOption {
            value: 286,
            label: "Fusilier Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 287,
            label: "Fusilier Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 288,
            label: "Fusilier Female (Divine)".to_string(),
        },
        FieldOption {
            value: 289,
            label: "Fusilier Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 290,
            label: "Fusilier Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 291,
            label: "Fusilier Male (Player)".to_string(),
        },
        FieldOption {
            value: 292,
            label: "Fusilier Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 293,
            label: "Fusilier Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 294,
            label: "Fusilier Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 295,
            label: "Fusilier Male (Walister)".to_string(),
        },
        FieldOption {
            value: 296,
            label: "Fusilier Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 297,
            label: "Fusilier Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 298,
            label: "Fusilier Male (Divine)".to_string(),
        },
        FieldOption {
            value: 299,
            label: "Fusilier Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 300,
            label: "Fusilier Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 301,
            label: "Hawkman (Player)".to_string(),
        },
        FieldOption {
            value: 302,
            label: "Hawkman (Bakram)".to_string(),
        },
        FieldOption {
            value: 303,
            label: "Hawkman (Galgastan)".to_string(),
        },
        FieldOption {
            value: 304,
            label: "Hawkman (Pirates)".to_string(),
        },
        FieldOption {
            value: 305,
            label: "Hawkman (Walister)".to_string(),
        },
        FieldOption {
            value: 306,
            label: "Hawkman (Outlaw)".to_string(),
        },
        FieldOption {
            value: 307,
            label: "Hawkman (Unknown)".to_string(),
        },
        FieldOption {
            value: 308,
            label: "Hawkman (Divine)".to_string(),
        },
        FieldOption {
            value: 309,
            label: "Hawkman (Bestial)".to_string(),
        },
        FieldOption {
            value: 310,
            label: "Hawkman (Umbral)".to_string(),
        },
        FieldOption {
            value: 311,
            label: "Hobyrim".to_string(),
        },
        FieldOption {
            value: 312,
            label: "Hobyrim Sighted".to_string(),
        },
        FieldOption {
            value: 313,
            label: "Hydra".to_string(),
        },
        FieldOption {
            value: 314,
            label: "Hydra (tan)".to_string(),
        },
        FieldOption {
            value: 315,
            label: "Hydra (dark green)".to_string(),
        },
        FieldOption {
            value: 316,
            label: "Hydra (gold)".to_string(),
        },
        FieldOption {
            value: 317,
            label: "Hydra (orange)".to_string(),
        },
        FieldOption {
            value: 318,
            label: "Hydra (Walister)".to_string(),
        },
        FieldOption {
            value: 319,
            label: "Jeunan".to_string(),
        },
        FieldOption {
            value: 320,
            label: "Catiua Princess".to_string(),
        },
        FieldOption {
            value: 321,
            label: "Dark Priest Crying".to_string(),
        },
        FieldOption {
            value: 322,
            label: "Dark Priest".to_string(),
        },
        FieldOption {
            value: 323,
            label: "Catiua".to_string(),
        },
        FieldOption {
            value: 324,
            label: "Catiua Queen".to_string(),
        },
        FieldOption {
            value: 325,
            label: "Catiua before crowning yelling".to_string(),
        },
        FieldOption {
            value: 326,
            label: "Klaire".to_string(),
        },
        FieldOption {
            value: 327,
            label: "Knight Female (Player)".to_string(),
        },
        FieldOption {
            value: 328,
            label: "Knight Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 329,
            label: "Knight Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 330,
            label: "Knight Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 331,
            label: "Knight Female (Walister)".to_string(),
        },
        FieldOption {
            value: 332,
            label: "Knight Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 333,
            label: "Knight Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 334,
            label: "Knight Female (Divine)".to_string(),
        },
        FieldOption {
            value: 335,
            label: "Knight Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 336,
            label: "Knight Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 337,
            label: "Knight Male (Player)".to_string(),
        },
        FieldOption {
            value: 338,
            label: "Knight Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 339,
            label: "Knight Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 340,
            label: "Knight Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 341,
            label: "Knight Male (Walister)".to_string(),
        },
        FieldOption {
            value: 342,
            label: "Knight Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 343,
            label: "Knight Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 344,
            label: "Knight Male (Divine)".to_string(),
        },
        FieldOption {
            value: 345,
            label: "Knight Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 346,
            label: "Knight Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 347,
            label: "Lamia (Player)".to_string(),
        },
        FieldOption {
            value: 348,
            label: "Lamia (Bakram)".to_string(),
        },
        FieldOption {
            value: 349,
            label: "Lamia (Galgastan)".to_string(),
        },
        FieldOption {
            value: 350,
            label: "Lamia (Pirates)".to_string(),
        },
        FieldOption {
            value: 351,
            label: "Lamia (Walister)".to_string(),
        },
        FieldOption {
            value: 352,
            label: "Lamia (Outlaw)".to_string(),
        },
        FieldOption {
            value: 353,
            label: "Lamia (Unknown)".to_string(),
        },
        FieldOption {
            value: 354,
            label: "Lamia (Divine)".to_string(),
        },
        FieldOption {
            value: 355,
            label: "Lamia (Bestial)".to_string(),
        },
        FieldOption {
            value: 356,
            label: "Lamia (Umbral)".to_string(),
        },
        FieldOption {
            value: 357,
            label: "Lanselot Hamilton".to_string(),
        },
        FieldOption {
            value: 358,
            label: "Lanselot Hamilton Recovering".to_string(),
        },
        FieldOption {
            value: 359,
            label: "Lanselot Hamilton Injured".to_string(),
        },
        FieldOption {
            value: 360,
            label: "Lanselot Tartaros".to_string(),
        },
        FieldOption {
            value: 361,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 362,
            label: "Leonar".to_string(),
        },
        FieldOption {
            value: 363,
            label: "Lich Female (Player)".to_string(),
        },
        FieldOption {
            value: 364,
            label: "Lich Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 365,
            label: "Lich Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 366,
            label: "Lich Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 367,
            label: "Lich Female (Walister)".to_string(),
        },
        FieldOption {
            value: 368,
            label: "Lich Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 369,
            label: "Lich Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 370,
            label: "Lich Female (Divine)".to_string(),
        },
        FieldOption {
            value: 371,
            label: "Lich Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 372,
            label: "Lich Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 373,
            label: "Lich Male (Player)".to_string(),
        },
        FieldOption {
            value: 374,
            label: "Lich Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 375,
            label: "Lich Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 376,
            label: "Lich Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 377,
            label: "Lich Male (Walister)".to_string(),
        },
        FieldOption {
            value: 378,
            label: "Lich Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 379,
            label: "Lich Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 380,
            label: "Lich Male (Divine)".to_string(),
        },
        FieldOption {
            value: 381,
            label: "Lich Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 382,
            label: "Lich Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 383,
            label: "Lindl".to_string(),
        },
        FieldOption {
            value: 384,
            label: "Lizardman (Player)".to_string(),
        },
        FieldOption {
            value: 385,
            label: "Lizardman (Bakram)".to_string(),
        },
        FieldOption {
            value: 386,
            label: "Lizardman (Galgastan)".to_string(),
        },
        FieldOption {
            value: 387,
            label: "Lizardman (Pirates)".to_string(),
        },
        FieldOption {
            value: 388,
            label: "Lizardman (Walister)".to_string(),
        },
        FieldOption {
            value: 389,
            label: "Lizardman (Outlaw)".to_string(),
        },
        FieldOption {
            value: 390,
            label: "Lizardman (Unknown)".to_string(),
        },
        FieldOption {
            value: 391,
            label: "Lizardman (Divine)".to_string(),
        },
        FieldOption {
            value: 392,
            label: "Lizardman (Bestial)".to_string(),
        },
        FieldOption {
            value: 393,
            label: "Lizardman (Umbral)".to_string(),
        },
        FieldOption {
            value: 394,
            label: "Spirit Rackham".to_string(),
        },
        FieldOption {
            value: 395,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 396,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 397,
            label: "Mannaflora".to_string(),
        },
        FieldOption {
            value: 398,
            label: "Martym".to_string(),
        },
        FieldOption {
            value: 399,
            label: "Mirdyn".to_string(),
        },
        FieldOption {
            value: 400,
            label: "Mreuva".to_string(),
        },
        FieldOption {
            value: 401,
            label: "Necromancer Female (Player)".to_string(),
        },
        FieldOption {
            value: 402,
            label: "Necromancer Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 403,
            label: "Necromancer Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 404,
            label: "Necromancer Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 405,
            label: "Necromancer Female (Walister)".to_string(),
        },
        FieldOption {
            value: 406,
            label: "Necromancer Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 407,
            label: "Necromancer Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 408,
            label: "Necromancer Female (Divine)".to_string(),
        },
        FieldOption {
            value: 409,
            label: "Necromancer Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 410,
            label: "Necromancer Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 411,
            label: "Necromancer Male (Player)".to_string(),
        },
        FieldOption {
            value: 412,
            label: "Necromancer Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 413,
            label: "Necromancer Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 414,
            label: "Necromancer Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 415,
            label: "Necromancer Male (Walister)".to_string(),
        },
        FieldOption {
            value: 416,
            label: "Necromancer Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 417,
            label: "Necromancer Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 418,
            label: "Necromancer Male (Divine)".to_string(),
        },
        FieldOption {
            value: 419,
            label: "Necromancer Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 420,
            label: "Necromancer Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 421,
            label: "Kunoichi (Player)".to_string(),
        },
        FieldOption {
            value: 422,
            label: "Kunoichi (Bakram)".to_string(),
        },
        FieldOption {
            value: 423,
            label: "Kunoichi (Galgastan)".to_string(),
        },
        FieldOption {
            value: 424,
            label: "Kunoichi (Pirates)".to_string(),
        },
        FieldOption {
            value: 425,
            label: "Kunoichi (Walister)".to_string(),
        },
        FieldOption {
            value: 426,
            label: "Kunoichi (Outlaw)".to_string(),
        },
        FieldOption {
            value: 427,
            label: "Kunoichi (Unknown)".to_string(),
        },
        FieldOption {
            value: 428,
            label: "Kunoichi (Divine)".to_string(),
        },
        FieldOption {
            value: 429,
            label: "Kunoichi (Bestial)".to_string(),
        },
        FieldOption {
            value: 430,
            label: "Kunoichi (Umbral)".to_string(),
        },
        FieldOption {
            value: 431,
            label: "Ninja (Player)".to_string(),
        },
        FieldOption {
            value: 432,
            label: "Ninja (Bakram)".to_string(),
        },
        FieldOption {
            value: 433,
            label: "Ninja (Galgastan)".to_string(),
        },
        FieldOption {
            value: 434,
            label: "Ninja (Pirates)".to_string(),
        },
        FieldOption {
            value: 435,
            label: "Ninja (Walister)".to_string(),
        },
        FieldOption {
            value: 436,
            label: "Ninja (Outlaw)".to_string(),
        },
        FieldOption {
            value: 437,
            label: "Ninja (Unknown)".to_string(),
        },
        FieldOption {
            value: 438,
            label: "Ninja (Divine)".to_string(),
        },
        FieldOption {
            value: 439,
            label: "Ninja (Bestial)".to_string(),
        },
        FieldOption {
            value: 440,
            label: "Ninja (Umbral)".to_string(),
        },
        FieldOption {
            value: 441,
            label: "Iuria".to_string(),
        },
        FieldOption {
            value: 442,
            label: "Nybeth".to_string(),
        },
        FieldOption {
            value: 443,
            label: "Nybeth Lich".to_string(),
        },
        FieldOption {
            value: 444,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 445,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 446,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 447,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 448,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 449,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 450,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 451,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 452,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 453,
            label: "Octopus".to_string(),
        },
        FieldOption {
            value: 454,
            label: "Olyvia".to_string(),
        },
        FieldOption {
            value: 455,
            label: "Orc (Player)".to_string(),
        },
        FieldOption {
            value: 456,
            label: "Orc (Bakram)".to_string(),
        },
        FieldOption {
            value: 457,
            label: "Orc (Galgastan)".to_string(),
        },
        FieldOption {
            value: 458,
            label: "Orc (Pirates)".to_string(),
        },
        FieldOption {
            value: 459,
            label: "Orc (Walister)".to_string(),
        },
        FieldOption {
            value: 460,
            label: "Orc (Outlaw)".to_string(),
        },
        FieldOption {
            value: 461,
            label: "Orc (Unknown)".to_string(),
        },
        FieldOption {
            value: 462,
            label: "Orc (Divine)".to_string(),
        },
        FieldOption {
            value: 463,
            label: "Orc (Bestial)".to_string(),
        },
        FieldOption {
            value: 464,
            label: "Orc (Umbral)".to_string(),
        },
        FieldOption {
            value: 465,
            label: "Oelias".to_string(),
        },
        FieldOption {
            value: 466,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 467,
            label: "Ocionne".to_string(),
        },
        FieldOption {
            value: 468,
            label: "Oz".to_string(),
        },
        FieldOption {
            value: 469,
            label: "Ozma".to_string(),
        },
        FieldOption {
            value: 470,
            label: "Ozma sad".to_string(),
        },
        FieldOption {
            value: 471,
            label: "Prancet".to_string(),
        },
        FieldOption {
            value: 472,
            label: "Prancet dying".to_string(),
        },
        FieldOption {
            value: 473,
            label: "Donnalto".to_string(),
        },
        FieldOption {
            value: 474,
            label: "Pumpkin (Player)".to_string(),
        },
        FieldOption {
            value: 475,
            label: "Pumpkin (Bakram)".to_string(),
        },
        FieldOption {
            value: 476,
            label: "Pumpkin (Galgastan)".to_string(),
        },
        FieldOption {
            value: 477,
            label: "Pumpkin (Pirates)".to_string(),
        },
        FieldOption {
            value: 478,
            label: "Pumpkin (Walister)".to_string(),
        },
        FieldOption {
            value: 479,
            label: "Pumpkin (Outlaw)".to_string(),
        },
        FieldOption {
            value: 480,
            label: "Pumpkin (Unknown)".to_string(),
        },
        FieldOption {
            value: 481,
            label: "Pumpkin (Pirates)".to_string(),
        },
        FieldOption {
            value: 482,
            label: "Pumpkin (Divine)".to_string(),
        },
        FieldOption {
            value: 483,
            label: "Pumpkin (Bestial)".to_string(),
        },
        FieldOption {
            value: 484,
            label: "Pumpkin (Umbral)".to_string(),
        },
        FieldOption {
            value: 485,
            label: "Rudlum".to_string(),
        },
        FieldOption {
            value: 486,
            label: "Ravness".to_string(),
        },
        FieldOption {
            value: 487,
            label: "-nothing-".to_string(),
        },
        FieldOption {
            value: 488,
            label: "Umbral Rodrick".to_string(),
        },
        FieldOption {
            value: 489,
            label: "Rodrick".to_string(),
        },
        FieldOption {
            value: 490,
            label: "Ronwey".to_string(),
        },
        FieldOption {
            value: 491,
            label: "Rogue Female (Player)".to_string(),
        },
        FieldOption {
            value: 492,
            label: "Rogue Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 493,
            label: "Rogue Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 494,
            label: "Rogue Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 495,
            label: "Rogue Female (Walister)".to_string(),
        },
        FieldOption {
            value: 496,
            label: "Rogue Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 497,
            label: "Rogue Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 498,
            label: "Rogue Female (Divine)".to_string(),
        },
        FieldOption {
            value: 499,
            label: "Rogue Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 500,
            label: "Rogue Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 501,
            label: "Rogue Male (Player)".to_string(),
        },
        FieldOption {
            value: 502,
            label: "Rogue Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 503,
            label: "Rogue Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 504,
            label: "Rogue Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 505,
            label: "Rogue Male (Walister)".to_string(),
        },
        FieldOption {
            value: 506,
            label: "Rogue Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 507,
            label: "Rogue Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 508,
            label: "Rogue Male (Divine)".to_string(),
        },
        FieldOption {
            value: 509,
            label: "Rogue Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 510,
            label: "Rogue Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 511,
            label: "Cerya".to_string(),
        },
        FieldOption {
            value: 512,
            label: "Sherri".to_string(),
        },
        FieldOption {
            value: 513,
            label: "Sherri injured".to_string(),
        },
        FieldOption {
            value: 514,
            label: "Male Villager".to_string(),
        },
        FieldOption {
            value: 515,
            label: "Cistina".to_string(),
        },
        FieldOption {
            value: 516,
            label: "Skeleton (Player)".to_string(),
        },
        FieldOption {
            value: 517,
            label: "Skeleton (Bakram)".to_string(),
        },
        FieldOption {
            value: 518,
            label: "Skeleton (Galgastan)".to_string(),
        },
        FieldOption {
            value: 519,
            label: "Skeleton (Pirates)".to_string(),
        },
        FieldOption {
            value: 520,
            label: "Skeleton (Walister)".to_string(),
        },
        FieldOption {
            value: 521,
            label: "Skeleton (Outlaw)".to_string(),
        },
        FieldOption {
            value: 522,
            label: "Skeleton (Unknown)".to_string(),
        },
        FieldOption {
            value: 523,
            label: "Skeleton (Divine)".to_string(),
        },
        FieldOption {
            value: 524,
            label: "Skeleton (Bestial)".to_string(),
        },
        FieldOption {
            value: 525,
            label: "Skeleton (Umbral)".to_string(),
        },
        FieldOption {
            value: 526,
            label: "Witch (Player)".to_string(),
        },
        FieldOption {
            value: 527,
            label: "Witch (Bakram)".to_string(),
        },
        FieldOption {
            value: 528,
            label: "Witch (Galgastan)".to_string(),
        },
        FieldOption {
            value: 529,
            label: "Witch (Pirates)".to_string(),
        },
        FieldOption {
            value: 530,
            label: "Witch (Walister)".to_string(),
        },
        FieldOption {
            value: 531,
            label: "Witch (Outlaw)".to_string(),
        },
        FieldOption {
            value: 532,
            label: "Witch (Unknown)".to_string(),
        },
        FieldOption {
            value: 533,
            label: "Witch (Divine)".to_string(),
        },
        FieldOption {
            value: 534,
            label: "Witch (Bestial)".to_string(),
        },
        FieldOption {
            value: 535,
            label: "Witch (Umbral)".to_string(),
        },
        FieldOption {
            value: 536,
            label: "Warlock (Player)".to_string(),
        },
        FieldOption {
            value: 537,
            label: "Warlock (Bakram)".to_string(),
        },
        FieldOption {
            value: 538,
            label: "Warlock (Galgastan)".to_string(),
        },
        FieldOption {
            value: 539,
            label: "Warlock (Pirates)".to_string(),
        },
        FieldOption {
            value: 540,
            label: "Warlock (Walister)".to_string(),
        },
        FieldOption {
            value: 541,
            label: "Warlock (Outlaw)".to_string(),
        },
        FieldOption {
            value: 542,
            label: "Warlock (Unknown)".to_string(),
        },
        FieldOption {
            value: 543,
            label: "Warlock (Divine)".to_string(),
        },
        FieldOption {
            value: 544,
            label: "Warlock (Bestial)".to_string(),
        },
        FieldOption {
            value: 545,
            label: "Warlock (Umbral)".to_string(),
        },
        FieldOption {
            value: 546,
            label: "Sorcerer (Modiliani 1)".to_string(),
        },
        FieldOption {
            value: 547,
            label: "Sorcerer (Modiliani 2)".to_string(),
        },
        FieldOption {
            value: 548,
            label: "Sorcerer (Brutakos 1)".to_string(),
        },
        FieldOption {
            value: 549,
            label: "Sorcerer (Brutakos 2)".to_string(),
        },
        FieldOption {
            value: 550,
            label: "Sorcerer (Unused Blue)".to_string(),
        },
        FieldOption {
            value: 551,
            label: "Sorcerer (Unused Red)".to_string(),
        },
        FieldOption {
            value: 552,
            label: "Sorcerer (Unused Purple)".to_string(),
        },
        FieldOption {
            value: 553,
            label: "Sorcerer (Divine)".to_string(),
        },
        FieldOption {
            value: 554,
            label: "Sorcerer (Bestial)".to_string(),
        },
        FieldOption {
            value: 555,
            label: "Sorcerer (Umbral)".to_string(),
        },
        FieldOption {
            value: 556,
            label: "Sorceress (Ramidos 1)".to_string(),
        },
        FieldOption {
            value: 557,
            label: "Sorceress (Unused Dark Red)".to_string(),
        },
        FieldOption {
            value: 558,
            label: "Sorceress (Unused Dark Blue)".to_string(),
        },
        FieldOption {
            value: 559,
            label: "Sorceress (Uused Dark Purple)".to_string(),
        },
        FieldOption {
            value: 560,
            label: "Sorceress (Unused Blue)".to_string(),
        },
        FieldOption {
            value: 561,
            label: "Sorceress (Unused Red)".to_string(),
        },
        FieldOption {
            value: 562,
            label: "Sorceress (Unused Purple)".to_string(),
        },
        FieldOption {
            value: 563,
            label: "Sorceress (Divine)".to_string(),
        },
        FieldOption {
            value: 564,
            label: "Sorceress (Bestial)".to_string(),
        },
        FieldOption {
            value: 565,
            label: "Sorceress (Umbral)".to_string(),
        },
        FieldOption {
            value: 566,
            label: "Swordmaster Female (Player)".to_string(),
        },
        FieldOption {
            value: 567,
            label: "Swordmaster Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 568,
            label: "Swordmaster Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 569,
            label: "Swordmaster Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 570,
            label: "Swordmaster Female (Walister)".to_string(),
        },
        FieldOption {
            value: 571,
            label: "Swordmaster Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 572,
            label: "Swordmaster Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 573,
            label: "Swordmaster Female (Divine)".to_string(),
        },
        FieldOption {
            value: 574,
            label: "Swordmaster Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 575,
            label: "Swordmaster Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 576,
            label: "Swordmaster Male (Player)".to_string(),
        },
        FieldOption {
            value: 577,
            label: "Swordmaster Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 578,
            label: "Swordmaster Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 579,
            label: "Swordmaster Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 580,
            label: "Swordmaster Male (Walister)".to_string(),
        },
        FieldOption {
            value: 581,
            label: "Swordmaster Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 582,
            label: "Swordmaster Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 583,
            label: "Swordmaster Male (Divine)".to_string(),
        },
        FieldOption {
            value: 584,
            label: "Swordmaster Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 585,
            label: "Swordmaster Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 586,
            label: "Terror Knight Female (Player)".to_string(),
        },
        FieldOption {
            value: 587,
            label: "Terror Knight Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 588,
            label: "Terror Knight Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 589,
            label: "Terror Knight Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 590,
            label: "Terror Knight Female (Walister)".to_string(),
        },
        FieldOption {
            value: 591,
            label: "Terror Knight Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 592,
            label: "Terror Knight Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 593,
            label: "Terror Knight Female (Divine)".to_string(),
        },
        FieldOption {
            value: 594,
            label: "Terror Knight Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 595,
            label: "Terror Knight Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 596,
            label: "Terror Knight Male (Player)".to_string(),
        },
        FieldOption {
            value: 597,
            label: "Terror Knight Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 598,
            label: "Terror Knight Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 599,
            label: "Terror Knight Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 600,
            label: "Terror Knight Male (Walister)".to_string(),
        },
        FieldOption {
            value: 601,
            label: "Terror Knight Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 602,
            label: "Terror Knight Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 603,
            label: "Terror Knight Male (Divine)".to_string(),
        },
        FieldOption {
            value: 604,
            label: "Terror Knight Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 605,
            label: "Terror Knight Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 606,
            label: "Gatialo (Old)".to_string(),
        },
        FieldOption {
            value: 607,
            label: "King Tristan".to_string(),
        },
        FieldOption {
            value: 608,
            label: "Valkyrie (Player)".to_string(),
        },
        FieldOption {
            value: 609,
            label: "Valkyrie (Bakram)".to_string(),
        },
        FieldOption {
            value: 610,
            label: "Valkyrie (Galgastan)".to_string(),
        },
        FieldOption {
            value: 611,
            label: "Valkyrie (Pirates)".to_string(),
        },
        FieldOption {
            value: 612,
            label: "Valkyrie (Walister)".to_string(),
        },
        FieldOption {
            value: 613,
            label: "Valkyrie (Outlaw)".to_string(),
        },
        FieldOption {
            value: 614,
            label: "Valkyrie (Unknown)".to_string(),
        },
        FieldOption {
            value: 615,
            label: "Valkyrie (Divine)".to_string(),
        },
        FieldOption {
            value: 616,
            label: "Valkyrie (Bestial)".to_string(),
        },
        FieldOption {
            value: 617,
            label: "Valkyrie (Umbral)".to_string(),
        },
        FieldOption {
            value: 618,
            label: "Rune Fencer (Player)".to_string(),
        },
        FieldOption {
            value: 619,
            label: "Rune Fencer (Bakram)".to_string(),
        },
        FieldOption {
            value: 620,
            label: "Rune Fencer (Galgastan)".to_string(),
        },
        FieldOption {
            value: 621,
            label: "Rune Fencer (Pirates)".to_string(),
        },
        FieldOption {
            value: 622,
            label: "Rune Fencer (Walister)".to_string(),
        },
        FieldOption {
            value: 623,
            label: "Rune Fencer (Outlaw)".to_string(),
        },
        FieldOption {
            value: 624,
            label: "Rune Fencer (Unknown)".to_string(),
        },
        FieldOption {
            value: 625,
            label: "Rune Fencer (Divine)".to_string(),
        },
        FieldOption {
            value: 626,
            label: "Rune Fencer (Bestial)".to_string(),
        },
        FieldOption {
            value: 627,
            label: "Rune Fencer (Umbral)".to_string(),
        },
        FieldOption {
            value: 628,
            label: "Vepahl".to_string(),
        },
        FieldOption {
            value: 629,
            label: "Queen Vernotta".to_string(),
        },
        FieldOption {
            value: 630,
            label: "Vyce Chapter".to_string(),
        },
        FieldOption {
            value: 631,
            label: "Vyce Chaos yelling".to_string(),
        },
        FieldOption {
            value: 632,
            label: "Vyce's Father".to_string(),
        },
        FieldOption {
            value: 633,
            label: "Vyce Chaos injured".to_string(),
        },
        FieldOption {
            value: 634,
            label: "Vyce Law".to_string(),
        },
        FieldOption {
            value: 635,
            label: "Vyce Chaos".to_string(),
        },
        FieldOption {
            value: 636,
            label: "Vyce Chaos Smile".to_string(),
        },
        FieldOption {
            value: 637,
            label: "Vogras".to_string(),
        },
        FieldOption {
            value: 638,
            label: "Volaq".to_string(),
        },
        FieldOption {
            value: 639,
            label: "Warren".to_string(),
        },
        FieldOption {
            value: 640,
            label: "Warrior Female (Player)".to_string(),
        },
        FieldOption {
            value: 641,
            label: "Warrior Female (Bakram)".to_string(),
        },
        FieldOption {
            value: 642,
            label: "Warrior Female (Galgastan)".to_string(),
        },
        FieldOption {
            value: 643,
            label: "Warrior Female (Pirates)".to_string(),
        },
        FieldOption {
            value: 644,
            label: "Warrior Female (Walister)".to_string(),
        },
        FieldOption {
            value: 645,
            label: "Warrior Female (Outlaw)".to_string(),
        },
        FieldOption {
            value: 646,
            label: "Warrior Female (Unknown)".to_string(),
        },
        FieldOption {
            value: 647,
            label: "Warrior Female (Divine)".to_string(),
        },
        FieldOption {
            value: 648,
            label: "Warrior Female (Bestial)".to_string(),
        },
        FieldOption {
            value: 649,
            label: "Warrior Female (Umbral)".to_string(),
        },
        FieldOption {
            value: 650,
            label: "Warrior Male (Player)".to_string(),
        },
        FieldOption {
            value: 651,
            label: "Warrior Male (Bakram)".to_string(),
        },
        FieldOption {
            value: 652,
            label: "Warrior Male (Galgastan)".to_string(),
        },
        FieldOption {
            value: 653,
            label: "Warrior Male (Pirates)".to_string(),
        },
        FieldOption {
            value: 654,
            label: "Warrior Male (Walister)".to_string(),
        },
        FieldOption {
            value: 655,
            label: "Warrior Male (Outlaw)".to_string(),
        },
        FieldOption {
            value: 656,
            label: "Warrior Male (Unknown)".to_string(),
        },
        FieldOption {
            value: 657,
            label: "Warrior Male (Divine)".to_string(),
        },
        FieldOption {
            value: 658,
            label: "Warrior Male (Bestial)".to_string(),
        },
        FieldOption {
            value: 659,
            label: "Warrior Male (Umbral)".to_string(),
        },
        FieldOption {
            value: 660,
            label: "Enchantress (Player)".to_string(),
        },
        FieldOption {
            value: 661,
            label: "Enchantress (Bakram)".to_string(),
        },
        FieldOption {
            value: 662,
            label: "Enchantress (Galgastan)".to_string(),
        },
        FieldOption {
            value: 663,
            label: "Enchantress (Pirates)".to_string(),
        },
        FieldOption {
            value: 664,
            label: "Enchantress (Walister)".to_string(),
        },
        FieldOption {
            value: 665,
            label: "Enchantress (Outlaw)".to_string(),
        },
        FieldOption {
            value: 666,
            label: "Enchantress (Unknown)".to_string(),
        },
        FieldOption {
            value: 667,
            label: "Enchantress (Divine)".to_string(),
        },
        FieldOption {
            value: 668,
            label: "Enchantress (Bestial)".to_string(),
        },
        FieldOption {
            value: 669,
            label: "Enchantress (Umbral)".to_string(),
        },
        FieldOption {
            value: 670,
            label: "Wizard (Player)".to_string(),
        },
        FieldOption {
            value: 671,
            label: "Wizard (Bakram)".to_string(),
        },
        FieldOption {
            value: 672,
            label: "Wizard (Galgastan)".to_string(),
        },
        FieldOption {
            value: 673,
            label: "Wizard (Pirates)".to_string(),
        },
        FieldOption {
            value: 674,
            label: "Wizard (Walister)".to_string(),
        },
        FieldOption {
            value: 675,
            label: "Wizard (Outlaw)".to_string(),
        },
        FieldOption {
            value: 676,
            label: "Wizard (Unknown)".to_string(),
        },
        FieldOption {
            value: 677,
            label: "Wizard (Divine)".to_string(),
        },
        FieldOption {
            value: 678,
            label: "Wizard (Bestial)".to_string(),
        },
        FieldOption {
            value: 679,
            label: "Wizard (Umbral)".to_string(),
        },
        FieldOption {
            value: 680,
            label: "Xapan Alternate Palette".to_string(),
        },
        FieldOption {
            value: 681,
            label: "Vyce Zombie".to_string(),
        },
        FieldOption {
            value: 682,
            label: "Hektor Zombie".to_string(),
        },
        FieldOption {
            value: 683,
            label: "Cassandra Zombie".to_string(),
        },
        FieldOption {
            value: 684,
            label: "Gildas Zombie".to_string(),
        },
        FieldOption {
            value: 685,
            label: "Leonar Zombie".to_string(),
        },
        FieldOption {
            value: 686,
            label: "Ravenman".to_string(),
        },
        FieldOption {
            value: 687,
            label: "Moldova Zombie".to_string(),
        },
        FieldOption {
            value: 688,
            label: "Vyce Zombie".to_string(),
        },
        FieldOption {
            value: 689,
            label: "Hektor Zombie".to_string(),
        },
        FieldOption {
            value: 690,
            label: "Gildas Zombie".to_string(),
        },
        FieldOption {
            value: 691,
            label: "Leonar Zombie".to_string(),
        },
        FieldOption {
            value: 692,
            label: "Villager Girl".to_string(),
        },
        FieldOption {
            value: 693,
            label: "Villager Man".to_string(),
        },
        FieldOption {
            value: 694,
            label: "Iuria".to_string(),
        },
        FieldOption {
            value: 695,
            label: "Sirene".to_string(),
        },
        FieldOption {
            value: 696,
            label: "Xaebos Zombie".to_string(),
        },
        FieldOption {
            value: 697,
            label: "Xaebos Smiling".to_string(),
        },
        FieldOption {
            value: 698,
            label: "Xaebos Frowning".to_string(),
        },
        FieldOption {
            value: 699,
            label: "Xapan".to_string(),
        },
    ]
}

fn opt_move_type_2() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Class Dependant".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Agile I".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Agile II".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Slow".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Blinkwalk".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Windwalk".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Cloudwalk".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Waterwalk".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Lavawalk".to_string(),
        },
    ]
}

fn opt_equipment_list() -> Vec<FieldOption> {
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
            label: "Punch".to_string(),
        },
        FieldOption {
            value: 378,
            label: "Slap".to_string(),
        },
        FieldOption {
            value: 379,
            label: "Rabbit Punch".to_string(),
        },
        FieldOption {
            value: 380,
            label: "Bite".to_string(),
        },
        FieldOption {
            value: 381,
            label: "Haymaker".to_string(),
        },
        FieldOption {
            value: 382,
            label: "Rake".to_string(),
        },
        FieldOption {
            value: 383,
            label: "Flog".to_string(),
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
            label: "Cast Stone".to_string(),
        },
        FieldOption {
            value: 388,
            label: "Shuriken".to_string(),
        },
        FieldOption {
            value: 389,
            label: "Sling Stone".to_string(),
        },
        FieldOption {
            value: 390,
            label: "Rail Against".to_string(),
        },
        FieldOption {
            value: 391,
            label: "Boulder Blow".to_string(),
        },
        FieldOption {
            value: 392,
            label: "Boulder Toss".to_string(),
        },
        FieldOption {
            value: 393,
            label: "Boss Punch".to_string(),
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
            label: "Rose Whip (Boss 3)".to_string(),
        },
        FieldOption {
            value: 684,
            label: "Cast Stone (Aym)".to_string(),
        },
        FieldOption {
            value: 685,
            label: "Shortbow (Aloser 2)".to_string(),
        },
        FieldOption {
            value: 686,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 720,
            label: "None".to_string(),
        },
        FieldOption {
            value: 721,
            label: "None".to_string(),
        },
        FieldOption {
            value: 722,
            label: "None".to_string(),
        },
        FieldOption {
            value: 723,
            label: "None".to_string(),
        },
        FieldOption {
            value: 724,
            label: "None".to_string(),
        },
        FieldOption {
            value: 725,
            label: "None".to_string(),
        },
        FieldOption {
            value: 726,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 748,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 760,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 764,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 768,
            label: "Mend Leaf".to_string(),
        },
        FieldOption {
            value: 769,
            label: "Mend Leaf +1".to_string(),
        },
        FieldOption {
            value: 770,
            label: "Mend Leaf +2".to_string(),
        },
        FieldOption {
            value: 771,
            label: "None".to_string(),
        },
        FieldOption {
            value: 772,
            label: "Mending Seed".to_string(),
        },
        FieldOption {
            value: 773,
            label: "Mending Seed +1".to_string(),
        },
        FieldOption {
            value: 774,
            label: "None".to_string(),
        },
        FieldOption {
            value: 775,
            label: "None".to_string(),
        },
        FieldOption {
            value: 776,
            label: "Mending Salve".to_string(),
        },
        FieldOption {
            value: 777,
            label: "Mending Salve +1".to_string(),
        },
        FieldOption {
            value: 778,
            label: "None".to_string(),
        },
        FieldOption {
            value: 779,
            label: "Mending Essence".to_string(),
        },
        FieldOption {
            value: 780,
            label: "Magic Leaf".to_string(),
        },
        FieldOption {
            value: 781,
            label: "Magic Leaf +1".to_string(),
        },
        FieldOption {
            value: 782,
            label: "Magic Leaf +2".to_string(),
        },
        FieldOption {
            value: 783,
            label: "None".to_string(),
        },
        FieldOption {
            value: 784,
            label: "Magic Seed".to_string(),
        },
        FieldOption {
            value: 785,
            label: "Magic Seed +1".to_string(),
        },
        FieldOption {
            value: 786,
            label: "None".to_string(),
        },
        FieldOption {
            value: 787,
            label: "None".to_string(),
        },
        FieldOption {
            value: 788,
            label: "Magic Salve".to_string(),
        },
        FieldOption {
            value: 789,
            label: "Magic Salve +1".to_string(),
        },
        FieldOption {
            value: 790,
            label: "None".to_string(),
        },
        FieldOption {
            value: 791,
            label: "Magic Essence".to_string(),
        },
        FieldOption {
            value: 792,
            label: "Fruit of the Adept".to_string(),
        },
        FieldOption {
            value: 793,
            label: "Fruit of the Adept +1".to_string(),
        },
        FieldOption {
            value: 794,
            label: "Fruit of the Sage".to_string(),
        },
        FieldOption {
            value: 795,
            label: "Fruit of the Sage +1".to_string(),
        },
        FieldOption {
            value: 796,
            label: "Fruit of the Seraph".to_string(),
        },
        FieldOption {
            value: 797,
            label: "Overripe Fruit".to_string(),
        },
        FieldOption {
            value: 798,
            label: "Zolia Draught".to_string(),
        },
        FieldOption {
            value: 799,
            label: "None".to_string(),
        },
        FieldOption {
            value: 800,
            label: "Zena Wine".to_string(),
        },
        FieldOption {
            value: 801,
            label: "Illumina Nectar".to_string(),
        },
        FieldOption {
            value: 802,
            label: "Gerun Powder".to_string(),
        },
        FieldOption {
            value: 803,
            label: "Feyrn Bolus".to_string(),
        },
        FieldOption {
            value: 804,
            label: "Maca Antidote".to_string(),
        },
        FieldOption {
            value: 805,
            label: "None".to_string(),
        },
        FieldOption {
            value: 806,
            label: "Jaarn's Poultice".to_string(),
        },
        FieldOption {
            value: 807,
            label: "Ishtar's Ambrosia".to_string(),
        },
        FieldOption {
            value: 808,
            label: "Ashmedai's Grog".to_string(),
        },
        FieldOption {
            value: 809,
            label: "Blessing Stone".to_string(),
        },
        FieldOption {
            value: 810,
            label: "Hallowing Stone".to_string(),
        },
        FieldOption {
            value: 811,
            label: "Areion Plume".to_string(),
        },
        FieldOption {
            value: 812,
            label: "Basin of Time".to_string(),
        },
        FieldOption {
            value: 813,
            label: "Spiritstone of the Stars".to_string(),
        },
        FieldOption {
            value: 814,
            label: "Faeriescale Powder".to_string(),
        },
        FieldOption {
            value: 815,
            label: "Crystallized Flame".to_string(),
        },
        FieldOption {
            value: 816,
            label: "Mercurial Phial".to_string(),
        },
        FieldOption {
            value: 817,
            label: "Jewel of the Avatar".to_string(),
        },
        FieldOption {
            value: 818,
            label: "Hair of the Unicorn".to_string(),
        },
        FieldOption {
            value: 819,
            label: "Philtre of Ashes".to_string(),
        },
        FieldOption {
            value: 820,
            label: "Black Lizard Powder".to_string(),
        },
        FieldOption {
            value: 821,
            label: "None".to_string(),
        },
        FieldOption {
            value: 822,
            label: "Dragon Steak".to_string(),
        },
        FieldOption {
            value: 823,
            label: "Braised Skewer".to_string(),
        },
        FieldOption {
            value: 824,
            label: "Steamed Mollusk".to_string(),
        },
        FieldOption {
            value: 825,
            label: "Minced Patty".to_string(),
        },
        FieldOption {
            value: 826,
            label: "None".to_string(),
        },
        FieldOption {
            value: 827,
            label: "None".to_string(),
        },
        FieldOption {
            value: 828,
            label: "None".to_string(),
        },
        FieldOption {
            value: 829,
            label: "None".to_string(),
        },
        FieldOption {
            value: 830,
            label: "None".to_string(),
        },
        FieldOption {
            value: 831,
            label: "Brand of the Sacrifice".to_string(),
        },
        FieldOption {
            value: 832,
            label: "Dynast-King's Mead".to_string(),
        },
        FieldOption {
            value: 833,
            label: "Echo Stone".to_string(),
        },
        FieldOption {
            value: 834,
            label: "Blackwing Leg".to_string(),
        },
        FieldOption {
            value: 835,
            label: "Rood Upright".to_string(),
        },
        FieldOption {
            value: 836,
            label: "Haunt's Tome".to_string(),
        },
        FieldOption {
            value: 837,
            label: "Darkscale Tome".to_string(),
        },
        FieldOption {
            value: 838,
            label: "Cursed Unicorn Blood".to_string(),
        },
        FieldOption {
            value: 839,
            label: "Skulldust Nostrum".to_string(),
        },
        FieldOption {
            value: 840,
            label: "Magedrain Gland".to_string(),
        },
        FieldOption {
            value: 841,
            label: "Shiftstone".to_string(),
        },
        FieldOption {
            value: 842,
            label: "Horn of the Savage".to_string(),
        },
        FieldOption {
            value: 843,
            label: "Coral Harp".to_string(),
        },
        FieldOption {
            value: 844,
            label: "Whirlwind Shot".to_string(),
        },
        FieldOption {
            value: 845,
            label: "Duststorm Shot".to_string(),
        },
        FieldOption {
            value: 846,
            label: "Thunder Shot".to_string(),
        },
        FieldOption {
            value: 847,
            label: "Torrent Shot".to_string(),
        },
        FieldOption {
            value: 848,
            label: "Conflagration Shot".to_string(),
        },
        FieldOption {
            value: 849,
            label: "Firnice Shot".to_string(),
        },
        FieldOption {
            value: 850,
            label: "Coruscate Shot".to_string(),
        },
        FieldOption {
            value: 851,
            label: "Murk Shot".to_string(),
        },
        FieldOption {
            value: 852,
            label: "Book of the Dead".to_string(),
        },
        FieldOption {
            value: 853,
            label: "Ring of the Dead".to_string(),
        },
        FieldOption {
            value: 854,
            label: "Ensanguined Rood".to_string(),
        },
        FieldOption {
            value: 855,
            label: "Seal of Rebirth".to_string(),
        },
        FieldOption {
            value: 856,
            label: "Void Orb".to_string(),
        },
        FieldOption {
            value: 857,
            label: "Gale Orb".to_string(),
        },
        FieldOption {
            value: 858,
            label: "Dust Orb".to_string(),
        },
        FieldOption {
            value: 859,
            label: "Storm Orb".to_string(),
        },
        FieldOption {
            value: 860,
            label: "Cataract Orb".to_string(),
        },
        FieldOption {
            value: 861,
            label: "Inferno Orb".to_string(),
        },
        FieldOption {
            value: 862,
            label: "Black Ice Orb".to_string(),
        },
        FieldOption {
            value: 863,
            label: "Radiant Orb".to_string(),
        },
        FieldOption {
            value: 864,
            label: "Gloom Orb".to_string(),
        },
        FieldOption {
            value: 865,
            label: "Elixir".to_string(),
        },
        FieldOption {
            value: 866,
            label: "Charm of Remission".to_string(),
        },
        FieldOption {
            value: 867,
            label: "None".to_string(),
        },
        FieldOption {
            value: 868,
            label: "None".to_string(),
        },
        FieldOption {
            value: 869,
            label: "Intelligence Card".to_string(),
        },
        FieldOption {
            value: 870,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 871,
            label: "MP Card".to_string(),
        },
        FieldOption {
            value: 872,
            label: "HP Card".to_string(),
        },
        FieldOption {
            value: 873,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 874,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 875,
            label: "Strength Card".to_string(),
        },
        FieldOption {
            value: 876,
            label: "Strength Card (2)".to_string(),
        },
        FieldOption {
            value: 877,
            label: "Intelligence Card (2)".to_string(),
        },
        FieldOption {
            value: 878,
            label: "Avoidance Card".to_string(),
        },
        FieldOption {
            value: 879,
            label: "Avoidance Card (2)".to_string(),
        },
        FieldOption {
            value: 880,
            label: "Vitality Card".to_string(),
        },
        FieldOption {
            value: 881,
            label: "Luck Card".to_string(),
        },
        FieldOption {
            value: 882,
            label: "Resistance Card".to_string(),
        },
        FieldOption {
            value: 883,
            label: "Luck Card (2)".to_string(),
        },
        FieldOption {
            value: 884,
            label: "Vitality Card (2)".to_string(),
        },
        FieldOption {
            value: 885,
            label: "Dexterity Card".to_string(),
        },
        FieldOption {
            value: 886,
            label: "Agility Card".to_string(),
        },
        FieldOption {
            value: 887,
            label: "Agility Card (2)".to_string(),
        },
        FieldOption {
            value: 888,
            label: "Dexterity Card (2)".to_string(),
        },
        FieldOption {
            value: 889,
            label: "Resistance Card (2)".to_string(),
        },
        FieldOption {
            value: 890,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 891,
            label: "None".to_string(),
        },
        FieldOption {
            value: 892,
            label: "None".to_string(),
        },
        FieldOption {
            value: 893,
            label: "None".to_string(),
        },
        FieldOption {
            value: 894,
            label: "None".to_string(),
        },
        FieldOption {
            value: 895,
            label: "None".to_string(),
        },
        FieldOption {
            value: 896,
            label: "None".to_string(),
        },
        FieldOption {
            value: 897,
            label: "None".to_string(),
        },
        FieldOption {
            value: 898,
            label: "None".to_string(),
        },
        FieldOption {
            value: 899,
            label: "None".to_string(),
        },
        FieldOption {
            value: 900,
            label: "None".to_string(),
        },
        FieldOption {
            value: 901,
            label: "None".to_string(),
        },
        FieldOption {
            value: 902,
            label: "None".to_string(),
        },
        FieldOption {
            value: 903,
            label: "None".to_string(),
        },
        FieldOption {
            value: 904,
            label: "None".to_string(),
        },
        FieldOption {
            value: 905,
            label: "None".to_string(),
        },
        FieldOption {
            value: 906,
            label: "None".to_string(),
        },
        FieldOption {
            value: 907,
            label: "None".to_string(),
        },
        FieldOption {
            value: 908,
            label: "None".to_string(),
        },
        FieldOption {
            value: 909,
            label: "None".to_string(),
        },
        FieldOption {
            value: 910,
            label: "None".to_string(),
        },
        FieldOption {
            value: 911,
            label: "None".to_string(),
        },
        FieldOption {
            value: 912,
            label: "None".to_string(),
        },
        FieldOption {
            value: 913,
            label: "Copper Oberynth".to_string(),
        },
        FieldOption {
            value: 914,
            label: "Bronze Oberynth".to_string(),
        },
        FieldOption {
            value: 915,
            label: "Silver Oberynth".to_string(),
        },
        FieldOption {
            value: 916,
            label: "Gold Oberynth".to_string(),
        },
        FieldOption {
            value: 917,
            label: "Platinum Oberynth".to_string(),
        },
        FieldOption {
            value: 918,
            label: "None".to_string(),
        },
        FieldOption {
            value: 919,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 920,
            label: "Deadshot II".to_string(),
        },
        FieldOption {
            value: 921,
            label: "Deadshot III".to_string(),
        },
        FieldOption {
            value: 922,
            label: "Deadshot IV".to_string(),
        },
        FieldOption {
            value: 923,
            label: "Tornado".to_string(),
        },
        FieldOption {
            value: 924,
            label: "Tornado II".to_string(),
        },
        FieldOption {
            value: 925,
            label: "Tornado III".to_string(),
        },
        FieldOption {
            value: 926,
            label: "Tornado IV".to_string(),
        },
        FieldOption {
            value: 927,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 928,
            label: "Sylphide II".to_string(),
        },
        FieldOption {
            value: 929,
            label: "Aeroflux".to_string(),
        },
        FieldOption {
            value: 930,
            label: "Aeroflux II".to_string(),
        },
        FieldOption {
            value: 931,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 932,
            label: "None".to_string(),
        },
        FieldOption {
            value: 933,
            label: "None".to_string(),
        },
        FieldOption {
            value: 934,
            label: "None".to_string(),
        },
        FieldOption {
            value: 935,
            label: "None".to_string(),
        },
        FieldOption {
            value: 936,
            label: "None".to_string(),
        },
        FieldOption {
            value: 937,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 938,
            label: "Vulcan Lance II".to_string(),
        },
        FieldOption {
            value: 939,
            label: "Vulcan Lance III".to_string(),
        },
        FieldOption {
            value: 940,
            label: "Vulcan Lance IV".to_string(),
        },
        FieldOption {
            value: 941,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 942,
            label: "Cragfall II".to_string(),
        },
        FieldOption {
            value: 943,
            label: "Cragfall III".to_string(),
        },
        FieldOption {
            value: 944,
            label: "Cragfall IV".to_string(),
        },
        FieldOption {
            value: 945,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 946,
            label: "Gnome II".to_string(),
        },
        FieldOption {
            value: 947,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 948,
            label: "Earthquake II".to_string(),
        },
        FieldOption {
            value: 949,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 950,
            label: "None".to_string(),
        },
        FieldOption {
            value: 951,
            label: "None".to_string(),
        },
        FieldOption {
            value: 952,
            label: "None".to_string(),
        },
        FieldOption {
            value: 953,
            label: "None".to_string(),
        },
        FieldOption {
            value: 954,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 955,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 956,
            label: "Lightning Bow II".to_string(),
        },
        FieldOption {
            value: 957,
            label: "Lightning Bow III".to_string(),
        },
        FieldOption {
            value: 958,
            label: "Lightning Bow IV".to_string(),
        },
        FieldOption {
            value: 959,
            label: "Thunderflare".to_string(),
        },
        FieldOption {
            value: 960,
            label: "Thunderflare II".to_string(),
        },
        FieldOption {
            value: 961,
            label: "Thunderflare III".to_string(),
        },
        FieldOption {
            value: 962,
            label: "Thunderflare IV".to_string(),
        },
        FieldOption {
            value: 963,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 964,
            label: "Thunderbird II".to_string(),
        },
        FieldOption {
            value: 965,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 966,
            label: "Thunderburst II".to_string(),
        },
        FieldOption {
            value: 967,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 968,
            label: "None".to_string(),
        },
        FieldOption {
            value: 969,
            label: "None".to_string(),
        },
        FieldOption {
            value: 970,
            label: "None".to_string(),
        },
        FieldOption {
            value: 971,
            label: "None".to_string(),
        },
        FieldOption {
            value: 972,
            label: "None".to_string(),
        },
        FieldOption {
            value: 973,
            label: "Aquablast".to_string(),
        },
        FieldOption {
            value: 974,
            label: "Aquablast II".to_string(),
        },
        FieldOption {
            value: 975,
            label: "Aquablast III".to_string(),
        },
        FieldOption {
            value: 976,
            label: "Aquablast IV".to_string(),
        },
        FieldOption {
            value: 977,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 978,
            label: "Acid Rain II".to_string(),
        },
        FieldOption {
            value: 979,
            label: "Acid Rain III".to_string(),
        },
        FieldOption {
            value: 980,
            label: "Acid Rain IV".to_string(),
        },
        FieldOption {
            value: 981,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 982,
            label: "Undine II".to_string(),
        },
        FieldOption {
            value: 983,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 984,
            label: "Dread Vapor II".to_string(),
        },
        FieldOption {
            value: 985,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 986,
            label: "None".to_string(),
        },
        FieldOption {
            value: 987,
            label: "None".to_string(),
        },
        FieldOption {
            value: 988,
            label: "None".to_string(),
        },
        FieldOption {
            value: 989,
            label: "None".to_string(),
        },
        FieldOption {
            value: 990,
            label: "None".to_string(),
        },
        FieldOption {
            value: 991,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 992,
            label: "Sparksphere II".to_string(),
        },
        FieldOption {
            value: 993,
            label: "Sparksphere III".to_string(),
        },
        FieldOption {
            value: 994,
            label: "Sparksphere IV".to_string(),
        },
        FieldOption {
            value: 995,
            label: "Firestorm".to_string(),
        },
        FieldOption {
            value: 996,
            label: "Firestorm II".to_string(),
        },
        FieldOption {
            value: 997,
            label: "Firestorm III".to_string(),
        },
        FieldOption {
            value: 998,
            label: "Firestorm IV".to_string(),
        },
        FieldOption {
            value: 999,
            label: "Salamander".to_string(),
        },
        FieldOption {
            value: 1000,
            label: "Salamander II".to_string(),
        },
        FieldOption {
            value: 1001,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 1002,
            label: "Supernova II".to_string(),
        },
        FieldOption {
            value: 1003,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 1004,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1005,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 1009,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 1010,
            label: "Iceblast II".to_string(),
        },
        FieldOption {
            value: 1011,
            label: "Iceblast III".to_string(),
        },
        FieldOption {
            value: 1012,
            label: "Iceblast IV".to_string(),
        },
        FieldOption {
            value: 1013,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 1014,
            label: "Avalanche II".to_string(),
        },
        FieldOption {
            value: 1015,
            label: "Avalanche III".to_string(),
        },
        FieldOption {
            value: 1016,
            label: "Avalanche IV".to_string(),
        },
        FieldOption {
            value: 1017,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 1018,
            label: "Wendigo II".to_string(),
        },
        FieldOption {
            value: 1019,
            label: "Ice Requiem".to_string(),
        },
        FieldOption {
            value: 1020,
            label: "Ice Requiem II".to_string(),
        },
        FieldOption {
            value: 1021,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 1022,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1023,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1024,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1025,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1026,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1027,
            label: "Spiritsurge".to_string(),
        },
        FieldOption {
            value: 1028,
            label: "Spiritsurge II".to_string(),
        },
        FieldOption {
            value: 1029,
            label: "Spiritsurge III".to_string(),
        },
        FieldOption {
            value: 1030,
            label: "Spiritsurge IV".to_string(),
        },
        FieldOption {
            value: 1031,
            label: "Judgement".to_string(),
        },
        FieldOption {
            value: 1032,
            label: "Judgement II".to_string(),
        },
        FieldOption {
            value: 1033,
            label: "Judgement III".to_string(),
        },
        FieldOption {
            value: 1034,
            label: "Judgement IV".to_string(),
        },
        FieldOption {
            value: 1035,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 1036,
            label: "Wisplight II".to_string(),
        },
        FieldOption {
            value: 1037,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 1038,
            label: "Heavenly Judge II".to_string(),
        },
        FieldOption {
            value: 1039,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 1040,
            label: "Exorcism II".to_string(),
        },
        FieldOption {
            value: 1041,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 1042,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1043,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1044,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 1045,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 1046,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 1047,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1048,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1049,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1050,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1051,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1052,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1053,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1054,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1055,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1056,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1057,
            label: "Ease".to_string(),
        },
        FieldOption {
            value: 1058,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 1059,
            label: "Heal II".to_string(),
        },
        FieldOption {
            value: 1060,
            label: "Heal III".to_string(),
        },
        FieldOption {
            value: 1061,
            label: "Heal IV".to_string(),
        },
        FieldOption {
            value: 1062,
            label: "Major Heal".to_string(),
        },
        FieldOption {
            value: 1063,
            label: "Major Heal II".to_string(),
        },
        FieldOption {
            value: 1064,
            label: "Major Heal III".to_string(),
        },
        FieldOption {
            value: 1065,
            label: "Resurrect".to_string(),
        },
        FieldOption {
            value: 1066,
            label: "Resurrect II".to_string(),
        },
        FieldOption {
            value: 1067,
            label: "Word of Pain".to_string(),
        },
        FieldOption {
            value: 1068,
            label: "Word of Pain II".to_string(),
        },
        FieldOption {
            value: 1069,
            label: "Word of Pain III".to_string(),
        },
        FieldOption {
            value: 1070,
            label: "Word of Pain IV".to_string(),
        },
        FieldOption {
            value: 1071,
            label: "Meteor Strike".to_string(),
        },
        FieldOption {
            value: 1072,
            label: "Meteor Strike II".to_string(),
        },
        FieldOption {
            value: 1073,
            label: "Meteor Strike III".to_string(),
        },
        FieldOption {
            value: 1074,
            label: "Meteor Strike IV".to_string(),
        },
        FieldOption {
            value: 1075,
            label: "Hellbound".to_string(),
        },
        FieldOption {
            value: 1076,
            label: "Hellbound II".to_string(),
        },
        FieldOption {
            value: 1077,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 1078,
            label: "Abyss II".to_string(),
        },
        FieldOption {
            value: 1079,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 1080,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 1081,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1082,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 1083,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1084,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 1085,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 1086,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 1087,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 1088,
            label: "Paralytic Wave".to_string(),
        },
        FieldOption {
            value: 1089,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 1090,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1091,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 1092,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 1093,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1094,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1095,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1096,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1097,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1098,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1099,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 1100,
            label: "Tempest II".to_string(),
        },
        FieldOption {
            value: 1101,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 1102,
            label: "Gaia Strike II".to_string(),
        },
        FieldOption {
            value: 1103,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 1104,
            label: "Vortex II".to_string(),
        },
        FieldOption {
            value: 1105,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 1106,
            label: "Deluge II".to_string(),
        },
        FieldOption {
            value: 1107,
            label: "Annihilation".to_string(),
        },
        FieldOption {
            value: 1108,
            label: "Annihilation II".to_string(),
        },
        FieldOption {
            value: 1109,
            label: "Iceover".to_string(),
        },
        FieldOption {
            value: 1110,
            label: "Iceover II".to_string(),
        },
        FieldOption {
            value: 1111,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 1112,
            label: "Starfall II".to_string(),
        },
        FieldOption {
            value: 1113,
            label: "Diablo's Spite".to_string(),
        },
        FieldOption {
            value: 1114,
            label: "Diablo's Spite II".to_string(),
        },
        FieldOption {
            value: 1115,
            label: "Palace Guide Book".to_string(),
        },
        FieldOption {
            value: 1116,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1117,
            label: "Springboard".to_string(),
        },
        FieldOption {
            value: 1118,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 1119,
            label: "Palace Guide Book II".to_string(),
        },
        FieldOption {
            value: 1120,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1121,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 1122,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 1123,
            label: "Negate Spell".to_string(),
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
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 1129,
            label: "Sacrifice".to_string(),
        },
        FieldOption {
            value: 1130,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1131,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 1132,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 1133,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 1134,
            label: "Curse II".to_string(),
        },
        FieldOption {
            value: 1135,
            label: "Curse III".to_string(),
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
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 1141,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 1142,
            label: "Putrify II".to_string(),
        },
        FieldOption {
            value: 1143,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1144,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 1145,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 1146,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 1147,
            label: "Wind Dervish".to_string(),
        },
        FieldOption {
            value: 1148,
            label: "Wind Dervish II".to_string(),
        },
        FieldOption {
            value: 1149,
            label: "Sand Spider".to_string(),
        },
        FieldOption {
            value: 1150,
            label: "Sand Spider II".to_string(),
        },
        FieldOption {
            value: 1151,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 1152,
            label: "Chimaera II".to_string(),
        },
        FieldOption {
            value: 1153,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 1154,
            label: "Water Tiger II".to_string(),
        },
        FieldOption {
            value: 1155,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 1156,
            label: "Fire Snake II".to_string(),
        },
        FieldOption {
            value: 1157,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 1158,
            label: "Rime Raven II".to_string(),
        },
        FieldOption {
            value: 1159,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 1160,
            label: "Palace Guide Book III".to_string(),
        },
        FieldOption {
            value: 1161,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 1162,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 1163,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 1164,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 1165,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 1166,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 1167,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 1168,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 1169,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 1170,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 1171,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 1172,
            label: "Bedeviling Dance".to_string(),
        },
        FieldOption {
            value: 1173,
            label: "Invirogating Dance".to_string(),
        },
        FieldOption {
            value: 1174,
            label: "Demonpetal Dance".to_string(),
        },
        FieldOption {
            value: 1175,
            label: "Ardent Conga".to_string(),
        },
        FieldOption {
            value: 1176,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 1177,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 1178,
            label: "Stiring Folclore".to_string(),
        },
        FieldOption {
            value: 1179,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 1180,
            label: "Escalating Sanat".to_string(),
        },
        FieldOption {
            value: 1181,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 1182,
            label: "Glass Pumpkin".to_string(),
        },
        FieldOption {
            value: 1183,
            label: "Heaven's Fork".to_string(),
        },
        FieldOption {
            value: 1184,
            label: "Warrior's Mark".to_string(),
        },
        FieldOption {
            value: 1185,
            label: "Archer's Mark".to_string(),
        },
        FieldOption {
            value: 1186,
            label: "Mage's Mark".to_string(),
        },
        FieldOption {
            value: 1187,
            label: "Sibyl's Mark".to_string(),
        },
        FieldOption {
            value: 1188,
            label: "Mage-Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1189,
            label: "Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1190,
            label: "Dreadknight's Mark".to_string(),
        },
        FieldOption {
            value: 1191,
            label: "Berserker's Mark".to_string(),
        },
        FieldOption {
            value: 1192,
            label: "Swordman's Mark".to_string(),
        },
        FieldOption {
            value: 1193,
            label: "Dragoon's Mark".to_string(),
        },
        FieldOption {
            value: 1194,
            label: "Ninja's Mark".to_string(),
        },
        FieldOption {
            value: 1195,
            label: "Bandit's Mark".to_string(),
        },
        FieldOption {
            value: 1196,
            label: "Fusilier's Mark".to_string(),
        },
        FieldOption {
            value: 1197,
            label: "Beastmaster's Mark".to_string(),
        },
        FieldOption {
            value: 1198,
            label: "Magus's Mark".to_string(),
        },
        FieldOption {
            value: 1199,
            label: "Necroprentice's Mark".to_string(),
        },
        FieldOption {
            value: 1200,
            label: "Footsoldier's Mark".to_string(),
        },
        FieldOption {
            value: 1201,
            label: "Juggernaut's Mark".to_string(),
        },
        FieldOption {
            value: 1202,
            label: "Chief's Mark".to_string(),
        },
        FieldOption {
            value: 1203,
            label: "Familiar's Mark".to_string(),
        },
        FieldOption {
            value: 1204,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1205,
            label: "Windwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1206,
            label: "Cragwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1207,
            label: "Stormwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1208,
            label: "Waterwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1209,
            label: "Firewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1210,
            label: "Icewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1211,
            label: "Gleamwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1212,
            label: "Gloomwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1213,
            label: "Sandstone's Mark".to_string(),
        },
        FieldOption {
            value: 1214,
            label: "Granite's Mark".to_string(),
        },
        FieldOption {
            value: 1215,
            label: "Black Iron's Mark".to_string(),
        },
        FieldOption {
            value: 1216,
            label: "Magesteel's Mark".to_string(),
        },
        FieldOption {
            value: 1217,
            label: "Sovereign's Mark".to_string(),
        },
        FieldOption {
            value: 1218,
            label: "Brave's Mark".to_string(),
        },
        FieldOption {
            value: 1219,
            label: "Abuna's Mark".to_string(),
        },
        FieldOption {
            value: 1220,
            label: "Heretic's Mark".to_string(),
        },
        FieldOption {
            value: 1221,
            label: "Princess's Mark".to_string(),
        },
        FieldOption {
            value: 1222,
            label: "Holy Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1223,
            label: "Star Seer's Mark".to_string(),
        },
        FieldOption {
            value: 1224,
            label: "Peregrine's Mark".to_string(),
        },
        FieldOption {
            value: 1225,
            label: "Magus's Mark".to_string(),
        },
        FieldOption {
            value: 1226,
            label: "Oracle's Mark".to_string(),
        },
        FieldOption {
            value: 1227,
            label: "Wicce's Mark".to_string(),
        },
        FieldOption {
            value: 1228,
            label: "Songstress's Mark".to_string(),
        },
        FieldOption {
            value: 1229,
            label: "Hagiaknight's Mark".to_string(),
        },
        FieldOption {
            value: 1230,
            label: "Pirate's Mark".to_string(),
        },
        FieldOption {
            value: 1231,
            label: "Inferior Ore".to_string(),
        },
        FieldOption {
            value: 1232,
            label: "Iron Sand".to_string(),
        },
        FieldOption {
            value: 1233,
            label: "Copper Ore".to_string(),
        },
        FieldOption {
            value: 1234,
            label: "Tin Ore".to_string(),
        },
        FieldOption {
            value: 1235,
            label: "Graphite".to_string(),
        },
        FieldOption {
            value: 1236,
            label: "Iron Ore".to_string(),
        },
        FieldOption {
            value: 1237,
            label: "Silver Ore".to_string(),
        },
        FieldOption {
            value: 1238,
            label: "Baldur Ore".to_string(),
        },
        FieldOption {
            value: 1239,
            label: "Gold Ore".to_string(),
        },
        FieldOption {
            value: 1240,
            label: "Platinum Ore".to_string(),
        },
        FieldOption {
            value: 1241,
            label: "Saltpeter".to_string(),
        },
        FieldOption {
            value: 1242,
            label: "Sulfur".to_string(),
        },
        FieldOption {
            value: 1243,
            label: "Limestone".to_string(),
        },
        FieldOption {
            value: 1244,
            label: "Skyiron".to_string(),
        },
        FieldOption {
            value: 1245,
            label: "Gemstones".to_string(),
        },
        FieldOption {
            value: 1246,
            label: "Krystallos Ore".to_string(),
        },
        FieldOption {
            value: 1247,
            label: "Bronze Ingot".to_string(),
        },
        FieldOption {
            value: 1248,
            label: "Iron Ingot".to_string(),
        },
        FieldOption {
            value: 1249,
            label: "Silver Ingot".to_string(),
        },
        FieldOption {
            value: 1250,
            label: "Baldur Ingot".to_string(),
        },
        FieldOption {
            value: 1251,
            label: "Steel Ingot".to_string(),
        },
        FieldOption {
            value: 1252,
            label: "Hagane Steel".to_string(),
        },
        FieldOption {
            value: 1253,
            label: "Wootz Steel".to_string(),
        },
        FieldOption {
            value: 1254,
            label: "Golden Ingot".to_string(),
        },
        FieldOption {
            value: 1255,
            label: "Platinum Ingot".to_string(),
        },
        FieldOption {
            value: 1256,
            label: "Fiery Gems".to_string(),
        },
        FieldOption {
            value: 1257,
            label: "Verdant Gems".to_string(),
        },
        FieldOption {
            value: 1258,
            label: "Regal Gems".to_string(),
        },
        FieldOption {
            value: 1259,
            label: "White Gems".to_string(),
        },
        FieldOption {
            value: 1260,
            label: "Black Gems".to_string(),
        },
        FieldOption {
            value: 1261,
            label: "Air Krystallos".to_string(),
        },
        FieldOption {
            value: 1262,
            label: "Earth Krystallos".to_string(),
        },
        FieldOption {
            value: 1263,
            label: "Lightning Krystallos".to_string(),
        },
        FieldOption {
            value: 1264,
            label: "Water Krystallos".to_string(),
        },
        FieldOption {
            value: 1265,
            label: "Fire Krystallos".to_string(),
        },
        FieldOption {
            value: 1266,
            label: "Ice Krystallos".to_string(),
        },
        FieldOption {
            value: 1267,
            label: "Light Krystallos".to_string(),
        },
        FieldOption {
            value: 1268,
            label: "Dark Krystallos".to_string(),
        },
        FieldOption {
            value: 1269,
            label: "Toneriwood".to_string(),
        },
        FieldOption {
            value: 1270,
            label: "Birnewood".to_string(),
        },
        FieldOption {
            value: 1271,
            label: "Ananawood".to_string(),
        },
        FieldOption {
            value: 1272,
            label: "Baobawood".to_string(),
        },
        FieldOption {
            value: 1273,
            label: "Beasthide".to_string(),
        },
        FieldOption {
            value: 1274,
            label: "Tannin".to_string(),
        },
        FieldOption {
            value: 1275,
            label: "Leather".to_string(),
        },
        FieldOption {
            value: 1276,
            label: "Parchment".to_string(),
        },
        FieldOption {
            value: 1277,
            label: "Ink".to_string(),
        },
        FieldOption {
            value: 1278,
            label: "Gold Leaf".to_string(),
        },
        FieldOption {
            value: 1279,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 1280,
            label: "Log".to_string(),
        },
        FieldOption {
            value: 1281,
            label: "Bundle of Herbs".to_string(),
        },
        FieldOption {
            value: 1282,
            label: "Herbal Extract".to_string(),
        },
        FieldOption {
            value: 1283,
            label: "Nightshade".to_string(),
        },
        FieldOption {
            value: 1284,
            label: "Nightshade Extract".to_string(),
        },
        FieldOption {
            value: 1285,
            label: "Fruit".to_string(),
        },
        FieldOption {
            value: 1286,
            label: "Spirits".to_string(),
        },
        FieldOption {
            value: 1287,
            label: "Hempen Thread".to_string(),
        },
        FieldOption {
            value: 1288,
            label: "Woolen Thread".to_string(),
        },
        FieldOption {
            value: 1289,
            label: "Cotton Thread".to_string(),
        },
        FieldOption {
            value: 1290,
            label: "Silken Thread".to_string(),
        },
        FieldOption {
            value: 1291,
            label: "Silver Thread".to_string(),
        },
        FieldOption {
            value: 1292,
            label: "Golden Thread".to_string(),
        },
        FieldOption {
            value: 1293,
            label: "Linen".to_string(),
        },
        FieldOption {
            value: 1294,
            label: "Pincord".to_string(),
        },
        FieldOption {
            value: 1295,
            label: "Flannel".to_string(),
        },
        FieldOption {
            value: 1296,
            label: "Velvet".to_string(),
        },
        FieldOption {
            value: 1297,
            label: "Satin".to_string(),
        },
        FieldOption {
            value: 1298,
            label: "Blackpowder".to_string(),
        },
        FieldOption {
            value: 1299,
            label: "Beast Horn".to_string(),
        },
        FieldOption {
            value: 1300,
            label: "Beast Fang".to_string(),
        },
        FieldOption {
            value: 1301,
            label: "Beast Claw".to_string(),
        },
        FieldOption {
            value: 1302,
            label: "Wyrm Fang".to_string(),
        },
        FieldOption {
            value: 1303,
            label: "Wyrm Claw".to_string(),
        },
        FieldOption {
            value: 1304,
            label: "Wyrm Scale".to_string(),
        },
        FieldOption {
            value: 1305,
            label: "Wyrm Horn".to_string(),
        },
        FieldOption {
            value: 1306,
            label: "Wyrm Whisker".to_string(),
        },
        FieldOption {
            value: 1307,
            label: "Wyrm Thighbone".to_string(),
        },
        FieldOption {
            value: 1308,
            label: "Tooth & Claw".to_string(),
        },
        FieldOption {
            value: 1309,
            label: "Unicorn Horn".to_string(),
        },
        FieldOption {
            value: 1310,
            label: "Enchanted Feather".to_string(),
        },
        FieldOption {
            value: 1311,
            label: "Ancient Wood".to_string(),
        },
        FieldOption {
            value: 1312,
            label: "Ancient Bone".to_string(),
        },
        FieldOption {
            value: 1313,
            label: "Orichalcum".to_string(),
        },
        FieldOption {
            value: 1314,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1315,
            label: "Daedalus Pinion".to_string(),
        },
        FieldOption {
            value: 1316,
            label: "Daedalus Rack".to_string(),
        },
        FieldOption {
            value: 1317,
            label: "Melee Weapons I".to_string(),
        },
        FieldOption {
            value: 1318,
            label: "Melee Weapons II".to_string(),
        },
        FieldOption {
            value: 1319,
            label: "The Fist".to_string(),
        },
        FieldOption {
            value: 1320,
            label: "Fist Enchiridion".to_string(),
        },
        FieldOption {
            value: 1321,
            label: "The Blade".to_string(),
        },
        FieldOption {
            value: 1322,
            label: "Dagger Enchiridion".to_string(),
        },
        FieldOption {
            value: 1323,
            label: "Sword Enchidirion".to_string(),
        },
        FieldOption {
            value: 1324,
            label: "2-H Sword Enchiridion".to_string(),
        },
        FieldOption {
            value: 1325,
            label: "Axe Spear & Hammer".to_string(),
        },
        FieldOption {
            value: 1326,
            label: "Axe Enchiridion".to_string(),
        },
        FieldOption {
            value: 1327,
            label: "Spear Enchiridion".to_string(),
        },
        FieldOption {
            value: 1328,
            label: "Hammer Enchiridion".to_string(),
        },
        FieldOption {
            value: 1329,
            label: "The Katana".to_string(),
        },
        FieldOption {
            value: 1330,
            label: "Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1331,
            label: "2-H Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1332,
            label: "Cudgel & Whip".to_string(),
        },
        FieldOption {
            value: 1333,
            label: "Cudgel Enchiridion".to_string(),
        },
        FieldOption {
            value: 1334,
            label: "Whip Enchiridion".to_string(),
        },
        FieldOption {
            value: 1335,
            label: "Transcription".to_string(),
        },
        FieldOption {
            value: 1336,
            label: "Musical Instruments I".to_string(),
        },
        FieldOption {
            value: 1337,
            label: "Musical Instruments II".to_string(),
        },
        FieldOption {
            value: 1338,
            label: "Ranged Weapons I".to_string(),
        },
        FieldOption {
            value: 1339,
            label: "Ranged Weapons II".to_string(),
        },
        FieldOption {
            value: 1340,
            label: "Ways of the Gerges".to_string(),
        },
        FieldOption {
            value: 1341,
            label: "The Bow".to_string(),
        },
        FieldOption {
            value: 1342,
            label: "Bow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1343,
            label: "The Crossbow".to_string(),
        },
        FieldOption {
            value: 1344,
            label: "Crossbow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1345,
            label: "The Fusil".to_string(),
        },
        FieldOption {
            value: 1346,
            label: "Fusil Enchiridion".to_string(),
        },
        FieldOption {
            value: 1347,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1348,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1349,
            label: "Smithing Armor I".to_string(),
        },
        FieldOption {
            value: 1350,
            label: "Smithing Armor II".to_string(),
        },
        FieldOption {
            value: 1351,
            label: "Armorcraft".to_string(),
        },
        FieldOption {
            value: 1352,
            label: "Shieldcraft".to_string(),
        },
        FieldOption {
            value: 1353,
            label: "Shield Enchiridion".to_string(),
        },
        FieldOption {
            value: 1354,
            label: "Helm Enchiridion".to_string(),
        },
        FieldOption {
            value: 1355,
            label: "Body Armor Enchiridion".to_string(),
        },
        FieldOption {
            value: 1356,
            label: "Armguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1357,
            label: "Legguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1358,
            label: "Codex of Jewelry I".to_string(),
        },
        FieldOption {
            value: 1359,
            label: "Codex of Jewelry II".to_string(),
        },
        FieldOption {
            value: 1360,
            label: "Codex of Jewelry III".to_string(),
        },
        FieldOption {
            value: 1361,
            label: "Codex of Jewelry IV".to_string(),
        },
        FieldOption {
            value: 1362,
            label: "Codex of Ores".to_string(),
        },
        FieldOption {
            value: 1363,
            label: "Codex of Gems".to_string(),
        },
        FieldOption {
            value: 1364,
            label: "Codex of Timber".to_string(),
        },
        FieldOption {
            value: 1365,
            label: "Codex of Textiles".to_string(),
        },
        FieldOption {
            value: 1366,
            label: "On Medicine I".to_string(),
        },
        FieldOption {
            value: 1367,
            label: "On Medicine II".to_string(),
        },
        FieldOption {
            value: 1368,
            label: "Secrets of the Master".to_string(),
        },
        FieldOption {
            value: 1369,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1370,
            label: "Ease II".to_string(),
        },
        FieldOption {
            value: 1371,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1372,
            label: "STR Charm".to_string(),
        },
        FieldOption {
            value: 1373,
            label: "VIT Charm".to_string(),
        },
        FieldOption {
            value: 1374,
            label: "DEX Charm".to_string(),
        },
        FieldOption {
            value: 1375,
            label: "AGI Charm".to_string(),
        },
        FieldOption {
            value: 1376,
            label: "AVD Charm".to_string(),
        },
        FieldOption {
            value: 1377,
            label: "INT Charm".to_string(),
        },
        FieldOption {
            value: 1378,
            label: "MND Charm".to_string(),
        },
        FieldOption {
            value: 1379,
            label: "RES Charm".to_string(),
        },
        FieldOption {
            value: 1380,
            label: "LUK Charm".to_string(),
        },
        FieldOption {
            value: 1381,
            label: "Air Charm".to_string(),
        },
        FieldOption {
            value: 1382,
            label: "Earth Charm".to_string(),
        },
        FieldOption {
            value: 1383,
            label: "Lightening Charm".to_string(),
        },
        FieldOption {
            value: 1384,
            label: "Water Charm".to_string(),
        },
        FieldOption {
            value: 1385,
            label: "Fire Charm".to_string(),
        },
        FieldOption {
            value: 1386,
            label: "Ice Charm".to_string(),
        },
        FieldOption {
            value: 1387,
            label: "Light Charm".to_string(),
        },
        FieldOption {
            value: 1388,
            label: "Dark Charm".to_string(),
        },
        FieldOption {
            value: 1389,
            label: "Experience Charm".to_string(),
        },
        FieldOption {
            value: 1390,
            label: "Experience Charm II".to_string(),
        },
        FieldOption {
            value: 1391,
            label: "Experience Charm III".to_string(),
        },
        FieldOption {
            value: 1392,
            label: "Experience Charm IV".to_string(),
        },
        FieldOption {
            value: 1393,
            label: "Experience Charm V".to_string(),
        },
        FieldOption {
            value: 1394,
            label: "Level Up Charm".to_string(),
        },
        FieldOption {
            value: 1395,
            label: "Grimoire Exorcisme".to_string(),
        },
        FieldOption {
            value: 1396,
            label: "Ease II".to_string(),
        },
    ]
}

fn opt_voices() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Denam".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Vyce".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Catiua".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Lancelot".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Warren".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Canopus".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Mirdyn".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Gildas".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Cerya".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Sherri".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Cistina".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Olyvia".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Deneb".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Gilbald".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Andoras".to_string(),
        },
        FieldOption {
            value: 16,
            label: "Donnalto".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Folcurt".to_string(),
        },
        FieldOption {
            value: 18,
            label: "Bayin".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Arycelle".to_string(),
        },
        FieldOption {
            value: 20,
            label: "Hobyrim".to_string(),
        },
        FieldOption {
            value: 21,
            label: "Jeunan".to_string(),
        },
        FieldOption {
            value: 22,
            label: "Ocionne".to_string(),
        },
        FieldOption {
            value: 23,
            label: "Xapan".to_string(),
        },
        FieldOption {
            value: 24,
            label: "Dievold".to_string(),
        },
        FieldOption {
            value: 25,
            label: "Oelias".to_string(),
        },
        FieldOption {
            value: 26,
            label: "Rudlum".to_string(),
        },
        FieldOption {
            value: 27,
            label: "Ganpp".to_string(),
        },
        FieldOption {
            value: 28,
            label: "Iuria".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Cressida".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Ozma".to_string(),
        },
        FieldOption {
            value: 31,
            label: "Lindl".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Leonar".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Ravness".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Azelstan".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Ehlrig".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Sara".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Voltare".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Felicia".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Chamos".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Phaesta".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Tamuz".to_string(),
        },
        FieldOption {
            value: 42,
            label: "None".to_string(),
        },
        FieldOption {
            value: 43,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 52,
            label: "None".to_string(),
        },
        FieldOption {
            value: 53,
            label: "None".to_string(),
        },
        FieldOption {
            value: 54,
            label: "None".to_string(),
        },
        FieldOption {
            value: 55,
            label: "None".to_string(),
        },
        FieldOption {
            value: 56,
            label: "None".to_string(),
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
            label: "Generic Male".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Generic Male".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Generic Female".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Generic Female".to_string(),
        },
        FieldOption {
            value: 69,
            label: "None".to_string(),
        },
        FieldOption {
            value: 70,
            label: "None".to_string(),
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
            label: "Skeleton/Phantom".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Gryphon/Cockatrice".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Octopus/Cyclops/Golem".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Dragon".to_string(),
        },
        FieldOption {
            value: 80,
            label: "None".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Bapal".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Agares".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Orba".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Moldova".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Brezen".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Bolis".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Grion".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Vance".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Genzo".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Farrel".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Daza".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Veldrei".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Dagon".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Ramidos".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Garba".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Nadia".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Wynoa".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Gatialo".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Gousin".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Modiliani".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Marino".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Alfred".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Hektor".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Cassandra".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Gildora".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Xaebos".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Vestiarri".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Mercure".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Botis".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Orgeau".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Gannon".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Felnatorre".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Kakrinoros".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Hiram".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Crossa".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Roberval".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Apollinaire".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Brutakos".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Uram".to_string(),
        },
        FieldOption {
            value: 120,
            label: "LeRozza".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Alessandro".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Pajeot".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Stanoska".to_string(),
        },
        FieldOption {
            value: 124,
            label: "Mimose".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Hanzo".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Halphas".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Derrain".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Vermado".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Latimer".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Brantyn".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Dorgalua".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Ragnar".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Rackham".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Lobos".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Aeshan".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Bravan".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Anaberg".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Josephine".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Leon".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Falfaday".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Xadoba".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Nybeth".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Georges".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Beelzebuth".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Vepahl".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Jilessa".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Cielo".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Reymos".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Dilthey".to_string(),
        },
        FieldOption {
            value: 150,
            label: "None".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Rodrick".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Oz".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Martym".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Barbas".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Volaq".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Balxephon".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Lanselot Tartaros".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Punkin".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Muntzer".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Nybeth (Lich)".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Vyce (Zombie)".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Gildas (Zombie)".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Leonard (Zombie)".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Xaebos (Zombie)".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Hektor (Zombie)".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Moldova (Zombie)".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Cassandra (Zombie)".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Lizardman".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Lamia".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Orc".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Umbra Male".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Umbra Female".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Faerie".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Pumpkin".to_string(),
        },
        FieldOption {
            value: 175,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 252,
            label: "None".to_string(),
        },
        FieldOption {
            value: 253,
            label: "None".to_string(),
        },
        FieldOption {
            value: 254,
            label: "None".to_string(),
        },
        FieldOption {
            value: 255,
            label: "None".to_string(),
        },
    ]
}

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "characters".to_string(),
        name: "Character Editor".to_string(),
        description: "Edit character stats and properties".to_string(),
        base_offset: 0x420A20,
        entry_count: 511,
        entry_size: 84,
        entry_names: vec![
            "None".to_string(),
            "Denam Morne".to_string(),
            "Vyce Bozeck".to_string(),
            "Catiua Pavel".to_string(),
            "Lanselot Hamilton".to_string(),
            "Warren Omon".to_string(),
            "Canopus Wolph".to_string(),
            "Myrdin Walhorn".to_string(),
            "Gildas W. Byrne".to_string(),
            "Cerya Phoraena".to_string(),
            "Sherri Phoraena".to_string(),
            "Cistina Phoraena".to_string(),
            "Olyvia Phoraena".to_string(),
            "Deneb Rove".to_string(),
            "NPC Gilbald O'Brien".to_string(),
            "NPC Andoras Gaffryn".to_string(),
            "Donnalto Presance".to_string(),
            "Folcurt Reeda Lynde".to_string(),
            "Bayin Rosen Orne".to_string(),
            "Arycelle Dania".to_string(),
            "Hobyrim V. Rahms".to_string(),
            "Jeunan Avertif".to_string(),
            "Ocionne Rabine".to_string(),
            "Xapan Illudas".to_string(),
            "Dievold Obdilord".to_string(),
            "Oelias Obdilord".to_string(),
            "Robert Rudlum".to_string(),
            "Ganpp Vochstein".to_string(),
            "Iuria Wolph".to_string(),
            "Cressida Obdilord".to_string(),
            "Ozma Moh Glacius".to_string(),
            "Jonathon Torgeaux Lindl".to_string(),
            "Leonar Reci Rimon".to_string(),
            "Ravness Loxaerion".to_string(),
            "Diego Galet Azelstan".to_string(),
            "Agrikora".to_string(),
            "NPC Generic Male".to_string(),
            "NPC Generic Male".to_string(),
            "Merrik Ehlrig".to_string(),
            "Berda the Younger".to_string(),
            "Obda the Younger".to_string(),
            "NPC Vyce Bozeck (Law Route)".to_string(),
            "NPC Catiua(Dark Priest)".to_string(),
            "NPC Gildas Zombie".to_string(),
            "NPC Sherri Phoraena".to_string(),
            "NPC Leonar Reci Rimon".to_string(),
            "NPC Leonar Reci Rimon Zombie".to_string(),
            "NPC Arycelle Dania".to_string(),
            "NPC Ocionne Rabine".to_string(),
            "NPC Xapan Illudas".to_string(),
            "NPC Jeunan Avertif".to_string(),
            "NPC Lanselot Tartaros".to_string(),
            "NPC Balxephon V. Rahms".to_string(),
            "NPC Volaq Windsalf".to_string(),
            "NPC Barbas Dahd Geuse".to_string(),
            "NPC Martym Noumous".to_string(),
            "NPC Oz Moh Glavius".to_string(),
            "NPC Ozma Moh Glaviusma".to_string(),
            "NPC Andoras Gaffryn".to_string(),
            "NPC Dorgalua Oberyth Valeria (Dark Lord I)".to_string(),
            "NPC Brantyn Morne".to_string(),
            "NPC Nybeth Obdilord".to_string(),
            "NPC Nybeth Obdilord Lich".to_string(),
            "NPC Xaebos Ronsenbach".to_string(),
            "NPC Xaebos Ronsenbach Zombie".to_string(),
            "NPC Ganpp Vochstein".to_string(),
            "NPC Rodrick Desmoria".to_string(),
            "NPC Cressida Obdilord".to_string(),
            "NPC Ravness Loxaerion".to_string(),
            "NPC Bayin Rosen Orne".to_string(),
            "NPC Vyce Bozeck (Chaos)".to_string(),
            "NPC Marino Arvine (Terror Knight)".to_string(),
            "NPC Agares Bazin (Knight)".to_string(),
            "NPC Johann Anaberg (Lich)".to_string(),
            "NPC Wynoa Canaletto (Cleric)".to_string(),
            "NPC Vepahl Dabran (Sorceress)".to_string(),
            "NPC Veldrei (Valkyrie)".to_string(),
            "NPC Vermado Johannsen (Knight)".to_string(),
            "NPC Uram Fenelon (Wizard)".to_string(),
            "NPC Aeshan (Swordmaster)".to_string(),
            "NPC Merrik Ehlrig".to_string(),
            "NPC Obda (Gryphon)".to_string(),
            "NPC Orgeau Manheim (Hawkman)".to_string(),
            "NPC Orba Brodel (Wizard)".to_string(),
            "NPC Bruno Kakrinoros (Wizard)".to_string(),
            "NPC Gousin Blum (Ninja)".to_string(),
            "NPC Farrel Ganache (Knight)".to_string(),
            "NPC Berruk Gannon (Berserker)".to_string(),
            "NPC Kamlott Roberval (Ninja)".to_string(),
            "NPC Garba Brondel (Mage)".to_string(),
            "NPC Dukas Windelband Gatialo (Terror Knight)".to_string(),
            "NPC Latimer Grandier (Knight)".to_string(),
            "NPC Genzo Ageja (Ninja)".to_string(),
            "NPC Xadoba (Lamia)".to_string(),
            "NPC Gildora Bastian (Witch)".to_string(),
            "NPC Swift Stanoska (Beast Tamer)".to_string(),
            "NPC Dagon Simmel (Wizard)".to_string(),
            "NPC Darza (Berserker)".to_string(),
            "NPC Hektor Didarro (Knight)".to_string(),
            "NPC Nadia Eginhard (Cleric)".to_string(),
            "NPC Hiram (Terror Knight)".to_string(),
            "NPC Bravan (Berserker)".to_string(),
            "NPC Juglar Pajeot (Dragoon)".to_string(),
            "NPC Pavan Bapal (Berserker)".to_string(),
            "NPC Halphas Hohenstaufen (Beast Tamer)".to_string(),
            "NPC Hanzo Rugen (Swordmaster)".to_string(),
            "NPC Crossa Bingham (Hawkman)".to_string(),
            "NPC Falfaday Geb Lesmoaria (Sorceress)".to_string(),
            "NPC Dilthey Pherenian (Fusiller)".to_string(),
            "NPC Felnatorre Savonarola (Knight)".to_string(),
            "NPC Blackmoor (Lich)".to_string(),
            "NPC Brutakos Kapote (Sorcerer)".to_string(),
            "NPC Tomasius Brezen (Berserker)".to_string(),
            "NPC Brondel Vestiarri (Enchantress)".to_string(),
            "NPC Beelzebuth".to_string(),
            "NPC Berda (Gryphon)".to_string(),
            "NPC Botis (Wizard)".to_string(),
            "NPC Schleiden Mercure (Swordmaster)".to_string(),
            "NPC Anelio Muntzer (Necromancer)".to_string(),
            "NPC Vailland Modiliani (Sorcerer)".to_string(),
            "NPC Moldova Obdilord (Necromancer)".to_string(),
            "NPC Derain Lowart (Terror Knight)".to_string(),
            "NPC Ramidos Mendoza (Sorceress)".to_string(),
            "NPC Bolis Rhumoth (Knight)".to_string(),
            "NPC Romulus LeRozza (Warlock)".to_string(),
            "NPC Grion Lexentale (Knight)".to_string(),
            "NPC Lobos (Rogue)".to_string(),
            "NPC Christos Vance (Rune Fencer)".to_string(),
            "NPC Banga (Cockatrice)".to_string(),
            "NPC Zanga (Cockatrice)".to_string(),
            "NPC Daesi Apollinaire (Knight)".to_string(),
            "NPC Cassandra Obdilord (Necromancer)".to_string(),
            "NPC Alessandro Zuloaga (Terror Knight)".to_string(),
            "Sara Ostvald".to_string(),
            "Voltare Montrose".to_string(),
            "Felicia Malxion".to_string(),
            "Chamos Zalman".to_string(),
            "Phaesta Morandi".to_string(),
            "Tamuz Fedorenko".to_string(),
            "NPC Dorgalua Oberyth Valeria (Dark Lord II)".to_string(),
            "NPC Mimose Prongniart (Valkyrie)".to_string(),
            "NPC Cielo Segur (Fusilier)".to_string(),
            "NPC Georges Sekendorff (Necromancer)".to_string(),
            "NPC Jilessa Krapelin (Cleric)".to_string(),
            "NPC Reymos Cavour (Necromancer)".to_string(),
            "Unknown Gryphon".to_string(),
            "Unknown Gryphon".to_string(),
            "NPC Ragnar (Berserker)".to_string(),
            "NPC Josephine Ysarc (Cleric)".to_string(),
            "NPC Leon Wilfred (Fusiler)".to_string(),
            "NPC Alfred Boulvart (Knight)".to_string(),
            "NPC Punkin".to_string(),
            "NPC Rackham".to_string(),
            "None".to_string(),
            "Ch3 Neutral Male Warrior".to_string(),
            "Ch3 Neutral Female Archer".to_string(),
            "Ch3 Neutral Female Knight".to_string(),
            "Ch3 Neutral Female Warrior".to_string(),
            "Ch3 Neutral Male Archer".to_string(),
            "Generic M(Recruit)".to_string(),
            "Generic M(Recruit) II".to_string(),
            "Generic M(Recruit) III".to_string(),
            "Generic M(Recruit) IV".to_string(),
            "Generic F(Recruit)".to_string(),
            "Generic F(Recruit) II".to_string(),
            "Generic F(Recruit) III".to_string(),
            "Generic F(Recruit) IV".to_string(),
            "Hawkman I".to_string(),
            "Hawkman II".to_string(),
            "Hawkman III".to_string(),
            "Hawkman IV".to_string(),
            "Lizardmen I".to_string(),
            "Lizardmen II".to_string(),
            "Lizardman III".to_string(),
            "Lizardmen IV".to_string(),
            "Lamia I".to_string(),
            "Lamia II".to_string(),
            "Lamia III".to_string(),
            "Lamia IV".to_string(),
            "Orc I".to_string(),
            "Orc II".to_string(),
            "Orc III".to_string(),
            "Orc IV".to_string(),
            "Skeleton M I".to_string(),
            "Skeleton M II".to_string(),
            "Skeleton M III".to_string(),
            "Skeleton M IV".to_string(),
            "Skeleton F I".to_string(),
            "Skeleton F II".to_string(),
            "Skeleton F III".to_string(),
            "Skeleton F IV".to_string(),
            "Phantom M I".to_string(),
            "Phantom M II".to_string(),
            "Phantom M III".to_string(),
            "Phantom M IV".to_string(),
            "Phantom F I".to_string(),
            "Phantom F II".to_string(),
            "Phantom F III".to_string(),
            "Phantom F IV".to_string(),
            "Faerie I".to_string(),
            "Faerie II".to_string(),
            "Faerie III".to_string(),
            "Faerie IV".to_string(),
            "Gremlin I".to_string(),
            "Gremlin II".to_string(),
            "Gremlin III".to_string(),
            "Gremlin IV".to_string(),
            "Pumpkin I".to_string(),
            "Pumpkin II".to_string(),
            "Pumpkin III".to_string(),
            "Pumpkin IV".to_string(),
            "Dragon I".to_string(),
            "Dragon II".to_string(),
            "Dragon III".to_string(),
            "Dragon IV".to_string(),
            "Hydra I".to_string(),
            "Hydra II".to_string(),
            "Hydra III".to_string(),
            "Hydra IV".to_string(),
            "Gryphon I".to_string(),
            "Gryphon II".to_string(),
            "Gryphon III".to_string(),
            "Gryphon IV".to_string(),
            "Cockatrice I".to_string(),
            "Cockatrice II".to_string(),
            "Cockatrice III".to_string(),
            "Cockatrice IV".to_string(),
            "Octopus I".to_string(),
            "Octopus II".to_string(),
            "Octopus III".to_string(),
            "Octopus IV".to_string(),
            "Cyclops I".to_string(),
            "Cyclops II".to_string(),
            "Cyclops III".to_string(),
            "Cyclops IV".to_string(),
            "Golem I".to_string(),
            "Golem II".to_string(),
            "Golem III".to_string(),
            "Golem IV".to_string(),
            "None".to_string(),
            "None".to_string(),
            "NPC Young Man (Vasque Survivor)".to_string(),
            "NPC Young Woman (Vasque Survivor)".to_string(),
            "NPC Middle Aged Man (Vasque Survivor)".to_string(),
            "NPC Middle Aged Woman(Vasque Survivor)".to_string(),
            "NPC Bald Old Man (Vasque Survivor)".to_string(),
            "NPC Old Woman (Vasque Survivor)".to_string(),
            "None".to_string(),
            "None".to_string(),
            "NPC Azelstan".to_string(),
            "NPC Dievold Obdilord".to_string(),
            "NPC Cassandra Obdilord Zombie".to_string(),
            "NPC Moldova Obdilord Zombie".to_string(),
            "NPC Hektor Diddaro Zombie".to_string(),
            "NPC Vyce Bozeck Zombie".to_string(),
            "None".to_string(),
            "PotD Gladiator Male".to_string(),
            "PotD Gladiator Male".to_string(),
            "PotD Gladiator Female".to_string(),
            "PotD Gladiator Female".to_string(),
            "PotD Blood Hunter Male".to_string(),
            "PotD Blood Hunter Male".to_string(),
            "PotD Blood Hunter Female".to_string(),
            "PotD Blood Hunter Female".to_string(),
            "PotD Death Eater Male".to_string(),
            "PotD Death Eater Male".to_string(),
            "PotD Death Eater Female".to_string(),
            "PotD Death Eater Female".to_string(),
            "PotD Cenobite Male".to_string(),
            "PotD Cenobite Male".to_string(),
            "PotD Cenobite Female".to_string(),
            "PotD Cenobite Female".to_string(),
            "PotD Cannibal Male".to_string(),
            "PotD Cannibal Male".to_string(),
            "PotD Cannibal Female".to_string(),
            "PotD Cannibal Female".to_string(),
            "PotD Shadow Knight Male".to_string(),
            "PotD Shadow Knight Male".to_string(),
            "PotD Shadow Knight Female".to_string(),
            "PotD Shadow Knight Female".to_string(),
            "PotD Dreadnought Male".to_string(),
            "PotD Dreadnought Male".to_string(),
            "PotD Dreadnought Female".to_string(),
            "PotD Dreadnought Female".to_string(),
            "PotD Executioner Male".to_string(),
            "PotD Executioner Male".to_string(),
            "PotD Executioner Female".to_string(),
            "PotD Executioner Female".to_string(),
            "PotD Kill Seeker Male".to_string(),
            "PotD Kill Seeker Male".to_string(),
            "PotD Kill Seeker Female".to_string(),
            "PotD Kill Seeker Female".to_string(),
            "PotD Crimson Uhlan Male".to_string(),
            "PotD Crimson Uhlan Male".to_string(),
            "PotD Crimson Uhlan Female".to_string(),
            "PotD Crimson Uhlan Female".to_string(),
            "PotD Dark Stalker Male".to_string(),
            "PotD Dark Stalker Male".to_string(),
            "PotD Dark Stalker Female".to_string(),
            "PotD Dark Stalker Female".to_string(),
            "PotD Grim Reaper Male".to_string(),
            "PotD Grim Reaper Male".to_string(),
            "PotD Grim Reaper Female".to_string(),
            "PotD Grim Reaper Female".to_string(),
            "PotD Sniper Male".to_string(),
            "PotD Sniper Male".to_string(),
            "PotD Sniper Female".to_string(),
            "PotD Sniper Female".to_string(),
            "PotD Iron Fist Male".to_string(),
            "PotD Iron Fist Male".to_string(),
            "PotD Iron Fist Female".to_string(),
            "PotD Iron Fist Female".to_string(),
            "PotD Loremaster Male".to_string(),
            "PotD Loremaster Male".to_string(),
            "PotD Loremaster Female".to_string(),
            "PotD Loremaster Female".to_string(),
            "PotD Witch King".to_string(),
            "PotD Witch King".to_string(),
            "PotD Witch Queen".to_string(),
            "PotD Witch Queen".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "NPC Ravenman".to_string(),
            "NPC Ravenman".to_string(),
            "NPC Ravenman".to_string(),
            "NPC Ravenman".to_string(),
            "NPC Lich King".to_string(),
            "NPC Lich King".to_string(),
            "NPC Lich Queen".to_string(),
            "NPC Lich Queen".to_string(),
            "NPC Etheral Vision Male".to_string(),
            "NPC Etheral Vision Male".to_string(),
            "NPC Etheral Vision Female".to_string(),
            "NPC Etheral Vision Female".to_string(),
            "NPC Night Crow".to_string(),
            "NPC Night Crow".to_string(),
            "NPC Night Crow".to_string(),
            "NPC Night Crow".to_string(),
            "NPC Blood Gavial".to_string(),
            "NPC Blood Gavial".to_string(),
            "NPC Blood Gavial".to_string(),
            "NPC Blood Gavial".to_string(),
            "NPC Gorgon".to_string(),
            "NPC Gorgon".to_string(),
            "NPC Gorgon".to_string(),
            "NPC Gorgon".to_string(),
            "NPC Uruk".to_string(),
            "NPC Uruk".to_string(),
            "NPC Uruk".to_string(),
            "NPC Uruk".to_string(),
            "NPC Wight Male".to_string(),
            "NPC Wight Male".to_string(),
            "NPC Wight Female".to_string(),
            "NPC Wight Female".to_string(),
            "NPC Wraith Male".to_string(),
            "NPC Wraith Male".to_string(),
            "NPC Wraith Female".to_string(),
            "NPC Wraith Female".to_string(),
            "NPC Banshee".to_string(),
            "NPC Banshee".to_string(),
            "NPC Banshee".to_string(),
            "NPC Banshee".to_string(),
            "NPC Incubus".to_string(),
            "NPC Incubus".to_string(),
            "NPC Incubus".to_string(),
            "NPC Incubus".to_string(),
            "NPC Boggart".to_string(),
            "NPC Boggart".to_string(),
            "NPC Boggart".to_string(),
            "NPC Boggart".to_string(),
            "NPC Crystal Dragon".to_string(),
            "NPC Crystal Dragon".to_string(),
            "NPC Crystal Dragon".to_string(),
            "NPC Crystal Dragon".to_string(),
            "NPC Onyx Dragon".to_string(),
            "NPC Onyx Dragon".to_string(),
            "NPC Onyx Dragon".to_string(),
            "NPC Onyx Dragon".to_string(),
            "NPC Scylla".to_string(),
            "NPC Scylla".to_string(),
            "NPC Scylla".to_string(),
            "NPC Scylla".to_string(),
            "NPC Naga".to_string(),
            "NPC Naga".to_string(),
            "NPC Naga".to_string(),
            "NPC Naga".to_string(),
            "NPC Damasc Golem".to_string(),
            "NPC Damasc Golem".to_string(),
            "NPC Damasc Golem".to_string(),
            "NPC Damasc Golem".to_string(),
            "NPC Flesh Golem".to_string(),
            "NPC Flesh Golem".to_string(),
            "NPC Flesh Golem".to_string(),
            "NPC Flesh Golem".to_string(),
            "NPC Hippogryph".to_string(),
            "NPC Hippogryph".to_string(),
            "NPC Hippogryph".to_string(),
            "NPC Hippogryph".to_string(),
            "NPC Phoenix".to_string(),
            "NPC Phoenix".to_string(),
            "NPC Phoenix".to_string(),
            "NPC Phoenix".to_string(),
            "NPC Ruhk".to_string(),
            "NPC Ruhk".to_string(),
            "NPC Ruhk".to_string(),
            "NPC Ruhk".to_string(),
            "NPC Basilisk".to_string(),
            "NPC Basilisk".to_string(),
            "NPC Basilisk".to_string(),
            "NPC Basilisk".to_string(),
            "NPC Kraken".to_string(),
            "NPC Kraken".to_string(),
            "NPC Kraken".to_string(),
            "NPC Kraken".to_string(),
            "NPC Dagon".to_string(),
            "NPC Dagon".to_string(),
            "NPC Dagon".to_string(),
            "NPC Dagon".to_string(),
            "NPC Spriggan".to_string(),
            "NPC Spriggan".to_string(),
            "NPC Spriggan".to_string(),
            "NPC Spriggan".to_string(),
            "NPC Titan".to_string(),
            "NPC Titan".to_string(),
            "NPC Titan".to_string(),
            "NPC Titan".to_string(),
            "NPC Aym".to_string(),
            "NPC Aloser".to_string(),
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
            "NPC Vija".to_string(),
            "NPC Enja".to_string(),
            "NPC Maitreya".to_string(),
            "NPC Ijana".to_string(),
            "NPC Chandra".to_string(),
            "NPC Vayu".to_string(),
            "NPC Indra".to_string(),
            "NPC Rakshas".to_string(),
            "NPC Ashurama".to_string(),
            "NPC Asurama".to_string(),
            "NPC Aditi".to_string(),
            "NPC Saranga".to_string(),
            "NPC Kandyce".to_string(),
            "None".to_string(),
            "NPC Sirene".to_string(),
            "NPC Vainateya".to_string(),
            "NPC Nathalork".to_string(),
            "NPC Xolotl".to_string(),
            "NPC Tlaloc".to_string(),
            "NPC Ifrit".to_string(),
            "NPC Lygenstzel".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "Race".to_string(),
                offset: 2,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_race_list()),
            },
            FieldDefinition {
                name: "Gender".to_string(),
                offset: 3,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_gender()),
            },
            FieldDefinition {
                name: "Sprite Aura".to_string(),
                offset: 5,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Undead".to_string(),
                offset: 6,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "Class Change Type".to_string(),
                offset: 56,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_class_groups()),
            },
            FieldDefinition {
                name: "Class Change Race".to_string(),
                offset: 57,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_race_list_2()),
            },
            FieldDefinition {
                name: "Sprite".to_string(),
                offset: 58,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_sprites()),
            },
            FieldDefinition {
                name: "Color Palette".to_string(),
                offset: 61,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_color_palette()),
            },
            FieldDefinition {
                name: "Player Portrait".to_string(),
                offset: 64,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Bakram Portrait".to_string(),
                offset: 66,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Galgastan Portrait".to_string(),
                offset: 68,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Pirates Portrait".to_string(),
                offset: 70,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Walister Portrait".to_string(),
                offset: 72,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Outlaw Portrait".to_string(),
                offset: 74,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Unknown Portrait".to_string(),
                offset: 76,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_portraits()),
            },
            FieldDefinition {
                name: "Recruitable".to_string(),
                offset: 20,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "Base HP".to_string(),
                offset: 24,
                size: 2,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base MP".to_string(),
                offset: 26,
                size: 2,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base STR".to_string(),
                offset: 30,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base VIT".to_string(),
                offset: 32,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base DEX".to_string(),
                offset: 34,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base AGI".to_string(),
                offset: 36,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base AVD".to_string(),
                offset: 38,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base INT".to_string(),
                offset: 40,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base MND".to_string(),
                offset: 42,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base RES".to_string(),
                offset: 44,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Base WT".to_string(),
                offset: 46,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Move Type".to_string(),
                offset: 48,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_move_type_2()),
            },
            FieldDefinition {
                name: "Melee Attack Type".to_string(),
                offset: 50,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Ranged Attack Type".to_string(),
                offset: 52,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Voice 1".to_string(),
                offset: 80,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_voices()),
            },
            FieldDefinition {
                name: "Voice 2".to_string(),
                offset: 81,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_voices()),
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
        assert_eq!(def.id, "characters");
        assert_eq!(def.base_offset, 0x420A20);
        assert_eq!(def.entry_count, 511);
        assert_eq!(def.entry_size, 84);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 32);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 511);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Race");
        assert_eq!(f.offset, 2);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[31];
        assert_eq!(f.name, "Voice 2");
        assert_eq!(f.offset, 81);
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
