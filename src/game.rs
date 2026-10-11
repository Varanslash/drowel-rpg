use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;

pub fn game(mut player: Player) {
    // Room Data Initialization
    let rooms = HashMap::from([
        (1, Room {
            description: "You are in a dark forest. [NS]".to_string(),
            north: 2,
            south: 3,
            east: 0,
            west: 0,
            monster: MonsterList::Goblin,
            encval: 20
        }),
        (2, Room {
            description: "You are on a mountain trail. [S]".to_string(),
            north: 0,
            south: 1,
            east: 0,
            west: 0,
            monster: MonsterList::Orc,
            encval: 20
        }),
        (3, Room {
            description: "You are in a cave. [N]".to_string(),
            north: 1,
            south: 0,
            east: 0,
            west: 0,
            monster: MonsterList::Goblin,
            encval: 20
        }),
    ]);

    // Monster Data Initialization
    let monsterdata = HashMap::from([
        (MonsterList::Goblin, Monster {
            strength: 8,
            dexterity: 12,
            constitution: 8,
            intelligence: 6,
            wisdom: 6,
            charisma: 4,
            level: 1,
            name: "Goblin".to_string(),
            hp: 15,
            attack_formula: mon_nanattack,
            attack_formula_2: mon_nanattack,
            attack_formula_3: mon_nanattack,
            attack_formula_4: mon_nanattack,
            xp: 10
        }),
        (MonsterList::Orc, Monster {
            strength: 16,
            dexterity: 10,
            constitution: 14,
            intelligence: 6,
            wisdom: 8,
            charisma: 6,
            level: 2,
            name: "Orc".to_string(),
            hp: 30,
            attack_formula: mon_nanattack,
            attack_formula_2: mon_nanattack,
            attack_formula_3: mon_nanattack,
            attack_formula_4: mon_nanattack,
            xp: 20
        }),
    ]);

    // Game Data Initialization
    let mut room = 1;
    let mut encvaltracker = 0; // This tracks a mod 4 value as a pseudo-random number generator for monster encounters, since I'm too lazy for crates
    let commandlist = vec!["north", "south", "east", "west", "help", "?", "n", "s", "e", "w", "help verbose", "hv"]; // List of valid commands for internal use


    while room != 0 && room != 3900000 { // 0 = invalid room; 3900000 = credits (the final boss's room is 3899999, one step before)
        let roomdata = rooms.get(&room).unwrap();
        if encvaltracker >= roomdata.encval { // Always check in beginning for monster encounters
            player = encounter(player, roomdata.monster, &monsterdata);
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
                let new_room = movement(room, roomdata.north, roomdata.south, roomdata.east, roomdata.west, cmd.to_string());
                if new_room != room {
                    room = new_room;
                    encvaltracker += new_room % 4; // Adds a value mod 4 to the encval tracker, so values of only -1<n<4 are added for seeming randomness
                    // It also compresses huge numbers so any edits to the game that have huge room numbers flatten to -1<n<4 and ensure encounters don't run rampant unless the coder wants them to
                }
            }
            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }
}

