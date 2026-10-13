use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use crate::encounter;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;
use encounter::*;

pub fn init() -> (HashMap<u32, Room>, HashMap<MonsterList, Monster>, HashMap<Area, String>) {
    let mut rooms = HashMap::from([
        (1, Room {
            description: "You are in a dark forest. [NS]".to_string(),
            description2: "You are in a dark forest. [NS]".to_string(),
            description3: "You are in a dark forest. [NS]".to_string(),
            north: 2,
            south: 3,
            east: 0,
            west: 0,
            monster: MonsterList::Goblin,
            area: Area::Forest,
            encval: 20
        }),
        (2, Room {
            description: "You are on a mountain trail. [S]".to_string(),
            description2: "You are on a mountain trail. [S]".to_string(),
            description3: "You are on a mountain trail. [S]".to_string(),
            north: 0,
            south: 1,
            east: 0,
            west: 0,
            monster: MonsterList::Orc,
            area: Area::Forest,
            encval: 20
        }),
        (3, Room {
            description: "You are in a cave. [N]".to_string(),
            description2: "You are in a cave. [N]".to_string(),
            description3: "You are in a cave. [N]".to_string(),
            north: 1,
            south: 0,
            east: 0,
            west: 0,
            monster: MonsterList::Goblin,
            area: Area::Forest,
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
            trail: 3,
            name: "Goblin".to_string(),
            hp: 15,
            attack_formula: mon_nanattack,
            attack_formula_2: mon_nanattack,
            attack_formula_3: mon_nanattack,
            attack_formula_4: mon_nanattack,
            xp: 10,
            items: Vec::new(),
            status: Status::None,
            passive: Passive::None,
            money: 5
        }),
        (MonsterList::Orc, Monster {
            strength: 16,
            dexterity: 10,
            constitution: 14,
            intelligence: 6,
            wisdom: 8,
            charisma: 6,
            level: 1,
            trail: 2,
            name: "Orc".to_string(),
            hp: 30,
            attack_formula: mon_nanattack,
            attack_formula_2: mon_nanattack,
            attack_formula_3: mon_nanattack,
            attack_formula_4: mon_nanattack,
            xp: 20,
            items: Vec::new(),
            status: Status::None,
            passive: Passive::None,
            money: 10
        }),
    ]);

    let maps: HashMap<Area, String> = HashMap::from([
        (Area::Forest,
        "
[2 ]
[1 ]
[3 ]".to_string())
    ]);

    return (rooms, monsterdata, maps);
}