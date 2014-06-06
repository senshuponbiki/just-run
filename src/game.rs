use std::cmp;
use std::io::Timer;
use std::vec::Vec;
use rand::{task_rng, Rng};

use game::units::{AsGame};

use sdl2::sdl;
use sdl2::event;
use sdl2::keycode;

pub mod backdrop;
pub mod collisions;
pub mod common;
pub mod enemies;
pub mod graphics;
pub mod input;
pub mod map;
pub mod player;
pub mod sprite;
pub mod units;
pub mod goal;

static TARGET_FRAMERATE: units::Fps  =  60;
static MAX_FRAME_TIME: units::Millis =  units::Millis(5 * (1000 / TARGET_FRAMERATE) as int);

pub static SCREEN_WIDTH:  units::Tile = units::Tile(20);
pub static SCREEN_HEIGHT: units::Tile = units::Tile(20);

pub static POSSIBLE_CHARACTER_TILES: uint = 18;
pub static POSSIBLE_GOAL_TILES:		 uint = 17;

/// An instance of the `rust-story` game with its own event loop.
pub struct Game {
	player:  player::Player,
	enemies: Vec<~enemies::Zombie>,
	goal:    goal::Goal,
	map:     map::Map,

	display:     graphics::Graphics,
	controller:  input::Input,
	paused:      bool,
	updates:     int,
	level:       int, 
}

