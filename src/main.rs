extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use std::collections::HashSet;
use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::{ReleaseEvent, UpdateEvent, ResizeEvent};
use piston::event_loop::{EventSettings, Events};
use piston::input::{RenderArgs, RenderEvent, UpdateArgs, Button, Key, PressEvent};
use piston::window::WindowSettings;

/// The Player Struct
struct Player {
    position: [f64; 2],
    rotation: f64,
    speed: f64,
    bounds: [f64; 2], // Window bounds
}

impl Player {
    fn new(bounds: [f64; 2]) -> Self {
        Self {
            position: [0.0, 0.0],
            rotation: 0.0,
            speed: 200.0,
            bounds,
        }
    }

    fn update(&mut self, pressed_keys: &HashSet<Key>, dt: f64) {
        let move_amount = self.speed * dt;
        let rotate_speed = 100.0 * dt;
        let triangle_half_size = 25.0;

        let half_width = self.bounds[0] / 2.0;
        let half_height = self.bounds[1] / 2.0;

        if pressed_keys.contains(&Key::W) {
            self.position[1] = (self.position[1] + move_amount).min(half_height - triangle_half_size);
        }
        if pressed_keys.contains(&Key::S) {
            self.position[1] = (self.position[1] - move_amount).max(-half_height + triangle_half_size);
        }
        if pressed_keys.contains(&Key::A) {
            self.position[0] = (self.position[0] + move_amount).min(half_width - triangle_half_size);
        }
        if pressed_keys.contains(&Key::D) {
            self.position[0] = (self.position[0] - move_amount).max(-half_width + triangle_half_size);
        }
        if pressed_keys.contains(&Key::Q) {
            self.rotation -= rotate_speed;
        }
        if pressed_keys.contains(&Key::E) {
            self.rotation += rotate_speed;
        }
        if pressed_keys.contains(&Key::R) {
            self.rotation = 0.0;
        }
    }

    fn render(&self, gl: &mut GlGraphics, args: &RenderArgs, window_size: [f64; 2]) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        let triangle = [
            [0.0, -25.0],
            [-25.0, 25.0],
            [25.0, 25.0],
        ];

        let (win_w, win_h) = (window_size[0], window_size[1]);
        let (x, y) = (win_w / 2.0, win_h / 2.0);

        gl.draw(args.viewport(), |c, gl| {
            clear(GREEN, gl);

            let transform = c
                .transform
                .trans(x + self.position[0], y - self.position[1])
                .rot_deg(self.rotation);

            polygon(RED, &triangle, transform, gl);
        });
    }

    fn resize(&mut self, new_bounds: [f64; 2]) {
        self.bounds = new_bounds;
    }
}

/// The Main App Struct
pub struct App {
    gl: GlGraphics,
    window_size: [f64; 2],
    player: Player,
    pressed_keys: HashSet<Key>,
    exit: bool,
}

impl App {
    fn update(&mut self, args: &UpdateArgs) {
        self.player.update(&self.pressed_keys, args.dt);
    }

    fn render(&mut self, args: &RenderArgs) {
        self.player.render(&mut self.gl, args, self.window_size);
    }

    fn handle_key_press(&mut self, key: Key) {
        self.pressed_keys.insert(key);
    }

    fn handle_key_release(&mut self, key: Key) {
        self.pressed_keys.remove(&key);
    }

    fn resize(&mut self, new_size: [f64; 2]) {
        self.window_size = new_size;
        self.player.resize(new_size);
    }
}

fn main() {
    let opengl = OpenGL::V2_1;
    let window_size = [500.0, 350.0];

    let mut window: Window = WindowSettings::new("centered-triangle", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        window_size,
        player: Player::new(window_size),
        pressed_keys: HashSet::new(),
        exit: false,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if app.exit {
            break;
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args() {
            app.update(&args);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }
        if let Some(new_size) = e.resize_args() {
            app.resize(new_size.window_size);
        }
    }
}
