use sdl2::controller::{Axis as SdlControllerAxis, Button as SdlControllerButton};
use sdl2::event::Event as SdlEvent;
use sdl2::mouse::MouseButton as SdlMouseButton;


pub enum MouseButton
{
	Unknown,
	Left,
	Middle,
	Right,
	Side1,
	Side2,
}

impl MouseButton
{
	pub fn from_sdl(sdl_mouse_btn: SdlMouseButton) -> MouseButton
	{
		match sdl_mouse_btn
		{
			SdlMouseButton::Unknown => MouseButton::Unknown,
			SdlMouseButton::Left => MouseButton::Left,
			SdlMouseButton::Middle => MouseButton::Middle,
			SdlMouseButton::Right => MouseButton::Right,
			SdlMouseButton::X1 => MouseButton::Side1,
			SdlMouseButton::X2 => MouseButton::Side2,
		}
	}
}

pub enum ControllerButton
{
	A,
	B,
	X,
	Y,
	Back,
	DUp,
	DLeft,
	DDown,
	DRight,
	Guide,
	LShoulder,
	RShoulder,
	LStick,
	RStick,
	Misc,
	Paddle1,
	Paddle2,
	Paddle3,
	Paddle4,
	Start,
	Touchpad,
}

impl ControllerButton
{
	pub fn from_sdl(sdl_controller_btn: SdlControllerButton) -> ControllerButton
	{
		match sdl_controller_btn
		{
			SdlControllerButton::A => ControllerButton::A,
			SdlControllerButton::B => ControllerButton::B,
			SdlControllerButton::X => ControllerButton::X,
			SdlControllerButton::Y => ControllerButton::Y,
			SdlControllerButton::Back => ControllerButton::Back,
			SdlControllerButton::DPadUp => ControllerButton::DUp,
			SdlControllerButton::DPadLeft => ControllerButton::DLeft,
			SdlControllerButton::DPadDown => ControllerButton::DDown,
			SdlControllerButton::DPadRight => ControllerButton::DRight,
			SdlControllerButton::Guide => ControllerButton::Guide,
			SdlControllerButton::LeftShoulder => ControllerButton::LShoulder,
			SdlControllerButton::RightShoulder => ControllerButton::RShoulder,
			SdlControllerButton::LeftStick => ControllerButton::LStick,
			SdlControllerButton::RightStick => ControllerButton::RStick,
			SdlControllerButton::Misc1 => ControllerButton::Misc,
			SdlControllerButton::Paddle1 => ControllerButton::Paddle1,
			SdlControllerButton::Paddle2 => ControllerButton::Paddle2,
			SdlControllerButton::Paddle3 => ControllerButton::Paddle3,
			SdlControllerButton::Paddle4 => ControllerButton::Paddle4,
			SdlControllerButton::Start => ControllerButton::Start,
			SdlControllerButton::Touchpad => ControllerButton::Touchpad,
		}
	}
}

pub struct MouseMotionEvent
{
	pub x: i32,
	pub y: i32,
}

pub struct MouseClickEvent
{
	pub button: MouseButton,
	pub x: i32,
	pub y: i32,
	pub clicks: u8,
}

pub struct AudioDeviceConnectionEvent
{
	pub device: u32,
	pub is_capture: bool,
}

pub struct ControllerAxisMotionEvent
{
	pub device: u32,
	pub axis: SdlControllerAxis,
	pub value: i16,
}

pub struct ControllerButtonEvent
{
	pub device: u32,
	pub button: SdlControllerButton,
}


pub enum Event
{
	None,
	Quit,
	MouseMotion(MouseMotionEvent),
	MouseDown(MouseClickEvent),
	MouseUp(MouseClickEvent),
	EnteredBackground,
	EnteredForeground,
	LowMemory,
	Quitting,
	WillEnterBackground,
	WillEnterForeground,
	AddedAudioDevice(AudioDeviceConnectionEvent),
	RemovedAudioDevice(AudioDeviceConnectionEvent),
	ModifyClipboard,
	ControllerAxisMotion(ControllerAxisMotionEvent),
	ControllerButtonDown(ControllerButtonEvent),
	ControllerButtonUp(ControllerButtonEvent),
}

impl Event
{
	pub fn from_sdl(sdl_event: SdlEvent) -> Event
	{
		match sdl_event
			{
				SdlEvent::Quit {..} => Event::Quit,

				SdlEvent::MouseMotion {
					x, y, .. 
				} => Event::MouseMotion(MouseMotionEvent {
					x: x,
					y: y,
				}),

				SdlEvent::MouseButtonDown {
					mouse_btn, clicks, x, y, ..
				} => {
					let btn = MouseButton::from_sdl(mouse_btn);
					Event::MouseDown(MouseClickEvent {
						button: btn,
						x: x,
						y: y,
						clicks: clicks,
					})
				},

				SdlEvent::MouseButtonUp {
					mouse_btn, clicks, x, y, ..
				} => {
					let btn = MouseButton::from_sdl(mouse_btn);
					Event::MouseUp(MouseClickEvent {
						button: btn,
						x: x,
						y: y,
						clicks: clicks,
					})
				},

				SdlEvent::AppDidEnterBackground { .. } => {
					Event::EnteredBackground
				},

				SdlEvent::AppDidEnterForeground { .. } => {
					Event::EnteredForeground
				},

				SdlEvent::AppLowMemory{ .. } => {
					Event::LowMemory
				},

				SdlEvent::AppTerminating { .. } => {
					Event::Quitting
				},

				SdlEvent::AppWillEnterBackground { .. } => {
					Event::WillEnterBackground
				},

				SdlEvent::AppWillEnterForeground { .. } => {
					Event::WillEnterForeground
				},

				SdlEvent::AudioDeviceAdded { which, iscapture, .. } => {
					Event::AddedAudioDevice(AudioDeviceConnectionEvent {
						device: which,
						is_capture: iscapture,
					})
				},

				SdlEvent::AudioDeviceRemoved { which, iscapture, .. } => {
					Event::RemovedAudioDevice(AudioDeviceConnectionEvent {
						device: which,
						is_capture: iscapture,
					})
				},

				SdlEvent::ClipboardUpdate { .. } => {
					Event::ModifyClipboard
				},

				SdlEvent::ControllerAxisMotion { which, axis, value, .. } => {
					Event::ControllerAxisMotion(ControllerAxisMotionEvent {
						device: which,
						axis: axis,
						value: value,
					})
				},

				SdlEvent::ControllerButtonDown { which, button, .. } => {
					Event::ControllerButtonDown(ControllerButtonEvent {
						device: which,
						button: button,
					})
				},

				SdlEvent::ControllerButtonUp { which, button, .. } => {
					Event::ControllerButtonUp(ControllerButtonEvent {
						device: which,
						button: button
					})
				},

				_ => Event::None
			}
	}
}
