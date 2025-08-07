use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Read;
use std::io::Write;
use std::process::exit;

// ANSI
const ANSI_BLUE: &str = "\x1b[38;2;0;95;255m";
const ANSI_RED: &str = "\x1b[38;2;217;44;84;255m";
const ANSI_ORANGE: &str = "\x1b[38;2;217;136;44;255m";
const ANSI_END: &str = "\x1b[0m";

// DB Operations
const CREATE: &str = "create";
const GET: &str = "get";
const SET: &str = "set";
const DEL: &str = "del";
const DUMP: &str = "dump";
const COMPACT: &str = "compact";

fn db_read() -> String {
    let mut db = File::open("data/data.db").unwrap_or_else(|error| {
        eprintln!("Error opening DB: {error:?}");
        exit(1)
    });

    let mut data = String::new();
    db.read_to_string(&mut data).unwrap_or_else(|error| {
        eprintln!("Error reading from DB: {error:?}");
        exit(1)
    });

    data
}

/// Writes a record to a specific database
fn write_to_db(db_name: String, data: String) {
    let db_full_path = format!("data/{}.db", db_name);

    let mut db = OpenOptions::new()
        .append(true)
        .open(db_full_path)
        .unwrap_or_else(|error| {
            eprintln!("Unable to open database: {error:?}");
            exit(1)
        });

    db.write(&data.into_bytes()).unwrap_or_else(|error| {
        eprintln!("Error writting data to the DB: {error:?}");
        exit(1)
    });
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).expect("Need a command to continue...");

    let _ = match command.as_str() {
        CREATE => create_db(args),
        GET => get_operation(),
        SET => set_operation(args),
        DEL => delete_operation(),
        DUMP => dump_operation(),
        COMPACT => compact_operartion(),
        _ => panic!("Unknown Command"),
    };

    // println!("Operation Result: {result}");
}

/// Creates a new DB
///
/// constructs a new db which must have a unique name
/// each new db needs a partition key to query the records. Each partition key must be unique
/// a sort key is optional for a composite key structure
fn create_db(args: Vec<String>) -> i32 {
    if args.len() != 4 {
        eprintln!(
            "Invalid number of arguments, create need's 3 -> [command, db_name, partition_key]"
        );
        exit(1)
    }

    let db_path = format!("data/{}.db", &args[2]);
    let mut db = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(db_path)
        .unwrap_or_else(|error| {
            eprintln!("Error creating a new DB table: {error}");
            exit(1)
        });

    let pk = format!("pk={}", &args[3]);
    db.write(&pk.into_bytes()).unwrap_or_else(|error| {
        eprintln!(
            "Invalid number of arguments for the set command -> set db_name n_attr pk: {error}"
        );
        exit(1)
    });

    9
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
fn set_operation(args: Vec<String>) -> i32 {
    if args.len() < 5 {
        eprintln!("Invalid number of arguments for the set command -> set db_name n_attr pk");
        exit(1)
    }

    let db_name = args[2].clone();
    let num_attr: usize = args[3].parse().unwrap_or_else(|error| {
        eprintln!("An error ocurred converting String to i32: {error}");
        exit(1)
    });

    let mut record_data = String::new();

    for i in 1..num_attr + 1 {
        record_data = format!("{}---{}", record_data, &args[i + 3])
    }

    write_to_db(db_name, record_data);

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
    let data = db_read();
    let data = match data.len() {
        0 => String::from("DB Empty 😢"),
        _ => data,
    };

    println!("DB Data:\n");

    let rows: Vec<&str> = data.split("\r\n").collect();

    for row in rows {
        let row_data: Vec<&str> = row.split(" ").collect();

        if row_data.len() < 3 {
            continue;
        }

        let key = row_data[0];
        let data = match row_data.len() {
            3 => row_data[1].to_string(),
            _ => row_data[1..row_data.len() - 1].join(" "),
        };
        let data_type = row_data[row_data.len() - 1];

        let row_dump = format!(
            "{}{}{} {}{}{} {}{}{}",
            ANSI_BLUE, key, ANSI_END, ANSI_RED, data, ANSI_END, ANSI_ORANGE, data_type, ANSI_END
        );
        println!("{row_dump}")
    }

    13
}

fn compact_operartion() -> i32 {
    14
}
