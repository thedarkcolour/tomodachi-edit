use std::mem::MaybeUninit;

use serde::{Serialize, Serializer, Deserialize};

// in main.rs
// const SAVE_DATA_SIZE: usize = 1985688;

const MONEY_LOC: usize = 0x1E4BB8; // amount of money held by the island owner
const ISLAND_NAME_LOC: usize = 0x1E4BCC; // name of the island
const ISLAND_NAME_PRONUNCIATION: usize = 0x1E4BF6; // pronunciation of the island's name
const PROBLEMS_SOLVED_LOC: usize = 0x1E4BC6; // number of problems solved by the island owner
const WEDDINGS_LOC: usize = 0x1E4BC0; // number of weddings held on the island
const CHILDREN_BORN_LOC: usize = 0x1E4BC2; // number of children born on the island
const TRAVELLERS_RECEIVED_LOC: usize = 0x1E4BBE; // number of travellers received on the island
const STREET_PASSES_LOC: usize = 0x1E4BBC; // number of StreetPass encounters
const TRAVELLERS_SENT_LOC: usize = 0x1E4BC4; // number of travellers sent from the island
const EVENT_FOUNTAIN_LOC: usize = 0x1E4AF8; // used to reactivate the fountain after an event is completed but still within hours
const APARTMENT_STAGE_LOC: usize = 0x1E4C79; // stage indicating the number of rooms in the apartment building
const LAST_SAVE_DATE_LOC: usize = 0x10; // idk why but the guy has this in a hex format
// for leaderboards, has 3 options: locked, newly unlocked, unlocked
const VITALITY_LEADERBOARD_LOC: usize = 0x1E4C3E;
const POPULARITY_LEADERBOARD_LOC: usize = 0x1E4C3F;
const BOY_CHARM_LEADERBOARD_LOC: usize = 0x1E4C42;
const GIRL_CHARM_LEADERBOARD_LOC: usize = 0x1E4C43;
const PAMPERED_LEADERBOARD_LOC: usize = 0x1E4C44;
const FRIENDSHIP_LEADERBOARD_LOC: usize = 0x1E4C40;
const ROMANCE_LEADERBOARD_LOC: usize = 0x1E4C41;
const ISLAND_LEADERBOARD_LOC: usize = 0x1E4C47;
const TRAVELERS_LEADERBOARD_LOC: usize = 0x1E4C46;
const SPLURGE_LEADERBOARD_LOC: usize = 0x1E4C45;
// island address, which was also in hex on the guy's thing
const ISLAND_ADDRESS_LOC: usize = 0x20;
//const MII_LIST_LOC: usize = 0x1C8A;
const MII_DATA_LOC: usize = 0x1C70;
const MII_DATA_SIZE: usize = 0x660;
const FRIENDS_LOC: usize = 0x299F0;
const FRIENDS_SIZE: usize = 0x100;
const FOODS_INVENTORY_LOC: usize = 0x17F0;

lazy_static::lazy_static! {
    pub(crate) static ref FOOD_REGISTRY: Vec<&'static str> = include_str!("foods.txt").lines().collect();
}

fn read_u8(bytes: &[u8], start_index: usize) -> u8 {
    return bytes[start_index]
}

fn read_u16(bytes: &[u8], start_index: usize) -> u16 {
    (bytes[start_index] as u16) | ((bytes[start_index + 1] as u16) << 8)
}

fn read_u32(bytes: &[u8], start_index: usize) -> u32 {
    (bytes[start_index] as u32) | ((bytes[start_index + 1] as u32) << 8) | ((bytes[start_index + 2] as u32) << 16) | ((bytes[start_index + 3] as u32) << 24)
}

fn read_char_utf16(bytes: &[u8], start_index: usize) -> char {
    let char2 = read_u16(bytes, start_index) as u32;
    char::from_u32(char2).unwrap_or('𖡄')
}

// str_len refers to the number of u16 chars to read (length of 1 would read two u8 bytes)
fn read_string_utf16(bytes: &[u8], start_index: usize, str_len: usize) -> String {
    let mut string = String::with_capacity(str_len);

    for i in 0..str_len {
        let index = start_index + (i * 2);
        let c = read_char_utf16(bytes, index);
        if c == '\0' {
            break;
        }
        string.push(c);
    }

    string
}

fn read_string_hex(bytes: &[u8], start_index: usize, str_len: usize) -> String {
    let mut string = String::with_capacity(str_len);

    for i in 0..str_len {
        let index = start_index + i;
        string.push_str(format!("{:02X}", read_u8(bytes, index)).as_str());
    }

    string
}

// Mii properties
const COPYING_ENABLED_OFFSET: usize = 1;
const NICKNAME_OFFSET: usize = 26;

