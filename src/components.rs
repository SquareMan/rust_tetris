use rltk::RGB;
use specs::prelude::*;
use specs_derive::*;

#[derive(Copy, Clone, Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Block {
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Held {}

#[derive(Component, Debug)]
pub struct Falling {}

#[derive(Component, Debug)]
pub struct Center {}
