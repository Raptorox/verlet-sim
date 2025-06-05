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
        let vec_len = (vec.x * vec.x + vec.y * vec.y).sqrt();

        if vec_len > self.length {
            let vec_norm = vec / vec_len;

            let max_stretch = 0.5;
            let clamped_len = vec_len.min(self.length + max_stretch);

            let displacement = clamped_len - vec_len;
            let stiffness = 999.;
            let vec_scaled = vec_norm * displacement * stiffness;

            let damping = 0.99;
            self.p1.borrow_mut().apply_force(-vec_scaled * damping);
            self.p2.borrow_mut().apply_force(vec_scaled * damping);
        }
    }
}
