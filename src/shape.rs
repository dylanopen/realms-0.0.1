use sdl2::rect::Rect as SdlRect;

use crate::{window::Window, Colour, RealmsError};

pub struct Rect
{
	pub x: i32,
	pub y: i32,
	pub width: u32,
	pub height: u32,
	pub colour: Colour,
	pub sdl: SdlRect,
}

impl Rect
{
	pub fn new(x: i32, y: i32, width: u32, height: u32, colour: Colour) -> Rect
	{
		Rect {
			x: x,
			y: y,
			width: width,
			height: height,
			colour: colour,
			sdl: SdlRect::new(x, y, width, height),
		}
	}

	pub fn draw(&self, window: &mut Window) -> Result<(), RealmsError>
	{
		window.sdl.canvas.set_draw_color(self.colour.to_sdl());
		match window.sdl.canvas.fill_rect(self.sdl)
		{
			Ok(()) => Ok(()),
			Err(error_info) => Err(RealmsError::DrawRectError(error_info))
		}
	}

	pub fn draw_outline(&self, window: &mut Window) -> Result<(), RealmsError>
	{
		window.sdl.canvas.set_draw_color(self.colour.to_sdl());
		match window.sdl.canvas.draw_rect(self.sdl)
		{
			Ok(()) => Ok(()),
			Err(error_info) => Err(RealmsError::DrawRectError(error_info))
		}
	}
}

