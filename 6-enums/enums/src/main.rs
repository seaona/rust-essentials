fn main() {
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    // we create instances of two variants - namespaced under its identifier
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    fn route(ip_kind: IpAddrKind) {

    }
    
    
    

    // Refactor: representing the same concept using just an enum is more consice
    // we can put data directly into each enum variant

    enum IpAddr2 {
        V4(String),
        V6(String),
    }

    let home1 = IpAddr2::V4(String::from("127.0.0.1"));
    let loopback1 = IpAddr2::V6(String::from("::1"));

    // IpAddr2::V4() is a function call that takes a String argument
    // and returns an instance of IpAddr2 type. We automatically get this constructior function deinfed as a result of defining the enum

    // each variant can have different types and amounts of associated data
    enum IpAddr3 {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    //let home2 = IpAddr2::V4(String::from("127", "0", "0", "1"));
    let loopback2 = IpAddr2::V6(String::from("::1"));

    // the standard library also has a definition of IpAddr
    struct Ipv4Addr {
        // --snip--
    }

    struct Ipv6Addr {
        // --snip--
    }

    enum IpAddr4 {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }

    // Another Enum Example
    enum Message {
        Quit,
        Move {x: i32, y: i32},
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // We could do the same using Structs, but they won't be groupped under Message type
    struct QuitMessage; // unit struct
    struct MoveMessage {
        x: i32,
        y: i32,
    }
    struct WriteMessage(String); // tuple struct
    struct ChangeColorMessage(i32, i32, i32); // tuple struct

    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }

    let m = Message::Write(String::from("hello"));
    m.call();
}

fn null_examples() {
    let some_number = Some(5);
    let some_char = Some('e');

    // Rust requires to annotate the overall Option type
    // bc the compiler can't infer it only looking at None
    let absent_number: Option<i32> = None;

    // Ex
    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    // this throws an error bc i8 and Option<i8> are different types
    let sum = x + y; 
    // you have to convert an Option<T> to a T before you can perform T operations with it
}