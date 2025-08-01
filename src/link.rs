use crate::math;
use crate::particle::Particle;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Link {
    p1: Rc<RefCell<Particle>>,
    p2: Rc<RefCell<Particle>>,
    length: f32,
}

impl Link {
    pub fn new(p1: Rc<RefCell<Particle>>, p2: Rc<RefCell<Particle>>, length: f32) -> Self {
        Link { p1, p2, length }
    }

    pub fn solve(&mut self) {
        let vec = self.p2.borrow().pos - self.p1.borrow().pos;
        let vec_len = math::vec_len(vec);

        if vec_len > self.length {
            let vec_norm = vec / vec_len;

            let displacement = self.length - vec_len;
            let vec_scaled = vec_norm * displacement / 2.;

            self.p1.borrow_mut().apply_vel(-vec_scaled);
            self.p2.borrow_mut().apply_vel(vec_scaled);
        }
    }
}
