# rusty-tesseract
A Rust wrapper for Google Tesseract

![GitHub last commit](https://img.shields.io/github/last-commit/thomasgruebl/rusty-tesseract?style=plastic) ![GitHub](https://img.shields.io/github/license/thomasgruebl/phone-scraper?style=plastic) <a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/stargazers">
<img src="https://img.shields.io/github/stars/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Stars">
</a>
<a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/fork">
<img src="https://img.shields.io/github/forks/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Forks">
</a>
![Github All Releases](https://img.shields.io/github/downloads/thomasgruebl/rusty-tesseract/total.svg?style=plastic)
<a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/issues">
<img src="https://img.shields.io/github/issues/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Issues">
</a>

## Description
- Brings all relevant command-line tesseract functionality to Rust
- Based on the Python wrapper for tesseract (i.e. https://github.com/madmaze/pytesseract)
- Enables testing a pre-trained tesseract model and outputting the results in different formats such as strings, bounding boxes, dicts, or dataframes.

## Dependencies
Tesseract: https://github.com/tesseract-ocr/tesseract

## Usage
### 1. Read Image
Create an Image object by specifying a path or alternatively an image array in (height, width, channel) format (similar to Python's numpy array for opencv).
Note: Leave the Array3 parameter as is if you don't intend to use it.
```rust
let mut img = Image::new(
    String::from("img/string.png"),
    Array3::<u8>::zeros((100, 100, 3))
);

// alternatively instantiate directly:

let mut img = Image {
    path: String::from("img/string.png"),
    ndarray: Array3::<u8>::zeros((100, 100, 3))  // example: creates an 100x100 pixel image with 3 colour channels (RGB)
};
```

### 2. Set tesseract parameters
Set tesseract parameters using the Args struct. 
```rust
let default_args = Args::new();

// the default parameters are
/* pub fn new() -> Args {
        Args {
            config: HashMap::new(),
            lang: "eng",
            out_filename: "out",
            dpi: 150,
            boxfile: false
            }
    }
*/

// fill your own argument struct if needed
let mut my_args = Args {
    out_filename: "out",        // name of output_file
    lang: "eng",                // model language (tesseract default = 'eng')
    config: HashMap::new(),     // create empty hashmap to fill with command line parameters such as --psm or --oem (see tesseract --help-extra)
    dpi: 150,                   // specify DPI for input image
    boxfile: false              // specify whether the output should be a bounding box or string output
};
image_to_string_args.config.insert("psm", "6");  // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
image_to_string_args.config.insert("oem", "3");  // define optical character recognition mode 3 (i.e. "Default, based on what is available")
```

### 3. Get the tesseract model output
Choose either string, bounding box or data output:
```rust
// string output
let output = rusty_tesseract::image_to_string(&img, my_args);
    println!("The String output is: {:?}", output.Output_STRING);

// define bounding box parameters
let mut image_to_boxes_args = Args {
    out_filename: "font_name.font.exp0",
    lang: "eng",
    config: HashMap::new(),
    dpi: 150,
    boxfile: true
};
image_to_boxes_args.config.insert("psm", "6");
image_to_boxes_args.config.insert("oem", "3");

// boxes printed in OUTPUT_DICT or OUTPUT_DATAFRAME format store the key as a string (i.e. the character) and 
// store the value as a list of strings (if the same character occurs more than once)
let boxes = rusty_tesseract::image_to_boxes(&img, image_to_boxes_args);
println!("The Boxfile output is: {:?}", boxes.Output_DATAFRAME);

// image_to_data prints out both the "image_to_string()" and "image_to_boxes()" information + a creates a TSV table with confidences
let data = rusty_tesseract::image_to_data(&img, default_args);
println!("The data output is: {:?}", data.Output_DICT);
```

### Get tesseract version
```rust
let tesseract_version = rusty_tesseract::get_tesseract_version();
    println!("The tesseract version is: {:?}", tesseract_version);
```

## Contributing

1. Fork the repository
2. Create a new feature branch (`git checkout -b my-feature-branch-name`)
3. Commit your new changes (`git commit -m 'commit message' <changed-file>`)
4. Push changes to the branch (`git push origin my-feature-branch-name`)
5. Create a Pull Request