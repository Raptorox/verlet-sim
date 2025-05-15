use sfml::system::Vector2f;

pub struct Particle {
    pub pos: Vector2f,
    prev_pos: Vector2f,
    accel: Vector2f
}

impl Particle {
    pub fn new(pos: Vector2f) -> Self {
        Particle {
            pos: pos,
            prev_pos: pos,
            accel: Vector2f::default()
        }
    }

    pub fn apply_force(&mut self, force: Vector2f) {
        self.accel += force;
    }

    pub fn update(&mut self, dt: f32) {
        let vel = (self.pos - self.prev_pos) * 0.99;
        let new_pos = self.pos + vel + self.accel * (dt * dt);

        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.accel = Vector2f::default();
    }
}