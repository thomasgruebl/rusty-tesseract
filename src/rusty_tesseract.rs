use std::collections::HashMap;
use multimap::MultiMap;
use polars::prelude::*;
use ndarray::Array3;
use image::RgbImage;
use std::fmt;
use std::string::ToString;
use std::io::BufRead;
use std::process::{Command, Stdio};
use std::fs;
use std::env::current_dir;

use crate::error::VersionError;
use crate::error::TesseractNotFoundError;
use crate::error::ImageFormatError;
use crate::error::ImageNotFoundError;

const Formats: [&'static str; 10] = ["JPEG",
                                    "JPG",
                                    "PNG",
                                    "PBM",
                                    "PGM",
                                    "PPM",
                                    "TIFF",
                                    "BMP",
                                    "GIF",
                                    "WEBP"];


pub struct ModelOutput {
    pub Output_INFO: String,
    pub Output_BYTES: Vec<u8>,
    pub Output_DICT: MultiMap<String, String>,
    pub Output_STRING: String,
    pub Output_DATAFRAME: Vec<Series>
}

impl ModelOutput {
    fn new() -> ModelOutput {
        ModelOutput {
            Output_INFO: String::new(),
            Output_BYTES: Vec::new(),
            Output_DICT: MultiMap::new(),
            Output_STRING: String::new(),
            Output_DATAFRAME: Vec::new()
        }
    }
}

impl fmt::Display for ModelOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.Output_STRING)
    }
}

#[derive(Clone)]
pub struct Args {
    pub out_filename: &'static str,
    pub lang: &'static str,
    pub config: HashMap<&'static str, &'static str>,
    pub timeout: i32,
    pub dpi: i32,
    pub boxfile: bool
}

impl Args {
    pub fn new() -> Args {
        Args {
            config: HashMap::new(),
            lang: "eng",
            out_filename: "out",
            timeout: 1000,
            dpi: 150,
            boxfile: false
            }
    }
}

#[derive(Clone)]
pub struct Image {
    pub path: String,
    pub ndarray: Array3<u8>
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
    }
}

impl Image {
    pub fn new(path: String, ndarray: Array3<u8>) -> Image {
        Image {
            path,
            ndarray
        }
    }

    fn is_empty_ndarray(&self) -> bool {
        let mut is_empty: bool = true;
        for elem in &self.ndarray {
            is_empty = false;
        }
        return is_empty;
    }

    fn size_of_ndarray(&self) -> (usize, usize, usize) {
        return self.ndarray.dim()
    }

    fn ndarray_to_image(self) -> RgbImage {
        let (height, width, _) = self.size_of_ndarray();
        let raw = self.ndarray.into_raw_vec();

        RgbImage::from_raw(width as u32, height as u32, raw)
            .expect("Couldnt convert ndarray to RgbImage.")
    }
}

fn type_of<T>(_: &T) -> String {
    let t = String::from(std::any::type_name::<T>());
    return t;
}

fn read_output_file(filename: &String) -> String{
    let f = fs::read_to_string(filename.to_owned())
        .expect("File reading error. Filename does not exist.");

    return f;
}

fn check_image_format(img: &Image) -> bool {
    let splits: Vec<&str> = img.path.split(".").collect();
    let format = splits.last().unwrap().to_string();
    let tmp = String::from(&format).to_uppercase();
    let tmp2 = String::from(&format).to_lowercase();
    let uppercase_format = tmp.as_str();
    let format = tmp2.as_str();

    if Formats.contains(&format) || Formats.contains(&uppercase_format) {
        return true;
    }
    else {
        return false;
    }
}

fn check_if_installed() -> bool {
    if cfg!(target_os = "windows") {
        match Command::new("tesseract.exe")
                .stdout(Stdio::null())
                .spawn() {
                    Ok(_) => return true,
                    Err(e) => return false, 
        }
    } else {
        match Command::new("tesseract")
                .stdout(Stdio::null())
                .spawn() {
                    Ok(_) => return true,
                    Err(e) => return false, 
        }
    }
}

pub fn get_tesseract_version() -> String {

    let is_installed: bool = check_if_installed();
    if !is_installed {
        panic!("{}", TesseractNotFoundError);
    }

    let command = if cfg!(target_os = "windows") {
        Command::new("tesseract.exe")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
                
    } else {
        Command::new("tesseract")
            .arg("--version")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };

    let output = command.wait_with_output().unwrap();

    let out = output.stdout;
    let err = output.stderr;
    let status = output.status;

    match status.code() {
        Some(code) => println!("Exited with status code: {}", code),
        None       => println!("Exited with error: {}", VersionError)
    }

    let mut str_res = String::new();
    if out.len() == 0 {
        err.lines().for_each(|line|
            str_res = format!("{}\n{}", str_res, line.unwrap())
        );
    }
    else {
        out.lines().for_each(|line|
            str_res = format!("{}\n{}", str_res, line.unwrap())
        );
    }

    return str_res;
}

pub fn image_to_data(image: &Image, args: Args) -> ModelOutput {
    let str_out: ModelOutput = image_to_string(&image, args.clone());

    let mut box_args = args.clone();
    box_args.boxfile = true;
    let box_out: ModelOutput = image_to_boxes(&image, box_args);

    let out = ModelOutput {
        Output_INFO: str_out.Output_INFO,
        Output_BYTES: str_out.Output_BYTES,
        Output_DICT: box_out.Output_DICT,
        Output_STRING: str_out.Output_STRING,
        Output_DATAFRAME: box_out.Output_DATAFRAME
    };

    // and return additional stuff such as confidences  --->    -c tessedit_create_tsv=1
    // check for version > 3.05
    let mut tesstable_args = args.clone();
    tesstable_args.config.insert("-c", "tessedit_create_tsv=1");
    let tesstable = run_tesseract(&image, &tesstable_args);

    if check_image_format(&image) {
        return out;
    }
    else {
        panic!("{}", ImageFormatError);
    }

    // maybe return Tuple of Modeloutput + Hashmap (where the hashmap contains the confidence tessdata table)

    // still need to add argchecks to run_tesseract function for -c tessedit ...
}

