use sdl2::image::LoadTexture;
use sdl2::render::Texture as SdlTexture;
use sdl2::render::TextureCreator as SdlTextureCreator;
use sdl2::video::WindowContext as SdlWindowContext;

use crate::window::Window;
use crate::RealmsError;

pub struct Texture<'a>
{
	pub sdl_texture: SdlTexture<'a>,
	pub width: u32,
	pub height: u32,
}

impl<'a> Texture<'a>
{
	pub fn new(filepath: &str, texture_creator: &'a SdlTextureCreator<SdlWindowContext>) -> Result<Texture<'a>, RealmsError>
	{
		match texture_creator.load_texture(filepath)
		{
			Ok(sdl_texture) => Ok({
				let image_info = &sdl_texture.query();
				Texture {
					sdl_texture: sdl_texture,
					width: image_info.width,
					height: image_info.height,
				}
			}),
			Err(error_info) => Err(RealmsError::LoadTextureError(error_info))
		}
	}
}

pub struct Sprite<'a>
{
	pub texture: &'a Texture<'a>,
	pub x: i32,
	pub y: i32,
}

impl<'a> Sprite<'a>
{
	pub fn new(texture: &'a Texture, x: i32, y: i32) -> Sprite<'a>
	{
		Sprite {
			texture: texture,
			x: x,
			y: y
		}
	}

	pub fn draw(&mut self, window: &mut Window) -> Result<(), RealmsError>
	{
		let src = sdl2::rect::Rect::new(0, 0, self.texture.width, self.texture.height);
		let dest = sdl2::rect::Rect::new(self.x, self.y, self.texture.width, self.texture.height);

		match window.sdl.canvas.copy(&self.texture.sdl_texture, src, dest)
		{
			Ok(()) => Ok(()),
			Err(error_info) => Err(RealmsError::DrawSpriteError(error_info))
		}
	}
}
