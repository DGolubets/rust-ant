extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate glutin_window;
extern crate opengl_graphics;

mod model;
mod settings;

use model::*;
use settings::Settings;

use piston_window::*;

pub struct App {
    world: World
}

impl App {
    fn render(&mut self, args: &RenderArgs, e: &PistonWindow, glyphs: &mut Glyphs) {
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

        e.draw_2d(|c, gl| {
            // Clear the screen.
            clear(GRAY, gl);

            for i in view.x0..(view.x1) {
                for j in view.y0..(view.y1) {
                    let color = match map.get_color(i as u32, j as u32) {
                        Color::Gray => GRAY,
                        Color::White => WHITE,
                        Color::Red => RED,
                        Color::Green => GREEN
                    };

                    // skip default color for performance
                    if color != GRAY {
                        let x = (i - view.x0) as f64 * cell_size;
                        let y = (j - view.y0) as f64 * cell_size;
                        let rect: [f64; 4] = [x, y, cell_size, cell_size];

                        // Draw a box rotating around the middle of the screen.
                        rectangle(color, rect, c.transform, gl);
                    }
                }
            }

            let transform = c.transform.trans(10.0, 42.0);
            text::Text::new_color(BLUE, 32).draw(
                format!("{}", self.world.get_move_number()).as_str(),
                glyphs,
                &c.draw_state,
                transform, gl
            );
        });
    }

    fn update(&mut self, args: &UpdateArgs) {
        self.world.tick(args.dt);
    }
}

fn main() {
    let settings = Settings::load().expect("Can't load settings.");

    // Create an Glutin window.
    let window: PistonWindow = WindowSettings::new(
            "Ant",
            [600, 600]
        )
        .exit_on_esc(true)
        .build()
        .unwrap();

    let world = World::new(settings.world_size, settings.moves_per_second);

    // Create a new game and run it.
    let mut app = App {
        world: world
    };

    let factory = window.factory.borrow().clone();
    let mut glyphs = Glyphs::new("resources/Superstar M54.ttf", factory).unwrap();

    for e in window {
        if let Some(r) = e.render_args() {
            app.render(&r, &e, &mut glyphs);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }
    }
}
