
![image](https://github.com/EloiStree/2024_06_08_MordhauPythonMidi/assets/20149493/6da5ef67-66ed-4b98-bf96-6e62c69add4b)


You will need this python file (for the moment):
https://github.com/EloiStree/2024_06_08_MordhauPythonMidi/blob/main/MordhauConsoleMusicByUDPIID.py

And to learn how to send UDP message.



https://code.visualstudio.com/docs/languages/rust




## Module :)

![image](https://github.com/EloiStree/HelloRustBending/assets/20149493/7784e0d8-1d6e-4eaf-bd91-19b725086421)


```
// utils.rs

pub mod utils {
    pub fn calculate_area(width: f64, height: f64) -> f64 {
        width * height
    }

    pub fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
}
```




--------------

# Pratice


## Learned

### Struct over Class and  composite over inheritence
![image](https://github.com/EloiStree/HelloRustBending/assets/20149493/7aa041bb-a01c-4725-8510-a1604259e464)

Wow  :) beautifull. Let's try it.
``` rust
// Define a trait
trait Summary {
    fn summarize(&self) -> String;
}

// Define a struct
struct Article {
    title: String,
    author: String,
    content: String,
}

// Implement the trait for the struct
impl Summary for Article {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.title, self.author)
    }
}

struct Tweet {
    username: String,
    content: String,
    retweet_count: u32,
}

// Implement the trait for another struct
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

fn main() {
    let article = Article {
        title: String::from("Rust Programming"),
        author: String::from("Jane Doe"),
        content: String::from("Rust is a systems programming language..."),
    };

    let tweet = Tweet {
        username: String::from("@johndoe"),
        content: String::from("Learning Rust is great!"),
        retweet_count: 100,
    };

    notify(&article);
    notify(&tweet);
}
```

### From and string
![image](https://github.com/EloiStree/HelloRustBending/assets/20149493/b7be1462-50a9-4234-b006-a34c009ffd98)

`String::from("Your string") ` very cool :), it store a string as a dynamic arry of unsigned integer if 8 bit.
Just that is proving the quality of Rust language ^^.

It use a Vec. and apparenlty:

```
Vec<u8> in Rust represents a vector of bytes. Here's a breakdown:

Vec: This is a dynamic array or vector in Rust. It can grow or shrink in size as needed and is stored on the heap. It's part of the standard library.
<u8>: This is a type parameter specifying the type of elements the vector holds. In this case, u8 represents an 8-bit unsigned integer, commonly used to represent bytes in Rust.
```



## How to name stuff ?

In Rust, there are conventions for naming functions, variables, and other identifiers that help maintain consistency and readability across codebases. Here are some common conventions:

### Functions
- **Snake Case**: Use lowercase letters and underscores to separate words.
  - Example: `calculate_area`, `process_data`, `parse_input`.

### Variables
- **Snake Case**: Same as functions, use lowercase letters and underscores to separate words.
  - Example: `user_name`, `total_count`, `result_value`.
- **Immutable Variables**: Prefix immutable variables with `let`.
  - Example: `let count = 10;`, `let name = "Alice";`.

### Constants
- **Upper Snake Case**: Use uppercase letters and underscores to separate words.
  - Example: `MAX_VALUE`, `DEFAULT_TIMEOUT`.

### Types
- **Camel Case**: Capitalize the first letter of each word except the first, without underscores.
  - Example: `String`, `HashMap`, `Result`.

### Enums and Structs
- **Pascal Case**: Capitalize the first letter of each word, without underscores.
  - Example: `User`, `HttpRequest`, `LogLevel`.

### Traits
- **Pascal Case**: Same as enums and structs.
  - Example: `Display`, `Clone`, `Serialize`.

### Modules
- **Snake Case**: Lowercase, with underscores separating words.
  - Example: `networking`, `data_processing`.

### Crate Names
- **Snake Case**: Lowercase, with underscores separating words.
  - Example: `rand`, `serde`, `actix_web`.

### Macro Names
- **Snake Case**: Lowercase, with underscores separating words.
  - Example: `println`, `vec`, `format`.

### Lifetimes
- **Short, Descriptive Names**: Single-letter names are common, but use meaningful names if clarity is improved.
  - Example: `'a`, `'input`, `'output`.

### Documentation
- **Comments**: Use `//` for single-line comments and `/* */` for multi-line comments.
- **Doc Comments**: Write documentation using `///` for documenting items and `//!` for documenting the containing item.

These conventions help make Rust code more readable and maintainable, and following them makes your code more idiomatic and understandable to other Rust developers. However, consistency within a project or team is more important than strict adherence to any single convention.



# Check list

# Rust Learning Checklist

## 1. Getting Started with Rust
- [ ] Install Rust using `rustup`
- [ ] Set up development environment (IDE with Rust support)

## 2. The Rust Toolchain
- [ ] Basic `cargo` commands
  - [ ] `cargo new`
  - [ ] `cargo build`
  - [ ] `cargo run`
  - [ ] `cargo test`
  - [ ] `cargo fmt`
  - [ ] `cargo clippy`

## 3. Syntax and Basics
- [ ] Variables, constants, and shadowing
- [ ] Data types (scalar and compound)
  - [ ] Integers, floats, booleans, characters
  - [ ] Tuples and arrays
- [ ] Functions
- [ ] Control flow
  - [ ] `if`, `else`, `else if`
  - [ ] `loop`, `while`, `for`

## 4. Ownership and Memory Management
- [ ] Ownership system
- [ ] Borrowing and references
- [ ] Slices

## 5. Structs and Enums
- [ ] Structs
  - [ ] Defining and instantiating
  - [ ] Method syntax
- [ ] Enums
  - [ ] Defining enums
  - [ ] Pattern matching

## 6. Error Handling
- [ ] Result and Option types
- [ ] Error propagation with `?`
- [ ] Custom error types

## 7. Collections and Iterators
- [ ] Common collections
  - [ ] Vectors
  - [ ] HashMaps
  - [ ] HashSets
- [ ] Iterators
  - [ ] Creating and using iterators
  - [ ] Iterator adaptors
  - [ ] Consuming adaptors

## 8. Traits and Generics
- [ ] Defining and implementing traits
- [ ] Using generics in functions and structs
- [ ] Lifetimes

## 9. Concurrency
- [ ] Basic concurrency with threads
- [ ] Message passing with channels
- [ ] Shared-state concurrency

## 10. Asynchronous Programming
- [ ] `async` and `await` syntax
- [ ] Futures and Streams
- [ ] Using `tokio` and `async-std`

## 11. Advanced Error Handling
- [ ] Error handling with `thiserror`
- [ ] Error handling with `anyhow`

## 12. Smart Pointers
- [ ] `Box`
- [ ] `Rc` and `Arc`
- [ ] `RefCell`

## 13. Macros
- [ ] Declarative macros (`macro_rules!`)
- [ ] Procedural macros

## 14. Unsafe Rust
- [ ] Understanding `unsafe`
- [ ] Raw pointers
- [ ] Unsafe functions and traits

## 15. FFI (Foreign Function Interface)
- [ ] Interfacing with C libraries
- [ ] Creating and using bindings

## 16. WebAssembly (Wasm)
- [ ] Compiling Rust to Wasm
- [ ] Using Wasm in web applications

## 17. Best Practices
- [ ] Code formatting with `rustfmt`
- [ ] Linting with `clippy`
- [ ] Writing tests
  - [ ] Unit tests
  - [ ] Integration tests
  - [ ] Property-based testing with `quickcheck`
- [ ] Writing documentation comments
- [ ] Generating documentation with `cargo doc`
- [ ] Profiling and performance tuning

## 18. Example Projects
- [ ] Building CLI applications with `clap`
- [ ] Web development with frameworks like `Rocket` and `Actix`
- [ ] Systems programming and embedded applications

## 19. Community and Resources
- [ ] Read "The Rust Programming Language" book
- [ ] Complete "Rust by Example"
- [ ] Practice with "Rustlings"
- [ ] Engage with the Rust community (forums, Reddit, Discord)










--------------


--------------


# Reading

Why library won't exist in Rust:
[![image](https://github.com/EloiStree/HelloRustBending/assets/20149493/ef031131-f0a7-439d-8e1b-4c501a576638)](https://youtu.be/769VqNup21Q)  
[https://youtu.be/769VqNup21Q](https://youtu.be/769VqNup21Q)  
- Expect long compile time
- Expect by binary type
- Rust app are just a bit binary of all the package loaded and your code that build in.
- C vs Rust:  https://youtu.be/NtYHC1KNGoc
  - All the world is 'C'
  - Rust is "operating system coding"
  - 


Memory file in Rust is possible ?
- Because I need to send big data from Unity to Rust and Rust to Unity
- https://chatgpt.com/share/acc00a77-e281-4002-9966-750885d929d4



https://youtu.be/UdE8_V05BI8?t=159
- Borrow checker : Who is responsible of a variable
- Can't access out of boundary memory