fn read_miis(save_data: &[u8]) -> [Mii; 100] {
    let mii_data = &save_data[MII_DATA_LOC .. MII_DATA_LOC + (100 * MII_DATA_SIZE)];
    let mut miis = [EMPTY_MII; 100];

    for (mii_index, mii_bytes) in mii_data.chunks_exact(MII_DATA_SIZE).enumerate() {
        let copying_enabled = !read_string_hex(mii_bytes, COPYING_ENABLED_OFFSET, 1).eq("00");
        let nickname = read_string_utf16(mii_bytes, NICKNAME_OFFSET, 10);
        let sharing_enabled = read_string_hex(mii_bytes, 48, 1).eq("00");
        let first_name = read_string_utf16(mii_bytes, 96, 15);
        let last_name = read_string_utf16(mii_bytes, 26 + 102, 15);
        let nickname_pronunciation = read_string_utf16(mii_bytes, 26 + 454, 20);
        let first_name_pronunciation = read_string_utf16(mii_bytes, 26 + 520, 30);
        let last_name_pronunciation = read_string_utf16(mii_bytes, 26 + 586, 30);
        let pampered_rating = read_u32(mii_bytes, 26 + 666);
        let economy_rating = read_u32(mii_bytes, 696);
        let emotions = read_u8(mii_bytes, 26 + 674);
        let relation_to_you = read_u8(mii_bytes, 26 + 675);
        let all_time_favorite = read_u16(mii_bytes, 1580);
        let worst_ever = read_u16(mii_bytes, 1578);
        let super_all_time_favorite = read_u16(mii_bytes, 1576);
        let worst = read_u16(mii_bytes, 1582);

        println!("Mii {} ({}) occupies bytes {:X} to {:X}", mii_index, first_name, MII_DATA_LOC + (MII_DATA_SIZE * mii_index), MII_DATA_LOC + (MII_DATA_SIZE * (mii_index + 1)));

        if !nickname.trim().is_empty() && !nickname.starts_with("\0") {
            let relationships = read_relationships(save_data, mii_index);

            miis[mii_index] = Mii {
                sharing_enabled,
                copying_enabled,
                first_name,
                first_name_pronunciation,
                last_name,
                last_name_pronunciation,
                nickname,
                nickname_pronunciation,
                relationships,
                pampered_rating,
                economy_rating,
                emotions,
                relation_to_you,
                all_time_favorite,
                super_all_time_favorite,
                worst,
                worst_ever,
            };
        }
    };
    return miis;
}

fn read_relationships(save_data: &[u8], mii_index: usize) -> [Relationship; 100] {
    let mut data: [MaybeUninit<Relationship>; 100] = unsafe {
        MaybeUninit::uninit().assume_init()
    };

    let friendships_loc = FRIENDS_LOC + (mii_index * FRIENDS_SIZE);
    let relationships_loc = (FRIENDS_LOC + 100) + (mii_index * FRIENDS_SIZE);

    for (i, elem) in data.iter_mut().enumerate() {
        elem.write(Relationship {
            value: read_u8(save_data, friendships_loc + i), 
            relation: read_u8(save_data, relationships_loc + i),
        });
    }

    unsafe { std::mem::transmute::<_, [Relationship; 100]>(data) }
    
}

fn read_food_items(save_data: &[u8], start_index: usize) -> [u8; 231] {
    let food_items = &save_data[start_index .. start_index + 231];
    
    for (i, count) in food_items.iter().enumerate() {
        if *count == 253 {
            println!("???x {}", FOOD_REGISTRY[i + 1]);
        } else {
            println!("{:3}x {}", count, FOOD_REGISTRY[i + 1]);
        }
    }

    food_items.try_into().expect("Slice with incorrect length")
}

#[derive(Debug, Serialize, Clone)]
pub(crate) struct Island {
    name: String,
    name_pronunciation: String,
    money: u32,
    problems_solved: u16,
    weddings: u16,
    children_born: u16,
    travellers_received: u16,
    streetpass_encounters: u16,
    travellers_sent: u16,
    event_fountain: u32,
    apartment_stage: u8,
    last_save_date: u32,
    vitality_leaderboard: u8,
    popularity_leaderboard: u8,
    boy_charm_leaderboard: u8,
    girl_charm_leaderboard: u8,
    pampered_leaderboard: u8,
    friendship_leaderboard: u8,
    romance_leaderboard: u8,
    island_leaderboard: u8,
    travelers_leaderboard: u8,
    splurge_leaderboard: u8,
    island_address: String,
    #[serde(with = "BigArray")]
    food_items: [u8; 231],
    #[serde(with = "BigArray")]
    miis: [Mii; 100],
}

