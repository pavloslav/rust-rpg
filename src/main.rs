mod warrior;

use warrior::Warrior;
use warrior::weapon::Weapon;

struct Arena1v1 {
    fighters : Vec<Warrior>,
}

fn main() {
    let mut arena = Arena1v1 { fighters:Vec::new() };
    arena.fighters.push( Warrior::new(50, Weapon::new("Sword",10,15), "Orc"    ) );
    arena.fighters.push( Warrior::new(60, Weapon::new("Axe",  15,15), "Knight" ) );
    arena.fighters[0].attack( &mut arena.fighters[1] );
    arena.fighters[1].attack( &mut arena.fighters[0] );
}
