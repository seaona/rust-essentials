// structs
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// unit-like struct
struct AlwaysEqual;

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // accessing and changing values
    user1.email = String::from("newmail@example.com");
    
    let user2 = build_user(String::from("somemail@example.com"),String::from("someusername"));
   
    // Access and print fields
    println!("Active: {}", user2.active);
    println!("Username: {}", user2.username);
    println!("Email: {}", user2.email);
    println!("Sign-in Count: {}", user2.sign_in_count);

    // creating instances from other instances with struct update syntax
    // now we can no longer use user1 because the String in the username was moved to user2
    let user3 = User {
        email: String::from("anothermail@example.com"),
        ..user1
    };
    // we could used it again if we had given String values for both mail and username
    // because active and sign_in_count are types that implement the copy trait

    // tuple structs
    let black = Color(0, 0, 0, 0);
    let origin = Point(0, 0, 0, 0);

    // unit-like struct
    let subject = AlwaysEqual;
}


fn build_user(email: String, username: String) -> User  {
    User {
        active: true,
        username, //because mail field and email parameter have the same name
        email,
        sign_in_count: 1,
    }
}

// tuple structs
struct Color(i32, i32, i32, i32);
struct Point(i32, i32, i32, i32);


// Ownership of Struct Data

// if we want to store reference of data owned by somehtin else,
// we need to use lifetimes (chapter 10)
struct User {
    active: bool,
    username: &str, // error: expected named lifetime parameter
    email: &str, // error: expected named lifetime parameter
    sign_in_count: u64
}

fn ref () {
    let user1 = User {
        active: true,
        username: "someusername",
        email: "somemail@exasmple.com",
        sign_in_count: 1,
    }
}
