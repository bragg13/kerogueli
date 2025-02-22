use rltk::RGB;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Clone, PartialEq, Copy)]
pub enum RunState {
    Paused,
    Running,
}

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: rltk::FontCharType, // would like to have a texture here at some point
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Monster {
    pub probability: i32,
}
