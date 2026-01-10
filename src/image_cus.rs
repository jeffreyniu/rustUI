use std::error::Error;
use image::{ImageBuffer, RgbImage};

pub fn save_image_as_png(data: Vec<u8>, width: u32, height: u32, path: &str) -> Result<(), Box<dyn Error>> {
    
    let image: Option<RgbImage> = ImageBuffer::from_raw(width, height, data);

    match image {
        Some(img) => {
            img.save(path)?;
        }
        None => {
            return Err("Failed to create image buffer".into());
        }
    }

    Ok(())
}
