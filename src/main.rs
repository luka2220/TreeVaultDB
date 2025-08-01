use std::env;

const GET: &str = "get";
const SET: &str = "set";
const DEL: &str = "del";
const DUMP: &str = "dump";
const COMPACT: &str = "compact";

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).expect("Need a command to continue...");

    let result = match command.as_str() {
        GET => get_operation(),
        SET => set_operation(),
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
fn get_operation() -> i32 {
    10
}

/// Set operation on DB
///
/// sets a key by value
fn set_operation() -> i32 {
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
