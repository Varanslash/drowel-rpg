#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Monster {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub level: i32,
    pub trail: i32,
    pub name: String,
    pub hp: i32,
    pub attack_formula: fn(&Monster) -> (i32, Status),
    pub attack_formula_2: fn(&Monster) -> (i32, Status),
    pub attack_formula_3: fn(&Monster) -> (i32, Status),
    pub attack_formula_4: fn(&Monster) -> (i32, Status),
    pub xp: u64,
    pub items: Vec<Item>,
    pub status: Status,
    pub passive: Passive,
    pub initiative: bool,
    pub money: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Player {
    pub strength: i32,
    pub dexterity: i32,
    pub constitution: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub charisma: i32,
    pub level: i32,
    pub name: String,
    pub hp: i32,
    pub maxhp: i32,
    pub class: Class,
    pub attack_formula: fn(&Player) -> (i32, Status),
    pub attack_formula_2: fn(&Player) -> (i32, Status),
    pub attack_formula_3: fn(&Player) -> (i32, Status),
    pub attack_formula_4: fn(&Player) -> (i32, Status),
    pub xp: u64,
    pub items: Vec<Item>,
    pub status: Status,
    pub passive: Passive,
    pub money: u64
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Room {
    pub description: String,
    pub description2: String,
    pub description3: String,
    pub north: u32, // Each of these four values are room indexes for the game to read and travel to noneuclideanly
    pub south: u32,
    pub east: u32,
    pub west: u32,
    pub monster: MonsterList,
    pub area: Area,
    pub encval: u32, // Every room has a set value in which encounters are triggered; 0 is guaranteed encounter, and a sufficiently high number is no encounter.
    pub npc: String,
    pub items: Vec<Item>,
    pub money: u64,
    pub shop: Vec<(Item, u64)>
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Area {
    Forest,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Item {
    FullPotion,
    HalfPotion,
    Stick,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Status {
    HatCurse,
    Poison,
    StrDrain,
    DexDrain,
    ConDrain,
    IntDrain,
    WisDrain,
    ChaDrain,
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Passive {
    Mending,
    None
}