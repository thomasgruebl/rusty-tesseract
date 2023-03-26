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
    #[error("Please assign a valid image path.")]
    ImageNotFoundError,
    #[error("Data could not be parsed.")]
    ParseError,
    #[error("Could not create tempfile.\n{0}")]
    TempfileError(String),
    #[error("Could not save dynamic image to tempfile.\n{0}")]
    DynamicImageError(String),
}

pub type TessResult<T> = Result<T, TessError>;
