<h1 align="center">DumbDB<br><img width="100px" src="./logo.png"/></h1>

Database for Dummies. A simple, easy-to-use, and lightweight database for small projects.
It is so simple a child could use it.

## API

The API is very simple, it has only 5 commands, and it's very easy to use.

The following commands are available:
[`SET`](#set), [`GET`](#get), [`DELETE`](#delete), [`CONTAINS`](#contains), [`CLEAR`](#clear).
All commands are case-insensitive.
Comments are also supported, they start with two dashes and end with a newline.
The Database has 4 data types: unsigned integer, string, boolean, and [`NULL`](#null).
[`NULL`](#null) is a special data type that represents the absence of a value.
It can only be returned by the [`GET`](#get) command when the key does not exist.
Setting a key to [`NULL`](#null) is equivalent to deleting the key.
You are able to chain commands together, multiple commands can be written in a single line, separated by a semicolon. As an example, `SET 0 "Hello"; GET 0` will set the key `0` to the value `"Hello"` and then get the value of the key `0`.

### **D**umb **Q**uery **L**anguage

```sql
-- This is a comment!
SET key value
GET key
DELETE key
CONTAINS key
CLEAR
```

### `SET`

Sets a value to a key.
If the key already exists, it will overwrite the value.
If the key does not exist, it will create a new key with the value.
If the value is [`NULL`](#null), it will delete the key.

### `GET`

Gets a value from a key.
If the key does not exist, it will return [`NULL`](#null).

### `DELETE`

Deletes a value from a key.
If the key does not exist, it will do nothing.

### `CONTAINS`

Checks if a key exists.
If the key exists, it will return [`TRUE`](#true-and-false), otherwise, it will return [`FALSE`](#true-and-false).

### `CLEAR`

Deletes all keys and values from the database.
This action is dangerous and permanent, use with caution.

### `TRUE` and `FALSE`

[`TRUE`](#true-and-false) and [`FALSE`](#true-and-false) are the only two boolean values in the database.
They are case-insensitive.

### `NULL`

[`NULL`](#null) is a special data type that represents the absence of a value.
It can only be returned by the `GET` command when the key does not exist.
Setting a key to [`NULL`](#null) is equivalent to deleting the key.

## Examples

### Example (Python)

Query:

```python
from dumbdb.query import DumbDB

db = DumbDBQuery("messages")
db.query("SET 0 {'content': 'Hello', 'user_id': 0}")
```

ORM:

```python
from dumbdb.orm import DumbDB

class Message:
    def __init__(self, content: str, user_id: int):
        self.content = content
        self.user_id = user_id
    def __str__(self):
        return f"Message(content={self.content}, user_id={self.user_id})"

db = DumbDB[Message]("messages")
db.set(0, Message("Hello", 0))
print(db.get(0)) # Message(content="Hello", user_id=0)
db.delete(0)
print(db.contains(0)) # False
db.clear()
```

### Example (Rust)

Query:

```rust
use dumbdb::DumbDB;

fn main() {
    let mut db = DumbDB::new("messages");
    db.query("SET 0 {'content': 'Hello', 'user_id': 0}");
}
```

ORM:

```rust
use dumbdb::DumbDB;

#[derive(Debug)]
struct Message {
    content: String,
    user_id: u64,
}

fn main() {
    let mut db = DumbDB::<Message>::new("messages");
    db.set(0, Message { content: "Hello".to_string(), user_id: 0 });
    println!("{:?}", db.get(0)); // Message { content: "Hello", user_id: 0 }
    db.delete(0);
    println!("{}", db.contains(0)); // false
    db.clear();
}
```
