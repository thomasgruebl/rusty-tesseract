use super::*;
use std::process::{Command, Stdio};
use std::string::ToString;

use crate::error::{TessError, TessResult};

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
    let mut command = get_tesseract_command();
    command
        .arg(image.get_image_path()?)
        .arg("stdout")
        .arg("-l")
        .arg(args.lang)
        .arg("-c")
        .arg(args.config_variables.to_string())
        .arg("--dpi")
        .arg(args.dpi.to_string())
        .arg("--psm")
        .arg(args.psm.to_string())
        .arg("--oem")
        .arg(args.oem.to_string());

    Ok(command)
}
