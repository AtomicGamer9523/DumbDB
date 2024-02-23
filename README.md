<h1 align="center">DumbDB<br><img width="100px" src="./logo.png"/></h1>

Database for Dummies. A simple, easy-to-use, and lightweight database for small projects.
It is so simple

## API

The API is very simple, it has only 5 commands, and it's very easy to use.

### **D**umb **Q**uery **L**anguage

```sql
-- This is a comment!
-- Comments are ignored by the database and are used to write notes in the code
-- Comments start with two dashes and end with a newline
-- Example: -- This is a comment!

-- The following commands are available:
-- SET, GET, DELETE, CONTAINS, CLEAR
-- All commands are case-insensitive
-- Example: set 0 "Hello"
-- The Database has 4 data types: unsigned integer, string, boolean, and null
-- Null is a special data type that represents the absence of a value
-- It can only be returned by the GET command when the key does not exist
-- Your aren't able to set a null value to a key, that will just delete the key
-- doing so will 

-- Set a value to a key (key is a unsigned integer and value is a string)
SET key value
-- Example: SET 0 "Hello"

-- Get a value from a key
-- If the key does not exist, it will return NULL
GET key
-- Example: GET 0

-- Delete a value from a key
DELETE key
-- Example: DELETE 0

-- Check if a key exists
CONTAINS key
-- Example: CONTAINS 0

-- Clears the database (Deletes all keys and values)
-- !!! WARNING !!! THIS ACTION IS DANGEROUS
CLEAR
```

## Examples

### Example (Python)

```python
from dumbdb import DumbDB

class Message():
    content: str
    user_id: int
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
