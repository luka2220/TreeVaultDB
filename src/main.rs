use std::env;
use std::fs::File;
use std::io::Read;
use std::io::Write;

const GET: &str = "get";
const SET: &str = "set";
const DEL: &str = "del";
const DUMP: &str = "dump";
const COMPACT: &str = "compact";

fn db_read() -> String {
    let mut db = File::open("data/data.db").unwrap_or_else(|error| {
        panic!("Error opening DB: {error:?}")
    });
 
    let mut data = String::new();
    db.read_to_string(&mut data).unwrap_or_else(|error| {
        panic!("Error reading from DB: {error:?}")
    });

    data
}

fn write_to_db(data: String) {
    let mut db = File::create("data/data.db").unwrap_or_else(|error| {
        panic!("Error opening DB: {error:?}")
    });
    db.write_all(&data.into_bytes()).unwrap_or_else(|error| {
        panic!("Error writting data to the DB: {error:?}")
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).expect("Need a command to continue...");

    let result = match command.as_str() {
        GET => get_operation(args),
        SET => set_operation(args),
        DEL => delete_operation(),
        DUMP => dump_operation(),
        COMPACT => compact_operartion(),
        _ => panic!("Unknown Command"),
    };

    println!("Operation Result: {result}");
}

/// Get operation on DB
///
/// gets a value by key name
fn get_operation(args: Vec<String>) -> i32 {
    let data = db_read();
    let data = match data.len() {
        0 => String::from("DB Empty 😢"),
        _ => data
    };

    println!("get oepration asrgs: {:?}", args);
    println!("DB Data -> {data}");

    10
}

/// Set operation on DB
///
/// sets a key by value
fn set_operation(args: Vec<String>) -> i32 {
    let (key, value, t) = match args.len() {
        5 => (args[2].clone(), args[3].clone(), args[4].clone()),
        _ => panic!("Invalid number of arguments, set need's 4 -> [command, key, value, type]")
    };

    let data = key + &value + &t;
    write_to_db(data);

    11
}

/// Delete operation on DB
///
/// deletes an item by key name
fn delete_operation() -> i32 {
    12
}

/// Dump operation on DB
///
/// dumps all of the kv pairs for debugging
fn dump_operation() -> i32 {
    13
}

fn compact_operartion() -> i32 {
    14
}
