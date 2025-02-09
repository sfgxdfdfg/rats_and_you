mod structs;
mod items;
use crate::structs::*;
// use crate::items::*;
use std::io::*;
use std::process::exit;
use std::cmp::Ordering;
use rand::Rng;
use ferris_says::say;

fn main() {
    let difficulty: i32; // Minel lejjebb annal nehezebb, minel feljebb annal konnyebb!
    let mut set_diff = String::new();
    println!("Before you start the game you NEED to set the difficulty!");
    let _ = stdout().flush();
    stdin().read_line(&mut set_diff).expect("Not good!");
    let set_diff: i32 = set_diff.trim().parse().expect("Not good!");

    println!("Setting difficulty: {}", set_diff);
    difficulty = set_diff;

    println!("You enter the dungeon!");
    println!("Write a w to move forward!");
    
    let rat = Rat {
        hp: 5,
        damage: 2
    };
    let progress = Progress {
        tile: 0
    };
    let player = Player {
        hp: 20,
        damage: 5
    };
    going_forward(progress.tile, difficulty, rat.hp, rat.damage, player.hp, player.damage, false);
}

fn going_forward(progress: i32, diff: i32, rhp: i32, rdamage: i32, php: i32, pd: i32, pu: bool) {
    let mut tsc = rand::rng().random_range(4..=6);
    if pu == true {
        tsc = 0
    }
    let mut forward_button = String::new();
    stdin().read_line(&mut forward_button).expect("Not good!");
    let f = forward_button.trim();
    forward(f, progress, diff, rhp, rdamage, php, pd, tsc);
}

fn forward(f: &str, mut p: i32, diff: i32, rhp: i32, rdamage: i32, php: i32, pd: i32, tsc: i32) {
    match f.trim() {
        "w" => {
            p += 1;
            println!("progress: {}", p);
            println!("you advanced 1 tile!\n");
            let rat_spawn_chance = rand::rng().random_range(1..=10);

            // if tsc == 5 {
            //     item_found( p, diff, rhp, rdamage, php, pd);
            // }
            // Debug start
            println!("tsc: {}", tsc);
            // Debug end

            // Debug start
            println!("rsc: {}", rat_spawn_chance);
            // Debug end
            match rat_spawn_chance.cmp(&diff) {
                Ordering::Less => going_forward(p, diff, rhp, rdamage, php, pd, true),
                Ordering::Equal => going_forward(p, diff, rhp, rdamage, php, pd, true),
                Ordering::Greater => {
                    println!("You encountered a rat!");
                    fight_with_rat(rhp, rdamage, diff, php, pd, p);
                }
            };
        },
        "ferris" => {
            let stdout = stdout();
            let message = String::from("Hello fellow Rustaceans!");
            let width = message.chars().count();

            let mut writer = BufWriter::new(stdout.lock());
            say(&message, width, &mut writer).unwrap();
        },
        "exit" => {
            println!("See you soon!");
            exit(0);
        },
        _ => {
            println!("w to move forward!");
        }
    }
}

fn fight_with_rat(mut rhp: i32, rdamage: i32, diff: i32, mut php: i32, pd: i32, p:i32) {
    let attack_chance = rand::rng().random_range(1..=10);
    // Debug start
    println!("ac: {}", attack_chance);
    // Debug end
    println!("Type f to fight!");
    let mut choice = String::new();
    stdin().read_line(&mut choice).expect("Not good!");
    let choice = choice.trim();
    match choice {
        "f" => match attack_chance.cmp(&diff) {
            Ordering::Less => {
                rhp -= pd;
                // Debug start
                println!("rhp: {}", rhp);
                println!("pd: {}", pd);
                // Debug end
                if rhp == 0 {
                    println!("You defeated the rat!\n");
                    rhp = 5;
                    going_forward(p, diff, rhp, rdamage, php, pd, true);
                } else {
                    println!("The rat is not dead!");
                    fight_with_rat(rhp, rdamage, diff, php, pd, p);
                }
            }
            Ordering::Equal => {
                println!("The rat strikes you!\n");
                php -= 2;
                if php == 0 {
                    println!("You are dead!");
                    exit(0);
                }
                // Debug start
                println!("php: {}", php);
                // Debug end
                fight_with_rat(rhp, rdamage, diff, php, pd, p);
            }
            Ordering::Greater => {
                println!("The rat strikes you!\n");
                php -= 2;
                if php == 0 {
                    println!("You are dead!");
                    exit(0);
                }
                // Debug start
                println!("php: {}", php);
                // Debug end
                fight_with_rat(rhp, rdamage, diff, php, pd, p);
            }
        }
        _ => println!("Type f to fight!")
    }
}