pub fn movement(room: u32, north: u32, south: u32, east: u32, west: u32, direction: String) -> u32 {
    let dest = match direction.to_lowercase().as_str() {
        "west" | "w" => west,
        "east" | "e" => east,
        "south" | "s" => south,
        "north" | "n" => north,
        "help" | "?" => {
            println!("Available commands: north (n), south (s), east (e), west (w), help (?)");
            println!("You can type \"help verbose\" (hv) to get a more detailed description of available actions.");
            return room;
        }
        "help verbose" | "hv" => {
            println!("-----START-----");
            println!("\"west\" (w) - Moves west if available.");
            println!("\"east\" (e) - Moves east if available.");
            println!("\"north\" (n) - Moves north if available.");
            println!("\"south\" (s) - Moves south if available.");
            println!("\"help\" (?) - Displays a simple list of commands.");
            println!("\"help verbose\" (hv) - Displays this list.");
            println!("---ATTACK COMMANDS---");
            println!("\"attack\" (a) - Attacks any one of four attacks based on player choice.");
            println!("\"run\" (r) - Runs from battle. If a monster outcompetes you on 2/3 physical stats, then the run fails and the monster gets a free turn.");
            println!("\"help\" (?) - Displays a simple list of attack commands.");
            println!("\"stat\" (s) - Displays player and monster stats.");
            println!("------END------");
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

pub fn encounter(player: Player, mut monster: MonsterList, monsterdata: &HashMap<MonsterList, Monster>) -> Player {
    let mut playerturn = true;
    println!("{:?} has appeared!", monster);
    let mut monsterfightdata = monsterdata.get(&monster).unwrap().clone();
    let mut playerfightdata = player;

    // Fight Loop
    while monsterfightdata.hp >= 0 && playerfightdata.hp >= 0 {
        match playerturn {

            // Player's Turn
            true => {
                println!("Your turn.");
                // Player chooses an action here
                println!("Choose an action: ");
                let mut action = String::new();
                std::io::stdin().read_line(&mut action).expect("Failed to read line");

                // Action processing
                match action.trim().to_lowercase().as_str() {

                    // This uses any one of the four attacks a player would possess. Will potentially add a list action to list attacks that ara available
                    "attack" | "a" => {
                        println!("Which attack would you like to use? (1-4): ");
                        let mut attackindex = String::new();
                        std::io::stdin().read_line(&mut attackindex).expect("Failed to read line");
                        let idx = match attackindex.trim().parse::<u8>() {
                            Ok(n) => n,
                            Err(_) => {
                                println!("Invalid attack index!");
                                continue;
                            }
                        };

                        match idx {
                            1 => {
                                let dmg = (playerfightdata.attack_formula)(&playerfightdata);
                                println!("{} did {} damage to {}!", playerfightdata.name, dmg, monsterfightdata.name);
                                monsterfightdata.hp -= dmg;
                                println!("{} has {} HP left!", monsterfightdata.name, monsterfightdata.hp);
                                playerturn = false;
                            }
                            2 => {
                                let dmg = (playerfightdata.attack_formula_2)(&playerfightdata);
                                println!("{} did {} damage to {}!", playerfightdata.name, dmg, monsterfightdata.name);
                                monsterfightdata.hp -= dmg;
                                println!("{} has {} HP left!", monsterfightdata.name, monsterfightdata.hp);
                                playerturn = false;
                            }
                            3 => {
                                let dmg = (playerfightdata.attack_formula_3)(&playerfightdata);
                                println!("{} did {} damage to {}!", playerfightdata.name, dmg, monsterfightdata.name);
                                monsterfightdata.hp -= dmg;
                                println!("{} has {} HP left!", monsterfightdata.name, monsterfightdata.hp);
                                playerturn = false;
                            }
                            4 => {
                                let dmg = (playerfightdata.attack_formula_4)(&playerfightdata);
                                println!("{} did {} damage to {}!", playerfightdata.name, dmg, monsterfightdata.name);
                                monsterfightdata.hp -= dmg;
                                println!("{} has {} HP left!", monsterfightdata.name, monsterfightdata.hp);
                                playerturn = false;
                            }
                            _ => {
                                println!("Not a valid attack index!");
                                continue;
                            }
                        }
                    }

                    // You can't run if the monster outmatches you on at least 2 physical stats
                    "run" | "r" => {
                        let mut runcheck = 0;
                        if monsterfightdata.strength > playerfightdata.strength {
                            runcheck += 1;
                        }
                        
                        if monsterfightdata.dexterity > playerfightdata.dexterity {
                            runcheck += 1;
                        }
                        
                        if monsterfightdata.constitution > playerfightdata.constitution {
                            runcheck += 1;
                        }
                        if runcheck > 1 {
                            println!("Can't run!");
                            playerturn = false;
                        }
                        else {
                            println!("You ran!");
                            break;
                        }
                    }
                    "help" | "?" => {
                        println!("Available commands in battle: attack (a), run (r), help (?), stat (s)");
                    }
                    "stat" | "s" => {
                        println!("-----STATS-----");
                        println!("Player HP: {}", playerfightdata.hp);
                        println!("Monster HP: {}", monsterfightdata.hp);
                        println!("Player stats: {}/{}/{}/{}/{}/{} (s/d/c/i/w/c format)", playerfightdata.strength, playerfightdata.dexterity, playerfightdata.constitution, playerfightdata.intelligence, playerfightdata.wisdom, playerfightdata.charisma);
                        println!("Monster stats: {}/{}/{}/{}/{}/{} (s/d/c/i/w/c format)", monsterfightdata.strength, monsterfightdata.dexterity, monsterfightdata.constitution, monsterfightdata.intelligence, monsterfightdata.wisdom, monsterfightdata.charisma);
                        println!("------END------");
                        continue;
                    }
                    _ => {
                        println!("Invalid action!");
                        continue;
                    }
                }
            }

            // Monster's turn
            false => {
                println!("Monster's turn.");
                let attackindex = monsterfightdata.hp % 4;
                // Choose an attack based on monster health; this makes it seem somewhat random since different attacks would do different damage to the monster
                match attackindex {
                    0 => {
                        let dmg = (monsterfightdata.attack_formula)(&monsterfightdata);
                        println!("{} did {} damage to {}!", monsterfightdata.name, dmg, playerfightdata.name);
                        playerfightdata.hp -= dmg;
                        println!("{} has {} HP left!", playerfightdata.name, playerfightdata.hp);
                        playerturn = true;
                    }
                    1 => {
                        let dmg = (monsterfightdata.attack_formula_2)(&monsterfightdata);
                        println!("{} did {} damage to {}!", monsterfightdata.name, dmg, playerfightdata.name);
                        playerfightdata.hp -= dmg;
                        println!("{} has {} HP left!", playerfightdata.name, playerfightdata.hp);
                        playerturn = true;
                    }
                    2 => {
                        let dmg = (monsterfightdata.attack_formula_3)(&monsterfightdata);
                        println!("{} did {} damage to {}!", monsterfightdata.name, dmg, playerfightdata.name);
                        playerfightdata.hp -= dmg;
                        println!("{} has {} HP left!", playerfightdata.name, playerfightdata.hp);
                        playerturn = true;
                    }
                    3 => {
                        let dmg = (monsterfightdata.attack_formula_4)(&monsterfightdata);
                        println!("{} did {} damage to {}!", monsterfightdata.name, dmg, playerfightdata.name);
                        playerfightdata.hp -= dmg;
                        println!("{} has {} HP left!", playerfightdata.name, playerfightdata.hp);
                        playerturn = true;
                    }

                    // This branch should not happen, which is why it errors and exits
                    _ => {
                        eprintln!("Error: Invalid attack index; mod 4 returned 4 or higher");
                        std::process::exit(1);
                    }
                }
            }
        }
    }

    if monsterfightdata.hp < 0 {
        playerfightdata.xp += monsterfightdata.xp;
        println!("You win! Gain {} xp.", monsterfightdata.xp);
    }

    return playerfightdata;
}