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
	map: Map,
	view: View,
}

impl Game {
	fn render(&mut self, arg: &RenderArgs) {
		let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(black, gl);
		});
		
		self.view.render(&mut self.gl, arg, &self.player, &self.map);
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
	dir_x: f32,
	dir_y: f32,
	dir: Direction,
}

impl Player {
	fn update(&mut self) {
		match self.dir {
			Direction::Left => self.pos_x -= 1.0,
			Direction::Right => self.pos_x += 1.0,
			Direction::Up => self.pos_y -= 1.0,
			Direction::Down => self.pos_y += 1.0,
		}
	}
}

struct Map {
	height: i32,
	width: i32,
	values: Vec<i32>,
}


struct View {
	plane_x: f32,
	plane_y: f32,
	max_dof: u8,
	rays: i32,
}

fn sign(x:f32) -> f32 {
	if x > 0.0 {
		return 1.0;
	}
	return -1.0;
}

impl View {
	fn render(&self, gl: &mut GlGraphics, arg: &RenderArgs, player: &Player, map: &Map) {
		let w:i32 = map.width*self.rays;
		for x in 1..w {
			let camera_x:f32 = 2.0 * x as f32 / self.rays as f32 / map.width as f32 - 1.0;
			let raydir_x:f32 = player.dir_x + self.plane_x * camera_x;
			let raydir_y:f32 = player.dir_y + self.plane_y * camera_x;
			
			if raydir_x == 0.0 || raydir_y == 0.0 {
				continue;
			}

			let mut map_x:i32 = player.pos_x as i32;
			let mut map_y:i32 = player.pos_y as i32;

			let steps_x:f32 = sign(raydir_x);
			let steps_y:f32 = sign(raydir_y);

			let delta_dist_x:f32 = steps_x / raydir_x;
			let delta_dist_y:f32 = steps_y / raydir_y;

			let mut side_dist_x:f32 = (steps_x * map_x as f32 + (steps_x + 1.0) * 0.5 + (-steps_x) * player.pos_x as f32) * delta_dist_x;
			let mut side_dist_y:f32 = (steps_y * map_y as f32 + (steps_y + 1.0) * 0.5 + (-steps_y) * player.pos_y as f32) * delta_dist_y;

			let mut hit:u8 = 0;
			let mut dof:u8 = 0;

			let mut side:i32 = 0;
			let mut field_value:i32 = 0;
			while hit == 0 && dof < self.max_dof {
				side = (side_dist_y < side_dist_x) as i32;
				if side == 1 {
					map_y = (map_y + steps_y as i32) % map.width;
					side_dist_y += delta_dist_y;
				} else {
					map_x = (map_x + steps_x as i32) % map.width;
					side_dist_x += delta_dist_x;
				}
				if (map_y * map.width + map_x) < map.width * map.height {
					field_value = map.values[(map_y*map.width+map_x) as usize];
				} else {
					field_value = 1;
				}
				
				if field_value > 0 {
					hit = 1;
				}	
				dof += 1;
			}

		}

				




		let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
		let square = graphics::rectangle::square((player.pos_x * 10.0) as f64,
			(player.pos_y * 10.0) as f64,
			20_f64);
	
		gl.draw(arg.viewport(), |c, gl| {
			let transform = c.transform;
			graphics::rectangle(red, square, transform, gl);
		});
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
		player: Player { pos_x: 1.0, pos_y: 1.0, dir_x: -1.0, dir_y: 0.0, dir: Direction::Right },
		map: Map { height: 3, width: 3, values: [1,1,1,1,0,1,1,1,1].to_vec() },
		view: View { plane_x: 0.0, plane_y:0.66, max_dof: 8, rays: 16 },
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
