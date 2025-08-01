use sfml::system::Vector2f;

pub struct Particle {
    pub pos: Vector2f,
    prev_pos: Vector2f,
    accel: Vector2f,

    immovable: bool,
}

impl Particle {
    pub fn new<P: Into<Vector2f>>(pos: P, immovable: bool) -> Self {
        let pos = pos.into();
        Particle {
            pos,
            prev_pos: pos,
            accel: Vector2f::default(),

            immovable,
        }
    }

    pub fn apply_force<F: Into<Vector2f>>(&mut self, force: F) {
        if self.immovable {
            return;
        }
        self.accel += force.into();
    }

    pub fn update(&mut self, dt: f32) {
        let damping = 0.99;
        let vel = (self.pos - self.prev_pos) * damping;
        let new_pos = self.pos + vel + self.accel * (dt * dt);

        self.prev_pos = self.pos;
        self.pos = new_pos;
        self.accel = Vector2f::default();
    }
}
