pub type Attack = fn(Player, Monster, bool) -> (i32, Status, Player, Monster);

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
    pub mtype: Type,
    pub attack_formula: Attack,
    pub attack_formula_2: Attack,
    pub attack_formula_3: Attack,
    pub attack_formula_4: Attack,
    pub xp: u64,
    pub items: Vec<Item>,
    pub keys: Vec<Item>,
    pub status: Status,
    pub healtimings: Vec<usize>,
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
    pub ptype: Type,
    pub attack_formula: Attack,
    pub attack_formula_2: Attack,
    pub attack_formula_3: Attack,
    pub attack_formula_4: Attack,
    pub xp: u64,
    pub items: Vec<Item>,
    pub keys: Vec<Item>,
    pub status: Status,
    pub healtimings: Vec<usize>,
    pub passive: Passive,
    pub money: u64,
    pub tempmod: Vec<i32>
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
    pub keys: Vec<Item>,
    pub money: u64,
    pub shop: Vec<(Item, u64)>
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum Type {
    Normal,
    Zombie,
    Dragon,
    Dinosaur,
    Bird,
    Fire,
    Earth,
    Air,
    Water,
    Plant,
    Light,
    Dark,
    Psychic,
    Fairy,
    Divine,
    Thessal,
    Ghost,
    Metal,
    Temporal,
    Magic,
    Holy,
    Unholy,
    Toxic,
    Chaotic,
    Uncanny
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
pub enum MonsterList {
    // Low Level Monsters
    Goblin,
    Orc,
    Zombie,
    Skeleton,
    Imp,
    BabyDragon,
    Elf,
    Kirin,
    GiantRat,
    Harpy,
    Kobold,
    Wisp,
    Cyclops,
    Slime,
    Wolf,
    Assailant,
    MushroomMan,
    Raptor,
    Sprite,
    Leprechaun,
    Ghost,
    ZoneEater,
    Leech,
    Watchman,
    Dreamling,
    Crab,
    GiantCrab,
    Kappa,
    Jellyfish,
    IceKirin,
    Dire,
    Thessalwolf,
    Deepwolf,
    DeepDragon,
    DireGoblin,
    Necrotica,

    // Mid Level Monsters
    Dragon,
    Knight,
    Withered,
    Perished,
    WillOWisp,
    Soldier,
    Mann,
    Sentry,
    Priest,
    Vampire,
    Siren,
    Gorgon,
    Megalodon,
    Manticore,
    Gargoyle,
    Apostle,
    Breaker,
    MagicalWarrior,
    Hawk,
    Banshee,
    RoseDragon,
    YamiKing,
    Troll,
    Ogre,
    Grue,
    Minotaur,
    Sphinx,
    Golem,
    Poltergeist,
    Hydra,
    Thessalhydra,
    Thessalknight,

    // High Level Monsters
    AncientDragon,
    Truthseeker,
    Radiance,
    Quantum,
    GoldDragon,
    Necroknight,
    Actinium,
    Druid,
    Wizard,
    Mage,
    Sorcerer,
    Bishop,
    GoldGeneral,
    Devil,
    Rook,
    BauthicElf,
    BauthicDragon,
    Basilisk,
    Deltaknight,
    Omegaknight,
    DeltaDragon,
    Gammaknight,
    Sigma,
    Epsilon,
    Kyoutakana,
    Thief,
    WeddingDragon,
    Thessaldragon,
    Yatagarasu,
    EntityOfInfluence,

    // Bosses
    Gilgamesh,
    Leviathan,
    Judas,
    Belphegor,
    Death,
    Gaiden,
    Yahweh,
    Azathoth,
    Warlock,
    Hat,
    Sytharax,
    Balgaroth,
    Lolth,
    Sebek,
    Repeat,
    Dream,
    Cyclone,
    Odia,
    Red,
    Alkali,
    Avarice,
    Atrifus,
    Consupi,
    Haught,
    Talos,
    Naon,
    Mindflower,
    Arika,
    Ulfilas,

    // Random thing
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
    Druid,
    Knight,
    Rogue,
    Amazoness,
    Psychic,
    Warlock
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Area {
    Camp,
    Forest,
    Mountains,
    TerDeep,
    AquDeep,
    AirDeep,
    HatKingdom,
    HatCastle,
    Universe,
    GodKingdom,
    YahwehCastle,
    GodVillage,
    CelesteCave,
    Desert,
    Cathedral,
    Deepworld,
    Avernus,
    Aeternus,
    DragonCave,
    Sardor,
    UrSardor,
    SardorCastle,
    DrosselCastle,
    Drossel,
    Mindscape,
    MountainCave,
    Celeste,
    Path
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Item {
    // Normal Items (for temp healing or the like)
    FullPotion,
    HalfPotion,
    QuarterPotion,
    FullInitPotion,
    HalfInitPotion,
    QuarterInitPotion,
    Steroids,
    Modafinil,
    CoinBag,
    CoinChest,
    CoinCrate,
    MiraclePotion,
    SavePotion,
    Wish,
    GenieGift,
    Stick,
    StrEnhance,
    DexEnhance,
    ConEnhance,
    IntEnhance,
    WisEnhance,
    ChaEnhance,
    FullHeal,
    MagicHeal,
    Weakening,
    Deliriant,
    StrDebuff,
    DexDebuff,
    ConDebuff,
    IntDebuff,
    WisDebuff,
    ChaDebuff,
    Poisoning,
    CursedNail,
    Drain,
    WitheredRose,
    PerishedRose,
    Bomb,
    SmokeBomb,
    Clock,
    FriedChicken,

    // Items (for overworld)
    PurpleWarpStone,
    BlueWarpStone,
    Attract,
    Repel,
    HellStone,
    HeavenStone,

    // Spell Items (for battle)
    Disintegrator,
    Fireball,
    GreenBullet,
    Destiny,
    Raigeki,
    FrozenLight,
    LightDagger,
    Charge,
    Mutualism,
    Torrent,
    Parasitism,
    Pitch,
    BishopBlessing,
    WarlockCurse,
    Idol,
    Extraneous,
    IceBeam,
    Blizzard,
    Frigid,
    GothicSpell,
    Phoenix,
    AzathothCurse,
    Distract,
    PetalFall,
    Lotus,
    Nightshade,
    Blooming,
    Vatican,
    Wildfire,
    TidalWave,
    Fry,
    Exponential,
    MindFlay,
    Estrogen,
    Slowing,
    Toxic,
    Lobotomy,
    Inducing,
    Mute,
    Schizotype,

    // Upgrade Items
    RareCandy,
    DeficitCandy,
    StrCandy,
    DexCandy,
    ConCandy,
    IntCandy,
    WisCandy,
    ChaCandy,

    // Key Items
    GothicBible,
    HorseEye,
    LeviScale,
    Cane,
    BelphiHorn,
    BrokenBlade,
    HighCrown,
    UniverseKey,
    AzaPiece,
    WarPiece,
    Hat,
    ItemOfPro,
    Webgamer,
    ItemOfWard,
    ItemOfIdem,
    BlueGem,
    GreenGem,
    RedGem,
    GreenKey,
    RedKey,
    OrangeKey,
    YellowKey,
    BlueKey,
    IndigoKey,
    VioletKey,
    Glasses,
    Tailfin,
    Psycho,
    LowCrown
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Status {
    HatCurse, // Blocked by Miracle
    Poison, // Blocked by Infectious and Save
    Curse, // Blocked by Miracle
    StrDrain, // Blocked by Rage and Draconian
    DexDrain, // Blocked by Initiative and Draconian
    ConDrain, // Blocked by Save
    IntDrain, // Blocked by Save
    WisDrain, // Blocked by Stoic
    ChaDrain, // Blocked by Miracle
    MagicDrain, // Unblockable
    Perishing, // Blocked by Miracle
    DestinyBond, // Blocked by Divinity
    Wither, // Blocked by Save and Mending
    Frozen, // Blocked by Draconian and Regular
    Distracted, // Blocked by Stoic
    Burned, // Unblockable
    Paralyzed, // Unblockable
    Schizophrenic, // Blocked by Stoic and Miracle
    Confused, // Blocked by Stoic
    Parasite, // Blocked by Infectious
    None
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum Passive {
    Mending, // heals CON/2 every turn
    Sturdy, // nothing, pure blocking; also against certain attack, has resistances
    Extraspeed, // increases dexterity by 1 every other turn
    Extrapower, // same as extraspeed but for strength
    Save, // just sturdy, but for physical deterioration
    Regular, // sturdy, but for magical attacks
    Miracle, // save, but magical
    Greed, // multiplies money gain by 1.5 after every fight
    Infectious, // protects against viruslike attacks and statuses, and has a 20% chance to infect opponent with Parasite every turn
    Stoic, // blocks mental stat debuffs and has resistances against certain statuses
    Bipolar, // 5% chance to give opponent schizo
    Thorns, // chip damage to opponent when hit
    SavingThrow, // 25% chance to recover some hp when hit
    Draconian, // helps dragon types and resists other dragon attacks
    Rage, // 25% chance to buff str by 2 every turn
    Palladium, // 10% chance to induce wither every turn
    Initiative, // 10% chance to get another turn
    MultiattackII, // get 2 turns per turn, uses Channel 8
    MultiattackIII, // get 2 turns per turn, uses Channel 8
    Divinity, // For those without Divine, it turns you to a Divine type if triggered in exchange for 25% of your HP. If you are Divine and have this ability, you will do 1.2x damage every time you attack.
    None
}