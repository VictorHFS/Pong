extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use piston::input::PressEvent;
use std::process;
use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events};
use piston::input::{Button, Key, ReleaseEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent };
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

struct Ball {
	pos_x: i32,
	pos_y: i32,
	vel_x: i32,
	vel_y: i32,
}

struct Player {
	score: i32,
	pos: i32,
	vel: i32
}

pub struct App {
	gl: GlGraphics,
	left_player: Player,
	right_player: Player,
	ball: Ball,
}

impl App {
	fn render(&mut self, args: &RenderArgs) {
		use graphics::*;
		const BACKGROUND: [f32; 4] = [0.0, 0.5, 0.5, 1.0];
		const FOREGROUND: [f32; 4] = [0.0, 0.0, 1.0, 1.0];

        let l_pl = &self.left_player;
        let r_pl = &self.right_player;
        let b_obj = &self.ball;

		let left = rectangle::square(0.0, 0.0, 50.0);
		let right = rectangle::square(0.0, 0.0, 50.0);
		let ball = rectangle::square(0.0, 0.0, 10.0);

		self.gl.draw(args.viewport(), |c, gl| {
			clear(BACKGROUND, gl);
			rectangle(FOREGROUND, left, c.transform.trans(-40.0, l_pl.pos as f64), gl);
			rectangle(FOREGROUND, right, c.transform.trans(args.width as f64 - 10.0, r_pl.pos as f64), gl);
			rectangle(FOREGROUND, ball,  c.transform.trans(b_obj.pos_x as f64, b_obj.pos_y as f64), gl);
		})
	}

    fn update_player(player: &mut Player) {
		const TOP: i32 = 294;
		const BOTTOM: i32 = 1;
		const UP :i32 = -1;
		const DOWN : i32 = 1;

    	if (player.vel == DOWN && player.pos < TOP) 
		|| (player.vel == UP && player.pos >= BOTTOM) {
			player.pos += player.vel;
		}
    }

	fn update(&mut self, _args: &UpdateArgs) {

		&mut App::update_player(&mut self.left_player);
		&mut App::update_player(&mut self.right_player);

		self.ball.pos_x += self.ball.vel_x;

		if self.ball.pos_x > 502 {
			self.ball.vel_x = - self.ball.vel_x;
			if self.ball.pos_y < self.right_player.pos || self.ball.pos_y > self.right_player.pos + 50 {
				self.left_player.score += 1;
				if self.left_player.score > 5 {
					println!("Left win!");
					process::exit(0);
				}
				self.ball.pos_x = 256;
				self.ball.pos_y = 171;
			}
		}
		if self.ball.pos_x < 1 {
			self.ball.vel_x = - self.ball.vel_x;
			if self.ball.pos_y < self.left_player.pos || self.ball.pos_y > self.left_player.pos + 50 {
				self.right_player.score += 1;
				if self.right_player.score >= 5 {
					println!("Right win!");
					process::exit(0);
				}
			}
		}

		self.ball.pos_y += self.ball.vel_y;
		if self.ball.pos_y > 332 || self.ball.pos_y < 1 {
			self.ball.vel_x = - self.ball.vel_x;
			self.ball.vel_y = - self.ball.vel_y;
		}
	}

	fn press(&mut self, args: &Button) {
		if let &Button::Keyboard(key) = args {
			match key {
				Key::Up => {
					self.right_player.vel = -1;
				},
				Key::Down => {
					self.right_player.vel = 1;
				},
				Key::W => {
					self.left_player.vel = -1;
				},
				Key::S => {
					self.left_player.vel = 1;
				},
				_ => {}
			}
		}
	}
	fn release(&mut self, args: &Button) {
		if let &Button::Keyboard(key) = args {
			match key {
				Key::Up => {
					self.right_player.vel = 0;
				},
				Key::Down => {
					self.right_player.vel = 0;
				},
				Key::W => {
					self.left_player.vel = 0;
				},
				Key::S => {
					self.left_player.vel = 0;
				},
				_ => {}
			}
		}
	}
}



fn main() {
    let open_gl = OpenGL::V3_2;
    let mut window: GlutinWindow = WindowSettings::new("Pong", [512, 342])
    	.opengl(open_gl)
    	.exit_on_esc(true)
    	.build()
    	.unwrap();

    let mut app = App {
    	gl: GlGraphics::new(open_gl),
    	left_player: Player{pos:0, vel:1, score:0},
    	right_player:Player{pos:0, vel:1, score:0},
    	ball: Ball{pos_x:0,pos_y:0,vel_x:1,vel_y:1},
    };

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
    	if let Some(r) = e.render_args() {
    		app.render(&r);
    	}

    	if let Some(u) = e.update_args() {
    		app.update(&u);
    	}

    	if let Some(b) = e.press_args() {
    		app.press(&b);
    	}

    	if let Some(b) = e.release_args() {
    		app.release(&b);
    	}
    }
}
