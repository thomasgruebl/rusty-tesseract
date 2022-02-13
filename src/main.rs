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
        String::from("img/string.png"),
        Array3::<u8>::zeros((100, 100, 3))
    );

    // or instantiate Image struct directly
    let mut img = Image {
        path: String::from("img/string.png"),
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
        dpi: 150,
        boxfile: false
    };
    image_to_string_args.config.insert("psm", "6");
    image_to_string_args.config.insert("oem", "3");

    let output = rusty_tesseract::image_to_string(&img, image_to_string_args);
    println!("\nThe String output is: {:?}", output.Output_STRING);

    let mut image_to_boxes_args = Args {
        out_filename: "font_name.font.exp0",
        lang: "eng",
        config: HashMap::new(),
        dpi: 150,
        boxfile: true
    };
    image_to_boxes_args.config.insert("psm", "6");
    image_to_boxes_args.config.insert("oem", "3");

    
    // boxes printed in OUTPUT_DICT or OUTPUT_DATAFRAME format store the Key as a string (i.e. the character) and 
    // store the value as a list of strings (if the same character occurs more than once)
    let boxes = rusty_tesseract::image_to_boxes(&img, image_to_boxes_args);
    println!("\nThe Boxfile output is: {:?}", boxes.Output_DATAFRAME);


    // image_to_data prints out both image_to_string and image_to_boxes information + a creates a TSV table with confidences
    let data = rusty_tesseract::image_to_data(&img, default_args);
    println!("\nThe data output is: {:?}", data.Output_DICT);
    println!("\nThe confidence tesstable can be found in the [out_filename].tsv file!\n");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vertical_text() {
        let mut img = Image::new(
            String::from("img/vertical_text.png"),
            Array3::<u8>::zeros((0, 0, 3))
        );

        let mut image_to_string_args = Args {
            out_filename: "out",
            lang: "eng",
            config: HashMap::new(),
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
            Array3::<u8>::zeros((0, 0, 3))
        );
        let default_args = Args::new();
        let output_test = rusty_tesseract::image_to_string(&img, default_args);
        assert_eq!(output_test.Output_STRING, "Lorem ipsum dolor sit amet\n\u{c}");
    }

    #[test]
    fn image_to_data() {
        let mut img = Image::new(
            String::from("img/string.png"),
            Array3::<u8>::zeros((0, 0, 3))
        );
        let default_args = Args::new();
        let output_test = rusty_tesseract::image_to_data(&img, default_args);
        assert_eq!(output_test.Output_STRING, "LOREM IPSUM DOLOR SIT AMET\n\u{c}");
    }

    #[test]
    fn image_to_string() {
        let mut img = Image::new(
            String::from("img/string.png"),
            Array3::<u8>::zeros((0, 0, 3))
        );
        let default_args = Args::new();
        let output_test = rusty_tesseract::image_to_string(&img, default_args);
        assert_eq!(output_test.Output_STRING, "LOREM IPSUM DOLOR SIT AMET\n\u{c}");
    }

    #[test]
    fn image_to_boxes() {
        let mut img = Image::new(
            String::from("img/string.png"),
            Array3::<u8>::zeros((0, 0, 3))
        );
        let mut image_to_boxes_args = Args {
            out_filename: "eng.testcase.exp0",
            lang: "eng",
            config: HashMap::new(),
            dpi: 150,
            boxfile: true
        };
        image_to_boxes_args.config.insert("psm", "6");
        image_to_boxes_args.config.insert("oem", "3");

        let boxes_test = rusty_tesseract::image_to_boxes(&img, image_to_boxes_args);
        assert_eq!(boxes_test.Output_STRING, "L 18 26 36 59 0\nO 35 25 70 60 0\nR 75 26 98 59 0\nE 103 26 122 59 0\nM 127 26 162 59 0\nI 181 26 185 59 0\nP 191 26 214 59 0\nS 216 25 237 60 0\nU 240 25 263 59 0\nM 269 26 304 59 0\nD 323 26 352 59 0\nO 355 25 390 60 0\nL 360 25 415 60 0\nO 395 26 413 59 0\nR 413 25 476 60 0\nS 490 25 509 60 0\nI 490 25 518 60 0\nT 521 26 540 59 0\nA 553 26 586 59 0\nM 589 26 624 59 0\nE 603 26 649 59 0\nT 630 26 671 59 0\n");
    }
}
