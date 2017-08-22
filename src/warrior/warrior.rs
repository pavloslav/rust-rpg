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
        self.hp >= 0
    }
    pub fn select_enemy<'a,I>( &self, others: I) -> Option<&'a mut Warrior>
            where I: Iterator<Item = &'a mut Warrior> {
        others.min_by_key(|warrior|warrior.hp)
    }
    pub fn percent( &self ) -> i32 {
        (100f64*(self.hp as f64)/(self.max_hp as f64)).round() as i32
    }
}

mod test{
    #[test]
    fn test_warrior() {
        use Weapon;
        use Warrior;

        let mut erik   = Warrior::new(50, Weapon::new( "Helmet", 5, 10 ), "Erik the Swift");
        let mut olaf   = Warrior::new(200,Weapon::new( "Shield", 0, 0  ), "Olaf the Stout");
        let mut baleog = Warrior::new(100,Weapon::new( "Sword", 10, 20 ), "Baleog the Fierce");
        assert!( erik.name == "Erik the Swift" );
        assert!( olaf.hp == 200 );
        assert!( baleog.weapon.get_name() == "Sword" );
        assert!( olaf.percent() == 100 );
        assert!( erik.percent() == 100 );
        erik.attack( &mut olaf );
        assert!( olaf.hp < olaf.max_hp );
        assert!( olaf.percent() < 100 );
        olaf.attack( &mut baleog );
        assert!( baleog.hp == baleog.max_hp );
        for _ in 0..5 {
            baleog.attack( &mut erik );
        }
        assert!( !erik.is_alive() );
        baleog.apply_damage(10);
        assert!( baleog.hp == 90 );
        assert!( baleog.percent() == 90 );
        assert!( olaf.max_hp == 200 );
        let mut goblins:Vec<_> = (0..5).map(|_|
                                    Warrior::new(10,Weapon::new("Stick", 1,2), "Pesky goblin"))
                                       .collect();
        goblins.push( Warrior::new( 1, Weapon::new( "Stick", 1, 2 ), "Gremlin" ) );
        match baleog.select_enemy( goblins.iter_mut() ) {
            Some(goblin) => {
                assert!(goblin.name == "Gremlin");
            },
            None => {
                panic!();
            }
        }
    }
}