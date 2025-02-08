use std::io::stdin;
use crate::going_forward;

pub fn item_found(progress: i32, diff: i32, rhp: i32, rdamage: i32, php: i32, pd: i32) {
    println!("You found an axe!");
    println!("Do you want to pick it up?");
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Nem jo!");
    match choice.trim() {
        "y" => {
            println!("You picked it up!");
            let picked_up = true;
            going_forward(progress, diff, rhp, rdamage, php, pd, picked_up);
        },
        _ => println!("Nem jo!")
    }
}
