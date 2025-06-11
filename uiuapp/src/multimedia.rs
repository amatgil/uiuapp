use crate::*;
use dioxus_logger::tracing::*;

use ScrollbackOutput as O;
const MIN_AUTO_IMAGE_DIM: usize = 30;
const AUDIO_SAMPLE_RATE: u32 = 44100; // TODO: This should be in the Settings menu

impl ScrollbackOutput {
    pub fn from_uiuavalue(value: uiua::Value) -> Self {
        use uiua::media::*;
        use uiua::Value;

        // Gif
        if let Ok(gif) = value_to_gif_bytes(&value, 16.0) {
            match &*value.shape {
                &[f, h, w] | &[f, h, w, _]
                    if h >= MIN_AUTO_IMAGE_DIM && w >= MIN_AUTO_IMAGE_DIM && f >= 5 =>
                {
                    trace!("Turning gif into bytes");
                    return O::Gif(gif);
                }
                _ => {}
            }
        }
        // Image?
        if let Ok(image) = value_to_image(&value) {
            if image.width() >= MIN_AUTO_IMAGE_DIM as u32
                && image.height() >= MIN_AUTO_IMAGE_DIM as u32
            {
                if let Ok(bytes) = image_to_bytes(&image, image::ImageFormat::Png) {
                    trace!("Turning image into bytes");
                    return O::Image(bytes);
                }
            }
        }
        // Audio?
        if value.row_count() as u32 >= AUDIO_SAMPLE_RATE / 4
            && matches!(&value, Value::Num(arr) if arr.elements().all(|x| x.abs() <= 5.0))
        {
            if let Ok(bytes) = value_to_wav_bytes(&value, AUDIO_SAMPLE_RATE) {
                return Self::Audio(bytes);
            }
        }

        O::Text(value.show())
    }
}
