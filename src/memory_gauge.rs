// Importing required modules from the standard library and Serenity crate.

use image::{DynamicImage, ImageBuffer, imageops, imageops::FilterType, Rgba};
use rusttype::{Font, Scale};

use crate::core_functions;

// Importing modules from the local crate.

pub async fn create_gauge(game: [i64; 9], table: i32) -> String {

    // Declarations
    let player_one_name = core_functions::to_tag(*game.first().unwrap() as u64).await;
    let player_two_name = core_functions::to_tag(*game.get(1).unwrap() as u64).await;
    let energy = *game.get(2).unwrap();
    let player_one_gauge = *game.get(4).unwrap();
    let player_two_gauge = *game.get(5).unwrap();
    let counter = *game.get(6).unwrap();
    let color = *game.get(7).unwrap();
    let color2 = *game.get(8).unwrap();

    // Open Images
    let counter = get_counter_bytes(counter).await.unwrap();
    let gauge_one = get_gauge_bytes(player_one_gauge).await.unwrap();
    let gauge_two = get_gauge_bytes(player_two_gauge).await.unwrap();
    let gauge_one = image::load_from_memory(gauge_one).expect("File not found!");
    let gauge_two = image::load_from_memory(gauge_two).expect("File not found!");
    let counter = image::load_from_memory(counter).expect("File not found!");
    let final_gauge = ImageBuffer::from_fn(1000, 350, |_x, _y| {
        Rgba([255, 255, 255, 255])
    });
    let final_gauge = DynamicImage::ImageRgba8(final_gauge);

    //Resize images to be correct
    let mut gauge_one = gauge_one.resize_exact(500, 350, FilterType::Nearest);
    let gauge_two = gauge_two.resize_exact(500, 350, FilterType::Nearest);
    let counter = counter.resize_exact(80, 80, FilterType::Nearest);

    //Flip Image
    let mut gauge_two = gauge_two.flipv().fliph();

    //Draw Text and Outline
    gauge_one = draw_text(player_one_name.clone(), Rgba([0, 0, 0, 100]), gauge_one.clone(), 292, 47, 61.0, 86.0).await;
    gauge_one = draw_text(player_one_name, from_bit_field(color), gauge_one.clone(), 290, 50, 60.0, 80.0).await;
    gauge_two = draw_text(player_two_name.clone(), Rgba([0, 0, 0, 100]), gauge_two.clone(), 52, 237, 61.0, 86.0).await;
    gauge_two = draw_text(player_two_name, from_bit_field(color2), gauge_two.clone(), 50, 240, 60.0, 80.0).await;

    // When game starts i64 is max (To allow either player to play), sets to 0 for image purposes
    let energy = if energy == i64::MAX { 0 } else { energy };

    //Find the counter location to place
    let xy = energy_position(energy);
    let x = xy.0;
    let y = xy.1;

    //Merge The Images
    let final_gauge = merge(final_gauge, &[gauge_one], 0, 0).await;
    let final_gauge = merge(final_gauge, &[gauge_two], 500, 0).await;
    let location = "./Table/".to_string() + table.to_string().as_str() + ".jpg";
    // Puts the counter on the board and saves file with table number
    merge(final_gauge.clone(), &[counter], x, y).await.save(location.clone()).expect("TODO: panic message");
    println!("{}", location);
    location
}

fn energy_position(energy: i64) -> (i64, i64) {
    match energy {
        -10 => (376, 224),
        -9 => (290, 224),
        -8 => (203, 224),
        -7 => (117, 224),
        -6 => (31, 224),
        -5 => (31, 134),
        -4 => (117, 134),
        -3 => (204, 134),
        -2 => (290, 134),
        -1 => (376, 134),
        0 => (459, 134),
        1 => (547, 134),
        2 => (633, 134),
        3 => (716, 134),
        4 => (802, 134),
        5 => (887, 134),
        6 => (887, 44),
        7 => (802, 44),
        8 => (716, 44),
        9 => (633, 44),
        10 => (547, 134),
        _ => (463, 134)
    }
}

// Overlays 2 images to help build the memory gauge
async fn merge(mut base: DynamicImage, img: &[DynamicImage], x: i64, y: i64) -> DynamicImage {
    for img in img {
        imageops::overlay(&mut base, img, x, y);
    }
    base
}

// Draws the colored text on the memory gauge
async fn draw_text(fact: String, color: Rgba<u8>, mut img: DynamicImage, mut x: i32, y: i32, text_width: f32, text_height: f32) -> DynamicImage {
    let font_bytes = include_bytes!("Templates/font.ttf");
    let font = Font::try_from_bytes(font_bytes).unwrap();
    let wrapped_fact = textwrap::wrap(fact.as_str(), 30);
    for line in wrapped_fact {
        imageproc::drawing::draw_text_mut(
            &mut img,
            color,
            x,
            y,
            Scale { x: text_width, y: text_height },
            &font,
            &line,
        );
        x += 25;
    }
    img
}

// Finds file for counter selected. include_bytes! is a macro and compiles runtime so it cannot have variables inside it, this is a workaround.
async fn get_counter_bytes(option: i64) -> Option<&'static [u8]> {
    match &*option.to_string() {
        "1" => Some(include_bytes!("Templates/Counters/1.png")),
        "2" => Some(include_bytes!("Templates/Counters/2.png")),
        "3" => Some(include_bytes!("Templates/Counters/3.png")),
        "4" => Some(include_bytes!("Templates/Counters/4.png")),
        "5" => Some(include_bytes!("Templates/Counters/5.png")),
        _ => None,
    }
}

