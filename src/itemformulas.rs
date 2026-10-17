use crate::gamedata;
use crate::playerattackdata;
use crate::monsterattackdata;
use crate::encounter;
use std::collections::HashMap;
use gamedata::*;
use playerattackdata::*;
use monsterattackdata::*;
use encounter::*;

fn nanitem(player: Player) -> Player {
    return player;
}