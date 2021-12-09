// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]
fn main() {
    let dead_player = Player {
        health: 0,
        mana: Some(0),
        level: 34,
    };
    let revived_player = dead_player
        .revive()
        .expect("reviving a dead player must return Some(player)");
}

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        return if self.health > 0 {
            None
        } else {
            if self.level >= 10 {
                return Some(Player { health: 100, mana: Some(100), level: self.level });
            }
            Some(Player { health: 100, mana: None, level: self.level })
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.mana == None {
            if mana_cost > self.health {
                self.health = 0;
                println!("{}", self.health);
            } else {
                self.health -= mana_cost;
            }
            return 0;
        } else if self.mana.unwrap() < mana_cost {
            0
        } else {
            self.mana = Some(self.mana.unwrap() - mana_cost);
            mana_cost * 2
        }
    }
}
