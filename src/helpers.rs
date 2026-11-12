use std::time::SystemTime;
use std::time::UNIX_EPOCH;
use crate::gamedata::*;

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

pub fn dice(x: u64, s: u64) -> u64 {
    let numberthing = (SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos() as u64).wrapping_mul(s);

    let mut z = numberthing.wrapping_mul(s);
    z ^= z >> 30;
    z = z.wrapping_mul(0xbf58476d1ce4e5b9);
    z ^= z >> 27;
    z = z.wrapping_mul(0x94d049bb133111eb);
    z ^= z >> 31;

    return (z % x) + 1;
}

pub fn salt(s: u64, ogs: u64, rounds: u32) -> u64 {
    let mut z = s.wrapping_mul(ogs);
    
    for _ in 0..rounds {
        z = z.wrapping_add(0x9e3779b97f4a7c15);
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d049bb133111eb);
        z ^= z >> 31;
    }
    
    return z;
}

pub fn typepasfetch(stat: &Player, mstat: &Monster, caster: bool) -> (Type, Passive) {
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

    return (optype, oppassive)
}