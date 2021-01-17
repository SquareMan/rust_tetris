use super::field::{FIELD_HEIGHT, FIELD_WIDTH};
use rltk::{Rltk, RGB};

pub const FIELD_X: usize = 80 / 2 - FIELD_WIDTH / 2;
pub const FIELD_Y: usize = 50 / 2 - FIELD_HEIGHT / 2;

pub fn draw_gui(ctx: &mut Rltk) {
    ctx.draw_box(
        FIELD_X - 1,
        FIELD_Y - 1,
        FIELD_WIDTH + 1,
        FIELD_HEIGHT + 1,
        RGB::named(rltk::WHITE),
        RGB::named(rltk::BLACK),
    );
}
