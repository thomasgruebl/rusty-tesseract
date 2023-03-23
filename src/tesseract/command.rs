use super::*;
use ndarray::Array3;
use std::env::current_dir;
use std::process::{Command, Stdio};
use std::string::ToString;

use crate::error::{TessError, TessResult};

pub const FORMATS: [&'static str; 10] = [
    "JPEG", "JPG", "PNG", "PBM", "PGM", "PPM", "TIFF", "BMP", "GIF", "WEBP",
];

fn type_of<T>(_: &T) -> String {
    let t = String::from(std::any::type_name::<T>());
    return t;
}

fn check_image_format(img: &Image) -> TessResult<()> {
    let splits: Vec<&str> = img.path.split(".").collect();
    let format = splits.last().unwrap().to_string();
    let tmp = String::from(&format).to_uppercase();
    let tmp2 = String::from(&format).to_lowercase();
    let uppercase_format = tmp.as_str();
    let format = tmp2.as_str();

    if FORMATS.contains(&format) || FORMATS.contains(&uppercase_format) {
        return Ok(());
    }
    Err(TessError::ImageFormatError)
}

fn get_tesseract_command() -> Command {
    let tesseract = if cfg!(target_os = "windows") {
        "tesseract.exe"
    } else {
        "tesseract"
    };

    Command::new(tesseract)
}

pub fn get_tesseract_version() -> TessResult<String> {
    let mut command = get_tesseract_command();
    command.arg("--version");

    run_tesseract_command(&mut command)
}

pub(crate) fn run_tesseract_command(command: &mut Command) -> TessResult<String> {
    if cfg!(debug_assertions) {
        show_command(command);
    }

    let child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| TessError::TesseractNotFoundError)?;

    let output = child
        .wait_with_output()
        .map_err(|_| TessError::TesseractNotFoundError)?;

    let out = String::from_utf8(output.stdout).unwrap();
    let err = String::from_utf8(output.stderr).unwrap();
    let status = output.status;

    match status.code() {
        Some(0) => Ok(out),
        _ => Err(TessError::VersionError(err)),
    }
}

fn show_command(command: &Command) {
    let params: Vec<String> = command
        .get_args()
        .map(|x| x.to_str().unwrap_or(""))
        .map(|s| s.to_string())
        .collect();

    println!(
        "Tesseract Command: {} {}",
        command.get_program().to_str().unwrap(),
        params.join(" ")
    );
}

pub fn image_to_string(image: &Image, args: &Args) -> TessResult<String> {
    let mut command = create_tesseract_command(image, args)?;
    let output = run_tesseract_command(&mut command)?;

    Ok(output)
}

pub(crate) fn create_tesseract_command(image: &Image, args: &Args) -> TessResult<Command> {
    check_image_format(image)?;

    assert_eq!(type_of(&image.path), type_of(&String::new()));
    assert_eq!(
        type_of(&image.ndarray),
        type_of(&Array3::<u8>::zeros((0, 0, 0)))
    );

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
            Ok(_r) => {
                println!("Image saved: {:?}", new_path);
                image_arg = new_path;
            }
            Err(e) => println!("Error while saving image: {:?}", e),
        }
    }
    // both image path and ndarray are empty
    else if image.path.len() == 0 && *is_empty_ndarray {
        return Err(TessError::ImageNotFoundError);
    }
    // path is filled
    else {
        check_image_format(&image)?;
        image_arg = image.to_string().replace('"', "").to_owned();
    }

    let mut command = get_tesseract_command();
    command
        .arg(image_arg)
        .arg("stdout")
        .arg("-l")
        .arg(args.lang)
        .arg("--dpi")
        .arg(args.dpi.to_string())
        .arg("--psm")
        .arg(args.psm.to_string())
        .arg("--oem")
        .arg(args.oem.to_string());

    Ok(command)
}
