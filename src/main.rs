#![crate_id="rust-story#0.0.1"]


#![feature(macro_rules)]
extern crate sdl2;
extern crate sdl2_mixer;
extern crate sdl2_ttf;
extern crate collections;

pub mod game;

pub fn main() {
	let mut story = game::Game::new();
	story.start();
}
