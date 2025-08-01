use sfml::system::Vector2f;

pub fn vec_len(vec: Vector2f) -> f32 {
    (vec.x * vec.x + vec.y * vec.y).sqrt()
}