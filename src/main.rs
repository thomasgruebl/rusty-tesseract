mod rusty_tesseract;
use rusty_tesseract::{Image, Args};
use ndarray::Array3;
use std::collections::HashMap;

mod error;


// the main function provides usage samples of the rusty-tesseract wrapper
fn main() {
    let mut img = Image {
        path: String::from("default"),
        ndarray: Array3::<i32>::zeros((1, 1, 1))
    };

    img.path = String::from("/home/thomas/Documents/PyCharmProjects/tesseract_test/test.png");

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
