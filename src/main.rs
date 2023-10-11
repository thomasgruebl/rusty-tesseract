use std::collections::HashMap;

use image::io::Reader as ImageReader;
use rusty_tesseract::{Args, Image};
/// Refer to https://github.com/thomasgruebl/rusty-tesseract

// the main function provides usage samples of the rusty-tesseract wrapper
fn main() {
    // create an Image object by specifying a path or alternatively a DynamicImage from the image crate https://docs.rs/image/latest/image/

    // you can use the from_path function
    let test_image = Image::from_path("img/string.png").unwrap();
    println!("test image is stored at: {}", test_image.get_image_path().unwrap());


    // or instantiate Image from a DynamicImage
    let dynamic_image = ImageReader::open("img/string.png")
        .unwrap()
        .decode()
        .unwrap();
    let img = Image::from_dynamic_image(&dynamic_image).unwrap();

    println!("temp image is stored at: {}", img.get_image_path().unwrap());

    // use default_args to call a function if no particular config is needed
    let default_args = Args::default();

    let tesseract_version = rusty_tesseract::get_tesseract_version().unwrap();
    println!("The tesseract version is: {}", tesseract_version);

    let tesseract_langs = rusty_tesseract::get_tesseract_langs().unwrap();
    println!("The available languages are: {:?}", tesseract_langs);

    // fill your own argument struct if needed
    let image_to_string_args = Args {
        lang: "eng".into(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };

    //you can get the list of available config variables with:
    let parameters = rusty_tesseract::get_tesseract_config_parameters().unwrap();
    println!(
        "Example config variable: {}",
        parameters.config_parameters.first().unwrap(),
    );

    let output = rusty_tesseract::image_to_string(&img, &image_to_string_args).unwrap();
    println!("\nThe String output is: {}", output);

    // image_to_boxes creates a BoxOutput containing the parsed output from Tesseract when using the "makebox" Parameter
    let box_output = rusty_tesseract::image_to_boxes(&img, &default_args).unwrap();
    println!(
        "The first boxfile symbol is: {}",
        box_output.boxes[0].symbol
    );
    println!("The full boxfile output is:\n{}", box_output.output);

    // image_to_data creates a DataOutput containing the parsed output from Tesseract when using the "TSV" Parameter
    let data_output = rusty_tesseract::image_to_data(&img, &default_args).unwrap();
    let first_text_line = &data_output.data[4];
    println!(
        "The first text is '{}' with confidence {}",
        first_text_line.text, first_text_line.conf
    );
    println!("The full data output is:\n{}", data_output.output);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_text() {
        let img = Image::from_path("img/vertical_text.png").unwrap();

        let image_to_string_args = Args {
            psm: Some(6),
            ..Default::default()
        };

        let output = rusty_tesseract::image_to_string(&img, &image_to_string_args).unwrap();
        assert_eq!(
            output.lines().collect::<Vec<&str>>(),
            vec!["D", "O", "L", "O", "R", "S", "I", "", "T"]
        );
    }

    #[test]
    fn horizontal_text() {
        let img = Image::from_path("img/horizontal_text.png").unwrap();
        let default_args = Args::default();
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();
        assert_eq!(output.trim(), "Lorem ipsum dolor sit amet");
    }

    #[test]
    fn image_to_string() {
        let img = Image::from_path("img/string.png").unwrap();
        let default_args = Args::default();
        let output = rusty_tesseract::image_to_string(&img, &default_args).unwrap();
        assert_eq!(output.trim(), "LOREM IPSUM DOLOR SIT AMET");
    }

    #[test]
    fn command_without_options() {
        let img = Image::from_path("img/string.png").unwrap();
        let args = Args {
            dpi: None,
            psm: None,
            oem: None,
            ..Args::default()
        };

        let output = rusty_tesseract::image_to_string(&img, &args).unwrap();
        assert_eq!(output.trim(), "LOREM IPSUM DOLOR SIT AMET");
    }

    #[test]
    fn command_with_partial_options() {
        let img = Image::from_path("img/string.png").unwrap();
        let args = Args {
            dpi: Some(300),
            psm: None,
            oem: None,
            ..Args::default()
        };

        let output = rusty_tesseract::image_to_string(&img, &args).unwrap();
        assert_eq!(output.trim(), "LOREM IPSUM DOLOR SIT AMET");
    }
}
