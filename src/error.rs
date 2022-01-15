use std::fmt;

#[derive(Debug, Clone)]
pub struct VersionError;

#[derive(Debug, Clone)]
pub struct TesseractNotFoundError;

#[derive(Debug, Clone)]
pub struct ImageFormatError;

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


