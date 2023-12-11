# Rust Essentials
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
