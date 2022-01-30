mod rusty_tesseract;
use rusty_tesseract::{Image, Args};
use ndarray::Array3;
use std::collections::HashMap;

mod error;


// the main function provides usage samples of the rusty-tesseract wrapper
fn main() {

    // create an Image object by specifying a path or alternatively an image array in (height, width, channel) format
    // if path is an empty string -> rusty-tesseract tries to use the ndarray

    // you can use the new function
    let mut img = Image::new(
        String::from("/home/thomas/Documents/PyCharmProjects/tesseract_test/test.png"),
        Array3::<u8>::zeros((100, 100, 3))
    );

    // or instantiate Image struct directly
    let mut img = Image {
        path: String::from("/home/thomas/Documents/PyCharmProjects/tesseract_test/test.png"),
        ndarray: Array3::<u8>::zeros((100, 100, 3))  // example: creates an 100x100 pixel image with 3 colour channels (RGB)
    };

    // use default_args to call a function if no particular config is needed
    let default_args = Args::new();
    
    let tesseract_version = rusty_tesseract::get_tesseract_version();
    println!("The tesseract version is: {:?}", tesseract_version);

    // fill your own argument struct if needed
    let mut image_to_string_args = Args {
        out_filename: "out",
        lang: "eng",
        config: HashMap::new(),
        timeout: 100,
        dpi: 150,
        boxfile: false
    };
    image_to_string_args.config.insert("psm", "6");
    image_to_string_args.config.insert("oem", "3");

    let output = rusty_tesseract::image_to_string(&img, image_to_string_args);
    println!("The String output is: {:?}", output.Output_STRING);

    let mut image_to_boxes_args = Args {
        out_filename: "font_name.font.exp0",
        lang: "eng",
        config: HashMap::new(),
        timeout: 100,
        dpi: 150,
        boxfile: true
    };
    image_to_boxes_args.config.insert("psm", "6");
    image_to_boxes_args.config.insert("oem", "3");

    
    // boxes printed in OUTPUT_DICT or OUTPUT_DATAFRAME format store the Key as a string (i.e. the character) and 
    // store the value as a list of strings (if the same character appears more than once)
    let boxes = rusty_tesseract::image_to_boxes(&img, image_to_boxes_args);
    println!("The Boxfile output is: {:?}", boxes.OUTPUT_DATAFRAME);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(0, 0);
        assert_eq!(0, 0);
        assert_eq!(0, 0);
        assert_eq!(0, 0);
    }

    #[test]
    fn vertical_text() {
        let mut img = Image::new(
            String::from("img/vertical_text.png"),
            Array3::<u8>::zeros((100, 100, 3))
        );

        let mut image_to_string_args = Args {
            out_filename: "out",
            lang: "eng",
            config: HashMap::new(),
            timeout: 100,
            dpi: 150,
            boxfile: false
        };

        image_to_string_args.config.insert("psm", "6");
        image_to_string_args.config.insert("oem", "3");
    
        let output_test = rusty_tesseract::image_to_string(&img, image_to_string_args);
        assert_eq!(output_test.Output_STRING, "D\nO\nL\nO\nR\nS\nI\n\nT\n\u{c}");
    }

    #[test]
    fn horizontal_text() {
        let mut img = Image::new(
            String::from("img/horizontal_text.png"),
            Array3::<u8>::zeros((100, 100, 3))
        );
        let default_args = Args::new();
        let output_test = rusty_tesseract::image_to_string(&img, default_args);
        assert_eq!(output_test.Output_STRING, "Lorem ipsum dolor sit amet\n\u{c}");
    }

    #[test]
    fn image_to_data() {

    }

    #[test]
    fn image_to_string() {

    }

    #[test]
    fn image_to_boxes() {

    }
}
