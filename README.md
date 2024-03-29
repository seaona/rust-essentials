# Rust Essentials
Notes based on the [Rust Programming Language book](https://doc.rust-lang.org/book/title-page.html).
## 1. Getting Started
### Basic Commands
- Compile a file: `rustc <<FILE_NAME>>`
- Run: `./<<FILE_NAME>>`

### Cargo
It's Rust's build system and package manager. Main commands:
1. Creating a project with CARGO
    `cargo new hello_cargo`
2. Building a project with CARGO
    `cargo build`
3. Building and running in one command
    `cargo run`
4. Check if the project compiles, but does not creaet an executable (it's much faster)
    `cargo check`

5. Building for release (to compile it with optimizations)
    `cargo build --release`

Generated files and folder structure:
```
src
    main.rs
target
    debug
        ....
        hello_cargo
    ...
.gitignore
Cargo.lock
Cargo.toml
```

- `Cargo.toml`: information about your program (name, version, edition of Rus usesd) and dependencies (crates)
- `Cargo.lock`: keeps track of the exact versions of dependencies in your project
- `src`: where your source program goes

### hello_world
```
fn main() {
    println!("Hello, world!");
}
```
- **Main function**: it's always the first code that runs in every executable of a Rust program
- **Macro**: they don't always follow the same rules as functions. They are called using `!`

## 2. Guessing Game
- `use std::io`: uses the `io` input/output library into scope. The `io` library comes from the standard library, `std`. By default, a set of items from the standard library come on every program (the **prelude**). If it's not there, you need to bring it into the scope explicitly with a `use` statement.
- `let mut guess = String::new()`:
    - we use `let` to create a variable.
    - variables are immutable by default, to make a variable mutable we add `mut` before the variable name
    - `=` we bind something to the variable
    - `::new` this indicates that `new` is an associated function of the `String` type (a function that is implemented on a type). This creates a new empty string
- `io::stdin()`, if we didn't import the `io` library, we could also write `std::io::stdin`
- `.read_line(&mut guess)`: takes the value the user types and appends it into a string, without overwriting its contents. The `&` indicates that this argument is a reference - a way to let multiple parts of the code access one piece of data without needing to copy the data into memory multiple times. **References are immutable by default** so we need to make it mutable.
- `.expect()`: `read_line` also returns a `Result` value - an enumeration, 'enum' whose variants are 'Ok' and 'Err'. Values of the `Result` type also have methods defined on them. An instance of `Result`has an expect method
- `println!("You guessed: {guess}");`: prints the string. {} is a placeholder for the value. You can also do `println!("3 + 2 = {}, 5")`
- `crate`: a collection of Rust source code files. Add dependencies to `Cargo.toml` file and then run `cargo build`. The specifier 0.8.5 is actually shorthand for ^0.8.5, which means any version that is at least 0.8.5 but below 0.9.0.
    - `cargo update`: for updating the version (grater than 0.8.5 and smaller than 0.9.0)
- `use rand:Rng;`: library for generating randomn numbers. The `Rng` is a `trait` and defines methods that random number generators implement, and  this trait must be in scope for us to use those methods.
- `rand::thread_rng().gen_range(1..=100);`: the `thread_rng` function gives a particular random number generator: one that is local to the current thread of execution and is seeded by the operating system. Then we call `gen_range` method, defined by the `Rng` trait that we brought into the scope.
- `use std::cpm::Ordering;` Ordering is another enum with variants `Less`, `Greater` and `Equal`
- `guess.cmp(&secret_number)` compares the two values and returns a variant of the `Ordering` enum
- `match` to decide what to do next based on which variant was returned. It's made up of **arms**. An arm consists of a pattern to match against, and the code that should be run fits the arm's pattern. The `match` expression ends after the first successful match
- `let guess: u32 = guess.trim().parse().expect("Please type a number!");` Rust allows us to shadow the previous value of guess with a new one. The trim will eliminate any whitespace at the beginning and end, which we must do to be able to compare the string to the u32, which can only contain numerical data. The user must press enter to satisfy the `read_line`, which adds a newline character to the string i.e. `5\n`. The trim method eliminates `\n`. The `parse()` method converts the string to another type. `u32` is unsigned 32-bit integer
- `loop {}` creates an infinite loop. Adding `break` makes the program exit the loop.
- `Err(_)` the underscore is a catch-all value, no matter what information they have inside.

## 3. Common Programming Concepts
### Variables and Mutability
- You declare variables using `let`
- By default variables are immutable, but adding `mut` in front of the variable name makes it mutable

#### Constants
- You declare constants using `const` and the type of value must be annotated.
- Constants values that are bound to a name and are not allowed to change
- You are not allowed to use `mut` with constants, they are always immutable
- Constants can be declared in any scope, including global scope
- Constants may be set only to a constant expression, not the result of a value that could only be computed at runtime
- Constants naming convention is to use uppercase and underscores

```
const THREE_HOURS_IN_SECONDS_: u32 = 60 * 60 * 3;
```
- The compiler is able to evaluate a limited set of operations at compile time, which lets us choose to write out this value in a way that’s easier to understand and verify, rather than setting this constant to the value 10,800.

#### Shadowing
- Happens when using the same variable's name and repeating the use of the `let` keyword
- It's different from reassigning a value, because we use `let` again
- We can change the type of the value but reuse the same name

```
    let spaces = "   ";
    let spaces = spaces.len();
```

### Data Types
- Rust is a statically typed languange. It must know the types of all variables at compile time
```
let guess: u32 = "42".parse().expect("Not a number");
```

#### Scalar Types
- A scalar type represents a single value. They are 4 types
    - **Integer Types**: a number without a fractional component. Each variable can be signed and unsigned and has an explicit size. It uses the two's complement representation (uses the binary digit with the greatest place value as the sign to indicate whether the binary number is positive or negative. When the most significant bit is 1, the number is signed as negative).
    If there is an overflow, in debug mode, Rust will panick, but in release mode, the value will wrap around to the minimum value
    - **Floating-point Types**: all floating points are signed. There are two types `f32` and `f64` (most commonly used)
    - **Boolean Types**: they are one byte size
    - **Characters Types**: the language most primitive alphabetic type. We specify char literals with single quotes, as opposed to string literals, which use double quotes. It is four bytes in size and represents a Unicode Scalar Value (can represent way more than just ASCII)

#### Compound Types
- They can group multiple values into one type
    - **Tuple Type**: they have a fixed length, once declared cannot grow in size. We can use pattern matching to destructure a tuple. The tuple without any value is called unit.
    - **Array Type**: unlike tuples, every element of an array must have the same type. Arrays have a fixed length. Arrays are useful when you want your data allocated on the stack rather than the heap. A **vector** is similar type provided by the standard library, but it is allowed to grow in size. An array is a single chunk of memory of a known, fixed size that can be allocated on the stack. You can access elements of an array using indexing.
    When you attempt to access an element using indexing, Rust will check that the index you’ve specified is less than the array length. If the index is greater than or equal to the length, Rust will panic. This check has to happen at runtime. This is an example of Rust’s memory safety principles in action.

### Functions
- Rust code uses **snake case** for functions and variable names: all letters are lowercase and underscodres separate words
- To define a function we use `fn`
- It does not matter the order of the functions, only that they are defined somewhere in a cope that can be seen by the caller
- They can have **parameters**, special variables that are part of the function signature
- The concrete values you provide for those parameters are called `arguments`
- In function signatures, you must declare the type of each parameter

#### Statements and Expressions
- **Statements**: are instructions that perform some action and do not return a value. `let y = 6;`. Function definitions are also statements. Statements do no return values.
- **Expressions**: evaluate to a resultant value. Calling a function or a macro is an expression, as well as a new scope block created with curly brackets. Expressions do not include ending semicolons.If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.

#### Functions with Return Values
- we don't name return values
- the return value is the value of the final expression in the block of the body function
- for returning early, we use `return` and specify a value

```
fn five() -> i32 {
    5
}
```
- there's no semicolon because it's an expression whose value we want to return
- if we add a semicolon, we transform it from a expression to a statement, and we get an error

### Comments
- Use `//`

### Control Flow
#### if Expressions
- Allows you to branch the code depending on conditions
- We must provide with a Boolean as its condition
- Rust only executers the block for the first `true` condition, and once it finds one, it doesn't even check the rest
- This means the values that have the potential to be results from each arm of the if must be the same type. Rust needs to know at compile time what type the number variable is. So this will throw an error:

```
let condition = true;
let number = if condition { 5 } else { "six" };
```
#### Repetitions with Loops
- `loop` tells Rust to execute a block of code until you explicitly tell it to stop
- `break` tells the program when to stop
- you can return a value after the `break` expression
- loop labels: must begin with a single quote, you can then use with `break` or `continue`

```
'counting_up: loop { ..}
```
- `while`: while condition evaluates to true, the code runs, otherwise, it exists the loop
- `for`: for looping over each item in a collection. So we make sure index is inside bounds always, and faster

## 4. Understanding Ownership
### 4.1. What is Ownership
- Each value in Rust has an owner
- There can only be one owner at a time
- When the owneer goes out of scope, the value will be dropped
- Scope: a range within a program for which an item is valid. The variable is valid from the point at which it's declared until the end of the current scope.
- **The String Type**: this type manages data allocated on the heap and as such, is able to store an amount of text that is unknown to us at compile time.
```
let mut s = String::from("hello");
s.push_str(", world"); // push appends a literal to a String
```
- The double colon :: operator allows to namespace the `from` function under the `String` type 
- To transform to string we can use:`("Example").to_string()`

#### Memory and Allocation
- With string literals, we know the contents at compile time: the text is hardcoded directly into the final executable. That's fast and efficient, because they are immutable.
- For supporting mutable `String` types, we can't put a blob of memory into the binary for each piece of text, whose size we don't know / might change while running the program. We need to allocate an amount of memory on the heap, unknown at compile time.
    - The memory must be requested from the memory allocator at runtime. This is done by us, when we call `String::from`
    - We need to return this memory to the allocator when we are finished with the `String`. Usually this is done by a grabage collector: cleans up memory that's not being used anymore. In Rust, memory is automatically returned once the variable that owns it goes out of scope

    ```
    {
        let s = String::from("Hello"); // s is valid from this point forward
    }
    // scope is over, s is no longer valid
    ```
- When the variable is out of scope, Rust calls `drop` function for us, and it's where the author of the `String` can put the code to return the memory.

#### Variables and Data Interacting with Move
- Multiple variables can interact with the same data in different ways.
- Strings are made of 3 parts:
    - pointer to the memory that holds the contents of the string
    - length: how much memory (in bytes) the contents of the String are curerntly using
    - capacity: total amount of memory (in bytes) that the String has received from the allocator
- This group of data is stored on the stack
- When we assign s1 to s2, the String data is copied, meaning we copy the pointer, the length and capacity that are on the stack. We don't copy the data on the heap that the pointer refers to.
- When both s1 and s2 go out of scope, they will both try to free the same memory. This is called `double free` error. Freeing memory twice can lead to memory corruption.
- To ensure memory safet, after assigng s1 to s2, `let s2 = s1` Rust considers `s1` no longer valid. Therefore Rust doesn't free anything when s1 goes out of scope
- `move`: it's called when we copy the pointer, length and capacity, and invalidate the first variable.
- Any automatic copying can be assumed to be inexpensive in terms of runtime performance.

#### Variables and Data Interacting with Clone
- If we `want` to deeply copy the heap data of the `String`, not just the stack data, we can use `clone`.
- Code may be expensibe

#### Stack-Only Data: Copy
- With types such as integers that have a known size at compile time are stored entirely on the stack, copies of the actual values are quick to make
- There is no difference of deep and shallow copying here.

#### Ownership and Functions
- Passing a variable to a function will move or copy, like assignments do.

#### Return Values and Scope
- Returning values can also transfer ownership

### 4.2. References and Borrowing
- A **reference** is like a pointer to the data stored at a particular address. The data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.
- Example of a function, referencing to an object as a parameter instead of taking ownership of the value

```
let s1 = String::from("hello")
let len = calculate_length(&s1)

fn calculate_length(s: &String) -> usize { s.len()}
```

- `&s1` lets us create a reference to s1 but does not own it. The value it points to will not be dropped when the reference stops being used.
- The signature of the function uses `&` to indicate that the type of the parameter `s` is a reference.
- The opposite of referencing is **dereferencing** whis is accomplished with the dereference operator `*`.
- We call **borrowing** the action of creating a reference.
- References cannot be modified. Thus this throws an error:
```
fn change(s: &String) {
    s.push_str("word");
}
```

#### Mutable References
- We can modify a borrowed value using a mutable reference: `fn change(some_string: &mut String)`
- Mutable references cannot have other references to that value. This prevents **data races** at compile time. They happen when these 3 points occur:
    - two or more pointers access the same data at the same time
    - at least one of the pointers is being used to write to the data
    - there's no mechanism beign used to synchronize access to data
- We can use curly brackets to create a new scope, allowing for mutiple mutable references, just not **simulateneous** ones.

```
let mut s = String::from("hello");
{
    let r1 = &mut s;
} // r1 goes out of scope, so we can make a new reference without problems

let r2 = &mut s;
```
- We cannot have a mutable reference while we have an immutable one to the same value.
- Multiple immutable references are allowed because no one who is just reading the data has the ability to affect anyone else's reading of the data

#### Dangling Referencess
- **dangling pointer**: a pointer that references a location in memory that may have been given to someone else. Rust guarantees that references will never be dangling: if you have a reference to some data, the compiler ensures that the data will not go out of scope before the reference to the data does.

### 4.3. The Slice Type
- **slice** refers to a contiguous sequence of elements in a collection, rather than the complete collection. A slice is a kind of reference, so it does not have ownership.
- `let slice = &s[0..5]` is a reference to a portion of the String s. We specify `[starting_index..ending_index]`
- If you want to start at index 0, you can drop the first value `let slice = &s[..2]`
- If the slice includes the last byte of the String you can drop the last value `let slice = &s[3..]`
- You can drop both values to take a slice of the entire string

#### String Literals as Slices
- `let s = "hello, world` the type of s is `&str`
- String literals are immutable: `&str` is an immutable reference

#### String Slices as Parameters
- defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality. So instead of:
    - `fn first:word(s: &String) -> &str`
    - we do: `fn first:word(s: &str) -> &str`
- This takes advantage of the **deref coercions**

#### Other Slices
- We can use slice with other types, like an array

```
let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
```

## 5. Using Structs to Structure Related Data
- A struct is a custom data type, like an object with attributes.
- They are like tuples, but you name each piece of data: they are more flexible, as you don't have to rely on the order of the data to specify or access the values of an instance
- To use a struct after we've defined it, we create an instance of that struct.
- If we want it to be mutable, the whole instance must be
- Struct update syntax uses `=` because it moves data.
- **Tuple Structs**: don't have names associated with their fields, just the types of the fields.
- **Unit-Like Structs**: structs that don't have any fields. USeful when you need to implement a trait on some type but don't have any data that you want to store in the type itself.
- **Ownership of Struct Data**: in the instance of `User` we used the owned `String` type rather than the `&str` string slice type, because we want each instance of this struct to own all of its data, and the data to be valid for as long as the entire struct is valid.
    - it's also possible for structs to store references to data owned by something else, but it requires the use of **lifetimes**, to ensure the data referenced is valid for as long as the struct is.
    ```
    struct User {
        active: bool,
        username: &str,
        email: &str,
        sign_in_count: u64
    }
    ```
- **Printing struct values**: 
    -   with structs, is less clear how to display the values (there are many possibilities) with Display. So we can use `{:?}` or `{:#?}` (easier to read) for debugging purposes to see the values of a struct and we add `#[derive(Debug)]` attribute before the struct definition
    - another way is use `dbg!` macro, which takes ownership of an expression (as opposed to `println!`), prints the file and line number where the macro call occurs with the value and returns ownership of the value

### Method Syntax
- We declare them with `fn` keyword and name, they can have parameters and a return value.
- Methods are defined within the context of a struct (or enum or a trait object) and the first parameter is always `self`, which represents the instance of the struct the method is being called on
- To define a function in the context of a struct we start an `impl` (implementation) block
- We do `fn area(&self)` where `&self`` is short for `self: &Self``
- Methods must have a parameter named self of type Self for their first parameter
- We can name a method the same name as one of the struct's fields. Often we give the same name when we want to return the value in the field and do nothing else. They are called **getters**. They are useful, because you can make the field private, but the method public, and thus enable read-only access to that field as part of the type's public API
- **Automatic referencing and Dereferencing**: when you call a method with `object.something()`, Rust automatically adds in `&`, `&mut` or `*` so `object` matches the signature of the methods. So these are the same:

```
p1.distance(&p2);
(&p1).distance(&p2);
```

- This automatic referencing behaviour works because methods have a clear receiver - the type `self`. Given teh receiver and name of a method, Rust can figure out definetively whether the method is reading (`&self`), mutating (`&mut self`)  or consuming (`self`)

### Associated Functions
- All functions defined within an `impl` block are **associated functions** because they are associated with the type named after `impl`.
- We can define associated functions that don't have the `self` as their first param (and thus they are not methods) because they don't need an instance of the type to work with
- To call associated functions we use `::` syntax with the struct name `let sq = Rectangle::square(3)`. The function is namespaced by the struct. The `::` syntax is used for both associated functions and namespaces created by modules.

### Multiple impl blocks
- Each struct is allowed to have multiple `impl` blocks.

## 6. Enums and Pattern Matching
- `enums` allow you to define a type by enumerating its possible variants. It gives you a way of saying a vlaue is one of a possible set of vlaues.
- The name of each enum variant becomes a function that constructs an instance of the enum.
- We are also able to define methods on structs using `impl`

### The Option Enum
- `Option` is an enum defined in the standard library. It encodes a very common scenario where a value could be something or nothing. Expressing this concept means the compiler can check whether you've handled all the cases you should be handling
- Rust does not have `null`but it does have an enum that can encode the concept of a value being present or absent. This enum is `Option<T>` and it's defined as:
```
enum Option<T> {
    None,
    Some(T),
}
```
- The `Option <T>` is included in the prelude: you don't need to bring it into scope explicitly. Its variants are also included in the prelude, you can use `Some(T)` and `None` without the `Option::` prefix.
- `<T>` means that the `Some` variant can hold one piece of data of any type, and each concrete type that gets used in place of `T` makes the overall `Option <T>` type a different type.

### The `match` Control Flow Construct
- The power of `match` comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled
- The first pattern the value "fits", the value falls into the associated code block to be used during execution
- The `match` keyword is followed by an expression (it can be any type, unlike `if` which needs a boolean)
- Arms have 2 parts: a pattern and some code. The `=>` separates the pattern and the code to run
- Each arm is separated by a comma

#### Patterns That Bind to Values
- We can have an anum with data `Quarter(UsState)` and then bind the state with the value with we call the function `value_in_cents(Coin::Quarter(UsState::Alaska))`. The arm of the match option can use the state info, binding it `Coin::Quarter(state) => println!("State quarter from {:?}!", state); }`

#### Matching with Option<T>
- Combining `match` and enums is common. Match against an enum, bind a variable to the data inside and then execute the code based on it.
- Matches are exhaustive: we must exhaust every last possibility in order for the code to be valid.

#### Catch-all Patterns and the _Placeholder
- If we want to catch-all the options, and use the value we can use the `other` arm
- If we want to catch-all the options, but don't want to use the value, we can use `_`, it matches any value and does not bind to that value.
- If we want to express that nothing happens for a certain arm, we can use `()`

### Consice Control Flow with `if` `let`
- Instead of using the `_` and `()` together, to indicate that nothing should happen for the rest of the cases, we can use the `if` `let` pattern

## 7. Managing Growing Projects with Packages, Crates and Modules
- A package can contain multiple binary crates and optionally one library crate
- Scope: the nested context in which the code is written has a set of names that are defined `in scope`. You can create scopes and change which names are in or out of scope. You can't have two items with the same name in the scope.
- The module system, includes:
    - **packages**: a cargo feature that lets build, test and share crates
    - **crates**: a tree of modules that produces a library or executable
    - **modules** and **use**: lets control the organization, scope and privacy of paths
    - **paths**: a way of naming an itrem, such as a struct, function or module

### 7.1 Packages and Crates
- A **crate** is the smaller amount of code that Rust compiler considers at a time. Crates can contain modules and the modules may be defined in other files that get compiled with the crate. Crates can come in 2 forms:
    - *Binary crates**: programs you can compile to an executable that you can run, such a command-line program or a server. Each musth have a function called `main` that defines what happens when the executable runs
    - **Library crates**: don't have a `main` function and they don't compile to an executable. Instead, they defined functionality intended to be shared with multiple projects. The **crate root** is a source file that Rust compiler starts from and makes up the root module of your crate.
- A **package** is a bundle of one or more crates that provides a set of functionality. A package contains a `Cargo.toml` file that describes how to build those crates. Cargo is actually a package that contains a library crate that the binary crate depends on.
    - a package can contain as many binary crates as you want, but at most only one library crate
    - a package must contain at least one crate, whether that's a library or a binary crate

### 7.2 Defining Modules to Control Scope and Privacy
#### Modules Cheat Sheet
- Start from the crate root: when compiling a crate, the compiler first looks in teh crate root file (`src/lib.rs`for a library crate or `src/main.rs` for a binary crate)
- Declaring modules: say you declare a new module `mod garden;`. The compiler will look for the module's code in these places:
    - inline, within curly brackets that replace the semicolon following `mod garden`
    - in the file `src/garden.rs`
    - in the file `src/garden/mod.rs`
- Declaring submodules: in any file other than the crate root, you can declare submodules. You can declare `mod vegetables;` in `src/garden.rs`. The compiler will look for the submodule in these places:
    - inline, within curly brackets that replace the semicolon following `mod vegetables`
    - in the file `src/garden/vegetables.rs`
    - in the file `src/garden/vegetables/mod.rs`
- Paths to code in modules: once a module is part of your crate, you can refer to code in that module from anywhere else in the same crate, as long as the privacy rules allow, using the path of the code. Ex an `Asparagus` type can be found at `crate::garden::vegetables::Asparagus`
- Private vs Public: code withing a module is private from its parent modules by default. To make a module public, declare it with `pub mod` instead of `mod`. To make items within a public module public as well, use `pub` before their declarations
- The `use` keyword: withing a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths. I.e. `use crate::garden::vegetables::Asparagus`
- To create a new library called restaurant: `cargo new restaurant --lib`
- We define a module with the keyword `mod` and the name of the module `mod serving`. Inside other module we can place other modules, definitions of other items such as structs, enums etc and functions.
- The entire module tree is rooted under the implicit module named `crate`. Modules can have siblings (defined in the same modules), parents and childs.

### 7.3 Paths for Referring to an Item in the Module Tree
- **Absolute path**: the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from current crate, it starts with the literal `crate`
- **Relative path**: starts from the current module and uses `self`, `super` or an identifier in the current module
- Our preference in general is to specify absolute paths because it's more likely we'll want to move code definitions and item calls independently of each other.
- Starting relative paths with `super`: allows us to reference an item that we know is in the parent module, 

#### Making `structs` and `enums` Public
- If we use `pub` before a struct definition, we make the struct public, but the struct's fields will still be private. We can make each field public or not on a case-by-case basis.
- If a struct has a private field, the struct needs to provide a public associated function that constructs an instance of the struct
- If we make an `enum` public, all its variants are then public

### 7.4 Bringing Paths into Scope with the `use` of Keyword
- We can create a shortcut to a path with the `use` keyword once, and then use the shorter name everywhere else in the scope
- It's similar to creating a symbolic link in the filesystem
- Paths brought into scope with `use` also check privacy.
- We can specify `as` and a new local name or alias for the type: `use std:io::Result as IoResult`
**Re-exporting**: when we bring a name into scope with `use`, the name available in the new scope is private. To enable the code that calls our code to refer to that name, as if it had been defined in that code's scope, we can combine `pub` and `use`.
- The standard `std` library is also a crate that's external to our package, but because is shipped with the Rust language, we don't need to change Cargo.toml to include `std`. But we do need to refer to it with the `use` to bring items from there into our package's scope. I.e. absolute path: `use std::collections::HashMap;`
- We can use nested paths to bring the same items into cope in one line.
```
use std::cmp::Ordering;
use std::io;
```
Instead, we use:
`use std::{cmp::Ordering, io};`
- **The Global Operator**: if we want to bring all public items defined in a path, into scope we can use `*`. Example: `use std::collections::*`. It's usually used under tests into the `tests` module. Sometimes also used as part of the prelude pattern.

## 8. Common Collections
- Unlike the built-in array and tuple types, the data the collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

### 8.1. Storing Lists of Values with Vectors
- `Vec<T>`: allow to store more than one value in a single data structure, that puts all the values next to each other in memory. They can only store values of the same type. Useful when you have a list of items.
- To create a vector: `lete v: Vec<i32> = Vec::new();`
- Since we didn't insert any values, we need to add the type. Vectors are implemented using generics. If we add initial values, Rust will infer the type
- Rust provides the `vec!` macro, which will create a new vector that holds the values you give it
- For updating a vector we can use the `push` method. We need to make the vector mutable
- For accessing the elements of a vector we can use:
    - `v.get(2)` we get an Option<&T> that we can use with `match`
    - `&v[2]` this gives us a reference to the element at the index value
- **Iterating over the values**: we iterate through all the elements rather than use indices to access one at a time. Types:
    - Immutable reference: `for i in &v`
    - Mutable Reference: `for i in &mut v { *i +=50}`. We add 50 to the current element. We need to derefernce to get the value in i before using the operator +=
    
#### Using an `enum` to Store Multiple Types
- Vectors can only store values with the same type. Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can use an enu,
- Rust needs to know what types will be in the vector at compile time, so it knows exactly how much memory on the heap will be needed to store each element. We must also be explicit about what types are allowed in the vector
- Using an `enum` plus a `match` expression means that Rust will ensure at compile time that every possible case is handled.

#### Dropping a Vector Drops its elements
- when the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up

### 8.2 Storing UTF-8 Encoded Text with Strings
- Strings are implemented as a collection of bytes, plus some methods to provide useful functionality when those bytes are interpreted as text
- Rust has only one string type in the core language, which is teh string slice `str`, usually seen in its borrowed form `&str`, which are references to some UTF-8 encoded string data stored elsewhere. String literals, for example, are stored in the program's binary and are therefore string slices
- The `String` type, which is provided by Rust's standard library, rather that coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

#### Creating a New String
- Many of the same operations available with `Vec<T>` are available with `String`, because is implemented as a wrapper around a vector of bytes with some extra guarantees, restrictions and capabilities
- Strings are UTF-8 encoded, so we can include any properly encoded data in them

#### Updating a String
- A string can grow in size, using `push_str`, `+`, `format!`
- `push_str` takes a parameter but does not take ownership of it
- The `push` method, takes a single character as a parameter and adds it to the String.
- To combine to existing strings, we use `+`. The `+` operator uses the `add` method, whose signature looks something like this: `fn add(self, s: &str) -> String`. We can only add a `&str` to a String. The compiler can coece the `&String` argument into a `&str`
- We can also use the `format!` macro for concatenating strings. It uses references to it does not take ownership of any of its parameters

#### Indexing into Strings
- Rust strings don't support indexing.
- The index of string's bytes does not always correlates to a valid unicode scalar value. I.e. the length of this string is not 12 `"Здравствуйте"`, but 24: that's the number of bytes it takes to encode it in UTF-8, bc each Unicode scalar value in that string takes 2 bytes of storage.
- `&"hello[0]` it would return `104` (the byte value - ASCII) not `h`. But Rust doesn't compile the code at all, to avuid misunderstanding.

#### Bytes and Scalar Values and Grapheme Clusters
- There are 3 ways to look at strings from Rust's perspective:
  - bytes
  - scalar values
  - grapheme clusters (letters)
- Example: the Hindi word “नमस्ते”:
    - as 18 bytes (a vector of `u8` values): `[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]`
    - as unicode scalar values (which are what Rust's char is): `['न', 'म', 'स', '्', 'त', 'े']` they are 6 char values, but the fourth and the sixth are not letters, they are diacritics, that don't make sense on their own
    - as grapheme clusters (we'd get what a person would call the 4 letters that make up for the word): `["न", "म", "स्", "ते"]`
- A reason Rust's doesn't allow us to index into a `String` to get a char is that indexing operations are expected to always take constant time (O(1)). But it isn't possilbe to guarantee that performance with a String, bc Rust would have to walk through the contents from the beginning to the index to determine how many valid characters there were

#### Slicing Strings
- Rather than indexing using `[]` with a single nmber, you can use `[]` with a range to create a string slice containing particular bytes
- You can only slice complete chars. I.e. having `let hello = "Здравствуйте";` you can slice `&hello[0..4]` - that would be 2 chars (4 bytes), but you cannot slice `&hello[0..1]` as this tries to slice only a part of a character's bytes
- `&str` (String Slice): This is a string slice, which is essentially a reference to a portion of a string. It is a borrowed reference to a sequence of UTF-8 bytes. It's a more flexible and efficient way to work with string data when you don't need ownership or want to avoid allocating memory. Functions that take &str typically work with existing string data without creating new instances.
- `String`: This is a heap-allocated, growable, mutable string. It is an owned type, meaning that the data it contains is owned by the String variable. It can be modified and resized as needed. If you need to manipulate the string and modify it, or if you need to own the string data, then String is the appropriate type.
- Ex `fn to_pig_latin(s: &str) -> String`. By accepting a reference to &str as the input parameter, the function can handle both string literals (&str) and String instances without requiring unnecessary conversions when calling the function. This makes the function more versatile and idiomatic in Rust.

#### Methods for Iterating over Strings
- You need to be explicit about whether you want characters or bytes.
    - For individual unicode values, use the `chars` method. `.chars()` separates the values and returns them of type char, and you can iterate over the result
    - For bytes, you can use `.bytes()`. Remember that valid unicode scalar values may be made up of more than 1 byte
    - Getting grapheme clusters from strings is complex, so this functionality is not provided by the standard library

### 8.3 Hash Maps
- The type `HashMap<K, V>` stores a mapping of keys of the type K to values of type V, using a hash function, to determine how it places these keys and values into memory.,
- It's useful when you want to look up data not by using an index, but by using a key that can be of any type.
- Hashmap is not included in the prelude, so for using it you need to bring it to scope `use std::collections::HashMap;`
- Like vectors, hashmaps store they data on the heap
- Like vectors, all the keys must have the same type, and all the values must have the same type
- To access values of a hashmap you can do `scores.get(&team_name).copied.unwrap_or(0);`
    - the `get` method reutns an `Option<&V>`. If there's no value for that key, it will return `None`
    - it handles the option by calling `copied()` to get an `Option<i32>` rather than an `Option<&i32>`
    - then unwrap_or() to set `score` to 0, if scores does not have any entry for the key

#### Hash Maps and Ownership
- For types that implement the `Copy` trait, like `i32`, the values are copied into the has map.
- For owned values like `String`, the values will be moved and the hashmap will be the owner of those

#### Updating a Hash Map
- Although the number of key and value pairs is growable, each unique key can only have one value associated with it at a time (but not vice versa)
- When you want to change the data in a hash map, you have to decide how to handle the case when a key has already a value assigned 
    - over write the old value
    - ignore the new value if a key already exists:  has maps have the API `entry` that takes the key you want to check as a parameter. The return value is an enum called `Entry` that represents a value that might or might not exist. The `or_insert` method on `Entry` is defined to return a mutable reference to the value for the corresponding `Entry` key if that key exists, and if not, inserts the parameter as the new value for this key and returns a mutable reference to the new value.
    - updating a value based on the old value

#### Hashing functions
- HashMap uses a hashing funtion called `SipHash` that can provide resistance to Denial of Service attacks involving tables. It is not the fastest hashing algorithm, but the trade-off for better security is worth it.
- You can switch to another function by specifying a different `hasher` (a type that implements 
the `BuildHasher` trait).


## 9. Error Handling
- Types of errors:
    - Recoverable: such as 'file not found', we mostly like to just report the problem to the user and retry the operation. It has the type `Result<T, E>` for recoverable errors
    - Unrecoverable: they are symptoms of bugs, like trying to access a location beyond the end of an array, so we want to immediately stop the program. It has the `panic!` macro that stops execution
- Rust doesn't have exceptions

### Unrecoverable Errors with `panic!`
- There are 2 ways to cause a panic in practice: taking an action that causes our code to panic or by calling the `panic!` macro explicitly
- The panic will: print the failure message, unwind, clean up the stack and quit. Via environmnet variable, you can also display the call stack when panic occurs
- `Unwinding`: walks back up the stack and cleans up the data from each function it encounters. It's a lot of work, so Rust allows you to choose the alternative of immediately aborting, which ends the program without cleaning up.
- Memory that the program was using needs to be cleaned up by the operating system
- You can switch from unwinding to aborting upon panic by adding `panic = 'abort` to the appropiate `[profile]` sections in your Cargo.toml file. Ex, abort on panic in release mode: 

```
[profile.release]
panic 'abort'
```
- **Buffer overread**: attempting to access an element from a vector which doesn't exist. It can lead to security vulnerabilities if the attacker is able to manipulate the index in such a way as to read data they shouldn't be allowed to that is stored after the data structure. To protect you from this, Rust stops execution and panics.
- **Backtrace**: a list of all the functions that have been called to get to this point. They are read from the top (start). `RUST_BACKTRACE=1 cargo run`

### Recoverable Errors with `Result`
- The `Result` enum has two variants `Ok` and `Err`:
```
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
- The T and E are generic types: T represents the type of the value that will be returned in a success case OK variant, E represents the type of error that will be returned in a case of failure case within the Err variant.
- The Result enum and its variants have been brought into scope by the prelude, so we don’t need to specify Result:: before the Ok and Err variants in the match arms.
- An alternative to using `match` is to use `.unwrap_or_else`

#### Shortcuts for Panic on Error: unwrap and expect
- The `unwrap` method is a shortcut which will return the value inside the ÇOk. If the result is Err, unwrap will call panic! for us
- The `expect` returns or calls panic!. It's more used than unwrap.

### Propagating Errors
- You can return the error to the calling code, so that it can decide what to do
- Rust provides the `?` operator to make it easier
- `?` operator can only be used in functions whose return type is compatible with the value the `?` is used on
- `?` can be used with return type `Result<T, E>` and `Option<T>` 

### To panic! or Not to panic!
- In prototype code and tests, it's more appropiate to write code that panics instead of returning a Result
- Methods like `unwrap` and `expect` are very handy when prototyping, before deciding how to handle errors.
- If you can ensure that you'll never have an `Err` variant, it's perfectly fine to call `unwrap` and document the reason you think you'll never have the Err variant in the expect text.
- When failure is expected, it’s more appropriate to return a Result than to make a panic!
- Functions often have **contracts**: their behavior is only guaranteed if the inputs meet particular requirements. Panicking when the contract is violated makes sense because a contract violation always indicates a caller-side bug and it’s not a kind of error you want the calling code to have to explicitly handle.

### Creating Custom Types for Validation
- We can make a new type and put the validations in a function to create an instance of the type, rather than repeating the validations everywhere (with if conditions)


## 10. Generic Types, Traits and Lifetimes
- **Generics**: abstract stand-ins for concrete types or other properties. Functions can take parameters of some generic type, instead of a concrete type like `i32`. I.e. `Vec<T>`, `Hashmap<K, V>`, `Result<T, E>`.
- **Traits**: to define behaviour in a generic way. You can combine traits with generic types to constrain a generic type to accept only those types that have a particular behaviour
- **Lifetimes**: a variety of generics that five the compiler information about how references relate to each other.

### Generic Data Types
- We place the generics in the signature of the function where we would usually specify the data types of the parameters and return value.
- We can define structs to use generic type parameter in one or more fields using `<>` syntax
- We can also use generics in enums
- We can also use generics in methods. `impl<T> Pint<T>`, this indicates that the type in the angle brackets in Point is a generic type rather than a concrete type.

#### Performance using Generics
- **Monomorphization**: is the process of turning generic code into specific code by filling in the concrete types that are used when compiled: the compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with

### Traits: Defining Shared Behaviour
- We can use traits to define shared behaviour in an abstract way
- We can use **trait bounds** to specify that a generic type can be any type that has certain behaviour
- A type's behaviour consists of the methods we can call on that type
- Trait definitions are a way to group method signatures together to define a set of behaviours
- Declare a trait called Summary: `pub trait Summary {}` and inside, we declare a function signature with a semicolon instead of providing an implementation `fn summarize(&self) -> String`
- Implementing a trait on a type: `impl Summary for Tweet {...}` we put the trait name and then use the `for` keyword. We put the method signature inside and fill the method body
- We can't implement external traits on external types. This restriction is part of a property called **coherence** and the **orphan rule**, because the parent type is not present. This ensures that other people's code can't break your code and vie cersa.

#### Default Implementations
- We can have default behaviour for some or all of the methods in a trait instead of requiring implementations for all methods on every type. Then, as we implement the trait on a particular type we can keep or override each method's default behaviour.
- Default implementations can call other methods in the same trait, even if those other methods don't have a default implementation

#### Traits as Parameters
- A parameter that accepts any type that implements the specified trait: `pub fn notify(item: &impl Summary) {...}`.
- This is syntatic sugar for a longer form known as **trait bound**, which looks like: `pub fn notify<T: Summary>(item: &T) {...}`. 
- We can use the impl Summary if we have 2 parameters that implement Summary and we want to have different types, as long as both types implement Summary: `pub fn notify(item1: &impl Summary, item2: &impl Summary) {...}`
- If we want to force both parameters to have the same type, however, we must use a trait bound like this: `pub fn notify<T: Summary>(item1: &T, item2: &T)`

#### Multiple Trait Bounds with the + Syntax
```
pub fn notify(item: &impl(Summary + Display)) {}
pub fn notify <T: Summary + Display>(item: &T)
```

#### Clearer Trait Bounds with the 'where' clause
- If there are too  many trait bounds, the function signature is hard to read
- Trait bounds can be specified using `where` clause after the function signature

#### Returning Types that implement Traits
- We can specify a return type only by the trait it implements `fn returns_summarizable() -> impl Summary {}`
- However, you can only use impl Trait if you are returning a single type.

#### Using Trait Bounds to Conditionally Implement Methods
- Implementations of a trait on any type that satisfies the trait bounds are called **blanket implementations**. They are commonly used in the standard library. Ex: the `ToString` trait is implemented on any type that implements the `Dsiplay` trait:
`impl<T: Display> ToString for T {..}`
- So we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait. Ex: we can turn integers into their corresponding `String` values: `let s= 3.to_string()`


### Validating References with Lifetimes
- Lifetimes ensure that references are valid as long as we need them to be
- Every reference in Rust has a lifetime, which is teh scope for which that reference is valid.
- Most lifetimes are implicit and inferred, we must annotate lifetimes when the lifetimes of references could be related in a few different ways

#### Preventing Dangling References with Lifetimes
- The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it's intended to reference.

#### The Borrow Checker
- The Borrow Checker compares scopes to determine whether all borrows are valid

#### Lifetime Annotation Syntax
- The names of lifetime parameters must start with an apostrophe `'` and are usually lowercase and very short. Most people use `'a` for the first annotation.
```
&i32 // a reference
&'a i32 // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
#### Thinking in Terms of Lifetimes
- When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function. However, this would be a dangling reference because the value will go out of scope at the end of the function. 

#### Lifetime Annotation in Struct Definitions
- We can define structs to hold refeences, but in that case, we would need to add a lifetime annotation on every reference in the struct's definition


#### Lifetime Elision
- Every reference has a lifetime and you need to specify lifetime parameters for functions or structs that use references.
- However, not always is necessary, as the borrow checker can infer the lifetimes in some situations and don't need explicit annotations. This is known as **lifetime elision rules**
- The elision rules don't provide full inference.
- **Input lifetimes**: lifetimes on function or method parameters
- **Output lifetimes**: lifetimes on return values
- Rules to figure out the lifetimes, applied to `fn` definitions and `impl` blocks:
    - Rule 1: the compiler assigns a lifetime parameter to each parameter that's a reference. A func with 1 parameter, get's one lifetime parameter `fn foo<'<>(x: &'< i32)`, a func with 2 parameters, gets two separate lifetime parameters `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`
    - Rule 2: if there is exactly one input lifetime parameter, that lifetime gets assigned to all output lifetime parameters `fn foo<'a>(x: &'a i32) -> &'a i32`
    - Rule 3: if there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.

#### Lifetime Annotations in Method Definitions
- Lifetime names for struct fields always need to be declared after the `impl` keyword and then used after the struct's name, because those lifetimes are part of the struct's type `impl<'a> ImportantExcerpt<'a> {}`

#### The Static Lifetime
- `'static` denotes that the affected reference can live for the entire duration of the program. All string literals have the `'static` lifetime: `let s: &'static str = "I have a static lifetime"`.
- The text in the string is stored directly in the program's binary, which is always available.

### 11. Write Automated Tests
- A test is a function that's annotated with the`test` attribute. Attributes are metadata about pieces of Rust code
- Add `#[test]` on the line before the `fn`
- When we make a new library project, a test module with a test function in it is automatically generated for us. `cargo new adder --lib`
- We might also want to have non-test functions in the `tests` module, to help set up common scenratios or perform operations
- To run tests `cargo test`
- There are benchmark tests, documentation tests..
- Tests fail when something in the test function panics

#### Checking Results with the `assert!` Macro
- The `assert!` macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to `true`. If the value is true, nothing happens, if is false, the macro calls `panic!` to cause the test fail.

#### Testing Equality with `assert_eq!` and `assert_ne!` Macros
- These macros compare two arguments for equality or inequality respectively
- The values being compared must implement the `PArtialEq` and `Debug` traits. All primitive types and most of the standard library types implement these traits. For structs and enums that you define yourself **you'll need to implement `PartialEq` and `Debug`** by adding `[#derive(PartialEq, Debug)]`

#### Adding Custom Failure Messages
- You can add a custom message to be printed with the failure method, as an optional argument to the `assert!`, `assert_eq!` and `assert_ne!`

#### Checking for Panics with `should_panic`
- We place the `#[should_panic]` attribute after the `#[test]` attribute and before the test function it applies to
- Tests that use should_panic can be imprecise, as they can panic for a different reason from the one we were expecting, so we can add an optional `expected` parameter to the `should_panic` attribute

#### Using `Result<T, E>` in Tests
- Writing tests so they return a `Result<T, E>` enables you to use the question mark operator in the body of tests
- You can't use the `#[should_panic]` annotation on tests that use `Result<T, E>`


### Controlling How Tests are Run
- `cargo test` compiles the code in test mode and runs the resulting test binary
- the default behaviour is to run all test in parallel and capture output related to the test results (not print, unless test fails)
- You can pass command line options `cargo test --help`

#### Running Tests in Parallel or Consecutively
- Tests should not depend on each other or on any shared tate, including a shared environment, such as the current working directory or environment varaibles
- To control over the number of threads used, you can send the `test-threads` flag and the number of threads you want to use `cargo test -- --test-threads=1` (ie. to not use parallelism)

#### Showing Function Output
- By default, if the test passes, we won't see any `println!` output in the terminal, only if the test fails
- To see the printed values `cargo test -- --show-output`

#### Running a Subset of Tests by Name
- To run a single test `cargo test TEST_FUNC_NAME`, the rest appear as filtered out
- To run multiple tests, w can specify part of a test name, and any test whose name matches that value will be run. I.e. `cargo test add`
- To ignore a few specific tests you can exclude them by adding the `#[ignore]` line to the test we want to exclude
- To run only ignored tests we can run `cargo test -- -ignored` and to run all of them (ignored and not) `cargot test -- --include-ignored`

#### Test Organization
- **Unit Tests**: small and focues, testing one module in isolation at a time, and can test private interfaces
    - You'll put unit tests in the `src` directory in each file with the code that they're testing
    - The convetion is to create a module named `tests` in each file to contain the test functions and to annotate the module with `cfg(test)`
    - The `#[cfg(test)]` tells Rust to compile and run the test code only when you run `cargo test` and not when you run `cargo build`. This saves compile time and saves space in the resulting compile artifact because tests are not included
    - Because integration tests go in a different directory, they don't need the `#[cfg(test)]` annotation, but since unit tests go in the same files as the code, you need it, so they are excluded in the compiled result
    - `cfg` means configuration, and tells that the item should only be included given a certain configuration option
    - Testing private functions: we can bring the module's parent's items into scope with `use super::*` and then call the private function

- **Integration Tests**: entirely external to your library and use you rcode in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test
    - You need a `tests` directory at the top level of the project, next to `src`
    - We can make as many test files as we want i.e. `integration_test.rs`
    - We don't need to annotate any code with `#[cfg(test)]`
    - We run `cargo test` and it will run: unit tests, integration tests and documentation tests. If a unit test fails, it won't continue running subsequent tests
    - To ruin all the tests in a particular integration test file `cargo test --test intergation_test`
    - If we have common functions for test setup, we don't won't them to appear in the test report, so we create it like `tests/common/mod.rs`
    - Files in subdirectories of the `tests` directory don't get compiled as separate crates or have sections in the test output
    - If our project is a binary crate that only contains a `src/main.rs` file and doesn't have a `src/lib.rs` file, we can't create integration tests in the `tests` directory and bring functions defined in the `src/main.rs` file into scope with a `use` statement. Only library crates expose functions that other crates can use. If the important functionality works, the small amount of code in the `src/main.rs` file will work as well, and that small amount of code doesn't ned to be tested

### Printing Errors to Standard Error
- Errors should not be included in the standard output, but to the standard error instead.
- `eprintln!("Problem")`: this macro prints to the standard error stream, so only data from successul run ends up in the output file -. not errors.


### 13. Functional Language Features: Iterators and Closures
#### Clousures; Anonymous Functions that Capture Their Environment
- Closures are anonymous functions you can save in a variable or pass as arguments to other functions
- You can create a closure in one place and call the closure elsewhere to evaluate it in a different context
- Closures can capture values from the scope in which they're defined
- Closures don't usually require you to annotate the types of the params or the return values like functions do
- Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within this limited contexts, the compiler can infer the types
- Closures will automatically implement one, two or all three of these `Fn` traits, in an additive fashion:
    1. `FnOnce`: applies to closures that can be called once. All closures implement this trait. A closure that moves captured values out of its body will only implement `FnOnce` and none of the other `Fn` traits
    2. `FnMut`: applies to closures that don't move captured values out of their body, mut that might mutate the caputred values. They can be called more than once
    3. `Fn`: applies to closures that don't move captured values out of their body and don't mutate captured values, as well as closures that capture nothing from their environment

#### Processing a Series of Items with Iterators
- Methods that call `next` are called **consuming adaptors**, because calling them consumes up the iterator.
- One example is the `sum` method, which takes ownership of the iterator and iterates through the items by repeatedly calling `next`, thus consuming the iterator. As iterates through, it adds each item to a running total and returns the total when the iteration is complete
- **Iterators adaptors** are methods defined on the `Iterator` trait that don't consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator. They are lazy, and we need to consume the iterator.

#### Comparing Performance: Loops vs Iterators
- Run a benchmark to see which one is faste: using `for` loop or iterators
- Iterators were a bit faster
- Rust unrolls the loop, is an optimization that removes the overhead of the loop controlling code and instead generates repetitive code for each iteration of the loop
- You can add/override any subset of the default settings, in `Cargo.toml` by adding `[profile.*]`

### 14. More About Cargo and Crates.io
#### Publishing a Crate
- **Release Profiles** are predefined and customizable profiles with different configurations that allow to have more control over various options for compiling code.
- Cargo has 2 main profiles: the `dev` profile, when you run `cargo build` and the `release` profile, when you run `cargo build --release`
- **Documentation comments**: will generate HTML documentation. They use three slashes `///` and support Markdown notation for formatting text
- We can generate the documentation running `cargo doc` and `cargo doc --open`
- Usually crate authors document: panics, errors and safety
- Running `cargo test` will run the code examples in your documentation as tests
- `//!` adds documentation to the item that contains the comments rather than to the items following the comments
- **Reestructuring the organization** for public se. You don't have to re-arrange the internal organization, but can use `pub use` to re-export items to make a public structure different than your private structure
- **Publishing crates**: login with the API key `cargo login API_KEY`
- **Yanking a crate version**: prevents new projects from depending on that version while allowing all existing projects that depend on it to continue. `cargo yank --vers 1.0.1` and to undo it you can run `cargo yank --vers 1.0.1 --undo`

#### Cargo Workspaces
- A Workspace is a set of packages that share the same `Cargo.lock` and output directory
- Run a package from the workspace directory `cargo run -p adder`
- To run all tests, just run `cargo test` in the top level of the workspace. To run tests from a particular package in the workspace pass a flag like `cargo test -p add_one`
- To install external packages `cargo install ripgrep`

### 15. Smart Pointers
- Smart pointers are data structures that act like a pointer but also have additional metadata and capabilities.
- **Reference counting** smart pointer: enables to allow data to have multiple owners by keeping track of the number of owners and, when no owners remain, clean up the data
- While references only borrow data, in many cases, smart pointers own the data they point to
- Ex. `String` and `Vec<T>` are smart pointers because they own some memory and allow you to manipulate it. They also have metadata and extra capabilities or guarantees (i.e. String stores its capacity as metadata and has the extra ability to ensure its data will always be valid UTF-8)
- Smart pointers are implemented using structs, but implement `Deref` and `Drop` traits
    - `Deref` trait: allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers
    - `Drop` trait: allows you to customize the code that's run when an instance of the smart pointer goes out of scope 

#### 15.1 Using Box<T> to Point to Data on the Heap
- `Box<T>` is a smart pointer that allows to store data on the heap rather than the stack. What remains on the stack is the pointer to the heap data.
- They don't have performance overhead, other than storing their data on the heap instead of on the stack, but they don't have many extra capabilities. Use cases
    - When you have a type whose size can't be known at compile time, and you want to use a value of that type in a context that requires the exact size
    - When you have a large amount of data and you want to transfer ownership but ensure the data won't be copied when you do so
    - When you want to own a value and you care only that it's a type that implements a particular trait rather than being of a specific type
- When it goes out of scope, the deallocation happens both for the box (stored in the stack) and the data it points to (stored on the heap)
- **Recursive Types**: we can enable recursive types by instering a box in the recursive type definition
    - **Cons List**: a data structure, made up of nested pairs: the value of the current item and the next item. The last item contains only a value called `Nil` without next item. I.e. `(1, (2, (3, Nil)))`

#### 15.2 Treating Smart Pointers like Regular References with Deref Trait
- Implementing the `Deref` trait allows to customize the behaviour of the dereference operator `*`. You can write code that operates on references and use that code with smart pointers too
- **Deref coercion**: converts a reference to a type that implements the `Deref` trait into a reference to another type. For example, deref coercion can convert `&String` to `&str` because `String` implements the `Deref` trait
- You can use `DerefMut` trait to override the * operator on mutable references
- Rust does deref coercion when it find types and trait implementaitons in 3 cases:
    - From `&T` to `&U` when `T: Deref<Target=U>`
    - From `&mut T` to

#### 15.3 Running Code on Cleanup with the Drop Trait
- `Drop` trait, which lets customize what happens when a value is about to go out of scope. You can provide an implementation for the `Drop` trait on any type, and that code can be used to release resources like files or network connections
- The functionality of the `Drop` trait is almost always used with smart pointers. I.e. when a `Box<T>` is dropped it will dellocate the space on the heap that the box points to
- The `Drop` trait requires to implement one method named `drop` that takes a mutable reference to `self`. 
- The `Drop` trait is included in the prelude, so we don't need to bring it into scope.
- Values are dropped in the reverse order of creation
- The `drop` method is called automatically, you cannot call `drop` manually, because Rust will still call it at the end of main, and would cause a **doulbe free** error, because Rust would be trying to clean up the same value twice
- Occasionally you want to clean up a value earlier: ex. when using smart pointers that manage locks - you might want to force the `drop` method that releases the lock so that other code in the same scope can acquire the lock.
- If you want to call `drop`, you need to do this instead: call the `std::mem::drop` function provided by the standard library

#### 15.4 Rc<T>, the Reference Counted Smart Pointer
- **Reference Counting**: to enable multiple ownership explicitly
- `Rc` keeps track of the number of references to a value to determine whetehr or not the value is still in use. If there are zero references to a value, the value can be cleaned up without any references becoming invalid
- We use the `Rc<T>` type when we want to allocate some data on the heap for multiple parts of our program to read and we can't determine at compile time which part will finish using the data last
- `Rc<T>` is only for use in single-threaded scenarios

#### 15.5 RefCell<T> and the Interior Mutability Pattern
- **Interior mutability** is a design pattern that allows to mutate data even when there are immutable references to that data; normally this action is disallowed by the borrowing rules
- To mutate data, the pattern uses `unsafe` code inside a data structure, to bend Rust's usual rules that govern mutation and borrowing. Unsafe code indicates to the compiler that we're checking the rules manually instead of relying on the compiler
- The `RefCell<T>` type represents a single ownership over the data it holds. The borrowing rules were:
    - at any given time, you can have either (but not both) one mutable reference or any number of immutable references but not both
    - references must always be valid
- The `Box<T>` enforces the borrowing rules invariants at compile time. If you break them, you get a compiler error
- The `RefCell<T>` enforces them at runtime. If you break them, your program will panic and exit
- Static analysis, like the Rust compiler, is inherently conservative. So if it can't be sure the code compiles with the ownership rules, it might reject a program.
- `RefCell<T>` is only for use in single-threaded scenarios and will give you a compile-time error if you try to use it in a multithreaded context
- **Interior Mutability**: when you can mutate a value in its methods but appear immutable to other code. Case: Mock Objects
- When creating immutable and mutable references, we use the `&` and the `&mut` syntax respectively. With `RefCell<T>` we use `borrow` and `borrow_mut` methods, which are part of the safe API that belongs to RefCell
- You can have multiple owners of mutable data by combining `Rc<T>` and `RefCell<T>`

#### 15.6 Reference Cycles can Leak Memory
- It is possible to create references where items refer to each other in a cycle, using `Rc<T>` and `RefCell<T>`. This creates memory leaks
- **Weak References**: don't express ownership relationship, and their current count doesn't affect when an `Rc<T>` instance is cleaned up. You can create a weak reference to the value by calling `Rc::downgrade`. 
- Calling `Rc::downgrade` increases the `weak_count` by 1 and weak_count doesn't have to be 0 for the `Rc<T>` instance to be cleaned up

### 16. Fearless Concurrency
#### 16.1 Using Threads to Run Code Simultaneously
- In OS, an executed program's code is run in a process, and the OS will manage multiple processes at once
- Within a program, you can also have independent parts that run simultaneously. The features that run these independent parts are called threads
- It can improve performance but increase complexity. Problems:
    - **Race conditions**: threads are accessing data or resources in an inconsistent order
    - **Deadlocks**: two threads are waiting for each other, preventing both threads from continuing
    - Bugs that happen only in certain situations an are hard to reproduce and fix reliably
- To create a new thread we call the `thread::spawn` function
- The return type of `thread::spawn` is `JoinHandle`, an owned value, that when we call the `join` method on it, will wait for its thread to finish
- We'll use `move` with `swpan` because the closure will then take ownership of the values it uses from the environment, thus transferring ownership of those values from one thread to another

#### 16.2 Usin Message Passing to Transfer Data Between Threads
- **Message passing**: threads communicate by sending each other messages containing data
- **Channel**: a concept in which data is sent from one thread to another. It has 2 halves:
    - **Transnmitter**: one part of the code, calls methods on the transmitter with the data you want to send
    - **Receiver**: checks the receiving end for arribing messages
- A channel is closed if either the transmitter or the receiver half is dropped

#### 16.3 Shared-State Concurrency
- Shared memory concurrency is like multiple ownership: multiple threads can access the same memory location at the same time
- **Mutex**: mutual exclusion, allows only on thread to access some data at any given time. To access the data, the thread must first signal that it wants access by asking to acquire the mutex's lock.
- **Lock**: is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
- Two rules for using mutex:
    - you must attemps to acquire the lock before using the data
    - when you are done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock
- **Atomic References**: `Arc<T>` is a type like `Rc<T>` that is safe to use in concurrent situations

#### 16.4 Extensible Concurrency with Sync and Send Traits
- **Allowing Transference of Ownership Between Threads with Send**: the `Send` marker trait indicates that ownership of values of the type implementing `Send` can be transferred between threads. Almost every Rust type is `Send`, but there are some exceptions like `Rc<T>`
- **Allowing Access from multiple Threads with Sync**: the `Sync` marker trait indicates that it is safe for teh type implementing `Sync` to be referenced from multiple threads. The smart pointer `Rc<T>` is also not `Sync`
- We don't have to implement `Send` and `Sync` manually - it's unsafe

### 17. Object Oriented Programming Features of Rus
#### 17.1 Characteristics of Object-Oriented Languages
- An object packages both data and the procedures that operate on that data (methods/operations). Rust have `impl` blocks that provide methods on structs and enums
- The implementation details of an object aren't accessible to code using that object (encapsulation). The only way to interact with an object is through its public API
- Inheritance as a Type System and as Code Sharing: in Rust, there is no way to define a struct that inherits the parent struct's fields and method implementations without using a macro. Reasons for using inheritence:
    - to reuse code: you can implement particular behaviour for one type, and inheritence enables you to reuse that implementation for a different type
    - to enable a child type to beb used in the same places as the parent type: **polymorphism** - you can substitute multiple objects for each other at runtime if they share certain characteristics
- Rust instead uses generics to abstract over different possible types and trait bounds to impose contraints on what those types must provide. This is called **bounded parametric polymorphism**
- Inheritance has fallen out of favour because it's often a risk of sharing more code than necessary. Subclasses shouldn't always share all characteristics of their parent class. For these reasons, Rust uses trait objects instead of inheritance.

#### 17.2 Using Trait Objects that Allow for Values of Different Types
- A **trait object** points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime
- We can't add data to a trait object. Their specific purpose is to allow abstraction across common behaviour

#### 17.3 Implementing an Object-Oriented Design Pattern
- **The state pattern**: we define a set o states a value can have internally. The states are represented by a set of state objects, and teh value's behaviour changes based on its state
- The state objects share functionality: in Rust, we use structs and traits rather than objects and inheritance.
- Each state is responsible for its own behaviour and for governing when it should change into another state.The value that holds a state object knows nothing about the different behaviour of the states or when to transition between states

### 18. Patterns and Matching
- **Patterns** are a special syntax in Rust for matching against the structure of types. A pattern consists of some combination of the following:
    - Literals
    - Destructed arrays, enums, structs, tuples
    - Variables
    - Wildcards
    - Placeholders
- To use a pattern, we compare it to some value. If the pattern matches the value, we use the value parts in our code

#### 18.1 All the Places Patterns Can Be Used
- **match Arms**: they need to be ***exhaustive**, meaning all the possibilities for the value in the match expression must be accounted for. One way to ensure this is to have a **catchall** pattern for the last arm. The `_` will match anything, but it never binds to a variable, so it's often used in the last arm
- **Conditional if let Expressions**: as a shorter way to write the equivalent of a match that only matches one case. Optionally, `if let` can have a corresponding `else` containing code to run if the pattern in the `if let` doesn't match. However, the compiler doesn't check for exhaustiveness here
- **while let Conditional Loops**: a while loop that runs as long as a pattern continues to match
- **for Loops**: the value that directly follows the keyword `for` is a pattern
- **let Statements**: `let PATTERN = EXPRESSION` for example `let (x, y, z) = (1, 2, 3);`
- **Function Parameters**: they can also be patterns

#### 18.2 Refutability: Whether a Pattern Might Fail to Match
- **Irrefutable**: patterns that will match for any possible value passed. I.e. `let x = 3`, because x matches anything and therefore cannot fail to match. They are: function parameters, `let` statements and `for` loops
- **Refutable**: patterns that can fail to match for some possible values. I.e. `if let Some(x) = a_value` because if the value in the `a_value` variable is `None` rather than `Some`, the `Some(x)` pattern will not match
- The `if let` and `while let` expressions can accept refutable and irrefutable patterns, but the compiler warns against the irrefutable patterns

### 19. Advanced Features
#### 19.1 Unsafe Rust
- Rust has a second language hidden inside it that doesn't enforce memory safety guarantees, it's called Unsafe Rust and gives superpowers:
    - dereference a raw pointer
    - call an unsafe function or method
    - access or modify a mutable static variable
    - implement an usnafe trait
    - access fields of `union`s
- You'll still get some degree of safety inside an unsage block, since the borrow checker is still enabled
- It's best to enclose unsafe code within a safe abstraction and provide a safe API. Parts of the standard library are implemented as safe abstractions over unsafe code that' has been audited
- **Dereferencing a Raw Pointer**: raw pointers can be immutable or mutable and are written as `*const T` and `*mut T` respectively. The asterisk isn't the dereference operator, it's part of the type name. Immutable here means that the pointer can't be directly assigned to after being dereferenced. Raw pointers:
    - are allowed to ignore teh borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    - aren't guaranteed to point to valid memory
    - are allowed to be null
    - don't implement any automatic cleanup
- **Calling an unsafe function or method**: they look like regular functions but have an extra `unsafe` before the rest of the definition. It indicates that the functions has requirements we need to uphold when we call this function
- **Using extern Functions to Call External Code**: if we need to interact with code written in another language, Rust has the keyword `extern` that facilitates and use of a Foreign Function Interface (FFI)
- **Accessing or Modifying a Mutable Static Variable**: in Rust, global variables are called static variables. Static variables can only stora references with the `'static` lifetime, which means te Rust compiler can figure out the lifetime and we aren't required to annotate explicitly. Accessing an immutable static variable is safe. A difference from constants, is that static variables have a fixed address in memory. Constants, on the other hand, are allowed to duplicate their data whenever they're used. Also static variables can be mutable. Accessing and modifying mutable static variables is unsafe
- **Implementing an Unsafe Trait**: a trait is unsafe when at least one of its methods has some invariant that the compiler can't verify
- ** Accessing fields of a union**: a union is similar to a struct, but only one declared field is used in a particular instance at one time.

#### 19.2 Advanced Traits
- **Associated Types**: connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures. So we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented
    - One example of a trait with an associated type is the `Iterator` trait that the Standard library provides
    - It seems similar to generics, but the difference is that when using generics, we need to annotate the types in each implementation; we can change the concrete types of the generic param each time. With associated types, we don't need to annotate types because we can't implement a trait on a type multiple times
- **Default Generic Type Parameters and Operator Overloading**: we can specify a default concrete type for the generic type, so we don't have to specify a concrete type if the default works for us.
    - Used for **Operator Overloading**: i nwhich you can customize the behaviour of an operator (such as `+`) in particular situations
- **Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Names**: when calling methods with the same name, you'll need to tell which one you want to use
- **Using Supertraits to Require One Trait's Functionality within another trait**: sometimes you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait. The trait your trait definition is relying on is called a `supertrait`
- **Using the Newtype Pattern to Implement External Traits on External Types**: using `newtype pattern` involves creating a new type in a tuple struct. It will have one field and be a thin wrapper around the type we want to implement a trait for

#### 19.3 Advanced Types
- **Newtype pattern for type safety an abstraction**:
    - newtypes statically enforce that values are never confused and indicating the units of a value
    - newtype pattern can also be used to abstract away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type
    - newtypes can also hide internal implementation
- **The Never Type that never returns**: there is a special type named `!` which has no values, it stands in the place of the return type when a function will never return
    - in a match arm, we cannot return 2 different types Ok(int) Err(string), but `continue` has a `!` value, so when Rust computes teh type, it looks at both match arms, the formar with a value int and the latter witha value `!`, because `!` can never have a value, Rust decides that the type is int
    - another useful case is with `match` and `panic`
- **Dynamically Sized Types and the Sized Trait**: ie `str` we can't know how long the string is until runtime, meaning we can't create a variable of type `str` nor can we take an argument of type `str`
    - `&str` they have an extra bit of metadata that store the size of the dynamic information. We must always put values of dynamically sized typed behind a pointer of some kind
    - Rust provides the `Sized` trait to determine whether or not a type's size is known at compile time. It is implemented for everything whose size is known at compile time

#### 19.4 Advanced Functions and Closures
- You can pass regular functions to functions
- Functions coerce to the type `fn`, function pointer. Passing functions with function pointers will allow you to use functions as arguments to other functions

#### 19.5 Macros
- Macros are a way of writing code that writes other code, which is known as metaprogramming. MAcros expand to produce more code than the code you've written manually
- Macros can take a variable number of parameters: we can call `println!("hello")` with one argument or `println!("hello {}", name)` with two arguments
- Macros are more difficult to read, undestand and maintain
- You must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere
- **Declarative Macros with macro_rules! for General Metaprogramming**:  to define a macro you use the `macro_rules!` construct
- The `#[macro_export]` annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope
- **Procedural Macros for Generating Code from Attributes**: acts more like a function. It accept some code as an input, operate on that code and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do. Types
    - custom derive
    - attribute-like
    - function-like
- **Attribute-like macros**: instead of generating code for the derive attribute, they allow to create new attributes. Attributes can be applied to structs, enums, functions..
- **Function-like macros**: take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do


## Other Useful Commands
- Run doc for a project overview: `cargo doc --open --no-deps`
- Run tests: `cargo test --release test_merkle_tree`
- Run tests and print the statements (by default the print statement will be eaten by the rust test ahrness) `cargo test --release test_merkle_tree -- --nocapture`
- Run a Rust formatter `cargo fmt`
- Run a bunch of lints to catch common mistakes `cargo clippy`
- There's no reason to declare your variables early in the function. That's the kind of thing you would do in very old C or JavaScript and isn't required in modern languages. Declare them as late as possible.
- Run a program with 1 envar and 2 arguments: `IGNORE_CASE=1 cargo run -- to poem.txt`
- Write the contents to a file `cargo run > output.txt` 