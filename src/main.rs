extern crate piston;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod model;
use model::*;

use piston::window::WindowSettings;
use piston::event_loop::*;
use piston::input::*;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{ GlGraphics, OpenGL };

pub struct App {
    gl: GlGraphics, // OpenGL drawing backend.
    world: World
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const WHITE: [f32; 4] = [1.0, 1.0, 1.0, 1.0];
        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED:   [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        const GRAY:   [f32; 4] = [0.5, 0.5, 0.5, 1.0];
        const BLUE:   [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let map = self.world.get_map();
        let ant = self.world.get_ant();
        let view = self.world.get_view();

        let cell_width = args.width as f64 / view.get_width() as f64;
        let cell_height = args.height as f64 / view.get_height() as f64;
        let cell_size = if cell_width < cell_height {cell_width} else {cell_height};

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen.
            clear(BLUE, gl);

            for i in view.x0..(view.x1) {
                for j in view.y0..(view.y1) {
                    let color = match map.get_color(i as u32, j as u32) {
                        Color::Gray => GRAY,
                        Color::White => WHITE,
                        Color::Red => RED,
                        Color::Green => GREEN
                    };
                    let x = (i - view.x0) as f64 * cell_size;
                    let y = (j - view.y0) as f64 * cell_size;
                    let rect: [f64; 4] = [x, y, cell_size, cell_size];

                    // Draw a box rotating around the middle of the screen.
                    rectangle(color, rect, c.transform, gl);
                }
            }
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.tick(args.dt);
    }
}

fn main() {
    // Change this to OpenGL::V2_1 if not working.
    let opengl = OpenGL::V3_2;

    // Create an Glutin window.
    let mut window: Window = WindowSettings::new(
            "Ant",
            [600, 600]
        )
        .opengl(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let world = World::new();

    // Create a new game and run it.
    let mut app = App {
        gl: GlGraphics::new(opengl),
        world: world
    };

    let mut events = window.events();
    while let Some(e) = events.next(&mut window) {
        if let Some(r) = e.render_args() {
            app.render(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
