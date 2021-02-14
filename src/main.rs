extern crate glutin_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate piston;

use glutin_window::GlutinWindow as Window;
use opengl_graphics::{GlGraphics, OpenGL};
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::*;
use piston::window::WindowSettings;

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

	fn update(&mut self) {
		if self.player.dir.iter().any(|&i| i == 1) {	
			self.player.update(&self.map);
			self.view.update(&self.player);
		}
	}

	fn pressed(&mut self, btn: &Button, v: i32) {
		match btn {
			&Button::Keyboard(Key::Up) => self.player.dir[0] = v,
			&Button::Keyboard(Key::Down) => self.player.dir[1] = v,
			&Button::Keyboard(Key::Left) => self.player.dir[2] = v,
			&Button::Keyboard(Key::Right) => self.player.dir[3] = v,
			_ => (),
		};
			
	}
}

struct Player {
	pos_x: f32,
	pos_y: f32,
	dir_x: f32,
	dir_y: f32,
	dir: Vec<i32>,
}

impl Player {
	fn update(&mut self, map: &Map) {
		let deg:f32 = 3.14159/30.0;
		let speed:f32 = 0.05;

		if self.dir[2] == 1 {		
			let old_dir_x:f32 = self.dir_x.clone();
			self.dir_x = old_dir_x * deg.cos() - self.dir_y * deg.sin();
			self.dir_y = old_dir_x * deg.sin() + self.dir_y * deg.cos();
		}
		if self.dir[3] == 1 {
				let old_dir_x:f32 = self.dir_x.clone();
			self.dir_x = old_dir_x * (-deg).cos() - self.dir_y * (-deg).sin();
			self.dir_y = old_dir_x * (-deg).sin() + self.dir_y * (-deg).cos();
		}
		if self.dir[0] == 1 {
			if map.values[((self.pos_y + self.dir_y * speed) as i32 * map.height + self.pos_x as i32) as usize] == 0 {
				self.pos_y += self.dir_y * speed;
			}
			if map.values[((self.pos_x + self.dir_x * speed) as i32 + self.pos_y as i32 * map.height) as usize] == 0 {
				self.pos_x += self.dir_x * speed;
			}
		}
		if self.dir[1] == 1 {
			if map.values[((self.pos_y - self.dir_y * speed) as i32 * map.height + self.pos_x as i32) as usize] == 0{
				self.pos_y -= self.dir_y * speed;
			}
			if map.values[((self.pos_x - self.dir_x * speed) as i32 + self.pos_y as i32 * map.height) as usize] == 0 {
				self.pos_x -= self.dir_x * speed;
			}
		} 
		
	}
}

struct Map {
	height: i32,
	width: i32,
	values: Vec<i32>,
}


struct View {
	screen_width: i32,
	screen_height: i32,
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


		let black: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		gl.draw(arg.viewport(), |_c, gl| {
			graphics::clear(black, gl);
		});



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
			let mut perp_wall_dist:f32 = 0.0;
			while hit == 0 && dof < self.max_dof {
				side = (side_dist_y < side_dist_x) as i32;
				if side == 1 {
					map_y = (map_y + steps_y as i32) % map.width;
					side_dist_y += delta_dist_y;
				} else {
					map_x = (map_x + steps_x as i32) % map.width;
					side_dist_x += delta_dist_x;
				}
				if 0 < (map_y * map.width + map_x) && (map_y * map.width + map_x) < map.width * map.height {
					field_value = map.values[(map_y*map.width+map_x) as usize];
					if field_value > 0 {
						hit = 1;
					}	
				} else {
					field_value = 1;
					hit = 1;
				}
				
				dof += 1;
			}
			if hit == 0 {
				continue;
			}

			if side == 1 {
				perp_wall_dist = (map_y as f32 - player.pos_y + (1.0 - steps_y) * 0.5) / raydir_y;
			} else {
				perp_wall_dist = (map_x as f32 - player.pos_x + (1.0 - steps_x) * 0.5) / raydir_x;
			}

			let mut lh:f32 = self.screen_height as f32;
			if perp_wall_dist > 0.0 {
				lh /= perp_wall_dist;
			}

			let lw:i32 = self.screen_width / w;
				
			let red: [f32; 4] = [1.0, 0.0, 0.0, 1.0 - (side as f32 * 0.3)]; // opacity depends on side that is hit by ray
			
			let square = graphics::rectangle::rectangle_by_corners((x * lw) as f64,
								((self.screen_height as f32 - lh) * 0.5) as f64,
								((x * lw) + lw) as f64, ((self.screen_height as f32 - lh) * 0.5 + lh) as f64);
	
			gl.draw(arg.viewport(), |c, gl| {
				let transform = c.transform;
				graphics::rectangle(red, square, transform, gl);
			});
		}
	}

	fn update(&mut self, player: &Player) {
		let deg:f32 = 3.14159/30.0;

		if player.dir[2] == 1 {
			let old_plane_x:f32 = self.plane_x.clone();
			self.plane_x = old_plane_x * deg.cos() - self.plane_y * deg.sin();
			self.plane_y = old_plane_x * deg.sin() + self.plane_y * deg.cos();
		}
		if player.dir[3] == 1 {	
				let old_plane_x:f32 = self.plane_x.clone();
				self.plane_x = old_plane_x * (-deg).cos() - self.plane_y * (-deg).sin();
				self.plane_y = old_plane_x * (-deg).sin() + self.plane_y * (-deg).cos();
		}
	}
}


fn main() {
	let opengl = OpenGL::V3_2;

	let mut window: Window = WindowSettings::new(
		"raycasting",
		[960, 480]
		).graphics_api(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();

	let mut game = Game {
		gl: GlGraphics::new(opengl),
		player: Player { pos_x:1.2 , pos_y: 1.2, dir_x: -1.0, dir_y: 0.0, dir: [0, 0, 0, 0].to_vec() },
		map: Map { height: 5, width: 5, values: [1,1,1,1,1,1,0,0,0,1,1,0,1,0,1,1,0,0,0,1,1,1,1,1,1].to_vec() },
		view: View { screen_width: 960, screen_height: 480, plane_x: 0.0, plane_y:0.66, max_dof: 5, rays: 192 },
	};

	let mut events = Events::new(EventSettings::new());//.ups(120);
	while let Some(e) = events.next(&mut window) {
		if let Some(args) = e.render_args() {
			game.render(&args);
		}
		
		if let Some(_u) = e.update_args() {
			game.update();
		}
		if let Some(k) = e.button_args() {
			if k.state == ButtonState::Press {
				game.pressed(&k.button, 1);
			}

			if k.state == ButtonState::Release {
				game.pressed(&k.button, 0);
			}

		}

	}
}
