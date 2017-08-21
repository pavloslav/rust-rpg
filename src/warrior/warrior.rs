use warrior::weapon::Weapon;

pub struct Warrior {
    hp: i32,
    weapon: Weapon,
    name: String,
}

impl Warrior {
    pub fn attack( &mut self, other: &mut Warrior) {
        println!("{} is attacking {} with {}",self.name, other.name, self.weapon.get_name());
        let result = self.weapon.hit(other);
        println!("{}", result)
    }
    pub fn apply_damage( &mut self, damage: i32 ) -> i32 {
        self.hp -= damage;
        damage
    }
    pub fn new( hp: i32, weapon: Weapon, name: &str) -> Warrior {
        Warrior {
            hp: hp,
            weapon: weapon,
            name: name.to_owned(),
        }
    }
    pub fn is_alive( &self ) -> bool {
        self.hp != 0
    }
}
