use egui::{include_image, ImageSource};
use strum::{Display, EnumIter};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Default, EnumIter, Display)]
pub enum ColorGradient {
    Cividis,
    Cool,
    #[strum(to_string = "Cube Helix")]
    CubeHelix,
    Inferno,
    Magma,
    Plasma,
    #[default]
    Turbo,
    Viridis,
    Warm,
}

impl ColorGradient {
    pub const fn gradient(&self) -> colorous::Gradient {
        match self {
            Self::Cividis => colorous::CIVIDIS,
            Self::Cool => colorous::COOL,
            Self::CubeHelix => colorous::CUBEHELIX,
            Self::Inferno => colorous::INFERNO,
            Self::Magma => colorous::MAGMA,
            Self::Plasma => colorous::PLASMA,
            Self::Turbo => colorous::TURBO,
            Self::Viridis => colorous::VIRIDIS,
            Self::Warm => colorous::WARM,
        }
    }

    pub const fn preview_image(&self) -> ImageSource<'_> {
        match self {
            ColorGradient::Cividis => include_image!("../../assets/cividis.png"),
            ColorGradient::Cool => include_image!("../../assets/cool.png"),
            ColorGradient::CubeHelix => {
                include_image!("../../assets/cubehelix.png")
            }
            ColorGradient::Inferno => include_image!("../../assets/inferno.png"),
            ColorGradient::Magma => include_image!("../../assets/magma.png"),
            ColorGradient::Plasma => include_image!("../../assets/plasma.png"),
            ColorGradient::Turbo => include_image!("../../assets/turbo.png"),
            ColorGradient::Viridis => include_image!("../../assets/viridis.png"),
            ColorGradient::Warm => include_image!("../../assets/warm.png"),
        }
    }
}
