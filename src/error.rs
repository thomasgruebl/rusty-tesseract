use std::fmt;

#[derive(Debug, Clone)]
pub struct VersionError;

#[derive(Debug, Clone)]
pub struct TesseractNotFoundError;

#[derive(Debug, Clone)]
pub struct ImageFormatError;

#[derive(Debug, Clone)]
pub struct ImageNotFoundError;

impl fmt::Display for VersionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid tesseract version!")
    }
}

impl fmt::Display for TesseractNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Tesseract not found. Please check installation path!")
    }
}

impl fmt::Display for ImageFormatError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Image format not within the list of allowed image formats: 
        ['JPEG',
        'JPG',
        'PNG',
        'PBM',
        'PGM',
        'PPM',
        'TIFF',
        'BMP',
        'GIF',
        'WEBP]")
    }
}

impl fmt::Display for ImageNotFoundError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Both the image path and the ndarray of your Image object are empty.
        Please assign an image path or pass an ndarray.")
    }
}


