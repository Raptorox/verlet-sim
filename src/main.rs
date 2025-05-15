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

fn populate_particles(mut particles: Vec<Rc<RefCell<Particle>>>, num: u32) {
    for i in 0..=num {
        particles.push(Rc::new(RefCell::new(Particle::new(Vector2f::new(100. * i as f32, 300.)))));
    }
}

fn populate_links(links: Vec<Link>, particles: Vec<Rc<RefCell<Particle>>>) {

}

fn apply_forces(particles: Vec<Rc<RefCell<Particle>>>) {
    for particle in particles {
        let mut borrowed = particle.borrow_mut();
        // borrowed.apply_force(Vector2f::new(0., GRAVITY));
        borrowed.apply_force(Vector2f::new(1., 1.));

    }
}

fn update_particles(particles: Vec<Rc<RefCell<Particle>>>, dt: f32) {
    for particle in particles { particle.borrow_mut().update(dt); }
}

fn solve_links(links: Vec<Link>) {
    for mut link in links { link.solve(); }
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

    let mut particles: Vec<Rc<RefCell<Particle>>> = vec![];
    populate_particles(particles, 3);
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