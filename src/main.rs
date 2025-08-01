use sfml::SfResult;
use sfml::graphics::*;
use sfml::system::{Clock, Vector2f};
use sfml::window::*;
use std::cell::RefCell;
use std::rc::Rc;

mod math;

mod particle;
use particle::Particle;

mod link;
use link::Link;

const WIN_WIDTH: u32 = 800;
const WIN_HEIGHT: u32 = 600;

const GRAVITY: f32 = 100.;

fn populate_particles(particles: &mut Vec<Rc<RefCell<Particle>>>, num: u32, cols: u32) {
    for i in 0..num {
        const OFFSET: u32 = 20;
        let x_pos = (WIN_WIDTH/2 - cols/2*OFFSET + OFFSET * (i % cols)) as f32;
        let y_pos = (WIN_HEIGHT/2 - (num/cols)/2*OFFSET + OFFSET * (i / cols)) as f32;
        let immovable = i < cols;
        let particle = Particle::new(Vector2f::new(x_pos, y_pos), immovable);
        particles.push(Rc::new(RefCell::new(particle)));
    }
}

fn populate_circles(circles: &mut Vec<CircleShape>, num: u32, rad: f32, pts: usize) {
    for _ in 0..num {
        let mut circle = CircleShape::new(rad, pts);
        circle.set_origin((rad, rad));
        circles.push(circle);
    }
}

fn populate_links(
    links: &mut Vec<Link>,
    particles: &[Rc<RefCell<Particle>>],
    width: usize,
    height: usize,
) {
    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            if x < width - 1 {
                let right = i + 1;
                let dist = math::vec_len(particles[i].borrow().pos - particles[right].borrow().pos);
                let link = Link::new(Rc::clone(&particles[i]), Rc::clone(&particles[right]), dist);
                links.push(link);
            }

            if y < height - 1 {
                let below = i + width;
                let dist = math::vec_len(particles[i].borrow().pos - particles[below].borrow().pos);
                let link = Link::new(Rc::clone(&particles[i]), Rc::clone(&particles[below]), dist);
                links.push(link);
            }
        }
    }
}

/*fn populate_lines(lines: &mut Vec<RectangleShape>, circles: &Vec<CircleShape>, num: u32) {
    for i in 0..num {
        let rect = RectangleShape::with_size(Vector2f::new(100., 100.));
    }
}*/

fn apply_forces(particles: &Vec<Rc<RefCell<Particle>>>) {
    for particle in particles {
        let mut borrowed = particle.borrow_mut();
        borrowed.apply_force(Vector2f::new(0., GRAVITY));
    }
}

fn update_particles(particles: &Vec<Rc<RefCell<Particle>>>, dt: f32) {
    for particle in particles {
        particle.borrow_mut().update(dt);
    }
}

fn solve_links(links: &mut Vec<Link>) {
    for link in links {
        link.solve();
    }
}

fn update_positions(circles: &mut Vec<CircleShape>, particles: &[Rc<RefCell<Particle>>]) {
    for (index, circle) in circles.iter_mut().enumerate() {
        circle.set_position(particles[index].borrow().pos);
    }
}

fn draw_all(
    window: &mut RenderWindow,
    circles: &Vec<CircleShape>, /* lines: &Vec<RectangleShape> */
) {
    for circle in circles {
        window.draw(circle);
    }

    // for line in lines {
    //     window.draw(line);
    // }
}

fn main() -> SfResult<()> {
    let mut window = RenderWindow::new((WIN_WIDTH, WIN_HEIGHT), "Verlet", Style::CLOSE, &Default::default())?;
    window.set_framerate_limit(60);

    let mut clock = Clock::start()?;

    let mouse_pos_prev = window.mouse_position();
    let mut mouse_coords_prev = window.map_pixel_to_coords_current_view(mouse_pos_prev);

    let particle_count: u32 = 600;
    let column_count: u32 = 30;
    let row_count: u32 = particle_count / column_count;

    let mut particles: Vec<Rc<RefCell<Particle>>> = vec![];
    populate_particles(&mut particles, particle_count, column_count);

    let mut links: Vec<Link> = vec![];
    populate_links(
        &mut links,
        &particles,
        column_count as usize,
        row_count as usize,
    );
    // let mut link3 = Link::new(Rc::clone(&(particles[0])), Rc::clone(&(particles[2])), 200.);

    let radius: f32 = 8.;
    let point_count: usize = 100;
    let mut circles: Vec<CircleShape> = vec![];
    populate_circles(&mut circles, particle_count, radius, point_count);

    // let mut lines: Vec<RectangleShape> = vec![];
    // populate_lines(&mut lines, &circles, particle_count);

    while window.is_open() {
        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed => window.close(),
                _ => {}
            }
        }

        let mouse_pos = window.mouse_position();
        let mouse_coords = window.map_pixel_to_coords_current_view(mouse_pos);
        let mouse_vel = mouse_coords - mouse_coords_prev;
        mouse_coords_prev = mouse_coords;

        if mouse::Button::is_pressed(mouse::Button::Left) {
            for particle in &particles {
                let p_pos = particle.borrow().pos;
                let dist_vec = mouse_coords - p_pos;
                let dist = math::vec_len(dist_vec);
                if dist < 20. {
                    particle.borrow_mut().apply_force(mouse_vel * 128.);
                }
            }
        }

        let dt = clock.restart().as_seconds();

        //particle.apply_force(Vector2f::new(0., GRAVITY));

        
        apply_forces(&particles);
        update_particles(&particles, dt);

        solve_links(&mut links);

        update_positions(&mut circles, &particles);

        window.clear(Color::BLACK);
        draw_all(&mut window, &circles /* &lines */);
        window.display();
    }

    Ok(())
}
