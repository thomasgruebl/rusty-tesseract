use image::RgbImage;
use ndarray::Array3;

use std::fmt;

#[derive(Clone)]
pub struct Args {
    pub lang: &'static str,
    pub dpi: i32,
    pub psm: i32,
    pub oem: i32,
}

impl Default for Args {
    fn default() -> Self {
        Args {
            lang: "eng",
            dpi: 150,
            psm: 3,
            oem: 3,
        }
    }
}

#[derive(Clone)]
pub struct Image {
    pub path: String,
    pub ndarray: Array3<u8>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Image {
    pub fn new(path: String, ndarray: Array3<u8>) -> Image {
        Image { path, ndarray }
    }

    pub(crate) fn is_empty_ndarray(&self) -> bool {
        let mut is_empty: bool = true;
        for _elem in &self.ndarray {
            is_empty = false;
        }
        return is_empty;
    }

    pub(crate) fn size_of_ndarray(&self) -> (usize, usize, usize) {
        return self.ndarray.dim();
    }

    pub(crate) fn ndarray_to_image(self) -> RgbImage {
        let (height, width, _) = self.size_of_ndarray();
        let raw = self.ndarray.into_raw_vec();

        RgbImage::from_raw(width as u32, height as u32, raw)
            .expect("Couldnt convert ndarray to RgbImage.")
    }
}
