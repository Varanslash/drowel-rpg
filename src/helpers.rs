use std::time::SystemTime;
use std::time::UNIX_EPOCH;

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

pub fn dice(x: u64) -> u64 {
    return ((SystemTime::now().duration_since(UNIX_EPOCH).expect("Time went backwards").as_nanos() as u64) % x) + 1;
}