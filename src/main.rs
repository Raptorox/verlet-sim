use sfml::graphics::*;
use sfml::window::*;
use sfml::SfResult;
use sfml::system::{Vector2f, Clock};
use std::rc::Rc;
use std::cell::RefCell;

mod particle;
use particle::Particle;

mod link;
use link::Link;

//const GRAVITY: f32 = 100.;

fn populate_particles(particles: Vec<Particle>) {

}

fn populate_links(links: Vec<Link>) {

}

fn apply_forces(particles: Vec<Particle>) {

}

fn update_particles(particles: Vec<Particle>) {

}

fn solve_links(links: Vec<Link>) {

}

fn update_positions(circles: Vec<CircleShape>) {

}

fn main() -> SfResult<()> {
    let mut window = RenderWindow::new(
        (800, 600),
        "Verlet",
        Style::CLOSE,
        &Default::default()
    )?;
    window.set_framerate_limit(60);

    let mut clock = Clock::start()?;

    let mut particles: Vec<Particle> = vec![];
    populate_particles(particles);
    let particle1 = Rc::new(RefCell::new(Particle::new(Vector2f::new(200., 300.))));
    let particle2 = Rc::new(RefCell::new(Particle::new(Vector2f::new(300., 300.))));
    let particle3 = Rc::new(RefCell::new(Particle::new(Vector2f::new(400., 300.))));

    let mut link1 = Link::new(Rc::clone(&particle1), Rc::clone(&particle2), 100.);
    let mut link2 = Link::new(Rc::clone(&particle2), Rc::clone(&particle3), 100.);
    let mut link3 = Link::new(Rc::clone(&particle1), Rc::clone(&particle3), 200.);

    let radius = 32.;
    let mut circle1 = CircleShape::new(radius, 100);
    let mut circle2 = CircleShape::new(radius, 100);
    let mut circle3 = CircleShape::new(radius, 100);
    circle1.set_origin((radius, radius));
    circle2.set_origin((radius, radius));
    circle3.set_origin((radius, radius));

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        if mouse::Button::is_pressed(mouse::Button::Left) {
            let mouse_pos = window.mouse_position();
            let mouse_coords = window.map_pixel_to_coords_current_view(mouse_pos);
            let p_pos = particle1.borrow().pos;
            particle2.borrow_mut().apply_force(mouse_coords - p_pos);
        }

        let dt = clock.restart().as_seconds();

        //particle.apply_force(Vector2f::new(0., GRAVITY));

        particle1.borrow_mut().update(dt);
        particle2.borrow_mut().update(dt);
        particle3.borrow_mut().update(dt);

        link1.solve();
        link2.solve();
        link3.solve();

        circle1.set_position(particle1.borrow().pos);
        circle2.set_position(particle2.borrow().pos);
        circle3.set_position(particle3.borrow().pos);

        window.clear(Color::BLACK);
        window.draw(&circle1);
        window.draw(&circle2);
        window.draw(&circle3);
        window.display();
    }

    Ok(())
}