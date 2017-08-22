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


mod test{

    #[test]
    fn test_weapon() {
        use Weapon;
        use Warrior;

        let club      = Weapon::new( "Club",     10, 15 );
        let sword     = Weapon::new( "Sword",    20, 30 );
        let toothpick = Weapon::new( "Toothpick", 1, 1  );
        assert!( club.name == "Club" );
        assert!( sword.min_dmg == 20 );
        assert!( toothpick.max_dmg==toothpick.min_dmg );
        for _ in 0..100 {
            let club_damage = club.get_rnd_damage();
            assert!( (10 <= club_damage) && (club_damage <= 15) );
            let sword_damage = sword.get_rnd_damage();
            assert!( (20 <= sword_damage) && (sword_damage <= 30) );
            let toothpick_damage = toothpick.get_rnd_damage();
            assert!( toothpick_damage == 1 );
        }
        let mut manchkin = Warrior::new( 100, club, "Dummy" );
        let old_hp = manchkin.percent();
        toothpick.hit( &mut manchkin );
        assert!( manchkin.percent() == 99);
        sword.hit( &mut manchkin );
        assert!( manchkin.percent() < old_hp );
    }
}