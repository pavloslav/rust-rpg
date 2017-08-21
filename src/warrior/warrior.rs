use warrior::weapon::Weapon;

pub struct Warrior {
    hp: i32,
    max_hp: i32,
    weapon: Weapon,
    name: String,
}

impl Warrior {
    pub fn new( hp: i32, weapon: Weapon, name: &str) -> Warrior {
        Warrior {
            hp: hp,
            max_hp: hp,
            weapon: weapon,
            name: name.to_owned(),
        }
    }
    pub fn attack( &mut self, other: &mut Warrior) {
        println!("{} is attacking {} with {}",self.name, other.name, self.weapon.get_name());
        let result = self.weapon.hit(other);
        println!("{}", result);
        println!("{} now has {}% hp", other.name, other.percent());
    }
    pub fn apply_damage( &mut self, damage: i32 ) -> i32 {
        self.hp -= damage;
        damage
    }
    pub fn is_alive( &self ) -> bool {
        self.hp != 0
    }
    pub fn select_enemy<'a,I>( &self, others: I) -> Option<&'a mut Warrior>
            where I: Iterator<Item = &'a mut Warrior> {
        others.min_by_key(|warrior|warrior.hp)
    }
    pub fn percent( &self ) -> i32 {
        (100f64*(self.hp as f64)/(self.max_hp as f64)).round() as i32
    }
}
