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
    let (f1, f2) = arena.fighters.split_at_mut(1);
    f1[0].attack( &mut f2[0] );
    f2[0].attack( &mut f1[0] );
}
