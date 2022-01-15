use std::collections::HashMap;
use multimap::MultiMap;
use subprocess::Popen;
use subprocess::PopenConfig;
use subprocess::Redirection;
use std::any::{Any, TypeId};
use polars::prelude::*;
use ndarray::Array3;
use std::fmt;
use std::string::ToString;
use std::str::FromStr;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::fs;
use substring::Substring;

use crate::error::VersionError;
use crate::error::TesseractNotFoundError;
use crate::error::ImageFormatError;


const language_pattern: &str = "";
const encoding: &str = "utf-8";
const rgb_mod: &str = "RGB";
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
    pub OUTPUT_DATAFRAME: Vec<Series>
}

impl ModelOutput {
    fn area(&self) -> f64 {
        return 10.0;
    }
}

impl fmt::Display for ModelOutput {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.Output_STRING)
    }
}

pub struct Args {
    pub out_filename: &'static str,
    pub lang: &'static str,
    pub config: HashMap<&'static str, &'static str>,
    pub timeout: i32,
    pub dpi: i32,
    pub boxfile: bool
}

impl Args{
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

pub struct Image {
    pub path: String,
    pub ndarray: Array3<i32>
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.path)
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


pub fn image_to_data(image: &Image, args: Args) {
    // return image_to_string(.....)
    // return image_to_boxes(......)
    // and return additional stuff such as confidences
    // check for version > 3.05

    // if check_image_format(&image) {
    //     return run_tesseract(&image, &args);
    // }
    // else {
    //     panic!("{}", ImageFormatError);
    // }
}

pub fn image_to_boxes(image: &Image, args: Args) -> ModelOutput {
    if check_image_format(&image) {
        return run_tesseract(&image, &args);
    }
    else {
        panic!("{}", ImageFormatError);
    }
}

pub fn image_to_string(image: &Image, args: Args) -> ModelOutput {
    if check_image_format(&image) {
        return run_tesseract(&image, &args);
    }
    else {
        panic!("{}", ImageFormatError);
    }
}

fn run_tesseract(image: &Image, args: &Args) -> ModelOutput {

    let is_installed: bool = check_if_installed();
    if !is_installed {
        panic!("{}", TesseractNotFoundError);
    }

    let image = &image.to_string().replace('"', "").to_owned();

    for (key, value) in &args.config {
        println!("Key and value: {:?} {:?}", key, value)
    }

    // check if boxmode is activated
    let mut boxarg = String::from("");
    if args.boxfile {
        boxarg = String::from("makebox");
    }

    let command = if cfg!(target_os = "windows") {
        Command::new("tesseract.exe")
            .arg(image)
            .arg(args.out_filename)
            .arg("-l")
            .arg(args.lang)
            .arg("--dpi")
            .arg("150")
            .arg(boxarg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
                
    } else {
        Command::new("tesseract")
            .arg(image)
            .arg(args.out_filename)
            .arg("-l")
            .arg(args.lang)
            .arg("--dpi")
            .arg("150")
            .arg(boxarg)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap()
    };

    let output = command.wait_with_output().unwrap();
    println!("outttttttttttttttttttttttt {:?}", output);

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
    

    let out = ModelOutput {
        Output_INFO: str_res,
        Output_BYTES: file_output.as_bytes().to_vec(),
        Output_DICT: dict,
        Output_STRING: file_output,
        OUTPUT_DATAFRAME: df
    };


    return out;
}
