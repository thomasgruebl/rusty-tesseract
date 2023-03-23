use thiserror::Error;

#[derive(Error, Debug)]
pub enum TessError {
    #[error("Tesseract not found. Please check installation path!")]
    TesseractNotFoundError,
    #[error("Invalid tesseract version!\n{0}")]
    VersionError(String),
    #[error(
        "Image format not within the list of allowed image formats:\n\
    ['JPEG','JPG','PNG','PBM','PGM','PPM','TIFF','BMP','GIF','WEBP']"
    )]
    ImageFormatError,
    #[error(
        "Both the image path and the ndarray of your Image object are empty.\n\
    Please assign an image path or pass an ndarray."
    )]
    ImageNotFoundError,
    #[error("Data could not be parsed.")]
    ParseError,
}

pub type TessResult<T> = Result<T, TessError>;
