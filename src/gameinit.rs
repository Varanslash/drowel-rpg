mod game;
use game::game;
mod gamedata;
use gamedata::{Player, Class};
mod playerattackdata;
use playerattackdata::*;
mod monsterattackdata;

fn main() {
    let char_points = 70;
    println!("
       █▀▄▄▄█▀                                                               ▄▄▄▀▀▀█
        █   █                                                                 █   █ 
        █ ░ █    ▀▄▄▄▄▄▄▄▀   ▀▄▄▄▄▄▄▄▀     ▀▄▄▄  █▄▀   ▄▄▄▀     ▄▀▀█▀▀█▀▄▄    █ ░ █ 
    ▄▄▄ █░▒░█   ██▓██▀██▓█  ██▓██▀██▓██   ██▓██  █▓██  ██▓██   █ ░▒█  ▒█▓█░   █░▒░█ 
 ▄▄▀▀▀▀▄█▒▓▒█  █▓▒▓ ▌ ▐ ▓▒ █▓▒▓     ▓▒▓█ █▓▒▓     ▓▒▓█   ▓▒▓█ █▒░▒░   ░▓▀▀▀   █▒▓▒█ 
░ ▒░█  ██▓█▓█  █▒░▒▓       █▒░▒▓   ▓▒░▒█ █▒░▒▓   ▓▒░▒█  ▓▒░▒█ █▒▓▒█▀■▀▀▀      █▓█▓░ 
▐░▓▒▌  ▐░█░██▌ █░ ░▓       █░ ░▓   ▓░ ░█ █░ ░▓   ▓░ ░█  ▓░ ░█ █▓█▓░    ▄▄■▄   █ ░ ▒ 
▓▓ ▓▓   ▒░▒░█▌ ██ ░░        █ ░▒ ▄ ▒░ █   █ ░▒▄ ▄▒░ ██▄▄▒░ █   █ ░▒▄ ▄▒░░▒▓▌  █░▒░▓ 
 ▄▀▀▀▀▄▄▄▀▀▀▀  ▄▄▀▄▄         ▀▄▄▄■▄▄▄▀     ▀▄▄▄▄▄▄▄▀ ▀▄▄▄▄▀      ▀▄▄▄▄▀▀▀▀    █▀▀▀  
                                                                             ▀▀     "); // the peakest title of all time
    println!("Choose your character's name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).expect("Failed to read line");

    println!("Choose your character's class: ");
    let mut classd = String::new();
    std::io::stdin().read_line(&mut classd).expect("Failed to read line");
    let class = match classd.trim().to_lowercase().as_str() {
        "wizard" => Class::Wizard,
        "fighter" => Class::Fighter,
        "bishop" => Class::Bishop,
        "paladin" => Class::Paladin,
        "barbarian" => Class::Barbarian,
        "bard" => Class::Bard,
        "druid" => Class::Druid,
        &_ => {
            println!("Not a valid class!");
            return;
        }
    };

    // We don't have dice, budget better
    println!("Choose your attributes in format str/dex/con/int/wis/cha ({} points budget): ", char_points);
    let mut stats = String::new();
    std::io::stdin().read_line(&mut stats).expect("Failed to read line");
    let split_stats: Vec<&str> = stats.trim().split('/').collect();
    // Check if there's actually enough stats to work with at all
    match split_stats.len() {
        6 => {
            // Originally stat init was contained here, but scope errors dictated otherwise
        }
        _ => {
            println!("Invalid number of attributes. Please enter 6 attributes separated by '/'.");
            return;
        }
    }

    
    let strength: u8 = match split_stats[0].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Strength must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for strength; please enter a number between 1 and 20.");
            return;
        }
    };
    let dexterity: u8 = match split_stats[1].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Dexterity must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for dexterity; please enter a number between 1 and 20.");
            return;
        }
    };
    let constitution: u8 = match split_stats[2].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Constitution must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for constitution; please enter a number between 1 and 20.");
            return;
        }
    };
    let intelligence: u8 = match split_stats[3].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Intelligence must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for intelligence; please enter a number between 1 and 20.");
            return;
        }
    };
    let wisdom: u8 = match split_stats[4].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Wisdom must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for wisdom; please enter a number between 1 and 20.");
            return;
        }
    };
    let charisma: u8 = match split_stats[5].parse() {
        Ok(val) => match val {
            1..=20 => val,
            _ => {
                println!("Charisma must be between 1 and 20.");
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for charisma; please enter a number between 1 and 20.");
            return;
        }
    };

    // Ensures the player doesn't overflow their budget, they are allowed to have less than 70 though
    let point_check = strength + dexterity + constitution + intelligence + wisdom + charisma;
    if point_check > 70 {
        println!("Total points must be less than or equal to 70.");
        return;
    }

    let player = Player {
        strength: strength,
        dexterity: dexterity,
        constitution: constitution,
        intelligence: intelligence,
        wisdom: wisdom,
        charisma: charisma,
        level: 1,
        name: name.trim().to_string(),
        hp: 40,
        class: class,
        attack_formula: nanattack,
        attack_formula_2: nanattack,
        attack_formula_3: nanattack,
        attack_formula_4: nanattack,
        xp: 0,
    };

    game(player);
}