use core::fmt;

use super::*;

#[derive(Debug, PartialEq)]
pub struct DataOutput {
    pub output: String,
    pub data: Vec<Data>,
}

impl fmt::Display for DataOutput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.output)
    }
}

#[derive(Debug, PartialEq)]
pub struct Data {
    pub level: i32,
    pub page_num: i32,
    pub block_num: i32,
    pub par_num: i32,
    pub line_num: i32,
    pub word_num: i32,
    pub left: i32,
    pub top: i32,
    pub width: i32,
    pub height: i32,
    pub conf: f32,
    pub text: String,
}

impl fmt::Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {} {} {} {} {} {} {} {}",
            self.level,
            self.page_num,
            self.block_num,
            self.par_num,
            self.line_num,
            self.word_num,
            self.left,
            self.top,
            self.width,
            self.height,
            self.conf,
            self.text,
        )
    }
}

impl Data {
    fn parse(line: &str) -> Option<Self> {
        let mut x = line.split_whitespace();
        let level = str::parse::<i32>(x.next()?).ok()?;
        let page_num = str::parse::<i32>(x.next()?).ok()?;
        let block_num = str::parse::<i32>(x.next()?).ok()?;
        let par_num = str::parse::<i32>(x.next()?).ok()?;
        let line_num = str::parse::<i32>(x.next()?).ok()?;
        let word_num = str::parse::<i32>(x.next()?).ok()?;
        let left = str::parse::<i32>(x.next()?).ok()?;
        let top = str::parse::<i32>(x.next()?).ok()?;
        let width = str::parse::<i32>(x.next()?).ok()?;
        let height = str::parse::<i32>(x.next()?).ok()?;
        let conf = str::parse::<f32>(x.next()?).ok()?;
        let text = x.next().unwrap_or("").to_string();

        Some(Data {
            level,
            page_num,
            block_num,
            par_num,
            line_num,
            word_num,
            left,
            top,
            width,
            height,
            conf,
            text,
        })
    }
}

pub fn image_to_data(image: &Image, args: &Args) -> TessResult<DataOutput> {
    let mut command = create_tesseract_command(image, args)?;
    command.arg("tsv");

    let output = run_tesseract_command(&mut command)?;

    let data = string_to_data(&output)?;

    Ok(DataOutput { output, data })
}

fn string_to_data(output: &str) -> TessResult<Vec<Data>> {
    output
        .lines()
        .into_iter()
        .skip(1)
        .map(|line| Data::parse(line.into()))
        .collect::<Option<Vec<Data>>>()
        .ok_or(TessError::ParseError)
}

#[cfg(test)]
mod tests {
    use ndarray::Array3;

    use crate::{output_data::string_to_data, *};

    #[test]
    fn test_string_to_data() {
        let result = string_to_data("level   page_num        block_num       par_num line_num        word_num        left    top     width   height  conf    text
        5       1       1       1       1       1       65      41      46      20      96.063751       The");
        assert_eq!(
            *result.unwrap().first().unwrap(),
            Data {
                level: 5,
                page_num: 1,
                block_num: 1,
                par_num: 1,
                line_num: 1,
                word_num: 1,
                left: 65,
                top: 41,
                width: 46,
                height: 20,
                conf: 96.063751,
                text: String::from("The"),
            }
        )
    }

    #[test]
    fn test_image_to_data() {
        let img = Image::new(
            String::from("img/string.png"),
            Array3::<u8>::zeros((0, 0, 3)),
        );
        let mut image_to_boxes_args = Args::default();
        image_to_boxes_args.psm = 6;

        let result = tesseract::image_to_data(&img, &image_to_boxes_args).unwrap();
        assert_eq!(
            result.data,
            string_to_data(
                r#"level   page_num        block_num       par_num line_num        word_num        left    top  width    height  conf    text
                1       1       0       0       0       0       0       0       696     89      -1
                2       1       1       0       0       0       18      29      653     35      -1
                3       1       1       1       0       0       18      29      653     35      -1
                4       1       1       1       1       0       18      29      653     35      -1
                5       1       1       1       1       1       18      29      144     35      95.643112    LOREM
                5       1       1       1       1       2       181     29      123     35      92.306282    IPSUM
                5       1       1       1       1       3       323     29      153     35      90.531677    DOLOR
                5       1       1       1       1       4       490     29      50      35      95.873787    SIT
                5       1       1       1       1       5       553     30      118     33      96.834381    AMET"#
            )
            .unwrap()
        );
    }
}
