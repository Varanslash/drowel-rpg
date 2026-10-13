use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use crate::encounter;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;
use encounter::*;

pub fn game(mut player: Player, mut rooms: HashMap<u32, Room>, monsterdata: HashMap<MonsterList, Monster>, maps: HashMap<Area, String>) {
    // Game Data Initialization
    let mut room = 1;
    let mut encvaltracker = 0; // This tracks a mod 4 value as a pseudo-random number generator for monster encounters, since I'm too lazy for crates
    let commandlist = vec!["north", "south", "east", "west", "help", "?", "n", "s", "e", "w", 
    "help verbose", "hv", 
    "map", "m", 
    "stat", "st"]; 
    // List of valid commands for internal use

    // Game loop
    while room != 0 && room != 3900000 { // 0 = invalid room; 3900000 = credits (the final boss's room is 3899999, one step before)
        let roomdata = rooms.get(&room).unwrap();
        if encvaltracker >= roomdata.encval { // Always check in beginning for monster encounters
            player = encounter::encounter(player, roomdata.monster, &monsterdata);
            if player.hp < 0 {
                println!("You died!");
                std::process::exit(0);
            }
            encvaltracker = 0;
            if player.xp >= (((2u64.pow(player.level as u32)) / 2) + 20 * (player.level as u64)) {
                player.level += 1;
                println!("You are now level {}!", player.level);
                if player.level % 4 == 0 {
                    player.strength += 1;
                    player.dexterity += 1;
                    player.constitution += 1;
                    player.intelligence += 1;
                    player.wisdom += 1;
                    player.charisma += 1;
                    println!("Character upgrade! +1 to every stat.")
                }
            }
        }

        // Room data usage; actually use the required room data to show the room to the player.
        println!("{}", roomdata.description);
        println!("What do you want to do? (type 'help' for commands)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");

        // Transfer to next room
        match input.trim().to_lowercase().as_str() {
            cmd if commandlist.contains(&cmd) => {
                // This launches movement function such that we can get the value for the next room
                match cmd {
                    "north" | "south" | "east" | "west" | "n" | "s" | "e" | "w" | "map" | "m" => {
                            let new_room = movement(room, roomdata.north, roomdata.south, roomdata.east, roomdata.west, cmd.to_string(), &maps, roomdata.area);
                            if new_room != room {
                                room = new_room;
                                encvaltracker += new_room % 4; // Adds a value mod 4 to the encval tracker, so values of only -1<n<4 are added for seeming randomness
                                // It also compresses huge numbers so any edits to the game that have huge room numbers flatten to -1<n<4 and ensure encounters don't run rampant unless the coder wants them to
                        }
                    }

                    "help" | "?" => {
                        println!("Available commands: north (n), south (s), east (e), west (w), help (?), help verbose (hv), map (m)");
                        println!("You can type \"help verbose\" (hv) to get a more detailed description of available actions.");
                    }

                    "help verbose" | "hv" => {
                        println!("-----START-----");
                        println!("\"west\" (w) - Moves west if available.");
                        println!("\"east\" (e) - Moves east if available.");
                        println!("\"north\" (n) - Moves north if available.");
                        println!("\"south\" (s) - Moves south if available.");
                        println!("\"help\" (?) - Displays a simple list of commands.");
                        println!("\"help verbose\" (hv) - Displays this list.");
                        println!("\"map\" (m) - Displays an indexed map of the current area.");
                        println!("\"stat\" (s) - Displays a detailed view of player stats.");
                        println!("---ATTACK COMMANDS---");
                        println!("\"attack\" (a) - Attacks any one of four attacks based on player choice.");
                        println!("\"run\" (r) - Runs from battle. If a monster outcompetes you on 2/3 physical stats, then the run fails and the monster gets a free turn.");
                        println!("\"help\" (?) - Displays a simple list of attack commands.");
                        println!("\"stat\" (s) - Displays player and monster stats.");
                        println!("------END------");
                    }

                    "stat" | "st" => {
                        println!("-----STATS-----");
                        println!("Player stats: {}/{}/{}/{}/{}/{} (s/d/c/i/w/c format)", player.strength, player.dexterity, player.constitution, player.intelligence, player.wisdom, player.charisma);
                        println!("Player name: {}", player.name);
                        println!("Player level: {}", player.level);
                        println!("Player XP: {}", player.xp);
                        println!("Player HP: {}", player.hp);
                        println!("Player class: {:?}", player.class);

                        if &player.items.len() > &0 {
                            println!("---INVENTORY---");
                            for item in &player.items {
                                println!("Player carries {:?}.", item);
                            }
                            println!("------END------");
                        }
                        else {
                            println!("Player carries no items.")
                        }

                        println!("Player status: {:?}", player.status);
                        println!("Player aura: {:?}", player.passive);
                        println!("Player wealth: {}", player.money);
                        println!("------END------");
                    }

                    _ => {
                        eprintln!("Every command here should be valid. If you see this message, either a command was coded but not implemented yet or a mangled command is somehow coded in. Command: {}", cmd);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }
}

pub fn movement(
    room: u32, 
    north: u32, 
    south: u32, 
    east: u32, 
    west: u32, 
    direction: String, 
    maps: &HashMap<Area, String>, 
    area: Area,
    ) -> u32 {

    let dest = match direction.to_lowercase().as_str() {
        "west" | "w" => west,
        "east" | "e" => east,
        "south" | "s" => south,
        "north" | "n" => north,
        "map" | "m" => {
            let map = maps.get(&area).unwrap();
            println!("{}", map);
            return room;
        }
        _ => {
            println!("Invalid command. Type 'help' for a list of commands.");
            return room;
        }
    };

    if dest == 0 {
        println!("You can't go that way.");
        return room;
    }

    return dest;
}