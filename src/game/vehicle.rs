use game::collisions::Rectangle;
use game::graphics;

use game::units;

pub trait Vehicle {
	fn damage_rectangle(&self) -> Rectangle;
	fn update(&mut self, elapsed_time: units::Millis);
	fn update_for_cinematic(&mut self);
	fn draw(&self, display: &mut graphics::Graphics);
	fn add_part(&mut self, part_num: u32);
	fn is_built(&self) -> bool;
	fn get_x(&self) -> units::Game;
	fn get_y(&self) -> units::Game;
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
	fn get_type(&self) -> i32;
}

pub trait Part {
	fn draw(&self, display: &mut graphics::Graphics);
	fn damage_rectangle(&self) -> Rectangle;
	fn part_type(&self) -> u32;
	fn get_x(&self) -> units::Game;
	fn get_y(&self) -> units::Game;
	fn get_map_x(&self) -> units::Game;
	fn get_map_y(&self) -> units::Game;
}