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
### What is Ownership
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

#### Memory and Allocation