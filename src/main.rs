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

pub struct App {
    gl: GlGraphics,  
    window_size: [f64; 2],
    rotation: f64,
    speed: f64,
    offset_x: f64, 
    offset_y: f64, 
    pressed_keys: HashSet<Key>,
    exit: bool,
}

impl App {
    fn render(&mut self, args: &RenderArgs) {
        use graphics::*;

        const GREEN: [f32; 4] = [0.0, 1.0, 0.0, 1.0];
        const RED: [f32; 4] = [1.0, 0.0, 0.0, 1.0];

        // Define a centered triangle
        let triangle = [
            [0.0, -25.0],  
            [-25.0, 25.0], 
            [25.0, 25.0],  
        ];

        let (win_w, win_h) = (self.window_size[0], self.window_size[1]);
        let (x, y) = (win_w / 2.0, win_h / 2.0); // Always at center

        self.gl.draw(args.viewport(), |c, gl| {
            // Clear the screen
            clear(GREEN, gl);

            // Apply movement by shifting the background
            let transform = c
                .transform
                .trans(x - self.offset_x, y - self.offset_y)
                .rot_deg(self.rotation);
            polygon(RED, &triangle, transform, gl);
        });
    }

    fn handle_key_press(&mut self, key: Key){
        self.pressed_keys.insert(key);
    }

    fn handle_key_release(&mut self, key: Key){
        self.pressed_keys.remove(&key);
    }

    fn update(&mut self, args: &UpdateArgs) {
        let move_amount = self.speed * args.dt;
        let rotate_speed = 100.0 * args.dt;

        // Define movement bounds
        let half_width = self.window_size[0] / 2.0;
        let half_height = self.window_size[1] / 2.0;
        let triangle_half_size = 25.0; // Since the triangle has 50x50 dimensions

        if self.pressed_keys.contains(&Key::W) {
            self.offset_y = (self.offset_y + move_amount).min(half_height - triangle_half_size);
        }
        if self.pressed_keys.contains(&Key::S) {
            self.offset_y = (self.offset_y - move_amount).max(-half_height + triangle_half_size);
        }
        if self.pressed_keys.contains(&Key::A) {
            self.offset_x = (self.offset_x + move_amount).min(half_width - triangle_half_size);
        }
        if self.pressed_keys.contains(&Key::D) {
            self.offset_x = (self.offset_x - move_amount).max(-half_width + triangle_half_size);
        }
        if self.pressed_keys.contains(&Key::Q) {
            self.rotation -= rotate_speed; 
        }
        if self.pressed_keys.contains(&Key::E) {
            self.rotation += rotate_speed; 
        }
        if self.pressed_keys.contains(&Key::R){
            self.rotation = 0.0;
        }
        if self.pressed_keys.contains(&Key::M) {
            self.exit = true;
        }
    }
}

fn main() {
    let opengl = OpenGL::V2_1;
    let window_size = [500.0, 350.0];

    // Create a Glutin window
    let mut window: Window = WindowSettings::new("centered-triangle", window_size)
        .graphics_api(opengl)
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut app = App {
        gl: GlGraphics::new(opengl),
        window_size,
        rotation: 0.0,
        speed: 200.0, 
        offset_x: 0.0, 
        offset_y: 0.0,
        pressed_keys: HashSet::new(),
        exit: false,
    };

    let mut events = Events::new(EventSettings::new());

    while let Some(e) = events.next(&mut window) {
        if app.exit{
            break;
        }
        if let Some(args) = e.render_args() {
            app.render(&args);
        }
        if let Some(args) = e.update_args(){
            app.update(&args);
        }
        if let Some(Button::Keyboard(key)) = e.press_args() {
            app.handle_key_press(key);
        }
        if let Some(Button::Keyboard(key)) = e.release_args() {
            app.handle_key_release(key);
        }
        if let Some(new_size) = e.resize_args() {
            app.window_size = [new_size.window_size[0], new_size.window_size[1]];
            // app.ensure_within_bounds();  // Adjust offsets to fit within the new window
        }
    }
}
