mod game;
mod gamedata;
mod playerattackdata;
mod monsterattackdata;
mod encounter;
mod world;
mod itemformulas;
use game::game;
use gamedata::*;
use playerattackdata::*;
use std::collections::HashMap;
use world::init;
use world::credits;

const budgetforstats: i32 = 64;
const statcap: i32 = 20;

fn main() {
    let attacklist: HashMap<Class, [fn(&Player) -> (i32, Status); 4]> = HashMap::from([
        (Class::Wizard, [nanattack as fn(&Player) -> (i32, Status), nanattack, nanattack, nanattack]),
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

    
    let strength: i32 = match split_stats[0].parse() {
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
    let dexterity: i32 = match split_stats[1].parse() {
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
    let constitution: i32 = match split_stats[2].parse() {
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
    let intelligence: i32 = match split_stats[3].parse() {
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
    let wisdom: i32 = match split_stats[4].parse() {
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
    let charisma: i32 = match split_stats[5].parse() {
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
        maxhp: 40,
        class: class,
        attack_formula: attackformulas[0],
        attack_formula_2: attackformulas[1],
        attack_formula_3: attackformulas[2],
        attack_formula_4: attackformulas[3],
        xp: 0,
        items: Vec::new(),
        keys: Vec::new(),
        status: Status::None,
        passive: Passive::None,
        money: 0
    };

    let world = init();
    game(player, world.0, world.1, world.2);
    credits();
}