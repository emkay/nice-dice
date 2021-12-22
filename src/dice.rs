use rand::Rng;

pub struct Die {
    pub sides: u16
}

pub struct Roll {
    pub outcome: u16
}

impl Die {
    pub fn new(sides: u16) -> Die {
        Die {
            sides
        }
    }

    pub fn roll(&self, mut num_dice: u16) -> Roll {
        let mut rng = rand::thread_rng();
        let mut outcome: u16 = 0;

        while num_dice > 0 {
            outcome += rng.gen_range(1..self.sides + 1);
            num_dice -= 1;
        }

        Roll {
            outcome
        }
    }
}