// Finds file for gauge selected. include_bytes! is a macro and compiles runtime so it cannot have variables inside it, this is a workaround.
async fn get_gauge_bytes(option: i64) -> Option<&'static [u8]> {
    match &*option.to_string() {
        "1" => Some(include_bytes!("Templates/Gauges/1.jpg")),
        "2" => Some(include_bytes!("Templates/Gauges/2.jpg")),
        "3" => Some(include_bytes!("Templates/Gauges/3.jpg")),
        "4" => Some(include_bytes!("Templates/Gauges/4.jpg")),
        "5" => Some(include_bytes!("Templates/Gauges/5.jpg")),
        "6" => Some(include_bytes!("Templates/Gauges/6.jpg")),
        "7" => Some(include_bytes!("Templates/Gauges/7.jpg")),
        "8" => Some(include_bytes!("Templates/Gauges/8.jpg")),
        "9" => Some(include_bytes!("Templates/Gauges/9.jpg")),
        "10" => Some(include_bytes!("Templates/Gauges/10.jpg")),
        "11" => Some(include_bytes!("Templates/Gauges/11.jpg")),
        "12" => Some(include_bytes!("Templates/Gauges/12.jpg")),
        "13" => Some(include_bytes!("Templates/Gauges/13.jpg")),
        "14" => Some(include_bytes!("Templates/Gauges/14.jpg")),
        "15" => Some(include_bytes!("Templates/Gauges/15.jpg")),
        "16" => Some(include_bytes!("Templates/Gauges/16.jpg")),
        "17" => Some(include_bytes!("Templates/Gauges/17.jpg")),
        _ => Some(include_bytes!("Templates/Gauges/1.jpg")),
    }
}

// Converts the input into bit_field number representation
pub fn to_bit_field(input: String) -> i64 {
    let input = to_rgba(input.to_ascii_lowercase());
    let red = rgba_to_binary(input, 0);
    let green = rgba_to_binary(input, 1);
    let blue = rgba_to_binary(input, 2);
    i64::from_str_radix(&(red + &green + &blue), 2).unwrap()
}

// Converts the bit_field number representation to a Rgba color
fn from_bit_field(input: i64) -> Rgba<u8> {
    let input = format!("{:24b}", input);
    let input: Vec<char> = input.chars().collect();
    let chunks: Vec<String> = input.chunks(8).map(|chunk| chunk.iter().collect()).collect();
    let red: u8 = extract_color(chunks.clone(), 0) as u8;
    let blue: u8 = extract_color(chunks.clone(), 1) as u8;
    let green: u8 = extract_color(chunks, 2) as u8;
    Rgba([red, blue, green, 100])
}

// Extracts color from the vector
fn extract_color(chunks: Vec<String>, i: usize) -> i32 {
    let test: &str = &chunks[i].replace(' ', "0");
    i32::from_str_radix(test, 2).unwrap()
}

// Converts from Hexadecimal, RGB notation, and plaintext colors to Rgba
fn to_rgba(input: String) -> Rgba<i32> {
    let mut rgb = "255,123,200".to_string();
    if input.starts_with('#') {
        if input.len() == 7 {
            rgb = hex_to_rgb(input);
        } else if input.len() == 4 {
            rgb = hex_to_rgb(input.chars().flat_map(|c| vec![c, c]).collect());
        }
    } else if input.contains(',') {
        rgb = input.to_string().replace([')', '('], "");
    } else {
        rgb = from_color(input);
    }
    let mut color = rgb.split(',');
    let red: i32 = color.next().unwrap().parse().unwrap();
    let green = color.next().unwrap().parse().unwrap();
    let blue = color.next().unwrap().parse().unwrap();
    Rgba([red, green, blue, 100])
}

// Predefined colors
fn from_color(input: String) -> String {
    match input.as_str() {
        "red" => "255,0,0",
        "orange" => "255,128,0",
        "yellow" => "255,255,0",
        "green" => "0,255,0",
        "cyan" => "0,255,255",
        "blue" => "0,0,255",
        "purple" => "128,0,255",
        "pink" => "255,0,128",
        "black" => "0,0,0",
        "gray" => "128,128,128",
        "white" => "255,255,255",
        "mine" => "255,153,204",
        _ => { "255,0,128" }
    }.to_string()
}

// Converts the hexadecimal into RGB notation
fn hex_to_rgb(input: String) -> String {
    let input = input.replace('#', "");
    let mut output = "".to_string();
    for i in (0..6).step_by(2) {
        let character1 = to_decimal(input.chars().nth(i + 1).unwrap().to_string());
        let character2 = to_decimal(input.chars().nth(i).unwrap().to_string()) * 16;
        let color = (character1 + character2).to_string();
        output = output + &color + ",";
    }
    output.truncate(output.len() - 1);
    output
}

// Hex characters and their decimal representations
fn to_decimal(input: String) -> u8 {
    match input.as_str() {
        "0" => 0,
        "1" => 1,
        "2" => 2,
        "3" => 3,
        "4" => 4,
        "5" => 5,
        "6" => 6,
        "7" => 7,
        "8" => 8,
        "9" => 9,
        "a" => 10,
        "b" => 11,
        "c" => 12,
        "d" => 13,
        "e" => 14,
        "f" => 15,

        _ => { 0 }
    }
}

// Converts Rgba color into binary string to be later made into bit_field
fn rgba_to_binary(input: Rgba<i32>, i: usize) -> String {
    format!("{:08b}", input.0.get(i).unwrap().clone())
}