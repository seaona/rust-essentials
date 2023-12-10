fn main() {
    // the binding for `state` will be the value of UsState:Alaska
    value_in_cents(Coin::Quarter(UsState::Alaska));

    // matching with Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // catch-all pattern binding the value
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }

    // catch-all pattern without using the value
    let dice_roll2 = 9;
    match dice_roll2 {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        5 => (), //nothing happens in this case
        _ => reroll(),
    }

    // if let pattern - to avoid boilerplatte _ => ()
    let config_max = Some(3u8);
    match config_max {
        Some(max) => println!("the max is configured to be {}", max),
        _ => (),
    }

    let config_max_2 = Some(3u8);
    if let Some(max) = config_max_2 {
        println!("the max is configured to be {}", max);
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}