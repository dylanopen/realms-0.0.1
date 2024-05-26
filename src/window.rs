use sdl2::rect::Rect as SdlRect;
use sdl2::render::Canvas;
use sdl2::{EventPump, Sdl, VideoSubsystem};
use sdl2::video::Window as SdlWindow;

use crate::event::Event;
use crate::Colour;
use crate::RealmsError;

pub struct SdlDisplay
{
	pub context: Sdl,
	pub video: VideoSubsystem,
	// pub window: SdlWindow,
	pub canvas: Canvas<SdlWindow>,
	pub event_pump: EventPump,
}

struct DefaultSurfaces
{
	fill_rect: SdlRect,
}

pub struct Window<'a>
{
	pub title: &'a str,
	pub width: u32,
	pub height: u32,
	pub sdl: SdlDisplay,
	default_surfaces: DefaultSurfaces,
	did_fill: bool,
}

impl Window<'_>
{
	/// You must store the Window object as a **mutable** object, or SDL functions will fail.
	/// 
	/// > (error handling needs testing)
	pub fn new(title: &str, width: u32, height: u32) -> Result<Window, RealmsError>
	{
		match sdl2::init()
		{
			Ok(sdl2_context) => match sdl2_context.video()
			{
				Ok(sdl_video_subsystem) => match sdl_video_subsystem.window(&title, width.clone(), height.clone()).build()
				{
					Ok(sdl_window) => match sdl_window.into_canvas().build()
					{
						Ok(sdl_canvas) => match sdl2_context.event_pump()
						{
							Ok(event_pump) =>
							{
								let sdl_display = SdlDisplay {
									context: sdl2_context,
									video: sdl_video_subsystem,
									// window: sdl_window,
									canvas: sdl_canvas,
									event_pump: event_pump,
								};

								Ok(Window {
									title: title,
									width: width,
									height: height,
									sdl: sdl_display,
									default_surfaces: Window::get_default_surfaces(width, height),
									did_fill: true
								})
							},
							Err(error_info) => Err(RealmsError::GetSdlEventPumpError(error_info))
						}
						Err(error_info) => Err(RealmsError::CreateSdlCanvasError(error_info.to_string()))
					},
					Err(error_info) => Err(RealmsError::BuildSdlWindowError(error_info.to_string()))
				}
				Err(error_info) => Err(RealmsError::CreateSdlVideoSubsystemError(error_info))
			}
			Err(error_info) => Err(RealmsError::CreateSdlContextError(error_info))
		}
	}

	pub fn get_events(&mut self) -> Vec<Event>
	{
		let mut events: Vec<Event> = Vec::new();

		for sdl_event in self.sdl.event_pump.poll_iter()
		{
			let event = Event::from_sdl(sdl_event);
			if matches!(event, Event::None)
				{ break }
			events.push(event);
		}

		events
	}

	pub fn fill(&mut self, colour: Colour) -> Result<(), RealmsError>
	{
		self.sdl.canvas.set_draw_color(colour.to_sdl());
		self.did_fill = true;
		match self.sdl.canvas.fill_rect(self.default_surfaces.fill_rect)
		{
			Ok(()) => Ok(()),
			Err(error_info) => Err(RealmsError::DrawRectError(error_info)),
		}
	}
	
	pub fn draw(&mut self)
	{
		if !self.did_fill
		{
			panic!("Realms error: The screen was not filled before calling `window.draw()`. Make sure you call `window.fill(colour)` at the beginning of each iteration, or the screen will not display properly.");
		}
		self.sdl.canvas.present();
		self.did_fill = false;
	}

	fn get_default_surfaces(width: u32, height: u32) -> DefaultSurfaces
	{
		DefaultSurfaces {
			fill_rect: SdlRect::new(0, 0, width, height),
		}
	}
}
