#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monster {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub level: u8,
    pub name: String,
    pub hp: i32,
    pub attack_formula: fn(&Monster) -> i32,
    pub attack_formula_2: fn(&Monster) -> i32,
    pub attack_formula_3: fn(&Monster) -> i32,
    pub attack_formula_4: fn(&Monster) -> i32,
    pub xp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub strength: u8,
    pub dexterity: u8,
    pub constitution: u8,
    pub intelligence: u8,
    pub wisdom: u8,
    pub charisma: u8,
    pub level: u8,
    pub name: String,
    pub hp: i32,
    pub class: Class,
    pub attack_formula: fn(&Player) -> i32,
    pub attack_formula_2: fn(&Player) -> i32,
    pub attack_formula_3: fn(&Player) -> i32,
    pub attack_formula_4: fn(&Player) -> i32,
    pub xp: u64,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum MonsterList {
    Goblin,
    Orc,
    Dragon,
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Class {
    Fighter,
    Wizard,
    Bishop,
    Paladin,
    Barbarian,
    Bard, 
    Druid
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Room {
    pub description: String,
    pub north: u32, // Each of these four values are room indexes for the game to read and travel to noneuclideanly
    pub south: u32,
    pub east: u32,
    pub west: u32,
    pub monster: MonsterList,
    pub encval: u32 // Every room has a set value in which encounters are triggered; 0 is guaranteed encounter, and a sufficiently high number is no encounter.
}