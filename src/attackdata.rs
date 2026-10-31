use crate::gamedata;
use crate::helpers;
use gamedata::*;
use helpers::*;

pub fn nanattack(stat: &Player, mstat: &Monster, caster: bool) -> (i32, Status) {
    return (1, Status::None);
}

pub fn disintegrate(stat: &Player, mstat: &Monster, caster: bool) -> (i32, Status) {
    if stat.passive != Passive::Sturdy {
        if stat.keys.contains(&Item::ItemOfPro) || stat.keys.contains(&Item::ItemOfWard) || stat.keys.contains(&Item::ItemOfIdem) {
            return (10, Status::Wither);
        }
        else {
            return (3900000, Status::None);
        }
    }
    else {
        return (10, Status::Wither);
    }
}

pub fn extremespeed(stat: &Player, mstat: &Monster, caster: bool) -> (i32, Status) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;

    if caster == true {
        if helpers::dice(50, salt) % 2 == 0 {
            salt = helpers::salt(salt, ogsalt);
            return ((((stat.dexterity / 2) + (stat.level * 2)) + (helpers::dice(3, salt) as i32)) as i32, Status::None);
        }
        else {
            salt = helpers::salt(salt, ogsalt);
            return ((((stat.dexterity / 2) + (stat.level * 2)) - (helpers::dice(3, salt) as i32)) as i32, Status::None);
        }
        salt = helpers::salt(salt, ogsalt);
    }

    else {
        if helpers::dice(50, salt) % 2 == 0 {
            salt = helpers::salt(salt, ogsalt);
            return ((((mstat.dexterity / 2) + (mstat.level * 2)) + (helpers::dice(3, salt) as i32)) as i32, Status::None);
        }
        else {
            salt = helpers::salt(salt, ogsalt);
            return ((((mstat.dexterity / 2) + (mstat.level * 2)) - (helpers::dice(3, salt) as i32)) as i32, Status::None);
        }
    }
}

pub fn bodyslam(stat: &Player, mstat: &Monster, caster: bool) -> (i32, Status) {
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
            helpers::dice(2, salt) as i32
        }
        _ => {
            -(helpers::dice(2, salt) as i32)
        }
    };


    salt = helpers::salt(salt, ogsalt);

    let statret = match helpers::dice(50, salt) < 11 {
        true => {
            Status::Paralyzed
        }
        _ => Status::None
    };

    
    salt = helpers::salt(salt, ogsalt);

    return ((strength + (modifier as i32)), statret)
}

pub fn dragonbreath(stat: &Player, mstat: &Monster, caster: bool) -> (i32, Status) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;

    let level = match caster {
        true => {
            stat.level * 2
        }
        _ => {
            mstat.level * 2
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt);
            helpers::dice(10, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt);
            -(helpers::dice(10, salt) as i32)
        }
    };

    salt = helpers::salt(salt, ogsalt);

    let status = match helpers::dice(100, salt) {
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

    return (level + modifier, status)
}