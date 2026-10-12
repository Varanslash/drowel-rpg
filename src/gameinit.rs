mod game;
use game::game;
mod gamedata;
use gamedata::{Player, Class};
mod playerattackdata;
use playerattackdata::*;
mod monsterattackdata;
use std::collections::HashMap;

const budgetforstats: u8 = 64;
const statcap: u8 = 20;

fn main() {
    let attacklist: HashMap<Class, [fn(&Player) -> i32; 4]> = HashMap::from([
        (Class::Wizard, [nanattack as fn(&Player) -> i32, nanattack, nanattack, nanattack]),
        (Class::Fighter, [nanattack, nanattack, nanattack, nanattack]),
        (Class::Bishop, [nanattack, nanattack, nanattack, nanattack]),
        (Class::Druid, [nanattack, nanattack, nanattack, nanattack]),
        (Class::Paladin, [nanattack, nanattack, nanattack, nanattack]),
        (Class::Barbarian, [nanattack, nanattack, nanattack, nanattack]),
        (Class::Bard, [nanattack, nanattack, nanattack, nanattack]),
    ]);
    let char_points = budgetforstats;
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
            1..=statcap => val,
            _ => {
                println!("Strength must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for strength; please enter a number between 1 and {}.", statcap);
            return;
        }
    };
    let dexterity: u8 = match split_stats[1].parse() {
        Ok(val) => match val {
            1..=statcap => val,
            _ => {
                println!("Dexterity must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for dexterity; please enter a number between 1 and {}.", statcap);
            return;
        }
    };
    let constitution: u8 = match split_stats[2].parse() {
        Ok(val) => match val {
            1..=statcap => val,
            _ => {
                println!("Constitution must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for constitution; please enter a number between 1 and {}.", statcap);
            return;
        }
    };
    let intelligence: u8 = match split_stats[3].parse() {
        Ok(val) => match val {
            1..=statcap => val,
            _ => {
                println!("Intelligence must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for intelligence; please enter a number between 1 and {}.", statcap);
            return;
        }
    };
    let wisdom: u8 = match split_stats[4].parse() {
        Ok(val) => match val {
            1..=statcap => val,
            _ => {
                println!("Wisdom must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for wisdom; please enter a number between 1 and {}.", statcap);
            return;
        }
    };
    let charisma: u8 = match split_stats[5].parse() {
        Ok(val) => match val {
            1..=statcap => val,
            _ => {
                println!("Charisma must be between 1 and {}.", statcap);
                return;
            }
        },
        Err(_) => {
            println!("Invalid input for charisma; please enter a number between 1 and {}.", statcap);
            return;
        }
    };

    // Ensures the player doesn't overflow their budget, they are allowed to have less than 70 though
    let point_check = strength + dexterity + constitution + intelligence + wisdom + charisma;
    if point_check > budgetforstats {
        println!("Total points must be less than or equal to {}.", budgetforstats);
        return;
    }

    let attackformulas = attacklist.get(&class).unwrap();

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
        attack_formula: attackformulas[0],
        attack_formula_2: attackformulas[1],
        attack_formula_3: attackformulas[2],
        attack_formula_4: attackformulas[3],
        xp: 0,
    };

    game(player);
}