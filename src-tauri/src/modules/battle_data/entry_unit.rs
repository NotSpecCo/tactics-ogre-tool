use crate::modules::types::*;

fn opt_names() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Random".to_string(),
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
            label: "Lanselot".to_string(),
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
            label: "Olivya".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Deneb".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Not Used".to_string(),
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
            label: "Iuriaa".to_string(),
        },
        FieldOption {
            value: 29,
            label: "Cressida".to_string(),
        },
        FieldOption {
            value: 30,
            label: "Not Used".to_string(),
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
            label: "Agorika".to_string(),
        },
        FieldOption {
            value: 36,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 37,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 38,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 39,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 40,
            label: "Punkinryk".to_string(),
        },
        FieldOption {
            value: 41,
            label: "Punkinzepan".to_string(),
        },
        FieldOption {
            value: 42,
            label: "Punkinmila".to_string(),
        },
        FieldOption {
            value: 43,
            label: "Punkinmilian".to_string(),
        },
        FieldOption {
            value: 44,
            label: "Punkinstopheles".to_string(),
        },
        FieldOption {
            value: 45,
            label: "Punkinmunt".to_string(),
        },
        FieldOption {
            value: 46,
            label: "Punkinka".to_string(),
        },
        FieldOption {
            value: 47,
            label: "Punkinovich".to_string(),
        },
        FieldOption {
            value: 48,
            label: "Punkinoff".to_string(),
        },
        FieldOption {
            value: 49,
            label: "Punkinmeyer".to_string(),
        },
        FieldOption {
            value: 50,
            label: "Punkinorz".to_string(),
        },
        FieldOption {
            value: 51,
            label: "Lanselot".to_string(),
        },
        FieldOption {
            value: 52,
            label: "Balxephon".to_string(),
        },
        FieldOption {
            value: 53,
            label: "Volaq".to_string(),
        },
        FieldOption {
            value: 54,
            label: "Barbas".to_string(),
        },
        FieldOption {
            value: 55,
            label: "Martym".to_string(),
        },
        FieldOption {
            value: 56,
            label: "Oz".to_string(),
        },
        FieldOption {
            value: 57,
            label: "Ozma".to_string(),
        },
        FieldOption {
            value: 58,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 59,
            label: "Dorgalua".to_string(),
        },
        FieldOption {
            value: 60,
            label: "Brantyn".to_string(),
        },
        FieldOption {
            value: 61,
            label: "Nybeth".to_string(),
        },
        FieldOption {
            value: 62,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 63,
            label: "Xaebos".to_string(),
        },
        FieldOption {
            value: 64,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 65,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 66,
            label: "Rodrick".to_string(),
        },
        FieldOption {
            value: 67,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 68,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 69,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 70,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 71,
            label: "Marino".to_string(),
        },
        FieldOption {
            value: 72,
            label: "Agares".to_string(),
        },
        FieldOption {
            value: 73,
            label: "Anaberg".to_string(),
        },
        FieldOption {
            value: 74,
            label: "Wynoa".to_string(),
        },
        FieldOption {
            value: 75,
            label: "Vepahl".to_string(),
        },
        FieldOption {
            value: 76,
            label: "Veldrei".to_string(),
        },
        FieldOption {
            value: 77,
            label: "Vermado".to_string(),
        },
        FieldOption {
            value: 78,
            label: "Uram".to_string(),
        },
        FieldOption {
            value: 79,
            label: "Aeshan".to_string(),
        },
        FieldOption {
            value: 80,
            label: "Ehlrig".to_string(),
        },
        FieldOption {
            value: 81,
            label: "Obda".to_string(),
        },
        FieldOption {
            value: 82,
            label: "Orgeau".to_string(),
        },
        FieldOption {
            value: 83,
            label: "Orba".to_string(),
        },
        FieldOption {
            value: 84,
            label: "Kakrinoros".to_string(),
        },
        FieldOption {
            value: 85,
            label: "Gousin".to_string(),
        },
        FieldOption {
            value: 86,
            label: "Farrel".to_string(),
        },
        FieldOption {
            value: 87,
            label: "Gannon".to_string(),
        },
        FieldOption {
            value: 88,
            label: "Roberval".to_string(),
        },
        FieldOption {
            value: 89,
            label: "Garba".to_string(),
        },
        FieldOption {
            value: 90,
            label: "Gatialo".to_string(),
        },
        FieldOption {
            value: 91,
            label: "Latimer".to_string(),
        },
        FieldOption {
            value: 92,
            label: "Genzo".to_string(),
        },
        FieldOption {
            value: 93,
            label: "Xadoba".to_string(),
        },
        FieldOption {
            value: 94,
            label: "Gildora".to_string(),
        },
        FieldOption {
            value: 95,
            label: "Stanoska".to_string(),
        },
        FieldOption {
            value: 96,
            label: "Dagon".to_string(),
        },
        FieldOption {
            value: 97,
            label: "Darza".to_string(),
        },
        FieldOption {
            value: 98,
            label: "Hektor".to_string(),
        },
        FieldOption {
            value: 99,
            label: "Nadia".to_string(),
        },
        FieldOption {
            value: 100,
            label: "Hiram".to_string(),
        },
        FieldOption {
            value: 101,
            label: "Barvan".to_string(),
        },
        FieldOption {
            value: 102,
            label: "Pajeot".to_string(),
        },
        FieldOption {
            value: 103,
            label: "Bapal".to_string(),
        },
        FieldOption {
            value: 104,
            label: "Halphas".to_string(),
        },
        FieldOption {
            value: 105,
            label: "Hanzo".to_string(),
        },
        FieldOption {
            value: 106,
            label: "Bingham".to_string(),
        },
        FieldOption {
            value: 107,
            label: "Fafadayl".to_string(),
        },
        FieldOption {
            value: 108,
            label: "Pherenian".to_string(),
        },
        FieldOption {
            value: 109,
            label: "Felnatorre".to_string(),
        },
        FieldOption {
            value: 110,
            label: "Blackmoor".to_string(),
        },
        FieldOption {
            value: 111,
            label: "Brukatos".to_string(),
        },
        FieldOption {
            value: 112,
            label: "Brezen".to_string(),
        },
        FieldOption {
            value: 113,
            label: "Vestiarri".to_string(),
        },
        FieldOption {
            value: 114,
            label: "Beelzebuth".to_string(),
        },
        FieldOption {
            value: 115,
            label: "Berda".to_string(),
        },
        FieldOption {
            value: 116,
            label: "Botis".to_string(),
        },
        FieldOption {
            value: 117,
            label: "Mercure".to_string(),
        },
        FieldOption {
            value: 118,
            label: "Muntzer".to_string(),
        },
        FieldOption {
            value: 119,
            label: "Modiliani".to_string(),
        },
        FieldOption {
            value: 120,
            label: "Moldova".to_string(),
        },
        FieldOption {
            value: 121,
            label: "Derain".to_string(),
        },
        FieldOption {
            value: 122,
            label: "Ramidos".to_string(),
        },
        FieldOption {
            value: 123,
            label: "Bolis".to_string(),
        },
        FieldOption {
            value: 124,
            label: "LeRozza".to_string(),
        },
        FieldOption {
            value: 125,
            label: "Grion".to_string(),
        },
        FieldOption {
            value: 126,
            label: "Lobos".to_string(),
        },
        FieldOption {
            value: 127,
            label: "Vance".to_string(),
        },
        FieldOption {
            value: 128,
            label: "Banga".to_string(),
        },
        FieldOption {
            value: 129,
            label: "Zanga".to_string(),
        },
        FieldOption {
            value: 130,
            label: "Apollinaire".to_string(),
        },
        FieldOption {
            value: 131,
            label: "Cassandra".to_string(),
        },
        FieldOption {
            value: 132,
            label: "Alessandro".to_string(),
        },
        FieldOption {
            value: 133,
            label: "Sara".to_string(),
        },
        FieldOption {
            value: 134,
            label: "Voltare".to_string(),
        },
        FieldOption {
            value: 135,
            label: "Felicia".to_string(),
        },
        FieldOption {
            value: 136,
            label: "Chamos".to_string(),
        },
        FieldOption {
            value: 137,
            label: "Phaesta".to_string(),
        },
        FieldOption {
            value: 138,
            label: "Tamuz".to_string(),
        },
        FieldOption {
            value: 139,
            label: "Diego".to_string(),
        },
        FieldOption {
            value: 140,
            label: "Mimose".to_string(),
        },
        FieldOption {
            value: 141,
            label: "Cielo".to_string(),
        },
        FieldOption {
            value: 142,
            label: "Georges".to_string(),
        },
        FieldOption {
            value: 143,
            label: "Jilessa".to_string(),
        },
        FieldOption {
            value: 144,
            label: "Reymos".to_string(),
        },
        FieldOption {
            value: 145,
            label: "Berda the Younger".to_string(),
        },
        FieldOption {
            value: 146,
            label: "Obda the Younger".to_string(),
        },
        FieldOption {
            value: 147,
            label: "Ragnar".to_string(),
        },
        FieldOption {
            value: 148,
            label: "Josephine".to_string(),
        },
        FieldOption {
            value: 149,
            label: "Leon".to_string(),
        },
        FieldOption {
            value: 150,
            label: "Alfred".to_string(),
        },
        FieldOption {
            value: 151,
            label: "Punkin".to_string(),
        },
        FieldOption {
            value: 152,
            label: "Rackham".to_string(),
        },
        FieldOption {
            value: 153,
            label: "Aym".to_string(),
        },
        FieldOption {
            value: 154,
            label: "Allocer".to_string(),
        },
        FieldOption {
            value: 155,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 156,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 157,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 158,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 159,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 160,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 161,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 162,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 163,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 164,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 165,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 166,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 167,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 168,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 169,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 170,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 171,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 172,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 173,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 174,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 175,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 176,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 177,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 178,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 179,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 180,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 181,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 182,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 183,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 184,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 185,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 186,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 187,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 188,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 189,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 190,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 191,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 192,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 193,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 194,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 195,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 196,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 197,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 198,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 199,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 200,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 201,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 202,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 203,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 204,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 205,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 206,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 207,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 208,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 209,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 210,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 211,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 212,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 213,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 214,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 215,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 216,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 217,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 218,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 219,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 220,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 221,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 222,
            label: "Not Used".to_string(),
        },
        FieldOption {
            value: 223,
            label: "Sylphide".to_string(),
        },
        FieldOption {
            value: 224,
            label: "Dryad".to_string(),
        },
        FieldOption {
            value: 225,
            label: "Nue".to_string(),
        },
        FieldOption {
            value: 226,
            label: "Undine".to_string(),
        },
        FieldOption {
            value: 227,
            label: "Firbolg".to_string(),
        },
        FieldOption {
            value: 228,
            label: "Centoetl".to_string(),
        },
        FieldOption {
            value: 229,
            label: "Mahn Meko".to_string(),
        },
        FieldOption {
            value: 230,
            label: "Vikrant".to_string(),
        },
        FieldOption {
            value: 231,
            label: "Medusa".to_string(),
        },
        FieldOption {
            value: 232,
            label: "Ylmarch".to_string(),
        },
        FieldOption {
            value: 233,
            label: "Fafnir".to_string(),
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
            label: "Ahurma".to_string(),
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
        FieldOption {
            value: 255,
            label: "Not Used".to_string(),
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

fn opt_alignment() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Law".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Neutral".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Chaos".to_string(),
        },
    ]
}

