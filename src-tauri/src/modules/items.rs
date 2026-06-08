use crate::modules::types::*;

fn opt_item_catagory() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 32,
            label: "Consumables".to_string(),
        },
        FieldOption {
            value: 33,
            label: "Vendor Trash".to_string(),
        },
        FieldOption {
            value: 34,
            label: "Spells".to_string(),
        },
        FieldOption {
            value: 35,
            label: "Classmarks".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Crafting Item".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Crafting Book".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Charms".to_string(),
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

fn opt_item_names() -> Vec<FieldOption> {
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
            label: "Dragon Steak".to_string(),
        },
        FieldOption {
            value: 1054,
            label: "Braised Skewer".to_string(),
        },
        FieldOption {
            value: 1055,
            label: "Steamed Mollusk".to_string(),
        },
        FieldOption {
            value: 1056,
            label: "Minced Patty".to_string(),
        },
        FieldOption {
            value: 1057,
            label: "Brand of the Sacrifice".to_string(),
        },
        FieldOption {
            value: 1058,
            label: "Dynast-King's Mead".to_string(),
        },
        FieldOption {
            value: 1059,
            label: "Echo Stone".to_string(),
        },
        FieldOption {
            value: 1060,
            label: "Blackwing Leg".to_string(),
        },
        FieldOption {
            value: 1061,
            label: "Rood Upright".to_string(),
        },
        FieldOption {
            value: 1062,
            label: "Haunt's Tome".to_string(),
        },
        FieldOption {
            value: 1063,
            label: "Darkscale Tome".to_string(),
        },
        FieldOption {
            value: 1064,
            label: "Cursed Unicorn Blood".to_string(),
        },
        FieldOption {
            value: 1065,
            label: "Skulldust Nostrum".to_string(),
        },
        FieldOption {
            value: 1066,
            label: "Magedrain Gland".to_string(),
        },
        FieldOption {
            value: 1067,
            label: "Shiftstone".to_string(),
        },
        FieldOption {
            value: 1068,
            label: "Palace Guide Book I".to_string(),
        },
        FieldOption {
            value: 1069,
            label: "Palace Guide Book II".to_string(),
        },
        FieldOption {
            value: 1070,
            label: "Palace Guide Book III".to_string(),
        },
        FieldOption {
            value: 1071,
            label: "Horn of the Savage".to_string(),
        },
        FieldOption {
            value: 1072,
            label: "Coral Harp".to_string(),
        },
        FieldOption {
            value: 1073,
            label: "Whirlwind Shot".to_string(),
        },
        FieldOption {
            value: 1074,
            label: "Duststorm Shot".to_string(),
        },
        FieldOption {
            value: 1075,
            label: "Thunder Shot".to_string(),
        },
        FieldOption {
            value: 1076,
            label: "Torrent Shot".to_string(),
        },
        FieldOption {
            value: 1077,
            label: "Conflagration Shot".to_string(),
        },
        FieldOption {
            value: 1078,
            label: "Firnice Shot".to_string(),
        },
        FieldOption {
            value: 1079,
            label: "Coruscate Shot".to_string(),
        },
        FieldOption {
            value: 1080,
            label: "Murk Shot".to_string(),
        },
        FieldOption {
            value: 1081,
            label: "Grimoire Exorcisme".to_string(),
        },
        FieldOption {
            value: 1082,
            label: "Book of the Dead".to_string(),
        },
        FieldOption {
            value: 1083,
            label: "Ring of the Dead".to_string(),
        },
        FieldOption {
            value: 1084,
            label: "Ensanguined Rood".to_string(),
        },
        FieldOption {
            value: 1085,
            label: "Seal of Rebirth".to_string(),
        },
        FieldOption {
            value: 1086,
            label: "Void Orb".to_string(),
        },
        FieldOption {
            value: 1087,
            label: "Gale Orb".to_string(),
        },
        FieldOption {
            value: 1088,
            label: "Dust Orb".to_string(),
        },
        FieldOption {
            value: 1089,
            label: "Storm Orb".to_string(),
        },
        FieldOption {
            value: 1090,
            label: "Cataract Orb".to_string(),
        },
        FieldOption {
            value: 1091,
            label: "Inferno Orb".to_string(),
        },
        FieldOption {
            value: 1092,
            label: "Black Ice Orb".to_string(),
        },
        FieldOption {
            value: 1093,
            label: "Radiant Orb".to_string(),
        },
        FieldOption {
            value: 1094,
            label: "Gloom Orb".to_string(),
        },
        FieldOption {
            value: 1095,
            label: "Elixir".to_string(),
        },
        FieldOption {
            value: 1096,
            label: "Charm of Remission".to_string(),
        },
        FieldOption {
            value: 1097,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1098,
            label: "Glass Pumpkin".to_string(),
        },
        FieldOption {
            value: 1099,
            label: "Heaven's Fork".to_string(),
        },
        FieldOption {
            value: 1100,
            label: "Intelligence Card".to_string(),
        },
        FieldOption {
            value: 1101,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1102,
            label: "MP Card".to_string(),
        },
        FieldOption {
            value: 1103,
            label: "HP Card".to_string(),
        },
        FieldOption {
            value: 1104,
            label: "Mind Card".to_string(),
        },
        FieldOption {
            value: 1105,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1106,
            label: "Strength Card".to_string(),
        },
        FieldOption {
            value: 1107,
            label: "Strength Card (2)".to_string(),
        },
        FieldOption {
            value: 1108,
            label: "Intelligence Card (2)".to_string(),
        },
        FieldOption {
            value: 1109,
            label: "Avoidance Card".to_string(),
        },
        FieldOption {
            value: 1110,
            label: "Avoidance Card (2)".to_string(),
        },
        FieldOption {
            value: 1111,
            label: "Vitality Card".to_string(),
        },
        FieldOption {
            value: 1112,
            label: "Luck Card".to_string(),
        },
        FieldOption {
            value: 1113,
            label: "Resistance Card".to_string(),
        },
        FieldOption {
            value: 1114,
            label: "Luck Card (2)".to_string(),
        },
        FieldOption {
            value: 1115,
            label: "Vitality Card (2)".to_string(),
        },
        FieldOption {
            value: 1116,
            label: "Dexterity Card".to_string(),
        },
        FieldOption {
            value: 1117,
            label: "Agility Card".to_string(),
        },
        FieldOption {
            value: 1118,
            label: "Agility Card (2)".to_string(),
        },
        FieldOption {
            value: 1119,
            label: "Dexterity Card (2)".to_string(),
        },
        FieldOption {
            value: 1120,
            label: "Resistance Card (2)".to_string(),
        },
        FieldOption {
            value: 1121,
            label: "Loyalty Card".to_string(),
        },
        FieldOption {
            value: 1122,
            label: "None".to_string(),
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
            label: "None".to_string(),
        },
        FieldOption {
            value: 1146,
            label: "Copper Oberynth".to_string(),
        },
        FieldOption {
            value: 1147,
            label: "Bronze Oberynth".to_string(),
        },
        FieldOption {
            value: 1148,
            label: "Silver Oberynth".to_string(),
        },
        FieldOption {
            value: 1149,
            label: "Gold Oberynth".to_string(),
        },
        FieldOption {
            value: 1150,
            label: "Platinum Oberynth".to_string(),
        },
        FieldOption {
            value: 1151,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1152,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1153,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1154,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1155,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1156,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1157,
            label: "Warrior's Mark".to_string(),
        },
        FieldOption {
            value: 1158,
            label: "Archer's Mark".to_string(),
        },
        FieldOption {
            value: 1159,
            label: "Mage's Mark".to_string(),
        },
        FieldOption {
            value: 1160,
            label: "Sibyl's Mark".to_string(),
        },
        FieldOption {
            value: 1161,
            label: "Mage-Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1162,
            label: "Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1163,
            label: "Dreadknight's Mark".to_string(),
        },
        FieldOption {
            value: 1164,
            label: "Berserker's Mark".to_string(),
        },
        FieldOption {
            value: 1165,
            label: "Swordman's Mark".to_string(),
        },
        FieldOption {
            value: 1166,
            label: "Dragoon's Mark".to_string(),
        },
        FieldOption {
            value: 1167,
            label: "Ninja's Mark".to_string(),
        },
        FieldOption {
            value: 1168,
            label: "Bandit's Mark".to_string(),
        },
        FieldOption {
            value: 1169,
            label: "Beastmaster's Mark".to_string(),
        },
        FieldOption {
            value: 1170,
            label: "Fusilier's Mark".to_string(),
        },
        FieldOption {
            value: 1171,
            label: "Magus's Mark".to_string(),
        },
        FieldOption {
            value: 1172,
            label: "Necroprentice's Mark".to_string(),
        },
        FieldOption {
            value: 1173,
            label: "Footsoldier's Mark".to_string(),
        },
        FieldOption {
            value: 1174,
            label: "Juggernaut's Mark".to_string(),
        },
        FieldOption {
            value: 1175,
            label: "Chief's Mark".to_string(),
        },
        FieldOption {
            value: 1176,
            label: "Familiar's Mark".to_string(),
        },
        FieldOption {
            value: 1177,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1178,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1179,
            label: "Windwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1180,
            label: "Cragwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1181,
            label: "Stormwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1182,
            label: "Waterwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1183,
            label: "Firewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1184,
            label: "Icewyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1185,
            label: "Gleamwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1186,
            label: "Gloomwyrm's Mark".to_string(),
        },
        FieldOption {
            value: 1187,
            label: "Sandstone's Mark".to_string(),
        },
        FieldOption {
            value: 1188,
            label: "Granite's Mark".to_string(),
        },
        FieldOption {
            value: 1189,
            label: "Black Iron's Mark".to_string(),
        },
        FieldOption {
            value: 1190,
            label: "Magesteel's Mark".to_string(),
        },
        FieldOption {
            value: 1191,
            label: "Sovereign's Mark".to_string(),
        },
        FieldOption {
            value: 1192,
            label: "Brave's Mark".to_string(),
        },
        FieldOption {
            value: 1193,
            label: "Abuna's Mark".to_string(),
        },
        FieldOption {
            value: 1194,
            label: "Princess's Mark".to_string(),
        },
        FieldOption {
            value: 1195,
            label: "Heretic's Mark".to_string(),
        },
        FieldOption {
            value: 1196,
            label: "Holy Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1197,
            label: "Star Seer's Mark".to_string(),
        },
        FieldOption {
            value: 1198,
            label: "Peregrine's Mark".to_string(),
        },
        FieldOption {
            value: 1199,
            label: "White Knight's Mark".to_string(),
        },
        FieldOption {
            value: 1200,
            label: "Oracle's Mark".to_string(),
        },
        FieldOption {
            value: 1201,
            label: "Wicce's Mark".to_string(),
        },
        FieldOption {
            value: 1202,
            label: "Songstress's Mark".to_string(),
        },
        FieldOption {
            value: 1203,
            label: "Hagiaknight's Mark".to_string(),
        },
        FieldOption {
            value: 1204,
            label: "Pirate's Mark".to_string(),
        },
        FieldOption {
            value: 1205,
            label: "Inferior Ore".to_string(),
        },
        FieldOption {
            value: 1206,
            label: "Iron Sand".to_string(),
        },
        FieldOption {
            value: 1207,
            label: "Copper Ore".to_string(),
        },
        FieldOption {
            value: 1208,
            label: "Tin Ore".to_string(),
        },
        FieldOption {
            value: 1209,
            label: "Graphite".to_string(),
        },
        FieldOption {
            value: 1210,
            label: "Iron Ore".to_string(),
        },
        FieldOption {
            value: 1211,
            label: "Silver Ore".to_string(),
        },
        FieldOption {
            value: 1212,
            label: "Baldur Ore".to_string(),
        },
        FieldOption {
            value: 1213,
            label: "Gold Ore".to_string(),
        },
        FieldOption {
            value: 1214,
            label: "Platinum Ore".to_string(),
        },
        FieldOption {
            value: 1215,
            label: "Saltpeter".to_string(),
        },
        FieldOption {
            value: 1216,
            label: "Sulfur".to_string(),
        },
        FieldOption {
            value: 1217,
            label: "Limestone".to_string(),
        },
        FieldOption {
            value: 1218,
            label: "Skyiron".to_string(),
        },
        FieldOption {
            value: 1219,
            label: "Gemstones".to_string(),
        },
        FieldOption {
            value: 1220,
            label: "Krystallos Ore".to_string(),
        },
        FieldOption {
            value: 1221,
            label: "Bronze Ingot".to_string(),
        },
        FieldOption {
            value: 1222,
            label: "Iron Ingot".to_string(),
        },
        FieldOption {
            value: 1223,
            label: "Silver Ingot".to_string(),
        },
        FieldOption {
            value: 1224,
            label: "Baldur Ingot".to_string(),
        },
        FieldOption {
            value: 1225,
            label: "Steel Ingot".to_string(),
        },
        FieldOption {
            value: 1226,
            label: "Hagane Steel".to_string(),
        },
        FieldOption {
            value: 1227,
            label: "Wootz Steel".to_string(),
        },
        FieldOption {
            value: 1228,
            label: "Golden Ingot".to_string(),
        },
        FieldOption {
            value: 1229,
            label: "Platinum Ingot".to_string(),
        },
        FieldOption {
            value: 1230,
            label: "Fiery Gems".to_string(),
        },
        FieldOption {
            value: 1231,
            label: "Verdant Gems".to_string(),
        },
        FieldOption {
            value: 1232,
            label: "Regal Gems".to_string(),
        },
        FieldOption {
            value: 1233,
            label: "White Gems".to_string(),
        },
        FieldOption {
            value: 1234,
            label: "Black Gems".to_string(),
        },
        FieldOption {
            value: 1235,
            label: "Air Krystallos".to_string(),
        },
        FieldOption {
            value: 1236,
            label: "Earth Krystallos".to_string(),
        },
        FieldOption {
            value: 1237,
            label: "Lightning Krystallos".to_string(),
        },
        FieldOption {
            value: 1238,
            label: "Water Krystallos".to_string(),
        },
        FieldOption {
            value: 1239,
            label: "Fire Krystallos".to_string(),
        },
        FieldOption {
            value: 1240,
            label: "Ice Krystallos".to_string(),
        },
        FieldOption {
            value: 1241,
            label: "Light Krystallos".to_string(),
        },
        FieldOption {
            value: 1242,
            label: "Dark Krystallos".to_string(),
        },
        FieldOption {
            value: 1243,
            label: "Toneriwood".to_string(),
        },
        FieldOption {
            value: 1244,
            label: "Birnewood".to_string(),
        },
        FieldOption {
            value: 1245,
            label: "Ananawood".to_string(),
        },
        FieldOption {
            value: 1246,
            label: "Baobawood".to_string(),
        },
        FieldOption {
            value: 1247,
            label: "Beasthide".to_string(),
        },
        FieldOption {
            value: 1248,
            label: "Tannin".to_string(),
        },
        FieldOption {
            value: 1249,
            label: "Leather".to_string(),
        },
        FieldOption {
            value: 1250,
            label: "Parchment".to_string(),
        },
        FieldOption {
            value: 1251,
            label: "Ink".to_string(),
        },
        FieldOption {
            value: 1252,
            label: "Gold Leaf".to_string(),
        },
        FieldOption {
            value: 1253,
            label: "Water".to_string(),
        },
        FieldOption {
            value: 1254,
            label: "Log".to_string(),
        },
        FieldOption {
            value: 1255,
            label: "Bundle of Herbs".to_string(),
        },
        FieldOption {
            value: 1256,
            label: "Herbal Extract".to_string(),
        },
        FieldOption {
            value: 1257,
            label: "Nightshade".to_string(),
        },
        FieldOption {
            value: 1258,
            label: "Nightshade Extract".to_string(),
        },
        FieldOption {
            value: 1259,
            label: "Fruit".to_string(),
        },
        FieldOption {
            value: 1260,
            label: "Spirits".to_string(),
        },
        FieldOption {
            value: 1261,
            label: "Hempen Thread".to_string(),
        },
        FieldOption {
            value: 1262,
            label: "Woolen Thread".to_string(),
        },
        FieldOption {
            value: 1263,
            label: "Cotton Thread".to_string(),
        },
        FieldOption {
            value: 1264,
            label: "Silken Thread".to_string(),
        },
        FieldOption {
            value: 1265,
            label: "Silver Thread".to_string(),
        },
        FieldOption {
            value: 1266,
            label: "Golden Thread".to_string(),
        },
        FieldOption {
            value: 1267,
            label: "Linen".to_string(),
        },
        FieldOption {
            value: 1268,
            label: "Pincord".to_string(),
        },
        FieldOption {
            value: 1269,
            label: "Flannel".to_string(),
        },
        FieldOption {
            value: 1270,
            label: "Velvet".to_string(),
        },
        FieldOption {
            value: 1271,
            label: "Satin".to_string(),
        },
        FieldOption {
            value: 1272,
            label: "Blackpowder".to_string(),
        },
        FieldOption {
            value: 1273,
            label: "Beast Horn".to_string(),
        },
        FieldOption {
            value: 1274,
            label: "Beast Fang".to_string(),
        },
        FieldOption {
            value: 1275,
            label: "Beast Claw".to_string(),
        },
        FieldOption {
            value: 1276,
            label: "Wyrm Fang".to_string(),
        },
        FieldOption {
            value: 1277,
            label: "Wyrm Claw".to_string(),
        },
        FieldOption {
            value: 1278,
            label: "Wyrm Scale".to_string(),
        },
        FieldOption {
            value: 1279,
            label: "Wyrm Horn".to_string(),
        },
        FieldOption {
            value: 1280,
            label: "Wyrm Whisker".to_string(),
        },
        FieldOption {
            value: 1281,
            label: "Wyrm Thighbone".to_string(),
        },
        FieldOption {
            value: 1282,
            label: "Tooth & Claw".to_string(),
        },
        FieldOption {
            value: 1283,
            label: "Unicorn Horn".to_string(),
        },
        FieldOption {
            value: 1284,
            label: "Enchanted Feather".to_string(),
        },
        FieldOption {
            value: 1285,
            label: "Ancient Wood".to_string(),
        },
        FieldOption {
            value: 1286,
            label: "Ancient Bone".to_string(),
        },
        FieldOption {
            value: 1287,
            label: "Orichalcum".to_string(),
        },
        FieldOption {
            value: 1288,
            label: "Daedalus Pinion".to_string(),
        },
        FieldOption {
            value: 1289,
            label: "Daedalus Rack".to_string(),
        },
        FieldOption {
            value: 1290,
            label: "Melee Weapons I".to_string(),
        },
        FieldOption {
            value: 1291,
            label: "Melee Weapons II".to_string(),
        },
        FieldOption {
            value: 1292,
            label: "The Fist".to_string(),
        },
        FieldOption {
            value: 1293,
            label: "Fist Enchiridion".to_string(),
        },
        FieldOption {
            value: 1294,
            label: "The Blade".to_string(),
        },
        FieldOption {
            value: 1295,
            label: "Dagger Enchiridion".to_string(),
        },
        FieldOption {
            value: 1296,
            label: "Sword Enchidirion".to_string(),
        },
        FieldOption {
            value: 1297,
            label: "2-H Sword Enchiridion".to_string(),
        },
        FieldOption {
            value: 1298,
            label: "Axe Spear & Hammer".to_string(),
        },
        FieldOption {
            value: 1299,
            label: "Axe Enchiridion".to_string(),
        },
        FieldOption {
            value: 1300,
            label: "Spear Enchiridion".to_string(),
        },
        FieldOption {
            value: 1301,
            label: "Hammer Enchiridion".to_string(),
        },
        FieldOption {
            value: 1302,
            label: "The Katana".to_string(),
        },
        FieldOption {
            value: 1303,
            label: "Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1304,
            label: "2-H Katana Enchiridion".to_string(),
        },
        FieldOption {
            value: 1305,
            label: "Cudgel & Whip".to_string(),
        },
        FieldOption {
            value: 1306,
            label: "Cudgel Enchiridion".to_string(),
        },
        FieldOption {
            value: 1307,
            label: "Whip Enchiridion".to_string(),
        },
        FieldOption {
            value: 1308,
            label: "Transcription".to_string(),
        },
        FieldOption {
            value: 1309,
            label: "Musical Instruments I".to_string(),
        },
        FieldOption {
            value: 1310,
            label: "Musical Instruments II".to_string(),
        },
        FieldOption {
            value: 1311,
            label: "Ranged Weapons I".to_string(),
        },
        FieldOption {
            value: 1312,
            label: "Ranged Weapons II".to_string(),
        },
        FieldOption {
            value: 1313,
            label: "Ways of the Gerges".to_string(),
        },
        FieldOption {
            value: 1314,
            label: "The Bow".to_string(),
        },
        FieldOption {
            value: 1315,
            label: "Bow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1316,
            label: "The Crossbow".to_string(),
        },
        FieldOption {
            value: 1317,
            label: "Crossbow Enchiridion".to_string(),
        },
        FieldOption {
            value: 1318,
            label: "The Fusil".to_string(),
        },
        FieldOption {
            value: 1319,
            label: "Fusil Enchiridion".to_string(),
        },
        FieldOption {
            value: 1320,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1321,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1322,
            label: "Smithing Armor I".to_string(),
        },
        FieldOption {
            value: 1323,
            label: "Smithing Armor II".to_string(),
        },
        FieldOption {
            value: 1324,
            label: "Armorcraft".to_string(),
        },
        FieldOption {
            value: 1325,
            label: "Shieldcraft".to_string(),
        },
        FieldOption {
            value: 1326,
            label: "Shield Enchiridion".to_string(),
        },
        FieldOption {
            value: 1327,
            label: "Helm Enchiridion".to_string(),
        },
        FieldOption {
            value: 1328,
            label: "Body Armor Enchiridion".to_string(),
        },
        FieldOption {
            value: 1329,
            label: "Armguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1330,
            label: "Legguard Enchiridion".to_string(),
        },
        FieldOption {
            value: 1331,
            label: "Codex of Jewelry I".to_string(),
        },
        FieldOption {
            value: 1332,
            label: "Codex of Jewelry II".to_string(),
        },
        FieldOption {
            value: 1333,
            label: "Codex of Jewelry III".to_string(),
        },
        FieldOption {
            value: 1334,
            label: "Codex of Jewelry IV".to_string(),
        },
        FieldOption {
            value: 1335,
            label: "Codex of Ores".to_string(),
        },
        FieldOption {
            value: 1336,
            label: "Codex of Gems".to_string(),
        },
        FieldOption {
            value: 1337,
            label: "Codex of Timber".to_string(),
        },
        FieldOption {
            value: 1338,
            label: "Codex of Textiles".to_string(),
        },
        FieldOption {
            value: 1339,
            label: "On Medicine I".to_string(),
        },
        FieldOption {
            value: 1340,
            label: "On Medicine II".to_string(),
        },
        FieldOption {
            value: 1341,
            label: "Secrets of the Master".to_string(),
        },
        FieldOption {
            value: 1342,
            label: "Deadshot".to_string(),
        },
        FieldOption {
            value: 1343,
            label: "Deadshot II".to_string(),
        },
        FieldOption {
            value: 1344,
            label: "Deadshot III".to_string(),
        },
        FieldOption {
            value: 1345,
            label: "Deadshot IV".to_string(),
        },
        FieldOption {
            value: 1346,
            label: "Tornado".to_string(),
        },
        FieldOption {
            value: 1347,
            label: "Tornado II".to_string(),
        },
        FieldOption {
            value: 1348,
            label: "Tornado III".to_string(),
        },
        FieldOption {
            value: 1349,
            label: "Tornado IV".to_string(),
        },
        FieldOption {
            value: 1350,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 1351,
            label: "Sylphide II".to_string(),
        },
        FieldOption {
            value: 1352,
            label: "Aeroflux".to_string(),
        },
        FieldOption {
            value: 1353,
            label: "Aeroflux II".to_string(),
        },
        FieldOption {
            value: 1354,
            label: "Instill Air".to_string(),
        },
        FieldOption {
            value: 1355,
            label: "Aeroguard".to_string(),
        },
        FieldOption {
            value: 1356,
            label: "Whirlwind".to_string(),
        },
        FieldOption {
            value: 1357,
            label: "Guarding Gale".to_string(),
        },
        FieldOption {
            value: 1358,
            label: "Balmy Breeze".to_string(),
        },
        FieldOption {
            value: 1359,
            label: "Black Williwaw".to_string(),
        },
        FieldOption {
            value: 1360,
            label: "Vulcan Lance".to_string(),
        },
        FieldOption {
            value: 1361,
            label: "Vulcan Lance II".to_string(),
        },
        FieldOption {
            value: 1362,
            label: "Vulcan Lance III".to_string(),
        },
        FieldOption {
            value: 1363,
            label: "Vulcan Lance IV".to_string(),
        },
        FieldOption {
            value: 1364,
            label: "Cragfall".to_string(),
        },
        FieldOption {
            value: 1365,
            label: "Cragfall II".to_string(),
        },
        FieldOption {
            value: 1366,
            label: "Cragfall III".to_string(),
        },
        FieldOption {
            value: 1367,
            label: "Cragfall IV".to_string(),
        },
        FieldOption {
            value: 1368,
            label: "Gnome".to_string(),
        },
        FieldOption {
            value: 1369,
            label: "Gnome II".to_string(),
        },
        FieldOption {
            value: 1370,
            label: "Earthquake".to_string(),
        },
        FieldOption {
            value: 1371,
            label: "Earthquake II".to_string(),
        },
        FieldOption {
            value: 1372,
            label: "Instill Earth".to_string(),
        },
        FieldOption {
            value: 1373,
            label: "Petroguard".to_string(),
        },
        FieldOption {
            value: 1374,
            label: "Protect".to_string(),
        },
        FieldOption {
            value: 1375,
            label: "Blade Ward".to_string(),
        },
        FieldOption {
            value: 1376,
            label: "Duststrorm".to_string(),
        },
        FieldOption {
            value: 1377,
            label: "Petrifog".to_string(),
        },
        FieldOption {
            value: 1378,
            label: "Lightning Bow".to_string(),
        },
        FieldOption {
            value: 1379,
            label: "Lightning Bow II".to_string(),
        },
        FieldOption {
            value: 1380,
            label: "Lightning Bow III".to_string(),
        },
        FieldOption {
            value: 1381,
            label: "Lightning Bow IV".to_string(),
        },
        FieldOption {
            value: 1382,
            label: "Thunderflare".to_string(),
        },
        FieldOption {
            value: 1383,
            label: "Thunderflare II".to_string(),
        },
        FieldOption {
            value: 1384,
            label: "Thunderflare III".to_string(),
        },
        FieldOption {
            value: 1385,
            label: "Thunderflare IV".to_string(),
        },
        FieldOption {
            value: 1386,
            label: "Thunderbird".to_string(),
        },
        FieldOption {
            value: 1387,
            label: "Thunderbird II".to_string(),
        },
        FieldOption {
            value: 1388,
            label: "Thunderburst".to_string(),
        },
        FieldOption {
            value: 1389,
            label: "Thunderburst II".to_string(),
        },
        FieldOption {
            value: 1390,
            label: "Instill Lightning".to_string(),
        },
        FieldOption {
            value: 1391,
            label: "Electricgaurd".to_string(),
        },
        FieldOption {
            value: 1392,
            label: "Galvanize".to_string(),
        },
        FieldOption {
            value: 1393,
            label: "Stormspark".to_string(),
        },
        FieldOption {
            value: 1394,
            label: "Stunbomb".to_string(),
        },
        FieldOption {
            value: 1395,
            label: "Stunslay".to_string(),
        },
        FieldOption {
            value: 1396,
            label: "Aquablast".to_string(),
        },
        FieldOption {
            value: 1397,
            label: "Aquablast II".to_string(),
        },
        FieldOption {
            value: 1398,
            label: "Aquablast III".to_string(),
        },
        FieldOption {
            value: 1399,
            label: "Aquablast IV".to_string(),
        },
        FieldOption {
            value: 1400,
            label: "Acid Rain".to_string(),
        },
        FieldOption {
            value: 1401,
            label: "Acid Rain II".to_string(),
        },
        FieldOption {
            value: 1402,
            label: "Acid Rain III".to_string(),
        },
        FieldOption {
            value: 1403,
            label: "Acid Rain IV".to_string(),
        },
        FieldOption {
            value: 1404,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 1405,
            label: "Undine II".to_string(),
        },
        FieldOption {
            value: 1406,
            label: "Dread Vapor".to_string(),
        },
        FieldOption {
            value: 1407,
            label: "Dread Vapor II".to_string(),
        },
        FieldOption {
            value: 1408,
            label: "Instill Water".to_string(),
        },
        FieldOption {
            value: 1409,
            label: "Aqaugaurd".to_string(),
        },
        FieldOption {
            value: 1410,
            label: "Quench".to_string(),
        },
        FieldOption {
            value: 1411,
            label: "Stagnate".to_string(),
        },
        FieldOption {
            value: 1412,
            label: "Poison Mist".to_string(),
        },
        FieldOption {
            value: 1413,
            label: "Sludgebind".to_string(),
        },
        FieldOption {
            value: 1414,
            label: "Sparksphere".to_string(),
        },
        FieldOption {
            value: 1415,
            label: "Sparksphere II".to_string(),
        },
        FieldOption {
            value: 1416,
            label: "Sparksphere III".to_string(),
        },
        FieldOption {
            value: 1417,
            label: "Sparksphere IV".to_string(),
        },
        FieldOption {
            value: 1418,
            label: "Firestorm".to_string(),
        },
        FieldOption {
            value: 1419,
            label: "Firestorm II".to_string(),
        },
        FieldOption {
            value: 1420,
            label: "Firestorm III".to_string(),
        },
        FieldOption {
            value: 1421,
            label: "Firestorm IV".to_string(),
        },
        FieldOption {
            value: 1422,
            label: "Salamander".to_string(),
        },
        FieldOption {
            value: 1423,
            label: "Salamander II".to_string(),
        },
        FieldOption {
            value: 1424,
            label: "Supernova".to_string(),
        },
        FieldOption {
            value: 1425,
            label: "Supernova II".to_string(),
        },
        FieldOption {
            value: 1426,
            label: "Instill Fire".to_string(),
        },
        FieldOption {
            value: 1427,
            label: "Pryogaurd".to_string(),
        },
        FieldOption {
            value: 1428,
            label: "Flame Fusion".to_string(),
        },
        FieldOption {
            value: 1429,
            label: "Pyrocrlastic Flow".to_string(),
        },
        FieldOption {
            value: 1430,
            label: "Misery".to_string(),
        },
        FieldOption {
            value: 1431,
            label: "Brimstone".to_string(),
        },
        FieldOption {
            value: 1432,
            label: "Iceblast".to_string(),
        },
        FieldOption {
            value: 1433,
            label: "Iceblast II".to_string(),
        },
        FieldOption {
            value: 1434,
            label: "Iceblast III".to_string(),
        },
        FieldOption {
            value: 1435,
            label: "Iceblast IV".to_string(),
        },
        FieldOption {
            value: 1436,
            label: "Avalanche".to_string(),
        },
        FieldOption {
            value: 1437,
            label: "Avalanche II".to_string(),
        },
        FieldOption {
            value: 1438,
            label: "Avalanche III".to_string(),
        },
        FieldOption {
            value: 1439,
            label: "Avalanche IV".to_string(),
        },
        FieldOption {
            value: 1440,
            label: "Wendigo".to_string(),
        },
        FieldOption {
            value: 1441,
            label: "Wendigo II".to_string(),
        },
        FieldOption {
            value: 1442,
            label: "Ice Requiem".to_string(),
        },
        FieldOption {
            value: 1443,
            label: "Ice Requiem II".to_string(),
        },
        FieldOption {
            value: 1444,
            label: "Instill Ice".to_string(),
        },
        FieldOption {
            value: 1445,
            label: "Frost Gaurd".to_string(),
        },
        FieldOption {
            value: 1446,
            label: "Icy Focus".to_string(),
        },
        FieldOption {
            value: 1447,
            label: "Indomitable Will".to_string(),
        },
        FieldOption {
            value: 1448,
            label: "Numbing Cold".to_string(),
        },
        FieldOption {
            value: 1449,
            label: "Freezing Gust".to_string(),
        },
        FieldOption {
            value: 1450,
            label: "Spiritsurge".to_string(),
        },
        FieldOption {
            value: 1451,
            label: "Spiritsurge II".to_string(),
        },
        FieldOption {
            value: 1452,
            label: "Spiritsurge III".to_string(),
        },
        FieldOption {
            value: 1453,
            label: "Spiritsurge IV".to_string(),
        },
        FieldOption {
            value: 1454,
            label: "Judgement".to_string(),
        },
        FieldOption {
            value: 1455,
            label: "Judgement II".to_string(),
        },
        FieldOption {
            value: 1456,
            label: "Judgement III".to_string(),
        },
        FieldOption {
            value: 1457,
            label: "Judgement IV".to_string(),
        },
        FieldOption {
            value: 1458,
            label: "Wisplight".to_string(),
        },
        FieldOption {
            value: 1459,
            label: "Wisplight II".to_string(),
        },
        FieldOption {
            value: 1460,
            label: "Heavenly Judge".to_string(),
        },
        FieldOption {
            value: 1461,
            label: "Heavenly Judge II".to_string(),
        },
        FieldOption {
            value: 1462,
            label: "Exorcism".to_string(),
        },
        FieldOption {
            value: 1463,
            label: "Exorcism II".to_string(),
        },
        FieldOption {
            value: 1464,
            label: "Instill Light".to_string(),
        },
        FieldOption {
            value: 1465,
            label: "Light guard".to_string(),
        },
        FieldOption {
            value: 1466,
            label: "Silent Light".to_string(),
        },
        FieldOption {
            value: 1467,
            label: "Boon of Swiftness".to_string(),
        },
        FieldOption {
            value: 1468,
            label: "Dispel".to_string(),
        },
        FieldOption {
            value: 1469,
            label: "Awaken".to_string(),
        },
        FieldOption {
            value: 1470,
            label: "Awaken II".to_string(),
        },
        FieldOption {
            value: 1471,
            label: "Innervate".to_string(),
        },
        FieldOption {
            value: 1472,
            label: "Singing Light".to_string(),
        },
        FieldOption {
            value: 1473,
            label: "Awaken Stone".to_string(),
        },
        FieldOption {
            value: 1474,
            label: "Liberate".to_string(),
        },
        FieldOption {
            value: 1475,
            label: "Cleanse".to_string(),
        },
        FieldOption {
            value: 1476,
            label: "Cleanse II".to_string(),
        },
        FieldOption {
            value: 1477,
            label: "Unburden".to_string(),
        },
        FieldOption {
            value: 1478,
            label: "Decurse".to_string(),
        },
        FieldOption {
            value: 1479,
            label: "Hearten".to_string(),
        },
        FieldOption {
            value: 1480,
            label: "Ease I/II".to_string(),
        },
        FieldOption {
            value: 1481,
            label: "Heal".to_string(),
        },
        FieldOption {
            value: 1482,
            label: "Heal II".to_string(),
        },
        FieldOption {
            value: 1483,
            label: "Heal III".to_string(),
        },
        FieldOption {
            value: 1484,
            label: "Heal IV".to_string(),
        },
        FieldOption {
            value: 1485,
            label: "Major Heal".to_string(),
        },
        FieldOption {
            value: 1486,
            label: "Major Heal II".to_string(),
        },
        FieldOption {
            value: 1487,
            label: "Major Heal III".to_string(),
        },
        FieldOption {
            value: 1488,
            label: "Resurrect".to_string(),
        },
        FieldOption {
            value: 1489,
            label: "Resurrect II".to_string(),
        },
        FieldOption {
            value: 1490,
            label: "Word of Pain".to_string(),
        },
        FieldOption {
            value: 1491,
            label: "Word of Pain II".to_string(),
        },
        FieldOption {
            value: 1492,
            label: "Word of Pain III".to_string(),
        },
        FieldOption {
            value: 1493,
            label: "Word of Pain IV".to_string(),
        },
        FieldOption {
            value: 1494,
            label: "Meteor Strike".to_string(),
        },
        FieldOption {
            value: 1495,
            label: "Meteor Strike II".to_string(),
        },
        FieldOption {
            value: 1496,
            label: "Meteor Strike III".to_string(),
        },
        FieldOption {
            value: 1497,
            label: "Meteor Strike IV".to_string(),
        },
        FieldOption {
            value: 1498,
            label: "Hellbound".to_string(),
        },
        FieldOption {
            value: 1499,
            label: "Hellbound II".to_string(),
        },
        FieldOption {
            value: 1500,
            label: "Abyss".to_string(),
        },
        FieldOption {
            value: 1501,
            label: "Abyss II".to_string(),
        },
        FieldOption {
            value: 1502,
            label: "Drain Heart".to_string(),
        },
        FieldOption {
            value: 1503,
            label: "Drain Mind".to_string(),
        },
        FieldOption {
            value: 1504,
            label: "Drain Power".to_string(),
        },
        FieldOption {
            value: 1505,
            label: "Instill Shadow".to_string(),
        },
        FieldOption {
            value: 1506,
            label: "Shadow Gaurd".to_string(),
        },
        FieldOption {
            value: 1507,
            label: "Spellcharge".to_string(),
        },
        FieldOption {
            value: 1508,
            label: "Paradigm Shift".to_string(),
        },
        FieldOption {
            value: 1509,
            label: "Torpor".to_string(),
        },
        FieldOption {
            value: 1510,
            label: "Petriburst".to_string(),
        },
        FieldOption {
            value: 1511,
            label: "Paralytic Wave".to_string(),
        },
        FieldOption {
            value: 1512,
            label: "Poison Cloud".to_string(),
        },
        FieldOption {
            value: 1513,
            label: "Deadly Poison".to_string(),
        },
        FieldOption {
            value: 1514,
            label: "Sleep".to_string(),
        },
        FieldOption {
            value: 1515,
            label: "Charm".to_string(),
        },
        FieldOption {
            value: 1516,
            label: "Dominate".to_string(),
        },
        FieldOption {
            value: 1517,
            label: "Shackle".to_string(),
        },
        FieldOption {
            value: 1518,
            label: "Fixate".to_string(),
        },
        FieldOption {
            value: 1519,
            label: "Gravity Flux".to_string(),
        },
        FieldOption {
            value: 1520,
            label: "Deadscream".to_string(),
        },
        FieldOption {
            value: 1521,
            label: "Dead Mans Ivy".to_string(),
        },
        FieldOption {
            value: 1522,
            label: "Tempest".to_string(),
        },
        FieldOption {
            value: 1523,
            label: "Tempest II".to_string(),
        },
        FieldOption {
            value: 1524,
            label: "Gaia Strike".to_string(),
        },
        FieldOption {
            value: 1525,
            label: "Gaia Strike II".to_string(),
        },
        FieldOption {
            value: 1526,
            label: "Vortex".to_string(),
        },
        FieldOption {
            value: 1527,
            label: "Vortex II".to_string(),
        },
        FieldOption {
            value: 1528,
            label: "Deluge".to_string(),
        },
        FieldOption {
            value: 1529,
            label: "Deluge II".to_string(),
        },
        FieldOption {
            value: 1530,
            label: "Annihilation".to_string(),
        },
        FieldOption {
            value: 1531,
            label: "Annihilation II".to_string(),
        },
        FieldOption {
            value: 1532,
            label: "Iceover".to_string(),
        },
        FieldOption {
            value: 1533,
            label: "Iceover II".to_string(),
        },
        FieldOption {
            value: 1534,
            label: "Starfall".to_string(),
        },
        FieldOption {
            value: 1535,
            label: "Starfall II".to_string(),
        },
        FieldOption {
            value: 1536,
            label: "Diablo's Spite".to_string(),
        },
        FieldOption {
            value: 1537,
            label: "Diablo's Spite II".to_string(),
        },
        FieldOption {
            value: 1538,
            label: "Detect".to_string(),
        },
        FieldOption {
            value: 1539,
            label: "Springboard".to_string(),
        },
        FieldOption {
            value: 1540,
            label: "Teleport".to_string(),
        },
        FieldOption {
            value: 1541,
            label: "Gift of Restoration".to_string(),
        },
        FieldOption {
            value: 1542,
            label: "Gift of Renewal".to_string(),
        },
        FieldOption {
            value: 1543,
            label: "Nullify Strike".to_string(),
        },
        FieldOption {
            value: 1544,
            label: "Negate Spell".to_string(),
        },
        FieldOption {
            value: 1545,
            label: "Dodge Blades".to_string(),
        },
        FieldOption {
            value: 1546,
            label: "Ballistics".to_string(),
        },
        FieldOption {
            value: 1547,
            label: "Enlighten".to_string(),
        },
        FieldOption {
            value: 1548,
            label: "Phantom Shell".to_string(),
        },
        FieldOption {
            value: 1549,
            label: "Holy Shield".to_string(),
        },
        FieldOption {
            value: 1550,
            label: "Sacrifice".to_string(),
        },
        FieldOption {
            value: 1551,
            label: "Living Corpse".to_string(),
        },
        FieldOption {
            value: 1552,
            label: "Banish".to_string(),
        },
        FieldOption {
            value: 1553,
            label: "Curse".to_string(),
        },
        FieldOption {
            value: 1554,
            label: "Curse II".to_string(),
        },
        FieldOption {
            value: 1555,
            label: "Curse III".to_string(),
        },
        FieldOption {
            value: 1556,
            label: "Tainted Love".to_string(),
        },
        FieldOption {
            value: 1557,
            label: "Prodigize".to_string(),
        },
        FieldOption {
            value: 1558,
            label: "Breed Suspicion".to_string(),
        },
        FieldOption {
            value: 1559,
            label: "Phantom Pain".to_string(),
        },
        FieldOption {
            value: 1560,
            label: "Life Force".to_string(),
        },
        FieldOption {
            value: 1561,
            label: "Putrify".to_string(),
        },
        FieldOption {
            value: 1562,
            label: "Putrify II".to_string(),
        },
        FieldOption {
            value: 1563,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1564,
            label: "Summon Darkness".to_string(),
        },
        FieldOption {
            value: 1565,
            label: "Black Plume".to_string(),
        },
        FieldOption {
            value: 1566,
            label: "Styx Shift".to_string(),
        },
        FieldOption {
            value: 1567,
            label: "Wind Dervish".to_string(),
        },
        FieldOption {
            value: 1568,
            label: "Wind Dervish II".to_string(),
        },
        FieldOption {
            value: 1569,
            label: "Sand Spider".to_string(),
        },
        FieldOption {
            value: 1570,
            label: "Sand Spider II".to_string(),
        },
        FieldOption {
            value: 1571,
            label: "Chimaera".to_string(),
        },
        FieldOption {
            value: 1572,
            label: "Chimaera II".to_string(),
        },
        FieldOption {
            value: 1573,
            label: "Water Tiger".to_string(),
        },
        FieldOption {
            value: 1574,
            label: "Water Tiger II".to_string(),
        },
        FieldOption {
            value: 1575,
            label: "Fire Snake".to_string(),
        },
        FieldOption {
            value: 1576,
            label: "Fire Snake II".to_string(),
        },
        FieldOption {
            value: 1577,
            label: "Rime Raven".to_string(),
        },
        FieldOption {
            value: 1578,
            label: "Rime Raven II".to_string(),
        },
        FieldOption {
            value: 1579,
            label: "Shadowbind".to_string(),
        },
        FieldOption {
            value: 1580,
            label: "Leaping Monkey".to_string(),
        },
        FieldOption {
            value: 1581,
            label: "Waterstep".to_string(),
        },
        FieldOption {
            value: 1582,
            label: "Decoy".to_string(),
        },
        FieldOption {
            value: 1583,
            label: "Bridle".to_string(),
        },
        FieldOption {
            value: 1584,
            label: "Benumb".to_string(),
        },
        FieldOption {
            value: 1585,
            label: "Envenom".to_string(),
        },
        FieldOption {
            value: 1586,
            label: "Lion Dance".to_string(),
        },
        FieldOption {
            value: 1587,
            label: "Harvest Dance".to_string(),
        },
        FieldOption {
            value: 1588,
            label: "Bellows Dance".to_string(),
        },
        FieldOption {
            value: 1589,
            label: "Shriving Dance".to_string(),
        },
        FieldOption {
            value: 1590,
            label: "Comely Dance".to_string(),
        },
        FieldOption {
            value: 1591,
            label: "Bedeviling Dance".to_string(),
        },
        FieldOption {
            value: 1592,
            label: "Invirogating Dance".to_string(),
        },
        FieldOption {
            value: 1593,
            label: "Demonpetal Dance".to_string(),
        },
        FieldOption {
            value: 1594,
            label: "Ardent Conga".to_string(),
        },
        FieldOption {
            value: 1595,
            label: "Weakening Joropo".to_string(),
        },
        FieldOption {
            value: 1596,
            label: "Taunting Mambo".to_string(),
        },
        FieldOption {
            value: 1597,
            label: "Stiring Folclore".to_string(),
        },
        FieldOption {
            value: 1598,
            label: "Somber Chacarera".to_string(),
        },
        FieldOption {
            value: 1599,
            label: "Escalating Sanat".to_string(),
        },
        FieldOption {
            value: 1600,
            label: "Poised Arabesque".to_string(),
        },
        FieldOption {
            value: 1601,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1602,
            label: "None".to_string(),
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
            label: "Max HP Charm".to_string(),
        },
        FieldOption {
            value: 1614,
            label: "Max MP Charm".to_string(),
        },
        FieldOption {
            value: 1615,
            label: "Air Charm".to_string(),
        },
        FieldOption {
            value: 1616,
            label: "Earth Charm".to_string(),
        },
        FieldOption {
            value: 1617,
            label: "Lightening Charm".to_string(),
        },
        FieldOption {
            value: 1618,
            label: "Water Charm".to_string(),
        },
        FieldOption {
            value: 1619,
            label: "Fire Charm".to_string(),
        },
        FieldOption {
            value: 1620,
            label: "Ice Charm".to_string(),
        },
        FieldOption {
            value: 1621,
            label: "Light Charm".to_string(),
        },
        FieldOption {
            value: 1622,
            label: "Dark Charm".to_string(),
        },
        FieldOption {
            value: 1623,
            label: "Experience Charm".to_string(),
        },
        FieldOption {
            value: 1624,
            label: "Experience Charm II".to_string(),
        },
        FieldOption {
            value: 1625,
            label: "Experience Charm III".to_string(),
        },
        FieldOption {
            value: 1626,
            label: "Experience Charm IV".to_string(),
        },
        FieldOption {
            value: 1627,
            label: "Experience Charm V".to_string(),
        },
        FieldOption {
            value: 1628,
            label: "Level Up Charm".to_string(),
        },
    ]
}