pub fn image_to_boxes(image: &Image, args: Args) -> ModelOutput {
    return run_tesseract(&image, &args);
}

pub fn image_to_string(image: &Image, args: Args) -> ModelOutput {
    return run_tesseract(&image, &args);
}

fn run_tesseract(image: &Image, args: &Args) -> ModelOutput {

    // check if tesseract is installed
    let is_installed: bool = check_if_installed();
    if !is_installed {
        panic!("{}", TesseractNotFoundError);
    }

    assert_eq!(type_of(&image.path), type_of(&String::new()));
    assert_eq!(type_of(&image.ndarray), type_of(&Array3::<u8>::zeros((0, 0, 0))));

    // check if image path or ndarray is provided
    let mut image_arg = String::from("");
    let is_empty_ndarray = &image.is_empty_ndarray();
    if image.path.len() == 0 && !*is_empty_ndarray {
        // convert ndarray to rgbimage and save image in parent directory
        let tmp_img = image.clone();
        let i = tmp_img.ndarray_to_image();
        let working_dir = current_dir().unwrap().as_path().display().to_string();
        let new_path = [working_dir, String::from("ndarray_converted.png")].join("/");
        
        match i.save(&new_path) {
            Ok(r) => {
                println!("Image saved: {:?}", new_path);
                image_arg = new_path;
            }, 
            Err(e) => println!("Error while saving image: {:?}", e),
        }
    }
    // both image path and ndarray are empty
    else if image.path.len() == 0 && *is_empty_ndarray {
        panic!("{}", ImageNotFoundError);
    }
    // path is filled
    else {
        if !check_image_format(&image) {
            panic!("{}", ImageFormatError);
        }
        image_arg = image.to_string().replace('"', "").to_owned();
    }

    for (key, value) in &args.config {
        println!("Key and value: {:?} {:?}", key, value)
    }

    // check if boxmode is activated
    let mut boxarg = String::from("");
    if args.boxfile {
        boxarg = String::from("makebox");
    }

    // check if psm and oem flags are set
    let mut psm = "3";
    let mut oem = "3";
    if args.config.contains_key("psm") {
        psm = args.config["psm"];
    }

    if args.config.contains_key("oem"){
        oem = args.config["oem"];
    }

    println!("the image arg is: {:?}", image_arg);

    let command = if cfg!(target_os = "windows") {
        Command::new("tesseract.exe")
            .arg(image_arg)
            .arg(args.out_filename)
            .arg("-l")
            .arg(args.lang)
            .arg("--dpi")
            .arg(args.dpi.to_string())
            .arg("--psm")
            .arg(psm)
            .arg("--oem")
            .arg(oem)
            .arg(boxarg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
                
    } else {
        Command::new("tesseract")
            .arg(image_arg)
            .arg(args.out_filename)
            .arg("-l")
            .arg(args.lang)
            .arg("--dpi")
            .arg(args.dpi.to_string())
            .arg("--psm")
            .arg(psm)
            .arg("--oem")
            .arg(oem)
            .arg(boxarg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };

    let output = command.wait_with_output().unwrap();
    println!("{:?}", output);

    let out = output.stdout;
    let err = output.stderr;
    let status = output.status;

    match status.code() {
        Some(code) => println!("Exited with status code: {}", code),
        None       => println!("Process terminated by signal")
    }

    let mut str_res = String::new();
    if out.len() == 0 {
        err.lines().for_each(|line|
            str_res = format!("{}\n{}", str_res, line.unwrap())
        );
    }
    else {
        out.lines().for_each(|line|
            str_res = format!("{}\n{}", str_res, line.unwrap())
        );
    }

    // read tesseract output from output file "out.txt"
    let mut out_f = String::new();
    if !args.boxfile {
        if !args.out_filename.contains(".txt") {
            out_f = format!("{}.txt", args.out_filename); 
        }
        else {
            out_f = args.out_filename.to_string();
        }
    }
    // if boxfile is requested -> read from .box file
    else {
        if !args.out_filename.contains(".box") {
            out_f = format!("{}.box", args.out_filename); 
        }
        else {
            out_f = args.out_filename.to_string();
        }
    }

    let file_output = read_output_file(&out_f);

    // multimap used for box files -> stores character as key and box boundaries as value (or list of values)
    let mut dict = MultiMap::new();
    let mut df = Vec::new();
    if args.boxfile {
        for line in file_output.lines() {
            if line.contains(" ") {
                // fill dict
                let tuple = line.split_once(" ").unwrap();
                dict.insert(
                    String::from(tuple.0),
                    String::from(tuple.1),
                );

                // fill DataFrame (Vec of Series)
                let character: &str = &tuple.0;
                let mut box_boundaries = Vec::new();
                for num in tuple.1.split(" ") {
                    let num_int: i32 = num.parse::<i32>().unwrap();
                    box_boundaries.push(num_int);
                }
                let tmp_series = Series::new(character, &box_boundaries);
                df.push(tmp_series);
            }
        }
    }
    
    let out = ModelOutput {
        Output_INFO: str_res,
        Output_BYTES: file_output.as_bytes().to_vec(),
        Output_DICT: dict,
        Output_STRING: file_output,
        Output_DATAFRAME: df
    };

    return out;
}