fn opt_clan() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Walister".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Galgastan".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Bakram".to_string(),
        },
        FieldOption {
            value: 4,
            label: "Xenobia".to_string(),
        },
        FieldOption {
            value: 5,
            label: "Lodis".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Balmoca".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Balboede".to_string(),
        },
    ]
}

fn opt_allegiance() -> Vec<FieldOption> {
    vec![
        FieldOption {
            value: 0,
            label: "None".to_string(),
        },
        FieldOption {
            value: 1,
            label: "Walister Resistance".to_string(),
        },
        FieldOption {
            value: 2,
            label: "Kingdom of Galgastan".to_string(),
        },
        FieldOption {
            value: 3,
            label: "Bakram-Valerian Kingdom".to_string(),
        },
        FieldOption {
            value: 4,
            label: "New Walister Alliance".to_string(),
        },
        FieldOption {
            value: 5,
            label: "The Liberation Front".to_string(),
        },
        FieldOption {
            value: 6,
            label: "Holy Lodissian Empire".to_string(),
        },
        FieldOption {
            value: 7,
            label: "Kingdom of New Xenobia".to_string(),
        },
        FieldOption {
            value: 8,
            label: "Order of Philaha".to_string(),
        },
        FieldOption {
            value: 9,
            label: "Headhunters".to_string(),
        },
        FieldOption {
            value: 10,
            label: "Highwaymen".to_string(),
        },
        FieldOption {
            value: 11,
            label: "Pirates".to_string(),
        },
        FieldOption {
            value: 12,
            label: "Tigers of Burnham".to_string(),
        },
        FieldOption {
            value: 13,
            label: "Remnants".to_string(),
        },
        FieldOption {
            value: 14,
            label: "Commons".to_string(),
        },
        FieldOption {
            value: 15,
            label: "Unknown".to_string(),
        },
        FieldOption {
            value: 16,
            label: "None".to_string(),
        },
        FieldOption {
            value: 17,
            label: "Unknown(Green)".to_string(),
        },
        FieldOption {
            value: 18,
            label: "None".to_string(),
        },
        FieldOption {
            value: 19,
            label: "Player's Clan(Godscale)".to_string(),
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

fn opt_drop_list() -> Vec<FieldOption> {
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

pub fn definition() -> ModuleDefinition {
    ModuleDefinition {
        id: "entry_unit".to_string(),
        name: "Entry Unit Editor".to_string(),
        description: "Edit starting unit configurations for battles".to_string(),
        dat_file: DatFile::BattleData,
        base_offset: 0x2E,
        entry_count: 10,
        entry_size: 196,
        entry_names: vec![],
        fields: vec![
            FieldDefinition {
                name: "Unknown".to_string(),
                offset: 0,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 1".to_string(),
                offset: 1,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Name".to_string(),
                offset: 2,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_names()),
            },
            FieldDefinition {
                name: "Character".to_string(),
                offset: 4,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_character_list()),
            },
            FieldDefinition {
                name: "Class".to_string(),
                offset: 8,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_class_list()),
            },
            FieldDefinition {
                name: "Unknown 9".to_string(),
                offset: 9,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Fixed Level".to_string(),
                offset: 10,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 11".to_string(),
                offset: 11,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "X Coordinate".to_string(),
                offset: 12,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Y Coordinate".to_string(),
                offset: 13,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 14".to_string(),
                offset: 14,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 15".to_string(),
                offset: 15,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 16".to_string(),
                offset: 16,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 17".to_string(),
                offset: 17,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 18".to_string(),
                offset: 18,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 19".to_string(),
                offset: 19,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 20".to_string(),
                offset: 20,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 21".to_string(),
                offset: 21,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Alignment".to_string(),
                offset: 22,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_alignment()),
            },
            FieldDefinition {
                name: "Loyalty".to_string(),
                offset: 23,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Clan".to_string(),
                offset: 24,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_clan()),
            },
            FieldDefinition {
                name: "Allegiance".to_string(),
                offset: 25,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_allegiance()),
            },
            FieldDefinition {
                name: "Unknown 26".to_string(),
                offset: 26,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 27".to_string(),
                offset: 27,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 28".to_string(),
                offset: 28,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
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
                name: "Unknown 34".to_string(),
                offset: 34,
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
                name: "Skill 1".to_string(),
                offset: 36,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Skill 1 Level".to_string(),
                offset: 38,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Skill 2".to_string(),
                offset: 40,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Skill 2 Level".to_string(),
                offset: 42,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Skill 3".to_string(),
                offset: 44,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Skill 3 Level".to_string(),
                offset: 46,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Skill 4".to_string(),
                offset: 48,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_skill_list()),
            },
            FieldDefinition {
                name: "Skill 4 Level".to_string(),
                offset: 50,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 52".to_string(),
                offset: 52,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 53".to_string(),
                offset: 53,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 54".to_string(),
                offset: 54,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 55".to_string(),
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
                name: "Unknown 59".to_string(),
                offset: 59,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 60".to_string(),
                offset: 60,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 61".to_string(),
                offset: 61,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 62".to_string(),
                offset: 62,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 63".to_string(),
                offset: 63,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 64".to_string(),
                offset: 64,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 65".to_string(),
                offset: 65,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 66".to_string(),
                offset: 66,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 67".to_string(),
                offset: 67,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 68".to_string(),
                offset: 68,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 69".to_string(),
                offset: 69,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 70".to_string(),
                offset: 70,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 71".to_string(),
                offset: 71,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 72".to_string(),
                offset: 72,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 73".to_string(),
                offset: 73,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 74".to_string(),
                offset: 74,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 75".to_string(),
                offset: 75,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Right Hand".to_string(),
                offset: 76,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Unknown 78".to_string(),
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
                name: "Left Hand".to_string(),
                offset: 80,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Unknown 82".to_string(),
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
                name: "Helm".to_string(),
                offset: 84,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Unknown 86".to_string(),
                offset: 86,
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
                name: "Armor".to_string(),
                offset: 88,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Unknown 90".to_string(),
                offset: 90,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 91".to_string(),
                offset: 91,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Hands".to_string(),
                offset: 92,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
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
                name: "Legs".to_string(),
                offset: 96,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
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
                name: "Accessory".to_string(),
                offset: 100,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_equipment_list()),
            },
            FieldDefinition {
                name: "Unknown 102".to_string(),
                offset: 102,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 103".to_string(),
                offset: 103,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Item 1".to_string(),
                offset: 104,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Unknown 106".to_string(),
                offset: 106,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 107".to_string(),
                offset: 107,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Item 2".to_string(),
                offset: 108,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Unknown 110".to_string(),
                offset: 110,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 111".to_string(),
                offset: 111,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Item 3".to_string(),
                offset: 112,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Unknown 114".to_string(),
                offset: 114,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 115".to_string(),
                offset: 115,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Item 4".to_string(),
                offset: 116,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_item_list()),
            },
            FieldDefinition {
                name: "Unknown 118".to_string(),
                offset: 118,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 119".to_string(),
                offset: 119,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Item Drop 1".to_string(),
                offset: 120,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_drop_list()),
            },
            FieldDefinition {
                name: "Amount".to_string(),
                offset: 122,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Drop Chance".to_string(),
                offset: 123,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Item Drop 2".to_string(),
                offset: 124,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_drop_list()),
            },
            FieldDefinition {
                name: "Amount".to_string(),
                offset: 126,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Drop Chance".to_string(),
                offset: 127,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Item Drop 3".to_string(),
                offset: 128,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_drop_list()),
            },
            FieldDefinition {
                name: "Amount".to_string(),
                offset: 130,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Drop Chance".to_string(),
                offset: 131,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Item Drop 4".to_string(),
                offset: 132,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_drop_list()),
            },
            FieldDefinition {
                name: "Amount".to_string(),
                offset: 134,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Drop Chance".to_string(),
                offset: 135,
                size: 1,
                field_type: FieldType::Uint,
                options: None,
            },
            FieldDefinition {
                name: "Spell 1".to_string(),
                offset: 136,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Spell 2".to_string(),
                offset: 138,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Spell 3".to_string(),
                offset: 140,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Spell 4".to_string(),
                offset: 142,
                size: 2,
                field_type: FieldType::Dropdown,
                options: Some(opt_spell_list()),
            },
            FieldDefinition {
                name: "Element".to_string(),
                offset: 154,
                size: 1,
                field_type: FieldType::Dropdown,
                options: Some(opt_element()),
            },
            FieldDefinition {
                name: "Unknown 155".to_string(),
                offset: 155,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 156".to_string(),
                offset: 156,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 157".to_string(),
                offset: 157,
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
                name: "Unknown 159".to_string(),
                offset: 159,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 160".to_string(),
                offset: 160,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 161".to_string(),
                offset: 161,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 162".to_string(),
                offset: 162,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 163".to_string(),
                offset: 163,
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
                name: "Unknown 165".to_string(),
                offset: 165,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 166".to_string(),
                offset: 166,
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
                name: "Unknown 168".to_string(),
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
                name: "Unknown 170".to_string(),
                offset: 170,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 171".to_string(),
                offset: 171,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 172".to_string(),
                offset: 172,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 173".to_string(),
                offset: 173,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 174".to_string(),
                offset: 174,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 175".to_string(),
                offset: 175,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 176".to_string(),
                offset: 176,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 177".to_string(),
                offset: 177,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 178".to_string(),
                offset: 178,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 179".to_string(),
                offset: 179,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 180".to_string(),
                offset: 180,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 181".to_string(),
                offset: 181,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 182".to_string(),
                offset: 182,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 183".to_string(),
                offset: 183,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 184".to_string(),
                offset: 184,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 185".to_string(),
                offset: 185,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 186".to_string(),
                offset: 186,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 187".to_string(),
                offset: 187,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 188".to_string(),
                offset: 188,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 189".to_string(),
                offset: 189,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 190".to_string(),
                offset: 190,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 191".to_string(),
                offset: 191,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 192".to_string(),
                offset: 192,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 193".to_string(),
                offset: 193,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 194".to_string(),
                offset: 194,
                size: 1,
                field_type: FieldType::Hex,
                options: None,
            },
            FieldDefinition {
                name: "Unknown 195".to_string(),
                offset: 195,
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
        assert_eq!(def.id, "entry_unit");
        assert_eq!(def.dat_file, DatFile::BattleData);
        assert_eq!(def.base_offset, 0x2E);
        assert_eq!(def.entry_count, 10);
        assert_eq!(def.entry_size, 196);
    }

    #[test]
    fn field_count() {
        assert_eq!(definition().fields.len(), 155);
    }

    #[test]
    fn entry_names_count() {
        let def = definition();
        assert!(def.entry_names.is_empty());
    }

    #[test]
    fn first_field() {
        let f = &definition().fields[0];
        assert_eq!(f.name, "Unknown");
        assert_eq!(f.offset, 0);
        assert_eq!(f.size, 1);
    }

    #[test]
    fn last_field() {
        let f = &definition().fields[154];
        assert_eq!(f.name, "Unknown 195");
        assert_eq!(f.offset, 195);
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
