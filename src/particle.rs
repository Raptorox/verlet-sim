use sfml::system::Vector2f;

pub struct Particle {
    pub pos: Vector2f,
    prev_pos: Vector2f,
    accel: Vector2f,

    immovable: bool
}

impl Particle {
    pub fn new(pos: Vector2f, immovable: bool) -> Self {
        Particle {
            pos: pos,
            prev_pos: pos,
            accel: Vector2f::default(),

            immovable: immovable
        }
    }

    pub fn apply_force(&mut self, force: Vector2f) {
        self.accel += force;
    }

    pub fn update(&mut self, dt: f32) {
        if self.immovable { return; }
        let damping = 0.99;
        let vel = (self.pos - self.prev_pos) * damping;
        let new_pos = self.pos + vel + self.accel * (dt * dt);

        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.accel = Vector2f::default();
    }
}