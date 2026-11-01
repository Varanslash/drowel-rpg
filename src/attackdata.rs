use crate::gamedata;
use crate::helpers;
use gamedata::*;
use helpers::*;

pub fn nanattack(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    return (1, Status::None, stat, mstat);
}

pub fn disintegrate(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    if caster {
        if mstat.passive != Passive::Sturdy {
            if mstat.keys.contains(&Item::ItemOfPro) || mstat.keys.contains(&Item::ItemOfWard) || mstat.keys.contains(&Item::ItemOfIdem) || mstat.mtype == Type::Divine || mstat.mtype == Type::Uncanny {
                return (10, Status::Wither, stat, mstat);
            }
            else {
                return (3900000, Status::None, stat, mstat);
            }
        }
        else {
            return (10, Status::Wither, stat, mstat);
        }
    }

    else {
        if stat.passive != Passive::Sturdy {
            if stat.keys.contains(&Item::ItemOfPro) || stat.keys.contains(&Item::ItemOfWard) || stat.keys.contains(&Item::ItemOfIdem) || stat.ptype == Type::Divine || stat.ptype == Type::Uncanny {
                return (10, Status::Wither, stat, mstat);
            }
            else {
                return (3900000, Status::None, stat, mstat);
            }
        }
        else {
            return (10, Status::Wither, stat, mstat);
        }
    }
}

pub fn extremespeed(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /*
        + Extremespeed - Rogue L1
        - types: weak Zombie, Bird, Psychic, Fairy | resist Dragon, Earth, Metal | immune Ghost
        - passives: none
        - monsters: weak Skeleton, Harpy, Sprite, Dreamling, Withered, Hawk | resist Dragon, Gargoyle, Golem, GoldDragon, Rook, Talos | immune Ghost, WillOWisp, Banshee, Poltergeist 
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Bird, Type::Psychic, Type::Fairy];
    let resist = [Type::Dragon, Type::Earth, Type::Metal];
    let immune = [Type::Ghost];

    let casdex = match caster {
        true => {
            stat.dexterity
        }
        _ => {
            mstat.dexterity
        }
    };

    let caslvl = match caster {
        true => {
            stat.level
        }
        _ => {
            mstat.level
        }
    };

    let optype = match !caster {
        true => {
            stat.ptype
        }
        _ => {
            mstat.mtype
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 1);
            helpers::dice(3, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(3, salt) as i32)
        }
    };

    let mut damage = (casdex / 2) + (caslvl * 2) + modifier;

    if weakness.contains(&optype) {
        damage *= 2
    }

    if resist.contains(&optype) {
        damage = damage / 2
    }

    if immune.contains(&optype) {
        damage = 0
    }

    return (damage, Status::None, stat, mstat);
}

pub fn bodyslam(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;

    let strength = match caster {
        true => {
            stat.strength
        }
        _ => {
            mstat.strength
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 1);
            helpers::dice(2, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(2, salt) as i32)
        }
    };


    salt = helpers::salt(salt, ogsalt, 1);

    let statret = match helpers::dice(50, salt) < 11 {
        true => {
            Status::Paralyzed
        }
        _ => Status::None
    };

    
    salt = helpers::salt(salt, ogsalt, 1);

    return ((strength + (modifier as i32)), statret, stat, mstat)
}

pub fn dragonbreath(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /*
        ~ Dragonbreath
        - types: weak Dragon | resist Metal | immune Fairy
        - passives: resist Draconian
        - monsters: weak BabyDragon, DeepDragon, Dragon, RoseDragon, AncientDragon, GoldDragon, BauthicDragon, DeltaDragon, WeddingDragon, Thessaldragon | resist Golem, GoldGeneral, Rook, Talos | immune Sprite, Leprechaun
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Dragon];
    let resist = [Type::Metal];
    let immune = [Type::Fairy];
    let presist = [Passive::Draconian];

    let level = match caster {
        true => {
            stat.level * 2
        }
        _ => {
            mstat.level * 2
        }
    };

    let optype = match !caster {
        true => {
            stat.ptype
        }
        _ => {
            mstat.mtype
        }
    };

    let oppassive = match !caster {
        true => {
            stat.passive
        }
        _ => {
            mstat.passive
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 1);
            helpers::dice(10, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(10, salt) as i32)
        }
    };

    salt = helpers::salt(salt, ogsalt, 1);

    let mut status = match helpers::dice(100, salt) {
        1..=5 => {
            Status::Frozen
        }
        6..=10 => {
            Status::Burned
        }
        11..=15 => {
            Status::Paralyzed
        }
        _ => {
            Status::None
        }
    };

    let mut damage = level + modifier;

    if weakness.contains(&optype) {
        damage *= 2;
        status = Status::Frozen
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage = damage / 2;
        status = Status::None
    }

    if immune.contains(&optype) {
        damage = 0;
        status = Status::None
    }

    return (damage, status, stat, mstat)
}

pub fn frost(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /*
        ~ Frost
        - types: weak Dragon, Dinosaur, Bird, Earth, Plant | resist Fire, Water, Metal
        - passives: resist Regular
        - monsters: weak BabyDragon, Harpy, Raptor, Dragon, Gargoyle, Hawk, RoseDragon | resist Imp, Slime, WillOWisp, Rook | immune IceKirin
    */

    let optype = match !caster {
        true => {
            stat.ptype
        }
        _ => {
            mstat.mtype
        }
    };

    let oppassive = match !caster {
        true => {
            stat.passive
        }
        _ => {
            mstat.passive
        }
    };

    let mut damage = 0;
    let mut status = Status::None;
    
    let weakness = [Type::Dragon, Type::Dinosaur, Type::Bird, Type::Earth, Type::Plant];
    let resist = [Type::Metal, Type::Fire, Type::Water];
    let presist = [Passive::Regular];

    if weakness.contains(&optype) {
        damage = 20;
        status = Status::Frozen
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        match helpers::dice(50, helpers::salt(11400714819323198485, 11400714819323198485, 1)) % 2 {
            0 => {
                status = Status::Frozen
            }
            _ => {}
        }
    }

    return (damage, status, stat, mstat)
}

