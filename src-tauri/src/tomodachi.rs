use std::fs::File;
use std::io;
use std::io::Read;
use std::os::windows::fs::MetadataExt;
use byteorder::{ByteOrder, LittleEndian};
use native_dialog::FileDialog;

mod tomodachi-edit;

const SAVE_DATA_SIZE: usize = 1985688;

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
const MII_LIST_LOC: usize = 0x1C8A;
const MII_DATA_SIZE: usize = 0x660;

fn main() {
    println!("Reading \"savedataArc.txt\" for bytes...");
    let optional_path = FileDialog::new()
        .set_location("~/Desktop")
        .add_filter("Tomodachi Life Save Data", &["txt"])
        .show_open_single_file()
        .unwrap();

    let food_registry: Vec<&str> = include_str!("foods.txt").lines().collect();

    let mut save_file = if let Some(path) = optional_path {
        let f = File::open(path).expect("File \"{}\" coudl not be opened");

        if f.metadata().unwrap().file_size() == SAVE_DATA_SIZE as u64 {
            f
        } else {
            println!("File size does not match");
            File::open("savedataArc.txt").expect("File \"savedataArc.txt\" not found!")
        }
    } else {
        println!("Reading default data instead...");
        File::open("savedataArc.txt").expect("File \"savedataArc.txt\" not found!")
    };

    let mut save_data_u8 = vec![0_u8; 0];
    save_file.read_to_end(&mut save_data_u8);

    let island = Island {
        name: read_string_utf16(&save_data_u8, ISLAND_NAME_LOC, 10),
        name_pronunciation: read_string_utf16(&save_data_u8, ISLAND_NAME_PRONUNCIATION, 20),
        money: read_u32(&save_data_u8, MONEY_LOC),
        problems_solved: read_u16(&save_data_u8, PROBLEMS_SOLVED_LOC),
        weddings: read_u16(&save_data_u8, WEDDINGS_LOC),
        children_born: read_u16(&save_data_u8, CHILDREN_BORN_LOC),
        travellers_received: read_u16(&save_data_u8, TRAVELLERS_RECEIVED_LOC),
        streetpass_encounters: read_u16(&save_data_u8, STREET_PASSES_LOC),
        travellers_sent: read_u16(&save_data_u8, TRAVELLERS_SENT_LOC),
        event_fountain: read_u32(&save_data_u8, EVENT_FOUNTAIN_LOC),
        apartment_stage: read_u8(&save_data_u8, APARTMENT_STAGE_LOC),
        last_save_date: read_u32(&save_data_u8, LAST_SAVE_DATE_LOC),
        vitality_leaderboard: read_u8(&save_data_u8, VITALITY_LEADERBOARD_LOC),
        popularity_leaderboard: read_u8(&save_data_u8, POPULARITY_LEADERBOARD_LOC),
        boy_charm_leaderboard: read_u8(&save_data_u8, BOY_CHARM_LEADERBOARD_LOC),
        girl_charm_leaderboard: read_u8(&save_data_u8, GIRL_CHARM_LEADERBOARD_LOC),
        pampered_leaderboard: read_u8(&save_data_u8, PAMPERED_LEADERBOARD_LOC),
        friendship_leaderboard: read_u8(&save_data_u8, FRIENDSHIP_LEADERBOARD_LOC),
        romance_leaderboard: read_u8(&save_data_u8, ROMANCE_LEADERBOARD_LOC),
        island_leaderboard: read_u8(&save_data_u8, ISLAND_LEADERBOARD_LOC),
        travelers_leaderboard: read_u8(&save_data_u8, TRAVELERS_LEADERBOARD_LOC),
        splurge_leaderboard: read_u8(&save_data_u8, SPLURGE_LEADERBOARD_LOC),
        island_address: read_string_hex(&save_data_u8, ISLAND_ADDRESS_LOC, 16)
    };

    let mii_data = &save_data_u8[MII_LIST_LOC .. MII_LIST_LOC + (99 * MII_DATA_SIZE)];

    mii_data.chunks_exact(MII_DATA_SIZE).for_each(|bytes| {
        let cursor = 0;
        let name = read_string_utf16(bytes, cursor, 10);

        if !name.trim().is_empty() {
            println!("Found name of Mii: {}", name);
        }
    });

    println!("Island Name: {}", island.name);
    println!("Money: {}", island.format_money());
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

fn read_string_utf16(bytes: &[u8], start_index: usize, str_len: usize) -> String {
    let mut string = String::with_capacity(str_len);

    for i in 0..str_len {
        let index = start_index + (i * 2);
        string.push(read_char_utf16(bytes, index));
    }

    string
}

fn read_string_hex(bytes: &[u8], start_index: usize, str_len: usize) -> String {
    let mut string = String::with_capacity(str_len);

    for i in 0..str_len {
        let index = start_index + i;
        string.push_str(format!("{:02x}", read_u8(bytes, index)).as_str());
    }

    string
}

#[derive(Debug)]
struct Island {
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
}

impl Island {
    fn format_money(&self) -> String {
        let money = self.money;
        format!("Money: ${}.{}{} (parsed from {})", money / 100, (money % 100) / 10, (money % 10), money)
    }
}

struct Mii {
    name: String,
    name_pronunciation: String,
}
