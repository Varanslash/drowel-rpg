use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use crate::encounter;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;
use encounter::*;
use crate::itemformulas::*;

pub fn game(mut player: Player, mut rooms: HashMap<u32, Room>, monsterdata: HashMap<MonsterList, Monster>, maps: HashMap<Area, String>) {
    // Game Data Initialization
    let mut room = 1;
    let mut encvaltracker = 0; // This tracks a mod 4 value as a pseudo-random number generator for monster encounters, since I'm too lazy for crates
    let commandlist = vec!["north", "south", "east", "west", "help", "?", "n", "s", "e", "w", 
    "help verbose", "hv", 
    "map", "m", 
    "stat", "st",
    "talk", "t",
    "gather", "g",
    "i'm from the 80's. we tried so hard to sound like we were from the year 2019, so that you kids would love us one day and forgive us for everything else we did.",
    "buy", "b",
    "use", "u",
    "inspect", "i",
    "buy", "bg",
    "wait", "wt",
    "trigger", "tr",
    "drop", "d"]; 
    // List of valid commands for internal use

    // Game loop
    while room != 0 && room != 3900000 { // 0 = invalid room; 3900000 = credits (the final boss's room is 3899999, one step before)
        let roomdata = rooms.get_mut(&room).unwrap();

        match room {
            _ => {}
        }

        if encvaltracker >= roomdata.encval { // Always check in beginning after special room scripting for monster encounters
            let encbool = encounter::encounter(player, roomdata.monster, &monsterdata);
            player = encbool.0;
            let wincheck = encbool.1;
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
                if player.constitution % 2 == 0 {
                    player.maxhp += player.constitution / 2;
                }
                else {
                    player.maxhp += (player.constitution - 1) / 2;
                }
            }

            match room {
                _ => {}
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
                        println!("Available commands: north (n), south (s), east (e), west (w), help (?), help verbose (hv), map (m), stat (st), gather (g), talk (t), buy (b), use (u), inspect (i), bag (bg), wait (wt), trigger (tr), drop (d)");
                        println!("You can type \"help verbose\" (hv) to get a more detailed description of available actions.");
                    }

                    "help verbose" | "hv" => {
                        hv();
                    }

                    "stat" | "st" => {
                        println!("-----STATS-----");
                        println!("Player stats: {}/{}/{}/{}/{}/{} (s/d/c/i/w/c format)", player.strength, player.dexterity, player.constitution, player.intelligence, player.wisdom, player.charisma);
                        println!("Player name: {}", player.name);
                        println!("Player level: {}", player.level);
                        println!("Player XP: {}", player.xp);
                        println!("Player HP/Max HP: {}/{}", player.hp, player.maxhp);
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

                    "talk" | "t" => {
                        println!("{}", roomdata.npc);
                    }

                    "gather" | "g" => {
                        let mut disposablecopy = rooms.get(&room).unwrap().clone();
                        player.money += disposablecopy.money;
                        println!("Gained {} money.", disposablecopy.money);
                        disposablecopy.money = 0;
                        if disposablecopy.items.len() > 0 {
                            for item in disposablecopy.items {
                                player.items.push(item);
                                println!("Gained {:?}.", item);
                            }
                            disposablecopy.items = Vec::new();
                        }
                        else {
                            println!("No items to collect.");
                        }
                        rooms.insert(room, disposablecopy);
                    }

                    "buy" | "b" => {
                        if roomdata.shop.len() > 0 {
                            println!("What item would you like to buy? (Type the index!)");
                            let mut itemidx = 1;
                            for item in &roomdata.shop {
                                println!("- Index: {}, Item: {:?}, Cost: {}", itemidx, item.0, item.1);
                                itemidx += 1;
                            }

                            let mut itembuy = String::new();
                            std::io::stdin().read_line(&mut itembuy).expect("Failed to read line");
                            let itembuyidx: i32 = match itembuy.trim().parse() {
                                Ok(val) => {
                                    val
                                }
                                Err(_) => {
                                    println!("You have to pick a number!");
                                    continue;
                                }
                            };

                            if itembuyidx <= 0 {
                                println!("Invalid item index!");
                                continue;
                            }

                            else if (itembuyidx as usize - 1) >= roomdata.shop.len() {
                                println!("This item doesn't exist!");
                                continue;
                            }

                            else {
                                let buyingitem = roomdata.shop[itembuyidx as usize - 1];
                                if player.money < buyingitem.1 {
                                    println!("You don't have enough money for this item!");
                                    continue;
                                }
                                else {
                                    player.money -= buyingitem.1;
                                    player.items.push(buyingitem.0);
                                    println!("Thank you for your purchase!");
                                }
                            }
                        }
                    }

                    "use" | "u" => {
                        if player.items.len() > 0 {
                            println!("What item would you like to use?");
                            let mut bagidx = 1;
                            for item in &player.items {
                                println!("- Index: {}, Item: {:?}", bagidx, item);
                                bagidx += 1;
                            }

                            let mut bagitem = String::new();
                            std::io::stdin().read_line(&mut bagitem).expect("Failed to read line");

                            let bagitemidx: i32 = match bagitem.trim().parse() {
                                Ok(val) => {
                                    val
                                }
                                Err(_) => {
                                    println!("You have to pick a number!");
                                    continue;
                                }
                            };

                            if bagitemidx <= 0 {
                                println!("Invalid item index!");
                                continue;
                            }

                            else if (bagitemidx as usize - 1) >= player.items.len() {
                                println!("This item doesn't exist!");
                                continue;
                            }

                            else {
                                let itemused = player.items.remove(bagitemidx as usize - 1);
                                match itemused {
                                    _ => {
                                        println!("{:?} was used.", itemused);
                                    }
                                }
                            }
                        }
                    }

                    "inspect" | "i" => {
                        println!("On inspection: {}", roomdata.description2);
                    }

                    "bag" | "bg" => {
                        if player.items.len() > 0 {
                            let mut bagidx = 1;
                            for item in &player.items {
                                println!("- Index: {}, Item: {:?}", bagidx, item);
                                bagidx += 1;
                            }
                        }
                        else {
                            println!("No items in bag!")
                        }
                    }

                    "wait" | "wt" => {
                        println!("You waited.");
                    }

                    "trigger" | "tr" => {
                        match room {
                            _ => { println!("You attempted to trigger something. Nothing happened."); }
                        }
                    }

                    "drop" | "d" => {
                        if player.items.len() > 0 {
                            println!("What item would you like to drop?");
                            let mut bagidx = 1;
                            for item in &player.items {
                                println!("- Index: {}, Item: {:?}", bagidx, item);
                                bagidx += 1;
                            }

                            let mut bagitem = String::new();
                            std::io::stdin().read_line(&mut bagitem).expect("Failed to read line");

                            let bagitemidx: i32 = match bagitem.trim().parse() {
                                Ok(val) => {
                                    val
                                }
                                Err(_) => {
                                    println!("You have to pick a number!");
                                    continue;
                                }
                            };

                            if bagitemidx <= 0 {
                                println!("Invalid item index!");
                                continue;
                            }

                            else if (bagitemidx as usize - 1) >= player.items.len() {
                                println!("This item doesn't exist!");
                                continue;
                            }

                            else {
                                let itemused = player.items.remove(bagitemidx as usize - 1);
                                println!("{:?} was used.", itemused);
                            }
                        }
                    }

                    "i'm from the 80's. we tried so hard to sound like we were from the year 2019, so that you kids would love us one day and forgive us for everything else we did." => {
                        println!("https://youtu.be/AJUuOMInBds")
                    }

                    _ => {
                        eprintln!("E-001; Every command here should be valid. If you see this message, either a command was coded but not implemented yet or a mangled command is somehow coded in. Command: {}", cmd);
                        std::process::exit(1);
                    }
                }
            }
            _ => {
                println!("Invalid command. Type 'help' for a list of commands.");
            }
        }
    }

    if room == 0 {
        eprintln!("E-003; Room index is invalid.");
        std::process::exit(1);
    }

    if room == 3900000 {
        return;
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

pub fn hv() {
    println!("-----START-----");
    println!("\"west\" (w) - Moves west if available.");
    println!("\"east\" (e) - Moves east if available.");
    println!("\"north\" (n) - Moves north if available.");
    println!("\"south\" (s) - Moves south if available.");
    println!("\"help\" (?) - Displays a simple list of commands.");
    println!("\"help verbose\" (hv) - Displays this list.");
    println!("\"map\" (m) - Displays an indexed map of the current area.");
    println!("\"stat\" (st) - Displays a detailed view of player stats.");
    println!("\"talk\" (t) - Talks to any NPC in the room if there is one.");
    println!("\"gather\" (g) - Gathers any items and money that may reside in the room.");
    println!("\"buy\" (b) - Buys an item from a shop, if there is one.");
    println!("\"use\" (u) - Uses an item from your inventory.");
    println!("\"inspect\" (i) - Inspects a room.");
    println!("\"bag\" (bg) - Displays your bag.");
    println!("\"wait\" (wt) - Waits. Does literally nothing but waits.");
    println!("\"trigger\" (tr) - Triggers special room events in certain rooms.");
    println!("\"drop\" (d) - Drops an item from your inventory.");
    println!("---ATTACK COMMANDS---");
    println!("\"attack\" (a) - Attacks any one of four attacks based on player choice.");
    println!("\"run\" (r) - Runs from battle. If a monster outcompetes you on 2/3 physical stats, then the run fails and the monster gets a free turn.");
    println!("\"help\" (?) - Displays a simple list of attack commands.");
    println!("\"stat\" (s) - Displays player and monster stats.");
    println!("------END------");
}