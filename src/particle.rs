use sfml::system::Vector2f;

pub struct Particle {
    pub pos: Vector2f,
    prev_pos: Vector2f,
    pub vel: Vector2f,
    accel: Vector2f,

    immovable: bool,
}

impl Particle {
    pub fn new<P: Into<Vector2f>>(pos: P, immovable: bool) -> Self {
        let pos = pos.into();
        Particle {
            pos,
            prev_pos: pos,
            vel: Vector2f::default(),
            accel: Vector2f::default(),

            immovable,
        }
    }

    pub fn apply_force<F: Into<Vector2f>>(&mut self, force: F) {
        self.accel += force.into();
    }

    pub fn update(&mut self, dt: f32) {
        if self.immovable {return}
        self.prev_pos = self.pos;
        self.vel += self.accel * dt;
        self.pos += self.vel * dt;
    }

    pub fn update_derivatives(&mut self, dt: f32) {
        self.vel = (self.pos - self.prev_pos) / dt;
        self.accel = Vector2f::default();
    }

    pub fn apply_vel(&mut self, vel: Vector2f) {
        if self.immovable {return}
        self.pos += vel;
    }
}
