use sdl2::pixels::Color as SdlColor;

#[derive(Clone)]
pub struct Colour
{
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Colour
{
	pub fn new() -> Colour
	{
		Colour {
			r: 0,
			g: 0,
			b: 0,
		}
	}

	pub fn from_rgb(red: u8, green: u8, blue: u8) -> Colour
	{
		Colour {
			r: red,
			g: green,
			b: blue,
		}
	}

	pub fn to_sdl(&self) -> SdlColor
	{
		SdlColor::RGB(self.r, self.g, self.b)
	}
}
