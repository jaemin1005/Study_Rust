#[derive(Debug)]
enum UsState {
    Albabama,
    Alaska,
    // --생략--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {
    value_in_cents(Coin::Quarter(UsState::Albabama));

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => roll()
    }

    let config_max = Some(3u8);
    let mut count = 0;
    if let Some(max) = config_max {
        println!("The Maximum is configured to be {}", max);
    } else {
        count += 1;
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn roll() {}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
