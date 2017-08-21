use std;

extern crate rand;
use self::rand::distributions::{Range, IndependentSample};

#[derive(Debug)]
pub enum AttackResult {
    Wounded,
    Missed,
    Killed
}

impl std::fmt::Display for AttackResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        let output = match *self {
            AttackResult::Wounded => "hitted",
            AttackResult::Missed  => "missed",
            AttackResult::Killed  => "killed"
        };
        write!(f, "{}", output)
    }
}

pub struct Weapon {
    name: String,
    min_dmg: i32,
    max_dmg: i32,
}

use warrior::warrior::Warrior;

impl Weapon {
    pub fn hit(&self, target: &mut Warrior) -> AttackResult {

        let damage_done = target.apply_damage(self.get_rnd_damage());
        
        if !target.is_alive() {
            AttackResult::Killed
        } else if damage_done > 0 {
            AttackResult::Wounded
        } else {
            AttackResult::Missed
        }
    }
    pub fn new(name: &str, min:i32, max: i32) -> Weapon {
        Weapon {
            name: name.to_owned(),
            min_dmg: min,
            max_dmg: max,
        }
    }
    fn get_rnd_damage(&self)->i32 {
        let mut rng = rand::thread_rng();
        let range = Range::new(self.min_dmg, self.max_dmg+1);
        range.ind_sample(&mut rng)
    }
    pub fn get_name( &self ) -> &str {
        &self.name
    }
}
