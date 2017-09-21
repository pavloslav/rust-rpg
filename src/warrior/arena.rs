pub struct Arena<'a> {
    warriors: Vec<Warrior>,
    queue: Vec<&'a mut Warrior>,
}

use Warrior;
use Weapon;

impl<'a> Arena<'a> {
    pub fn new_1x1() -> Arena<'a> {
        let mut arena = Arena { warriors: Vec::new(), 
                                queue: Vec::new() };
        arena.warriors.push( Warrior::new(50, Weapon::new("Sword",10,15), "Orc"    ) );
        arena.warriors.push( Warrior::new(60, Weapon::new("Axe",  15,15), "Knight" ) );
        for warrior in arena.warriors {
            arena.queue.push(&mut warrior);
        }
        arena
    }
    pub fn fight(&mut self) {
        let next = self.queue.pop().unwrap();
        let opponent = next.select_enemy(self.queue.iter()).unwrap();

        

    }
}