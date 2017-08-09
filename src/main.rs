extern crate rand;

use rand::distributions::{Range, IndependentSample};

struct Warrior {
	hp: i32,
	weapon: Weapon,
	name: String,
}

impl Warrior {
	fn attack( &mut self, other: &mut Warrior) {
		println!("{} is attacking {} with {}",self.name, other.name,self.weapon.name);
		let result = self.weapon.hit(other);
		println!("{:?}", result)
	}
	fn apply_damage( &mut self, damage: i32 ) -> i32 {
		self.hp -= damage;
		damage
	}
	fn new( hp: i32, weapon: Weapon, name: &str) -> Warrior {
		Warrior {
			hp: hp,
			weapon: weapon,
			name: name.to_owned(),
		}
	}
	fn is_alive( &self ) -> bool {
		self.hp != 0
	}
}

struct Weapon {
	name: String,
	min_dmg: i32,
	max_dmg: i32,
}

#[derive(Debug)]
enum AttackResult {
	Wounded,
	Missed,
	Killed
}

impl Weapon {
	fn hit(&self, target: &mut Warrior) -> AttackResult {
		let damage_done = target.apply_damage(self.get_rnd_damage());
		if !target.is_alive() {
			AttackResult::Killed
		} else if damage_done > 0 {
			AttackResult::Wounded
		} else {
			AttackResult::Missed
		}
	}
	fn new(name: &str, min:i32, max: i32) -> Weapon {
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
}



fn main() {
    let mut fighter1 = Warrior::new(50, Weapon::new("Sword",10,15), "Orc" );
    let mut fighter2 = Warrior::new(60, Weapon::new("Axe",15,15), "Knight" );
    fighter1.attack(&mut fighter2);
    fighter2.attack(&mut fighter1);

}
