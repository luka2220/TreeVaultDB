### Tasks

- [x] Add functionality to read from the DB
- [x] Add functionality to write to the DB
- [x] Add the functionality to create new DB tables
- [ ] Build the set command functionality

#### Table Defintion

Build the functionality to create new DB tables

- Each table is it's own DB binary file
- Each DB must have a unique name
- When creating a db user must define a partition index
- store the pk in the database file (the first line in every database file will be configurations)
- User can optionally define a sort index to form a composite key (hash of the partition key and sort key)

### Set/Insert Command

Build the functionality to insert a record into a specified database

- Keys must be unique
- Must contain at least the partition key
- Each attribute on an insert must have a valid type
- Design a proper binary storage format
- Each record in a table can have multiple attributes
- Attributes do not need to be persisted accross records for a table

#### Wire Protocol

- The wire protocol will be JSON similar to dynamoDB. This will allow other languages to talk to the same service.
- JSON will just be used as a transport protocol, not as a storage format

```json
{
  "id": { "S": "SOME-ID" },
  "count": { "N": "12333" }
}
```

- TreeVault running on the server will only take POST requests
- Below is an example with a table name of pets and a pk of AnimalType and sk of Name:

```json
{
  "TableName": "Pets",
  "Key": {
    "AnimalType": { "S": "Dog" },
    "Name": { "S": "Fido" }
  }
}
```

- In the POST request headers the X-Amz-Target will be the DB operation and the Authorization header will be used for authentication to the DB:

Authorization: ...
X-Amz-Target: TreeValutDb.GetItem

For example a database with a pk of id an insert record would look like:

set db_name num_attr pk|value|type attr|value|type attr|value|type

-> set shopping-cart 3 id|jdk-0290-sss|S name|John Smith|S age|45|N

### Commands

- Create: creates a new database -> create db_name partition_key
  i.e create cart id
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
