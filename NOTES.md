<!--https://github.com/paolorechia/steeldb and https://medium.com/@paolorechia/building-a-database-from-scratch-in-rust-part-1-6dfef2223673-->
# Break it down into small steps
what is needed?

If we wanted to do this properly, we would use this:
```
- REPL
- Tokenizer
- Parser
- Code generator
- VM to interperet generated code
- B+ tree
- Pager
- interface
```

We will simplify it to this:
```
- A REPL, with pretty printing of the query result
- A tokenizer and a parser that support the SELECT clause
- A code generator (more precisely, a command generator)
- A virtual machine that interprets the generated code (or commands)
- A table struct that stores data in a memory as a HashMap of Vectors
- A hardcoded table for testing
- Proper error propagation / handling (no panic, all errors are handled/displayed back to the user)
- A serialization / deserialization method to write/read data from file
```

But we don't need to remake/use all that. Instead we will do this:
(* = already made in a different repo by AtomicGamer9523 or Phbar1945)
```
- *A REPL, with pretty printing of the query result [convert GO output from AOC2023 to rust]
- *A tokenizer and a parser that support the SELECT clause [forget where, but you did this before]
- A table struct that stores data in a memory as a HashMap of Vectors
- pager
- *A serialization / deserialization method to write/read data from file [serde]
- A hardcoded table for testing
- Proper error propagation / handling (no panic, all errors are handled/displayed back to the user)
```

What we don't need:
```
- A code generator (more precisely, a command generator)    |   we only have 6 commands
- A hardcoded table for testing                             |   In what world do we test our code?!?! "Push to prod and pray to God."
```
<sup><sup><sup><sup><sup>This is based off of SQLite</sup></sup></sup></sup></sup>