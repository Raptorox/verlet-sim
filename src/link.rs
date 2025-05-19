use crate::particle::Particle;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Link {
    p1: Rc<RefCell<Particle>>,
    p2: Rc<RefCell<Particle>>,
    length: f32
}

impl Link {
    pub fn new(p1: Rc<RefCell<Particle>>, p2: Rc<RefCell<Particle>>, length: f32) -> Self {
        Link {
            p1: p1,
            p2: p2,
            length: length
        }
    }

    pub fn solve(&mut self) {
        let vec = self.p2.borrow().pos - self.p1.borrow().pos;
        let vec_len = (vec.x*vec.x + vec.y*vec.y).sqrt();
        let vec_norm = vec/vec_len;
        let vec_scaled = vec_norm * (self.length-vec_len) * 99.;

        self.p1.borrow_mut().apply_force(-vec_scaled);
        self.p2.borrow_mut().apply_force(vec_scaled);
    }
}
