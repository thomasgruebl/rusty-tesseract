# rusty-tesseract

A Rust wrapper for Google Tesseract

![GitHub last commit](https://img.shields.io/github/last-commit/thomasgruebl/rusty-tesseract?style=plastic) ![GitHub](https://img.shields.io/github/license/thomasgruebl/rusty-tesseract?style=plastic) <a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/stargazers">
<img src="https://img.shields.io/github/stars/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Stars">
</a>
<a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/fork">
<img src="https://img.shields.io/github/forks/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Forks">
</a>
![Github All Releases](https://img.shields.io/github/downloads/thomasgruebl/rusty-tesseract/total.svg?style=plastic)
<a style="text-decoration: none" href="https://github.com/thomasgruebl/rusty-tesseract/issues">
<img src="https://img.shields.io/github/issues/thomasgruebl/rusty-tesseract.svg?style=plastic" alt="Issues">
</a>

## Installation

Add the following line to your <b>Cargo.toml</b> file:

```rust
rusty-tesseract = "1.1.4"
```

## Description

- Brings all relevant command-line tesseract functionality to Rust
- Based on the Python wrapper for tesseract (i.e. https://github.com/madmaze/pytesseract)
- Enables testing a pre-trained tesseract model and outputting the results in different formats such as strings, bounding boxes, dicts, or dataframes.

## Dependencies

Tesseract: https://github.com/tesseract-ocr/tesseract

## Usage

### 1. Read Image

Create an Image object by specifying a path or alternatively a DynamicImage from the image crate https://docs.rs/image/latest/image/

```rust
// you can use the from_path function
let _ = Image::from_path("img/string.png");


// or instantiate Image from a DynamicImage
let dynamic_image = ImageReader::open("img/string.png")
    .unwrap()
    .decode()
    .unwrap();
let img = Image::from_dynamic_image(&dynamic_image).unwrap();
```

### 2. Set tesseract parameters

Set tesseract parameters using the Args struct.

```rust
let default_args = Args::default();

// the default parameters are
/*
Args {
    lang: "eng",
    dpi: 150,
    psm: 3,
    oem: 3,
}
*/

// fill your own argument struct if needed
let mut my_args = Args {
    //model language (tesseract default = 'eng')
    //available languages can be found by running 'rusty_tesseract::get_tesseract_langs()'
    lang: "eng",

    //map of config variables
    //this example shows a whitelist for the normal alphabet. Multiple arguments are allowed.
    //available arguments can be found by running 'rusty_tesseract::get_tesseract_config_parameters()'
    config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
    dpi: 150,       // specify DPI for input image
    psm: 3,         // define page segmentation mode 6 (i.e. "Assume a single uniform block of text")
    oem: 3,         // define optical character recognition mode 3 (i.e. "Default, based on what is available")
};
```

### 3. Get the tesseract model output

Choose either string, bounding box or data output:

```rust
// define parameters
let mut my_args = Args {
    lang: "eng",
    config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
    dpi: 150,
    psm: 6,
    oem: 3
};

// string output
let output = rusty_tesseract::image_to_string(&img, &my_args).unwrap();
    println!("The String output is: {:?}", output);



// image_to_boxes creates a BoxOutput containing the parsed output from Tesseract when using the "makebox" Parameter
let box_output = rusty_tesseract::image_to_boxes(&img, &my_args).unwrap();
println!(
    "The first boxfile symbol is: {}",
    box_output.boxes[0].symbol
);
println!("The full boxfile output is:\n{}", box_output.output);

// image_to_data creates a DataOutput containing the parsed output from Tesseract when using the "TSV" Parameter
let data_output = rusty_tesseract::image_to_data(&img, &my_args).unwrap();
let first_text_line = &data_output.data[4];
println!(
    "The first text is '{}' with confidence {}",
    first_text_line.text, first_text_line.conf
);
println!("The full data output is:\n{}", data_output.output);
```

### Get informations about tesseract

```rust
//tesseract version
let tesseract_version = rusty_tesseract::get_tesseract_version().unwrap();
println!("The tesseract version is: {:?}", tesseract_version);

//available languages
let tesseract_langs = rusty_tesseract::get_tesseract_langs().unwrap();
println!("The available languages are: {:?}", tesseract_langs);

//available config parameters
let parameters = rusty_tesseract::get_tesseract_config_parameters().unwrap();
println!("Example config parameter: {}", parameters.config_parameters.first().unwrap());
```

## Contributing

1. Fork the repository
2. Create a new feature branch (`git checkout -b my-feature-branch-name`)
3. Commit your new changes (`git commit -m 'commit message' <changed-file>`)
4. Push changes to the branch (`git push origin my-feature-branch-name`)
5. Create a Pull Request