impl Game {
	/// Starts running this games event loop, note that this will block indefinitely.
	/// This function will return to the caller when the escape key is pressed.
	pub fn new() -> Game {
		println!("initalizing sdl ...");
		
		// initialize all major subsystems
		// hide the mouse cursor in our drawing context
		sdl::init([sdl::InitEverything]);
		let mut display = graphics::Graphics::new();
		let controller  = input::Input::new();
		let mut rng = task_rng();
		let enemies_vector: Vec<~enemies::Zombie> = Vec::new();

		let mut game = Game {
				map: map::Map::create_test_map(&mut display),
				player: player::Player::new(
					&mut display,
					(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
					(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
				),

				enemies: enemies_vector,

				goal: goal::Goal::new(
					&mut display, 
					(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
					(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
				),

				display:     display,
				controller:  controller, 
				paused:      true,
				updates:     0,
				level:       0,
			};
		game.spawn_zombie(2);

		game
	}

	pub fn spawn_zombie(&mut self, kind: int) {
		let mut rng = task_rng();
		match kind {
			1 => {
				let zombie = ~enemies::SlowZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as ~enemies::Zombie);
			}
			2 => {
				let zombie = ~enemies::CrazyZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as ~enemies::Zombie);
			}
			3 => {
				let zombie = ~enemies::RandomZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as ~enemies::Zombie);
			}
			_ => {
				let zombie = ~enemies::CloudZombie::new(
								&mut self.display, 
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
								(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
							);
				self.enemies.push(zombie as ~enemies::Zombie);
			}
		};
	}

	pub fn start(&mut self) {
		self.event_loop();
		sdl::quit();
	}

	pub fn restart(&mut self) {
		println!("Restarting game...");

		let mut rng = task_rng();
		let enemies_vector: Vec<~enemies::Zombie> = Vec::new();

		self.player = player::Player::new(
				&mut self.display,
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(),
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game()
			);

		self.enemies = enemies_vector;
		self.spawn_zombie(2);

		self.goal = goal::Goal::new(
				&mut self.display, 
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
				(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
			);

		self.paused = true;
		self.updates = 0;
		self.level = 0;
	}

	/// Polls current input events & dispatches them to the engine.
	///
	/// Then renders a snapshot of the world-state and then waits
	/// until its next frame deadline.
	fn event_loop(&mut self) {
		// event loop control
		let frame_delay          = units::Millis(1000 / TARGET_FRAMERATE as int);
		let mut last_update_time = units::Millis(sdl::get_ticks() as int);
		
		let mut running = true;
		let mut timer   = Timer::new().unwrap();
		
		while running {
			let start_time_ms = units::Millis(sdl::get_ticks() as int);
			self.controller.begin_new_frame();

			// drain event queue once per frame
			// ideally should do in separate task
			match event::poll_event() {
				event::KeyDownEvent(_,_,key_cap,_,_) => {
					self.controller.key_down_event(key_cap);
				},
				event::KeyUpEvent(_,_,key_cap,_,_) => {
					self.controller.key_up_event(key_cap);
				},
				_ => {},
			}

			// Handle exit game
			if self.controller.was_key_released(keycode::EscapeKey) {
				running = false;
			}

			// Handle paused game
			if self.controller.was_key_released(keycode::ReturnKey) {
				if self.paused {
					self.paused = false;
				} else {
					self.paused = true;
				}
			}

			// Handle player movement
			if self.controller.is_key_held(keycode::LeftKey)
				&& self.controller.is_key_held(keycode::RightKey) {

				self.player.stop_moving_horizontally();
			} else if self.controller.is_key_held(keycode::LeftKey) {
				self.player.start_moving_left();
			} else if self.controller.is_key_held(keycode::RightKey) {
				self.player.start_moving_right();
			} else {
				self.player.stop_moving_horizontally();
			}

			// Handle player looking
			if self.controller.is_key_held(keycode::UpKey)
				&& self.controller.is_key_held(keycode::DownKey) {

				self.player.stop_moving_vertically();
			} else if self.controller.is_key_held(keycode::UpKey) {
				self.player.start_moving_up();
			} else if self.controller.is_key_held(keycode::DownKey) {
				self.player.start_moving_down();
			} else {
				self.player.stop_moving_vertically();
			}

			// inform actors of how much time has passed since last frame
			let current_time_ms = units::Millis(sdl::get_ticks() as int);
			let elapsed_time    = current_time_ms - last_update_time;
		
			// only update if not in paused state, or when drawing the initial frame
			if !self.paused || self.updates == 0 {
				self.update(cmp::min(elapsed_time, MAX_FRAME_TIME));
				last_update_time = current_time_ms;

				// draw
				self.display.clear_buffer(); // clear back-buffer
				self.draw();
				self.display.switch_buffers();
			}

			// throttle event-loop based on iteration time vs frame deadline
			let iter_time = units::Millis(sdl::get_ticks() as int) - start_time_ms;
			let next_frame_time: u64 = if frame_delay > iter_time { 
				let (units::Millis(fd), units::Millis(it)) = (frame_delay, iter_time);
				(fd - it) as u64
			} else { 0 as u64 };
			
			self.updates = self.updates + 1;

			timer.sleep(next_frame_time);

			/* Print current FPS to stdout
			let units::Millis(start_time) = start_time_ms;
			let seconds_per_frame =  (sdl::get_ticks() as int - start_time) as f64 / 1000.0;
			let fps = 1.0 / (seconds_per_frame);

			println!("fps: {}", fps);
			*/
			
		}

	}

	/// Instructs our actors to draw their current state to the screen.
	fn draw(&self) {
		// background
		self.map.draw_background(&self.display);
		self.map.draw_sprites(&self.display);

		// foreground
		self.goal.draw(&self.display);
		self.player.character.draw(&self.display);
		for enemy in self.enemies.iter() { enemy.draw(&self.display); }
		self.map.draw(&self.display);
	}

	/// Passes the current time in milliseconds to our underlying actors.
	fn update(&mut self, elapsed_time: units::Millis) {
		self.map.update(elapsed_time);
		self.player.update(elapsed_time, &self.map);
		for i in range(0u, self.enemies.len()) { 
			let enemy = self.enemies.get_mut(i);
			enemy.set_acceleration(self.player.character.center_x(), self.player.character.center_y()); 
			enemy.update(elapsed_time, &self.map); 
		}
		self.goal.update(elapsed_time);

		let mut collidedWithZombie = false;
		for enemy in self.enemies.iter() { 
			if enemy.damage_rectangle().collides_with_player(&self.player.character.damage_rectangle()) {
			 	collidedWithZombie = true;
			 	break;
			 }
		}
		let enteredGoal = self.goal.damage_rectangle().collides_with(&self.player.character.damage_rectangle());

		if enteredGoal {
			let mut rng = task_rng();
			self.goal = goal::Goal::new(
				&mut self.display, 
				(units::Tile(rng.gen_range(1u, POSSIBLE_CHARACTER_TILES))).to_game(), 
				(units::Tile(rng.gen_range(1u, POSSIBLE_GOAL_TILES))).to_game()
			);
			self.spawn_zombie(2);
			self.level = self.level + 1;
		}
		if collidedWithZombie {
			// print progress and start a new game
			println!("Game Over! You made it to level {}", self.level);
			self.restart();
		}
	}
}