fn opt_background() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Base".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Upgraded +1".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Upgraded +2".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Special".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Relic".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Cursed".to_string(),
        },
    ]
}

fn opt_item_unknown() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Use Items".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Debuff Items".to_string(),
        },
        FieldOption {
            value: 250,
            label: "Most Items".to_string(),
        },
    ]
}

fn opt_crafting_items() -> Vec<FieldOption> {
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
            label: "Magus's Mark".to_string(),
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

fn opt_charm_type() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Level".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Experience".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Stat".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Element".to_string(),
        },
    ]
}

fn opt_charm_value() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "HP/None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "MP/Air".to_string(),
        },
        FieldOption {
            value: 2,
            label: "STR/Earth".to_string(),
        },
        FieldOption {
            value: 3,
            label: "VIT/Lightning".to_string(),
        },
        FieldOption {
            value: 4,
            label: "DEX/Water".to_string(),
        },
        FieldOption {
            value: 5,
            label: "AGI/Fire".to_string(),
        },
        FieldOption {
            value: 6,
            label: "AVD/Ice".to_string(),
        },
        FieldOption {
            value: 7,
            label: "INT/Light".to_string(),
        },
        FieldOption {
            value: 8,
            label: "MND/Dark".to_string(),
        },
        FieldOption {
            value: 9,
            label: "RES".to_string(),
        },
        FieldOption {
            value: 10,
            label: "LUK".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Loyalty".to_string(),
        },
    ]
}

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "items".to_string(),
        name: "Item Editor".to_string(),
        description: "Edit consumable item properties".to_string(),
        base_offset: 0x3B0460,
        entry_count: 630,
        entry_size: 56,
        entry_names: vec![
            "Mend Leaf".to_string(),
            "Mend Leaf +1".to_string(),
            "Mend Leaf +2".to_string(),
            "None".to_string(),
            "Mending Seed".to_string(),
            "Mending Seed +1".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Mending Salve".to_string(),
            "Mending Salve +1".to_string(),
            "None".to_string(),
            "Mending Essence".to_string(),
            "Magic Leaf".to_string(),
            "Magic Leaf +1".to_string(),
            "Magic Leaf +2".to_string(),
            "None".to_string(),
            "Magic Seed".to_string(),
            "Magic Seed +1".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Magic Salve".to_string(),
            "Magic Salve +1".to_string(),
            "None".to_string(),
            "Magic Essence".to_string(),
            "Fruit of the Adept".to_string(),
            "Fruit of the Adept +1".to_string(),
            "Fruit of the Sage".to_string(),
            "Fruit of the Sage +1".to_string(),
            "Fruit of the Seraph".to_string(),
            "Overripe Fruit".to_string(),
            "Zolia Draught".to_string(),
            "None".to_string(),
            "Zena Wine".to_string(),
            "Illumina Nectar".to_string(),
            "Gerun Powder".to_string(),
            "Feyrn Bolus".to_string(),
            "Maca Antidote".to_string(),
            "None".to_string(),
            "Jaarn's Poultice".to_string(),
            "Ishtar's Ambrosia".to_string(),
            "Ashmedai's Grog".to_string(),
            "Blessing Stone".to_string(),
            "Hallowing Stone".to_string(),
            "Areion Plume".to_string(),
            "Basin of Time".to_string(),
            "Spiritstone of the Stars".to_string(),
            "Faeriescale Powder".to_string(),
            "Crystallized Flame".to_string(),
            "Mercurial Phial".to_string(),
            "Jewel of the Avatar".to_string(),
            "Hair of the Unicorn".to_string(),
            "Philtre of Ashes".to_string(),
            "Black Lizard Powder".to_string(),
            "None".to_string(),
            "Dragon Steak".to_string(),
            "Braised Skewer".to_string(),
            "Steamed Mollusk".to_string(),
            "Minced Patty".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Brand of the Sacrifice".to_string(),
            "Dynast-King's Mead".to_string(),
            "Echo Stone".to_string(),
            "Blackwing Leg".to_string(),
            "Rood Upright".to_string(),
            "Haunt's Tome".to_string(),
            "Darkscale Tome".to_string(),
            "Cursed Unicorn Blood".to_string(),
            "Skulldust Nostrum".to_string(),
            "Magedrain Gland".to_string(),
            "Shiftstone".to_string(),
            "Horn of the Savage".to_string(),
            "Coral Harp".to_string(),
            "Whirlwind Shot".to_string(),
            "Duststorm Shot".to_string(),
            "Thunder Shot".to_string(),
            "Torrent Shot".to_string(),
            "Conflagration Shot".to_string(),
            "Firnice Shot".to_string(),
            "Coruscate Shot".to_string(),
            "Murk Shot".to_string(),
            "Book of the Dead".to_string(),
            "Ring of the Dead".to_string(),
            "Ensanguined Rood".to_string(),
            "Seal of Rebirth".to_string(),
            "Void Orb".to_string(),
            "Gale Orb".to_string(),
            "Dust Orb".to_string(),
            "Storm Orb".to_string(),
            "Cataract Orb".to_string(),
            "Inferno Orb".to_string(),
            "Black Ice Orb".to_string(),
            "Radiant Orb".to_string(),
            "Gloom Orb".to_string(),
            "Elixir".to_string(),
            "Charm of Remission".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Intelligence Card".to_string(),
            "Mind Card".to_string(),
            "MP Card".to_string(),
            "HP Card".to_string(),
            "Mind Card".to_string(),
            "Loyalty Card".to_string(),
            "Strength Card".to_string(),
            "Strength Card (2)".to_string(),
            "Intelligence Card (2)".to_string(),
            "Avoidance Card".to_string(),
            "Avoidance Card (2)".to_string(),
            "Vitality Card".to_string(),
            "Luck Card".to_string(),
            "Resistance Card".to_string(),
            "Luck Card (2)".to_string(),
            "Vitality Card (2)".to_string(),
            "Dexterity Card".to_string(),
            "Agility Card".to_string(),
            "Agility Card (2)".to_string(),
            "Dexterity Card (2)".to_string(),
            "Resistance Card (2)".to_string(),
            "Loyalty Card".to_string(),
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
            "Copper Oberynth".to_string(),
            "Bronze Oberynth".to_string(),
            "Silver Oberynth".to_string(),
            "Gold Oberynth".to_string(),
            "Platinum Oberynth".to_string(),
            "None".to_string(),
            "Deadshot".to_string(),
            "Deadshot II".to_string(),
            "Deadshot III".to_string(),
            "Deadshot IV".to_string(),
            "Tornado".to_string(),
            "Tornado II".to_string(),
            "Tornado III".to_string(),
            "Tornado IV".to_string(),
            "Sylphide".to_string(),
            "Sylphide II".to_string(),
            "Aeroflux".to_string(),
            "Aeroflux II".to_string(),
            "Instill Air".to_string(),
            "Aeroguard".to_string(),
            "Whirlwind".to_string(),
            "Guarding Gale".to_string(),
            "Balmy Breeze".to_string(),
            "Black Williwaw".to_string(),
            "Vulcan Lance".to_string(),
            "Vulcan Lance II".to_string(),
            "Vulcan Lance III".to_string(),
            "Vulcan Lance IV".to_string(),
            "Cragfall".to_string(),
            "Cragfall II".to_string(),
            "Cragfall III".to_string(),
            "Cragfall IV".to_string(),
            "Gnome".to_string(),
            "Gnome II".to_string(),
            "Earthquake".to_string(),
            "Earthquake II".to_string(),
            "Instill Earth".to_string(),
            "Petroguard".to_string(),
            "Protect".to_string(),
            "Blade Ward".to_string(),
            "Duststrorm".to_string(),
            "Petrifog".to_string(),
            "Lightning Bow".to_string(),
            "Lightning Bow II".to_string(),
            "Lightning Bow III".to_string(),
            "Lightning Bow IV".to_string(),
            "Thunderflare".to_string(),
            "Thunderflare II".to_string(),
            "Thunderflare III".to_string(),
            "Thunderflare IV".to_string(),
            "Thunderbird".to_string(),
            "Thunderbird II".to_string(),
            "Thunderburst".to_string(),
            "Thunderburst II".to_string(),
            "Instill Lightning".to_string(),
            "Electricgaurd".to_string(),
            "Galvanize".to_string(),
            "Stormspark".to_string(),
            "Stunbomb".to_string(),
            "Stunslay".to_string(),
            "Aquablast".to_string(),
            "Aquablast II".to_string(),
            "Aquablast III".to_string(),
            "Aquablast IV".to_string(),
            "Acid Rain".to_string(),
            "Acid Rain II".to_string(),
            "Acid Rain III".to_string(),
            "Acid Rain IV".to_string(),
            "Undine".to_string(),
            "Undine II".to_string(),
            "Dread Vapor".to_string(),
            "Dread Vapor II".to_string(),
            "Instill Water".to_string(),
            "Aqaugaurd".to_string(),
            "Quench".to_string(),
            "Stagnate".to_string(),
            "Poison Mist".to_string(),
            "Sludgebind".to_string(),
            "Sparksphere".to_string(),
            "Sparksphere II".to_string(),
            "Sparksphere III".to_string(),
            "Sparksphere IV".to_string(),
            "Firestorm".to_string(),
            "Firestorm II".to_string(),
            "Firestorm III".to_string(),
            "Firestorm IV".to_string(),
            "Salamander".to_string(),
            "Salamander II".to_string(),
            "Supernova".to_string(),
            "Supernova II".to_string(),
            "Instill Fire".to_string(),
            "Pryogaurd".to_string(),
            "Flame Fusion".to_string(),
            "Pyrocrlastic Flow".to_string(),
            "Misery".to_string(),
            "Brimstone".to_string(),
            "Iceblast".to_string(),
            "Iceblast II".to_string(),
            "Iceblast III".to_string(),
            "Iceblast IV".to_string(),
            "Avalanche".to_string(),
            "Avalanche II".to_string(),
            "Avalanche III".to_string(),
            "Avalanche IV".to_string(),
            "Wendigo".to_string(),
            "Wendigo II".to_string(),
            "Ice Requiem".to_string(),
            "Ice Requiem II".to_string(),
            "Instill Ice".to_string(),
            "Frost Gaurd".to_string(),
            "Icy Focus".to_string(),
            "Indomitable Will".to_string(),
            "Numbing Cold".to_string(),
            "Freezing Gust".to_string(),
            "Spiritsurge".to_string(),
            "Spiritsurge II".to_string(),
            "Spiritsurge III".to_string(),
            "Spiritsurge IV".to_string(),
            "Judgement".to_string(),
            "Judgement II".to_string(),
            "Judgement III".to_string(),
            "Judgement IV".to_string(),
            "Wisplight".to_string(),
            "Wisplight II".to_string(),
            "Heavenly Judge".to_string(),
            "Heavenly Judge II".to_string(),
            "Exorcism".to_string(),
            "Exorcism II".to_string(),
            "Instill Light".to_string(),
            "Light guard".to_string(),
            "Silent Light".to_string(),
            "Boon of Swiftness".to_string(),
            "Dispel".to_string(),
            "Awaken".to_string(),
            "Awaken II".to_string(),
            "Innervate".to_string(),
            "Singing Light".to_string(),
            "Awaken Stone".to_string(),
            "Liberate".to_string(),
            "Cleanse".to_string(),
            "Cleanse II".to_string(),
            "Unburden".to_string(),
            "Decurse".to_string(),
            "Hearten".to_string(),
            "Ease".to_string(),
            "Heal".to_string(),
            "Heal II".to_string(),
            "Heal III".to_string(),
            "Heal IV".to_string(),
            "Major Heal".to_string(),
            "Major Heal II".to_string(),
            "Major Heal III".to_string(),
            "Resurrect".to_string(),
            "Resurrect II".to_string(),
            "Word of Pain".to_string(),
            "Word of Pain II".to_string(),
            "Word of Pain III".to_string(),
            "Word of Pain IV".to_string(),
            "Meteor Strike".to_string(),
            "Meteor Strike II".to_string(),
            "Meteor Strike III".to_string(),
            "Meteor Strike IV".to_string(),
            "Hellbound".to_string(),
            "Hellbound II".to_string(),
            "Abyss".to_string(),
            "Abyss II".to_string(),
            "Drain Heart".to_string(),
            "Drain Mind".to_string(),
            "Drain Power".to_string(),
            "Instill Shadow".to_string(),
            "Shadow Gaurd".to_string(),
            "Spellcharge".to_string(),
            "Paradigm Shift".to_string(),
            "Torpor".to_string(),
            "Petriburst".to_string(),
            "Paralytic Wave".to_string(),
            "Poison Cloud".to_string(),
            "Deadly Poison".to_string(),
            "Sleep".to_string(),
            "Charm".to_string(),
            "Dominate".to_string(),
            "Shackle".to_string(),
            "Fixate".to_string(),
            "Gravity Flux".to_string(),
            "Deadscream".to_string(),
            "Dead Mans Ivy".to_string(),
            "Tempest".to_string(),
            "Tempest II".to_string(),
            "Gaia Strike".to_string(),
            "Gaia Strike II".to_string(),
            "Vortex".to_string(),
            "Vortex II".to_string(),
            "Deluge".to_string(),
            "Deluge II".to_string(),
            "Annihilation".to_string(),
            "Annihilation II".to_string(),
            "Iceover".to_string(),
            "Iceover II".to_string(),
            "Starfall".to_string(),
            "Starfall II".to_string(),
            "Diablo's Spite".to_string(),
            "Diablo's Spite II".to_string(),
            "Palace Guide Book".to_string(),
            "Detect".to_string(),
            "Springboard".to_string(),
            "Teleport".to_string(),
            "Palace Guide Book II".to_string(),
            "Gift of Restoration".to_string(),
            "Gift of Renewal".to_string(),
            "Nullify Strike".to_string(),
            "Negate Spell".to_string(),
            "Dodge Blades".to_string(),
            "Ballistics".to_string(),
            "Enlighten".to_string(),
            "Phantom Shell".to_string(),
            "Holy Shield".to_string(),
            "Sacrifice".to_string(),
            "None".to_string(),
            "Living Corpse".to_string(),
            "Banish".to_string(),
            "Curse".to_string(),
            "Curse II".to_string(),
            "Curse III".to_string(),
            "Tainted Love".to_string(),
            "Prodigize".to_string(),
            "Breed Suspicion".to_string(),
            "Phantom Pain".to_string(),
            "Life Force".to_string(),
            "Putrify".to_string(),
            "Putrify II".to_string(),
            "None".to_string(),
            "Summon Darkness".to_string(),
            "Black Plume".to_string(),
            "Styx Shift".to_string(),
            "Wind Dervish".to_string(),
            "Wind Dervish II".to_string(),
            "Sand Spider".to_string(),
            "Sand Spider II".to_string(),
            "Chimaera".to_string(),
            "Chimaera II".to_string(),
            "Water Tiger".to_string(),
            "Water Tiger II".to_string(),
            "Fire Snake".to_string(),
            "Fire Snake II".to_string(),
            "Rime Raven".to_string(),
            "Rime Raven II".to_string(),
            "Shadowbind".to_string(),
            "Palace Guide Book III".to_string(),
            "Leaping Monkey".to_string(),
            "Waterstep".to_string(),
            "Decoy".to_string(),
            "Bridle".to_string(),
            "Benumb".to_string(),
            "Envenom".to_string(),
            "Lion Dance".to_string(),
            "Harvest Dance".to_string(),
            "Bellows Dance".to_string(),
            "Shriving Dance".to_string(),
            "Comely Dance".to_string(),
            "Bedeviling Dance".to_string(),
            "Invirogating Dance".to_string(),
            "Demonpetal Dance".to_string(),
            "Ardent Conga".to_string(),
            "Weakening Joropo".to_string(),
            "Taunting Mambo".to_string(),
            "Stiring Folclore".to_string(),
            "Somber Chacarera".to_string(),
            "Escalating Sanat".to_string(),
            "Poised Arabesque".to_string(),
            "Glass Pumpkin".to_string(),
            "Heaven's Fork".to_string(),
            "Warrior's Mark".to_string(),
            "Archer's Mark".to_string(),
            "Mage's Mark".to_string(),
            "Sibyl's Mark".to_string(),
            "Mage-Knight's Mark".to_string(),
            "Knight's Mark".to_string(),
            "Dreadknight's Mark".to_string(),
            "Berserker's Mark".to_string(),
            "Swordman's Mark".to_string(),
            "Dragoon's Mark".to_string(),
            "Ninja's Mark".to_string(),
            "Bandit's Mark".to_string(),
            "Fusilier's Mark".to_string(),
            "Beastmaster's Mark".to_string(),
            "Magus's Mark".to_string(),
            "Necroprentice's Mark".to_string(),
            "Footsoldier's Mark".to_string(),
            "Juggernaut's Mark".to_string(),
            "Chief's Mark".to_string(),
            "Familiar's Mark".to_string(),
            "None".to_string(),
            "Windwyrm's Mark".to_string(),
            "Cragwyrm's Mark".to_string(),
            "Stormwyrm's Mark".to_string(),
            "Waterwyrm's Mark".to_string(),
            "Firewyrm's Mark".to_string(),
            "Icewyrm's Mark".to_string(),
            "Gleamwyrm's Mark".to_string(),
            "Gloomwyrm's Mark".to_string(),
            "Sandstone's Mark".to_string(),
            "Granite's Mark".to_string(),
            "Black Iron's Mark".to_string(),
            "Magesteel's Mark".to_string(),
            "Sovereign's Mark".to_string(),
            "Brave's Mark".to_string(),
            "Abuna's Mark".to_string(),
            "Heretic's Mark".to_string(),
            "Princess's Mark".to_string(),
            "Holy Knight's Mark".to_string(),
            "Star Seer's Mark".to_string(),
            "Peregrine's Mark".to_string(),
            "White Knight's Mark".to_string(),
            "Oracle's Mark".to_string(),
            "Wicce's Mark".to_string(),
            "Songstress's Mark".to_string(),
            "Hagiaknight's Mark".to_string(),
            "Pirate's Mark".to_string(),
            "Inferior Ore".to_string(),
            "Iron Sand".to_string(),
            "Copper Ore".to_string(),
            "Tin Ore".to_string(),
            "Graphite".to_string(),
            "Iron Ore".to_string(),
            "Silver Ore".to_string(),
            "Baldur Ore".to_string(),
            "Gold Ore".to_string(),
            "Platinum Ore".to_string(),
            "Saltpeter".to_string(),
            "Sulfur".to_string(),
            "Limestone".to_string(),
            "Skyiron".to_string(),
            "Gemstones".to_string(),
            "Krystallos Ore".to_string(),
            "Bronze Ingot".to_string(),
            "Iron Ingot".to_string(),
            "Silver Ingot".to_string(),
            "Baldur Ingot".to_string(),
            "Steel Ingot".to_string(),
            "Hagane Steel".to_string(),
            "Wootz Steel".to_string(),
            "Golden Ingot".to_string(),
            "Platinum Ingot".to_string(),
            "Fiery Gems".to_string(),
            "Verdant Gems".to_string(),
            "Regal Gems".to_string(),
            "White Gems".to_string(),
            "Black Gems".to_string(),
            "Air Krystallos".to_string(),
            "Earth Krystallos".to_string(),
            "Lightning Krystallos".to_string(),
            "Water Krystallos".to_string(),
            "Fire Krystallos".to_string(),
            "Ice Krystallos".to_string(),
            "Light Krystallos".to_string(),
            "Dark Krystallos".to_string(),
            "Toneriwood".to_string(),
            "Birnewood".to_string(),
            "Ananawood".to_string(),
            "Baobawood".to_string(),
            "Beasthide".to_string(),
            "Tannin".to_string(),
            "Leather".to_string(),
            "Parchment".to_string(),
            "Ink".to_string(),
            "Gold Leaf".to_string(),
            "Water".to_string(),
            "Log".to_string(),
            "Bundle of Herbs".to_string(),
            "Herbal Extract".to_string(),
            "Nightshade".to_string(),
            "Nightshade Extract".to_string(),
            "Fruit".to_string(),
            "Spirits".to_string(),
            "Hempen Thread".to_string(),
            "Woolen Thread".to_string(),
            "Cotton Thread".to_string(),
            "Silken Thread".to_string(),
            "Silver Thread".to_string(),
            "Golden Thread".to_string(),
            "Linen".to_string(),
            "Pincord".to_string(),
            "Flannel".to_string(),
            "Velvet".to_string(),
            "Satin".to_string(),
            "Blackpowder".to_string(),
            "Beast Horn".to_string(),
            "Beast Fang".to_string(),
            "Beast Claw".to_string(),
            "Wyrm Fang".to_string(),
            "Wyrm Claw".to_string(),
            "Wyrm Scale".to_string(),
            "Wyrm Horn".to_string(),
            "Wyrm Whisker".to_string(),
            "Wyrm Thighbone".to_string(),
            "Tooth & Claw".to_string(),
            "Unicorn Horn".to_string(),
            "Enchanted Feather".to_string(),
            "Ancient Wood".to_string(),
            "Ancient Bone".to_string(),
            "Orichalcum".to_string(),
            "None".to_string(),
            "Daedalus Pinion".to_string(),
            "Daedalus Rack".to_string(),
            "Melee Weapons I".to_string(),
            "Melee Weapons II".to_string(),
            "The Fist".to_string(),
            "Fist Enchiridion".to_string(),
            "The Blade".to_string(),
            "Dagger Enchiridion".to_string(),
            "Sword Enchidirion".to_string(),
            "2-H Sword Enchiridion".to_string(),
            "Axe Spear & Hammer".to_string(),
            "Axe Enchiridion".to_string(),
            "Spear Enchiridion".to_string(),
            "Hammer Enchiridion".to_string(),
            "The Katana".to_string(),
            "Katana Enchiridion".to_string(),
            "2-H Katana Enchiridion".to_string(),
            "Cudgel & Whip".to_string(),
            "Cudgel Enchiridion".to_string(),
            "Whip Enchiridion".to_string(),
            "Transcription".to_string(),
            "Musical Instruments I".to_string(),
            "Musical Instruments II".to_string(),
            "Ranged Weapons I".to_string(),
            "Ranged Weapons II".to_string(),
            "Ways of the Gerges".to_string(),
            "The Bow".to_string(),
            "Bow Enchiridion".to_string(),
            "The Crossbow".to_string(),
            "Crossbow Enchiridion".to_string(),
            "The Fusil".to_string(),
            "Fusil Enchiridion".to_string(),
            "None".to_string(),
            "None".to_string(),
            "Smithing Armor I".to_string(),
            "Smithing Armor II".to_string(),
            "Armorcraft".to_string(),
            "Shieldcraft".to_string(),
            "Shield Enchiridion".to_string(),
            "Helm Enchiridion".to_string(),
            "Body Armor Enchiridion".to_string(),
            "Armguard Enchiridion".to_string(),
            "Legguard Enchiridion".to_string(),
            "Codex of Jewelry I".to_string(),
            "Codex of Jewelry II".to_string(),
            "Codex of Jewelry III".to_string(),
            "Codex of Jewelry IV".to_string(),
            "Codex of Ores".to_string(),
            "Codex of Gems".to_string(),
            "Codex of Timber".to_string(),
            "Codex of Textiles".to_string(),
            "On Medicine I".to_string(),
            "On Medicine II".to_string(),
            "Secrets of the Master".to_string(),
            "None".to_string(),
            "Ease II".to_string(),
            "None".to_string(),
            "STR Charm".to_string(),
            "VIT Charm".to_string(),
            "DEX Charm".to_string(),
            "AGI Charm".to_string(),
            "AVD Charm".to_string(),
            "INT Charm".to_string(),
            "MND Charm".to_string(),
            "RES Charm".to_string(),
            "LUK Charm".to_string(),
            "Air Charm".to_string(),
            "Earth Charm".to_string(),
            "Lightening Charm".to_string(),
            "Water Charm".to_string(),
            "Fire Charm".to_string(),
            "Ice Charm".to_string(),
            "Light Charm".to_string(),
            "Dark Charm".to_string(),
            "Experience Charm".to_string(),
            "Experience Charm II".to_string(),
            "Experience Charm III".to_string(),
            "Experience Charm IV".to_string(),
            "Experience Charm V".to_string(),
            "Level Up Charm".to_string(),
            "Grimoire Exorcisme".to_string(),
            "Max HP Charm".to_string(),
            "Max MP Charm".to_string(),
        ],
        fields: vec![
            FieldDefinition {
                name: "Item Catagory".to_string(),
                offset: 2,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_catagory()),
            },
            FieldDefinition {
                name: "Item Effect".to_string(),
                offset: 4,
                size: 2,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Charges".to_string(),
                offset: 6,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Price".to_string(),
                offset: 8,
                size: 2,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Description".to_string(),
                offset: 10,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Rare Item".to_string(),
                offset: 12,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "Prescious Item".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "Throwable".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "AI Throwable".to_string(),
                offset: 16,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "AI Unusable".to_string(),
                offset: 17,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
            },
            FieldDefinition {
                name: "Use Consumable Range".to_string(),
                offset: 18,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Sort ID".to_string(),
                offset: 20,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_names()),
            },
            FieldDefinition {
                name: "Icon Image".to_string(),
                offset: 22,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Icon Image Palette".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Icon Background".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_background()),
            },
            FieldDefinition {
                name: "Icon Foreground".to_string(),
                offset: 26,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "AI Cost".to_string(),
                offset: 27,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_unknown()),
            },
            FieldDefinition {
                name: "Crafting Book".to_string(),
                offset: 30,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_crafting_items()),
            },
            FieldDefinition {
                name: "First Ingriedient".to_string(),
                offset: 32,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_crafting_items()),
            },
            FieldDefinition {
                name: "Second Ingriedient".to_string(),
                offset: 34,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_crafting_items()),
            },
            FieldDefinition {
                name: "Third Ingriedient".to_string(),
                offset: 36,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_crafting_items()),
            },
            FieldDefinition {
                name: "Fourth Ingriedient".to_string(),
                offset: 38,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_crafting_items()),
            },
            FieldDefinition {
                name: "Crafting Success".to_string(),
                offset: 40,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Bonus Stat (Unused)".to_string(),
                offset: 41,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Name".to_string(),
                offset: 42,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Charm Type".to_string(),
                offset: 44,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_charm_type()),
            },
            FieldDefinition {
                name: "Charm Value".to_string(),
                offset: 46,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_charm_value()),
            },
            FieldDefinition {
                name: "Found Flag ID".to_string(),
                offset: 48,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Battle Stage ID".to_string(),
                offset: 50,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Shop Sell %".to_string(),
                offset: 52,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Non Shop Sell %".to_string(),
                offset: 53,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Delay to Targets Turn".to_string(),
                offset: 54,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_yes_no()),
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
        assert_eq!(def.id, "items");
        assert_eq!(def.base_offset, 0x3B0460);
        assert_eq!(def.entry_count, 630);
        assert_eq!(def.entry_size, 56);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 32);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert_eq!(def.entry_names.len(), 630);
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Item Catagory");
        assert_eq!(f.offset, 2);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[31];
        assert_eq!(f.name, "Delay to Targets Turn");
        assert_eq!(f.offset, 54);
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
