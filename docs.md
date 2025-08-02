### Tasks

- [x] Add functionality to read from the DB
- [x] Add functionality to write to the DB
- [ ] Add the functionality to create new DB tables

#### Table Defintion

Build the functionality to create new DB tables

- Each table is it's own DB binary file
- Each DB must have a unique name
- When creating a db user must define a partition index
- User can optionally define a sort index to form a composite key (hash of the partition key and sort key)

#### Set/Insert Command

Build the functionality to insert a record into a specified database

- Keys must be unique
- Each attribute on an insert must have a valid type
- Design a proper binary storage format
- Each record in a table can have multiple attributes
- Attributes do not need to be persisted accross records for a table

### Commands

- Create: creates a new database -> create db_name partition_key sort_key?
  i.e create cart id
  i.e create person id email
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
- Number: N
- Boolean: B

Types of records are specified in the set commands:

- set name 'Luka Piplica' S
- set age 69 N
- set active true B

### Data Storage Format

Structure of how the data is stored for serialization and parsing:

- k1%@%v1%@%S\r\n
- k2%@%v2%@%N\r\n
- k3%@%v3%@%B\r\n
- k4%@%v4%@%S\r\n

### Goals

- The current goal of the codebase is to make it somewhat robust and scalable for future changes
- Design of APIs must be thought with extention in mind

### Roadblocks

- How to convert a String into a string slice?
- View DB data: xxd myfile.db
