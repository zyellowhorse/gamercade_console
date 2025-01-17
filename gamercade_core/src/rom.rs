use serde::{Deserialize, Serialize};

use gamercade_audio::SoundRom;

use super::graphics::{FrameRate, GraphicsData, Resolution};
use crate::{ColorIndex, PaletteIndex, PixelBuffer, BYTES_PER_PIXEL};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rom {
    pub resolution: Resolution,
    pub frame_rate: FrameRate,
    pub player_count: (usize, usize),
    pub graphics: GraphicsData,
    pub sounds: SoundRom,
    pub code: Box<[u8]>,
}

impl Default for Rom {
    fn default() -> Self {
        Self {
            resolution: Default::default(),
            frame_rate: Default::default(),
            graphics: Default::default(),
            sounds: Default::default(),
            code: Default::default(),
            player_count: (1, 1),
        }
    }
}

impl Rom {
    pub fn clear_buffer(&self, color: ColorIndex, palette: PaletteIndex, target: &mut PixelBuffer) {
        let color = if let Some(Some(color)) = self
            .graphics
            .palette(palette)
            .map(|palette| palette.colors.get(color.0 as usize))
        {
            color.into_pixel_data()
        } else {
            return;
        };
        target
            .pixel_buffer
            .chunks_exact_mut(BYTES_PER_PIXEL)
            .for_each(|pixel| pixel.copy_from_slice(&color));
    }

    pub fn height(&self) -> i32 {
        self.resolution.height()
    }

    pub fn width(&self) -> i32 {
        self.resolution.width()
    }
}
