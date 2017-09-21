mod warrior;

use warrior::Warrior;
use warrior::Weapon;
use warrior::Arena;

struct Arena1v1<'a> {
    fighters : Vec<&'a mut Warrior>,
}

fn main() {
    let mut arena = Arena1v1 { fighters:Vec::new() };
    arena.fighters.push( &mut Warrior::new(50, Weapon::new("Sword",10,15), "Orc"    ) );
    arena.fighters.push( &mut Warrior::new(60, Weapon::new("Axe",  15,15), "Knight" ) );
    let (mut f1, mut f2) = arena.fighters.split_at_mut(1);
    {
        let f1_tgt = f1[0].select_enemy(f2.iter_mut()).unwrap();
        f1[0].attack( f1_tgt );
    }
    {
        let f2_tgt = f2[0].select_enemy(f1.iter_mut()).unwrap();
        f2[0].attack( f2_tgt );
    }
}
