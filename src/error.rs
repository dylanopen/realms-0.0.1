use std::fmt;

pub enum RealmsError
{
	CreateSdlContextError(String),
	CreateSdlVideoSubsystemError(String),
	BuildSdlWindowError(String),
	CreateSdlCanvasError(String),
	GetSdlEventPumpError(String),
	DrawRectError(String),
	LoadTextureError(String),
	DrawSpriteError(String),
}

impl RealmsError
{
	pub fn get_error_info(&self) -> &String
	{
		match self
		{
			RealmsError::CreateSdlContextError(info) => info,
			RealmsError::CreateSdlVideoSubsystemError(info) => info,
			RealmsError::BuildSdlWindowError(info) => info,
			RealmsError::CreateSdlCanvasError(info) => info,
			RealmsError::GetSdlEventPumpError(info) => info,
			RealmsError::DrawRectError(info) => info,
			RealmsError::LoadTextureError(info) => info,
			RealmsError::DrawSpriteError(info) => info,
		}
	}
}

impl fmt::Debug for RealmsError
{
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result
	{
		write!(f, "{}", self.get_error_info())
	}
}

