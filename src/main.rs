extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::*; //{RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use piston::window::WindowSettings;

#[derive(Clone, PartialEq)]
enum Direction {
	Right, Left, Up, Down
}

struct Game {
	gl: GlGraphics,
	player: Player,
	//map: Map,
}

impl Game {
	fn render(&mut self, arg: &RenderArgs) {
		let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(black, gl);
		});
		
		self.player.render(&mut self.gl, arg)
	}

//	fn update(&mut self) {
//	}

	fn pressed(&mut self, btn: &Button) {
		let last_direction = self.player.dir.clone();

		self.player.dir = match btn {
			&Button::Keyboard(Key::Up)
				if last_direction != Direction::Down => Direction::Up,
			&Button::Keyboard(Key::Down)
				if last_direction != Direction::Up => Direction::Down,
			&Button::Keyboard(Key::Left)
				if last_direction != Direction::Right => Direction::Left,
			&Button::Keyboard(Key::Right)
				if last_direction != Direction::Left => Direction::Right,
			_ => last_direction
		};
		
		self.player.update();
	}
}

struct Player {
	pos_x: f32,
	pos_y: f32,
	dir: Direction,
}

impl Player {
	fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs) {
		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		let square = graphics::rectangle::square((self.pos_x * 10.0) as f64,
			(self.pos_y * 10.0) as f64,
			20_f64);
	
		gl.draw(arg.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(red, square, transform, gl);
		});
	}

	fn update(&mut self) {
		match self.dir {
			Direction::Left => self.pos_x -= 1.0,
			Direction::Right => self.pos_x += 1.0,
			Direction::Up => self.pos_y -= 1.0,
			Direction::Down => self.pos_y += 1.0,
		}
	}
}

fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new(
		"raycasting",
		[800, 800]
		).graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game {
		gl: GlGraphics::new(opengl),
		player: Player { pos_x: 1.0, pos_y: 1.0, dir: Direction::Right},
	};

	let mut events = Events::new(EventSettings::new());//.ups(120);
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			game.render(&args);
		}
		/*
		if let Some(_u) = e.update_args() {
			game.update();
		}
		*/
		if let Some(k) = e.button_args() {
			if k.state == ButtonState::Press {
				game.pressed(&k.button);
		}
		}

	}
}
