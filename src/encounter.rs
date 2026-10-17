use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;

pub fn encounter(player: Player, mut monster: MonsterList, monsterdata: &HashMap<MonsterList, Monster>) -> (Player, bool) {
    let mut strmod = 0;
    let mut dexmod = 0;
    let mut conmod = 0;
    let mut intmod = 0;
    let mut wismod = 0;
    let mut chamod = 0;
    let mut turncounter = 0;
    println!("{:?} has appeared!", monster);
    let mut monsterfightdata = monsterdata.get(&monster).unwrap().clone();
    let mut playerfightdata = player;
    let mut playerturn = !monsterfightdata.initiative;

    if monsterfightdata.trail >= playerfightdata.level {
        monsterfightdata.level = 1;
    }
    else {
        monsterfightdata.level = playerfightdata.level - monsterfightdata.trail;
    }

    // Fight Loop
    'fight: while monsterfightdata.hp >= 0 && playerfightdata.hp >= 0 {
        match playerturn {

            // Player's Turn
            true => {

                // Status checker
                match playerfightdata.status {
                    Status::None => {

                    }
                    _ => {
                        eprintln!("E-001; Either this status doesn't exist (wrong naming, mangled) or you forgot to implement a status here. Status: {:?}", playerfightdata.status);
                        std::process::exit(1);
                    }
                }

                // Passive Ability checker
                match playerfightdata.passive {
                    Passive::None => {

                    }
                    _ => {
                        eprintln!("E-001; Either this passive ability doesn't exist (wrong naming, mangled) or you forgot to implement one here. Status: {:?}", playerfightdata.passive);
                        std::process::exit(1);
                    }
                }

                // Action Loop
                loop {
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

                            let attacksforplayer = [playerfightdata.attack_formula, playerfightdata.attack_formula_2, playerfightdata.attack_formula_3, playerfightdata.attack_formula_4];

                            match idx {
                                1 | 2 | 3 | 4 => {
                                    let dmg = (attacksforplayer[idx as usize - 1])(&playerfightdata);
                                    println!("{} did {} damage to {}!", playerfightdata.name, dmg.0, monsterfightdata.name);
                                    monsterfightdata.hp -= dmg.0;
                                    println!("{} has {} HP left!", monsterfightdata.name, monsterfightdata.hp);
                                    if dmg.1 != Status::None {
                                        monsterfightdata.status = dmg.1;
                                        println!("{} is afflicted with {:?}!", monsterfightdata.name, dmg.1);
                                    }
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
                                break 'fight;
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

                        "use" | "u" => {
                            if playerfightdata.items.len() > 0 {
                                println!("What item would you like to use?");
                                let mut bagidx = 1;
                                for item in &playerfightdata.items {
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

                                else if (bagitemidx as usize - 1) >= playerfightdata.items.len() {
                                    println!("This item doesn't exist!");
                                    continue;
                                }

                                else {
                                    let itemused = playerfightdata.items.remove(bagitemidx as usize - 1);
                                    match itemused {
                                        _ => {
                                            println!("{:?} was used.", itemused);
                                        }
                                    }
                                }
                            }

                            else {
                                println!("No items in bag!");
                                continue;
                            }

                            playerturn = false;
                        }
                        _ => {
                            println!("Invalid action!");
                            continue;
                        }
                    }

                    if playerfightdata.hp > playerfightdata.maxhp {
                        playerfightdata.hp = playerfightdata.maxhp;
                    }

                    if playerturn == false {
                        break;
                    }
                }
            }

            // Monster's turn
            false => {

                match monsterfightdata.status {
                    Status::None => {

                    }
                    _ => {
                        eprintln!("E-001; Either this status doesn't exist (wrong naming, mangled) or you forgot to implement a status here. Status: {:?}", monsterfightdata.status);
                        std::process::exit(1);
                    }
                }

                match monsterfightdata.passive {
                    Passive::None => {

                    }
                    _ => {
                        eprintln!("E-001; Either this passive ability doesn't exist (wrong naming, mangled) or you forgot to implement one here. Status: {:?}", monsterfightdata.passive);
                        std::process::exit(1);
                    }
                }

                loop {
                    println!("Monster's turn.");
                    let attackindex = (monsterfightdata.hp % 4).abs();
                    // Choose an attack based on monster health; this makes it seem somewhat random since different attacks would do different damage to the monster
                    let attacksformonster = [monsterfightdata.attack_formula, monsterfightdata.attack_formula_2, monsterfightdata.attack_formula_3, monsterfightdata.attack_formula_4];

                    match attackindex {
                        0 | 1 | 2 | 3 => {
                            let dmg = (attacksformonster[attackindex as usize])(&monsterfightdata);
                            println!("{} did {} damage to {}!", monsterfightdata.name, dmg.0, playerfightdata.name);
                            playerfightdata.hp -= dmg.0;
                            println!("{} has {} HP left!", playerfightdata.name, playerfightdata.hp);
                            if dmg.1 != Status::None {
                                playerfightdata.status = dmg.1;
                                println!("{} is afflicted with {:?}!", playerfightdata.name, dmg.1);
                            }
                            playerturn = true;
                        }

                        // This branch should not happen, which is why it errors and exits
                        _ => {
                            eprintln!("E-002; Invalid attack index; mod 4 returned 4 or higher");
                            std::process::exit(1);
                        }
                    }

                    if playerturn == true {
                        turncounter += 1;
                        break;
                    }
                }
            }
        }
    }

    let mut win = false;

    if monsterfightdata.hp < 0 {
        win = true;
        playerfightdata.xp += monsterfightdata.xp;
        playerfightdata.money += monsterfightdata.money;
        println!("You win! Gain {} xp and {} gold.", monsterfightdata.xp, monsterfightdata.money);
        if monsterfightdata.items.len() > 0 {
            for item in monsterfightdata.items {
                playerfightdata.items.push(item);
                println!("Acquired {:?}!", item);
            }
            for item in monsterfightdata.keys {
                playerfightdata.keys.push(item);
                println!("Acquired key item {:?}!", item);
            }
        }
    }

    playerfightdata.strength -= strmod;
    playerfightdata.dexterity -= dexmod;
    playerfightdata.constitution -= conmod;
    playerfightdata.intelligence -= intmod;
    playerfightdata.wisdom -= wismod;
    playerfightdata.charisma -= chamod;

    if playerfightdata.status != Status::HatCurse {
        playerfightdata.status = Status::None;
    }

    return (playerfightdata, win);
}