pub fn influence(stat: Player, mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    salt = helpers::salt(salt, ogsalt, 2);

    let dmg = match caster {
        true => {
            (stat.intelligence * 2)
        }
        _ => {
            (mstat.intelligence * 2)
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 1);
            helpers::dice(4, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(4, salt) as i32)
        }
    };

    salt = helpers::salt(salt, ogsalt, 1);

    match helpers::dice(4, salt) {
        4 => {
            return (0, Status::Schizophrenic, stat, mstat)
        }
        _ => {
            return (dmg+modifier as i32, Status::None, stat, mstat)
        }
    }
}

pub fn reckless(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    salt = helpers::salt(salt, ogsalt, 1);

    let dmg = match caster {
        true => {
            stat.strength
        }
        _ => {
            mstat.strength
        }
    };

    let dmg2 = match caster {
        true => {
            stat.dexterity
        }
        _ => {
            mstat.dexterity
        }
    };

    let mut damage = helpers::dice(((dmg+dmg2) as u64), salt);

    salt = helpers::salt(salt, ogsalt, 1);

    match helpers::dice(50, salt) % 2 {
        0 => {

        }

        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            let recoil = (helpers::dice(26, salt) as i32);
            if caster {
                stat.hp -= recoil
            }
            else {
                mstat.hp -= recoil
            }
            println!("Took {} damage as recoil!", recoil);
            damage *= 2
        }
    }

    if caster {
        if mstat.mtype == Type::Ghost {
            damage = 0
        }
        else if mstat.mtype == Type::Dragon || mstat.mtype == Type::Metal {
            damage = damage / 2
        }
    }
    else {
        if stat.ptype == Type::Ghost {
            damage = 0
        }
        else if stat.ptype == Type::Dragon || stat.ptype == Type::Metal {
            damage = damage / 2
        }
    }

    return ((damage/2) as i32, Status::None, stat, mstat)
}