impl Island {
    pub(crate) fn read(bytes: &[u8]) -> Island {
        Island {
            name: read_string_utf16(&bytes, ISLAND_NAME_LOC, 10),
            name_pronunciation: read_string_utf16(&bytes, ISLAND_NAME_PRONUNCIATION, 20),
            money: read_u32(&bytes, MONEY_LOC),
            problems_solved: read_u16(&bytes, PROBLEMS_SOLVED_LOC),
            weddings: read_u16(&bytes, WEDDINGS_LOC),
            children_born: read_u16(&bytes, CHILDREN_BORN_LOC),
            travellers_received: read_u16(&bytes, TRAVELLERS_RECEIVED_LOC),
            streetpass_encounters: read_u16(&bytes, STREET_PASSES_LOC),
            travellers_sent: read_u16(&bytes, TRAVELLERS_SENT_LOC),
            event_fountain: read_u32(&bytes, EVENT_FOUNTAIN_LOC),
            apartment_stage: read_u8(&bytes, APARTMENT_STAGE_LOC),
            last_save_date: read_u32(&bytes, LAST_SAVE_DATE_LOC),
            vitality_leaderboard: read_u8(&bytes, VITALITY_LEADERBOARD_LOC),
            popularity_leaderboard: read_u8(&bytes, POPULARITY_LEADERBOARD_LOC),
            boy_charm_leaderboard: read_u8(&bytes, BOY_CHARM_LEADERBOARD_LOC),
            girl_charm_leaderboard: read_u8(&bytes, GIRL_CHARM_LEADERBOARD_LOC),
            pampered_leaderboard: read_u8(&bytes, PAMPERED_LEADERBOARD_LOC),
            friendship_leaderboard: read_u8(&bytes, FRIENDSHIP_LEADERBOARD_LOC),
            romance_leaderboard: read_u8(&bytes, ROMANCE_LEADERBOARD_LOC),
            island_leaderboard: read_u8(&bytes, ISLAND_LEADERBOARD_LOC),
            travelers_leaderboard: read_u8(&bytes, TRAVELERS_LEADERBOARD_LOC),
            splurge_leaderboard: read_u8(&bytes, SPLURGE_LEADERBOARD_LOC),
            island_address: read_string_hex(&bytes, ISLAND_ADDRESS_LOC, 16),
            food_items: read_food_items(&bytes, FOODS_INVENTORY_LOC),
            miis: read_miis(&bytes),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
struct Mii {
    sharing_enabled: bool,
    copying_enabled: bool,
    nickname: String,
    nickname_pronunciation: String,
    first_name: String,
    first_name_pronunciation: String,
    last_name: String,
    last_name_pronunciation: String,
    #[serde(with = "BigArray")]
    relationships: [Relationship; 100],
    relation_to_you: u8,
    pampered_rating: u32,
    economy_rating: u32,
    emotions: u8,
    all_time_favorite: u16,
    worst_ever: u16,
    super_all_time_favorite: u16,
    worst: u16,
}

const EMPTY_MII: Mii = Mii {
    nickname: String::new(),
    sharing_enabled: true,
    copying_enabled: false,
    nickname_pronunciation: String::new(),
    first_name: String::new(),
    first_name_pronunciation: String::new(),
    last_name: String::new(),
    last_name_pronunciation: String::new(),
    relationships: [ Relationship { value: 0, relation: 0 }; 100],
    relation_to_you: 0,
    pampered_rating: 0,
    economy_rating: 0,
    emotions: 0,
    all_time_favorite: 0,
    worst_ever: 0,
    super_all_time_favorite: 0,
    worst: 0,
};

#[derive(Debug, Serialize, Clone, Copy)]
struct Relationship {
    value: u8, // number between 0 to 200
    relation: u8,
}

// Relation types
const UNKNOWN: u8 = 0;
const FRIEND: u8 = 1;
const LOVER: u8 = 2;
const EX_LOVER: u8 = 3;
const SPOUSE: u8 = 4;
const SPOUSE_ALT: u8 = 5; // dunno what this is
const EX_SPOUSE: u8 = 6;
const PARENT: u8 = 7;
const SIBLING: u8 = 8;
const FRIEND_IN_CONFLICT: u8 = 9;
const LOVER_IN_CONFLICT: u8 = 10;
const SPOUSE_IN_CONFLICT: u8 = 11;
const BEST_FRIEND: u8 = 12;

// Inspired by https://github.com/est31/serde-big-array
// but without requiring implementation of Deserialize
trait BigArray : Sized {
    fn serialize<S : Serializer>(&self, serializer: S) -> Result::<S::Ok, S::Error>;
}

impl <T : Serialize, const N: usize> BigArray for [T; N] {
    fn serialize<S : Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        let mut seq = serializer.serialize_tuple(self.len())?;
        for elem in &self[..] {
            serde::ser::SerializeTuple::serialize_element(&mut seq, elem)?;
        }
        serde::ser::SerializeTuple::end(seq)
    }
}

// #[derive(Serialize, Deserialize)]
// enum FieldChanged {
//     FirstName { newValue: String },
//     FirstNamePronunciation { newValue: String },
// }