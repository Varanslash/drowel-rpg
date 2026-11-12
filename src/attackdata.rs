use crate::gamedata;
use crate::helpers;
use gamedata::*;
use helpers::*;

/*
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Dark, Type::Ghost, Type::Unholy, Type::Chaotic, Type::Uncanny];
    let resist = [Type::Light, Type::Divine, Type::Holy];
    let immune = [];
    let pweakness = [];
    let presist = [Passive::Regular];
    let pimmune = [Passive::Sturdy];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

    let dmg = match caster {
        true => {
            (stat.intelligence * 2)
        }
        _ => {
            (mstat.intelligence * 2)
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
            helpers::dice(4, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(4, salt) as i32)
        }
    };

    let mut status = Status::Schizophrenic;
    let mut nstatus = Status::None;
    let mut damage = dmg+modifier;
    let mut sdamage = 0;

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {

    }

    if resist.contains(&optype) || presist.contains(&oppassive) {

    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {

    }

    (mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster)

    kristine

    these will all be very helpful for you
*/

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

    /* 
        ~ Body Slam
        - types: weak Zombie, Bird, Psychic, Fairy | resist Dragon, Earth, Metal | immune Ghost
        - passives: resist Sturdy
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Bird, Type::Psychic, Type::Fairy];
    let resist = [Type::Dragon, Type::Earth, Type::Metal];
    let immune = [Type::Ghost];
    let pweakness = [];
    let presist = [Passive::Sturdy];
    let pimmune = [];

    let strength = match caster {
        true => {
            stat.strength
        }
        _ => {
            mstat.strength
        }
    };

    let level = match caster {
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
            helpers::dice(2, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(2, salt) as i32)
        }
    };


    salt = helpers::salt(salt, ogsalt, 1);

    let mut statret = match helpers::dice(50, salt) < 11 {
        true => {
            Status::Paralyzed
        }
        _ => Status::None
    };

    let mut damage = (strength + (modifier as i32)) + (level);
    
    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        damage *= 2;
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage = damage / 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0;
        statret = Status::None
    }

    return (damage, statret, stat, mstat)
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
    /* 
        ~ Influence
        - types: weak Normal, Dinosaur, Toxic | resist Dark, Psychic, Metal, Uncanny | immune Zombie
        - passives: weak Bipolar | resist Regular, Stoic
        - monsters: weak Goblin, Orc, Cyclops, Wolf, Raptor, Troll, Ogre, Minotaur | resist Dreamling, Sphinx, Rook, EntityOfInfluence, Dream, Mindflower | immune Zombie, Skeleton, Withered, Perished, Golem
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    salt = helpers::salt(salt, ogsalt, 2);
    let weakness = [Type::Normal, Type::Dinosaur, Type::Toxic];
    let resist = [Type::Dark, Type::Psychic, Type::Metal, Type::Uncanny];
    let immune = [Type::Zombie];
    let pweakness = [Passive::Bipolar];
    let presist = [Passive::Regular, Passive::Stoic];
    let pimmune = [];

    let dmg = match caster {
        true => {
            (stat.intelligence as f32 * 1.5) as i32
        }
        _ => {
            (mstat.intelligence as f32 * 1.5) as i32
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
            helpers::dice(4, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(4, salt) as i32)
        }
    };

    salt = helpers::salt(salt, ogsalt, 1);

    let mut status = Status::Schizophrenic;
    let mut nstatus = Status::None;
    let mut damage = dmg+modifier;
    let mut sdamage = 0;

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        match helpers::dice(50, salt) % 2 {
            0 => {
                salt = helpers::salt(salt, ogsalt, 1);
                sdamage = damage;
                damage *= 2;
            }
            _ => {
                salt = helpers::salt(salt, ogsalt, 1);
                nstatus = Status::Schizophrenic
            }
        }
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage = damage / 2;
        nstatus = Status::None
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0;
        sdamage = 0;
        status = Status::None;
        nstatus = Status::None;
    }

    salt = helpers::salt(salt, ogsalt, 1);

    match helpers::dice(4, salt) {
        4 => {
            return (sdamage as i32, status, stat, mstat)
        }
        _ => {
            return (damage as i32, nstatus, stat, mstat)
        }
    }
}

pub fn reckless(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Reckless
        - types: weak Zombie, Bird, Psychic, Fairy | resist Dragon, Earth, Metal | immune Ghost
        - passives: resist Sturdy
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    salt = helpers::salt(salt, ogsalt, 1);
    let weakness = [Type::Zombie, Type::Bird, Type::Psychic, Type::Fairy];
    let resist = [Type::Dragon, Type::Earth, Type::Metal];
    let immune = [Type::Ghost];
    let pweakness = [];
    let presist = [Passive::Sturdy];
    let pimmune = [];

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

    damage = damage / 2;

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        damage *= 2
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage /= 2
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0
    }

    return (damage as i32, Status::None, stat, mstat)
}

pub fn vaticano(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Vaticano - Bishop L20
        - types: weak Zombie, Dark, Ghost, Unholy, Chaotic, Uncanny | resist Light, Divine, Holy
        - passives: resist Regular | immune Sturdy
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Dark, Type::Ghost, Type::Unholy, Type::Chaotic, Type::Uncanny];
    let resist = [Type::Light, Type::Divine, Type::Holy];
    let immune = [];
    let pweakness = [];
    let presist = [Passive::Regular];
    let pimmune = [Passive::Sturdy];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

    let mut dmg = match caster {
        true => {
            (stat.intelligence * 2)
        }
        _ => {
            (mstat.intelligence * 2)
        }
    };

    let mut status = Status::None;

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        status = Status::Perishing;
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        dmg /= 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        dmg = 0;
    }

    return (dmg, status, stat, mstat)
}

pub fn ancientpower(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Ancient Power - Druid L20
        - types: weak Air, Water | resist Dragon, Plant, Metal, Toxic
        - passives: none
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Air, Type::Water];
    let resist = [Type::Dragon, Type::Plant, Type::Metal, Type::Toxic];
    let immune = [];
    let pweakness = [];
    let presist = [];
    let pimmune = [];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

    let mut damage = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 2);
            40 + helpers::dice(15, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 2);
            40 - helpers::dice(15, salt) as i32
        }
    };

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        damage *= 2;
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage /= 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0;
    }

    return (damage, Status::None, stat, mstat)
}

pub fn dragonspower(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Dragon's Power - Barbarian L20
        - types: weak Zombie, Bird, Psychic | resist Earth, Metal | immune Fairy, Ghost
        - passives: resist Sturdy, Draconian
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Bird, Type::Psychic];
    let resist = [Type::Earth, Type::Metal];
    let immune = [Type::Fairy, Type::Ghost];
    let pweakness: [Passive; 0] = [];
    let presist = [Passive::Draconian, Passive::Sturdy];
    let pimmune: [Passive; 0] = [];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

    let boulderthrow = helpers::dice(5, salt) as i32;

    for _ in 0..boulderthrow {
        println!("Threw boulder!")
    }

    salt = helpers::salt(salt, ogsalt, 1);
    let dmg = match caster {
        true => {
            stat.strength - 2
        }
        _ => {
            mstat.strength - 2
        }
    };

    let modifier = match helpers::dice(50, salt) % 2 {
        0 => {
            salt = helpers::salt(salt, ogsalt, 1);
            helpers::dice(5, salt) as i32
        }
        _ => {
            salt = helpers::salt(salt, ogsalt, 1);
            -(helpers::dice(5, salt) as i32)
        }
    };
    let damage1 = dmg + modifier;
    let mut damage = damage1 * boulderthrow;

    salt = helpers::salt(salt, ogsalt, 1);
    let mut status = match helpers::dice(100, salt) % 5 {
        0 => {
            Status::Paralyzed
        }
        _ => {
            Status::None
        }
    };

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        damage *= 2;
        status = Status::Paralyzed
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage /= 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0;
        status = Status::None
    }

    return (damage, status, stat, mstat)
}

pub fn psychic(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Psychic - Psychic L20
        - types: weak Normal, Dinosaur, Toxic | resist Dark, Psychic, Metal, Uncanny | immune Zombie
        - passives: weak Bipolar | resist Regular, Stoic    
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Normal, Type::Dinosaur, Type::Toxic];
    let resist = [Type::Dark, Type::Psychic, Type::Metal, Type::Uncanny];
    let immune = [Type::Zombie];
    let pweakness = [Passive::Bipolar];
    let presist = [Passive::Regular, Passive::Stoic];
    let pimmune = [];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

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
    let mut status = match helpers::dice(100, salt) % 20 {
        0..=1 => {
            Status::Confused
        }
        2 => {
            Status::Schizophrenic
        }
        _ => {
            Status::None
        }
    };

    let mut damage = dmg+modifier;

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        salt = helpers::salt(salt, ogsalt, 1);
        match helpers::dice(50, salt) % 2 {
            0 => {
                status = Status::Confused;
            }
            _ => {
                damage *= 2;
            }
        };
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        damage /= 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0;
        status = Status::None;
    }

    return (damage, status, stat, mstat)
}

pub fn ultimatum(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Dark, Type::Ghost, Type::Unholy, Type::Chaotic, Type::Uncanny];
    let resist = [Type::Light, Type::Divine, Type::Holy];
    let immune = [];
    let pweakness = [];
    let presist = [Passive::Regular];
    let pimmune = [Passive::Sturdy];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;


}

pub fn judgement(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /* 
        ~ Judgement - Paladin L20
        - types: weak Zombie, Dark, Ghost, Unholy, Chaotic, Uncanny | resist Light, Divine, Holy
        - passives: resist Regular
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Unholy, Type::Chaotic, Type::Uncanny];
    let mweakness = [Type::Zombie, Type::Dark, Type::Ghost];
    let resist = [Type::Light, Type::Divine, Type::Holy];
    let immune = [];
    let pweakness = [];
    let presist = [Passive::Regular];
    let pimmune = [Passive::Counter];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

    let mut damage = match caster {
        true => {
            stat.level * 2
        }
        _ => {
            mstat.level * 2
        }
    };

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        damage *= 3
    }

    if mweakness.contains(&optype) {
        damage *= 2
    }

    if presist.contains(&oppassive) || resist.contains(&optype) {
        damage /= 2
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        damage = 0
    }

    return (damage, Status::None, stat, mstat)
}

pub fn arcanoid(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Zombie, Type::Dark, Type::Ghost, Type::Unholy, Type::Chaotic, Type::Uncanny];
    let resist = [Type::Light, Type::Divine, Type::Holy];
    let immune = [];
    let pweakness = [];
    let presist = [Passive::Regular];
    let pimmune = [Passive::Sturdy];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;
}

pub fn greenbullet(mut stat: Player, mut mstat: Monster, caster: bool) -> (i32, Status, Player, Monster) {
    /*
        ~ Green Bullet - Rogue L20
        - types: weak Dragon, Dinosaur, Bird, Plant | resist Zombie, Earth, Metal | immune Ghost
        - passives: none 
    */

    let mut ogsalt = 11400714819323198485;
    let mut salt = 11400714819323198485;
    let weakness = [Type::Dragon, Type::Dinosaur, Type::Bird, Type::Plant];
    let resist = [Type::Zombie, Type::Earth, Type::Metal];
    let immune = [Type::Ghost];
    let pweakness = [];
    let presist = [];
    let pimmune = [];
    let plstatuses = [Status::Confused, Status::Schizophrenic, Status::Paralyzed];
    let op = helpers::typepasfetch(&stat, &mstat, caster);
    let optype = op.0;
    let oppassive = op.1;

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

    let mut dmg = match caster {
        true => {
            (stat.dexterity * 2) + modifier
        }
        _ => {
            (mstat.dexterity * 2) + modifier
        }
    };

    let plstatus = match caster {
        true => {
            stat.status
        }
        _ => {
            mstat.status
        }
    };

    let opstatus = match !caster {
        true => {
            stat.status
        }
        _ => {
            mstat.status
        }
    };

    if plstatuses.contains(&plstatus) {
        dmg /= 2;
    }

    if plstatuses.contains(&opstatus) {
        dmg *= 2;
    }

    salt = helpers::salt(salt, ogsalt, 1);
    match helpers::dice(50, salt) % 5 {
        0 => {
            match caster {
                true => {
                    mstat.money /= 2;
                    stat.money *= 2;
                }
                _ => {
                    stat.money /= 2;
                    mstat.money *= 2;
                }
            }
        }
        _ => {
            
        }
    }

    salt = helpers::salt(salt, ogsalt, 1);
    let status = match helpers::dice(50, salt) % 5 {
        0 => {
            match caster {
                true => {
                    if oppassive != Passive::Stoic {
                        Status::Confused
                    }
                    else {
                        Status::None
                    }
                }
                _ => {
                    if stat.passive != Passive::Stoic {
                        Status::Confused
                    }
                    else {
                        Status::None
                    }
                }
            }
        }
        _ => {
            Status::None
        }
    };

    if weakness.contains(&optype) || pweakness.contains(&oppassive) {
        dmg *= 2;
    }

    if resist.contains(&optype) || presist.contains(&oppassive) {
        dmg /= 2;
    }

    if immune.contains(&optype) || pimmune.contains(&oppassive) {
        dmg = 0;
    }

    return (dmg, status, stat, mstat)
}