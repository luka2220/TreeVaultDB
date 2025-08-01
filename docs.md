### Tasks

- [ ] Add functionality to set key-values in the DB
- [x] Add functionality to read from the DB
- [x] Add functionality to write to the DB

### Commands

- Set: stores a kv pair in the DB -> set key value type
  i.e set age 16 N
- Get: gets a kv pair from the DB -> get key
  i.e get age
- Del: deletes a kv pair from the DB -> del key
  i.e del age
- Dump: displays all the DB data -> dump
  i.e dump

### Storage Types

TreeVault should be able to store the following types of data:

- String: S
- Number N
- Boolean: B

Types of records are specified in the set commands:

- set name 'Luka Piplica' S
- set age 69 N
- set active true B

### Data Storage Format

Structure of how the data is stored for serialization and parsing:

k1%@%v1%@%S\r\n
k2%@%v2%@%N\r\n
k3%@%v3%@%B\r\n
k4%@%v4%@%S\r\n

### Goals

- The current goal of the codebase is to make it somewhat robust and scalable for future changes
- Design of APIs must be thought with extention in mind

### Roadblocks

- How to convert a String into a string slice?
- View DB data: xxd myfile.db
