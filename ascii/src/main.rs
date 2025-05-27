use image::{GenericImageView, ImageBuffer, Rgb};
use term_size::dimensions;
use termion::color;

fn main() {
    let mut img = match image::open("../input5.png") {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Ошибка при открытии изображения: {}", e);
            return;
        }
    };

    img = img.adjust_contrast(20.0);
    let symbol_ratio = 0.5;
    let aspect_ratio = img.width() as f32 / img.height() as f32;

    // Желаемая высота ASCII арта
    let target_height = 50;

    let target_width = ((target_height as f32 * aspect_ratio) / symbol_ratio).ceil() as u32;
    let img = img.resize_exact(target_width, target_height, image::imageops::Lanczos3);
    let ascii_chars = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@'];
    let colors = [
        color::Rgb(255, 255, 255),
        color::Rgb(200, 200, 200),
        color::Rgb(100, 100, 100),
        color::Rgb(255, 50, 50),
        color::Rgb(200, 0, 0),
        color::Rgb(50, 255, 50),
        color::Rgb(0, 200, 0),
        color::Rgb(50, 50, 255),
        color::Rgb(0, 0, 200),
        color::Rgb(255, 255, 50),
        color::Rgb(255, 50, 255),
        color::Rgb(50, 255, 255),
        color::Rgb(200, 100, 0),
        color::Rgb(150, 50, 150),
    ];
    let mut ascii_art = Vec::new();

    for y in 0..target_height {
        let mut row = String::new();
        for x in 0..target_width {
            let pixel = img.get_pixel(x, y);
            let rgb = pixel.0;
            
            let intensity = (0.299 * rgb[0] as f32 + 0.587 * rgb[1] as f32 + 0.114 * rgb[2] as f32) as u8;
            let char_index = ((intensity as f32 / 255.0).powf(0.6) * (ascii_chars.len() - 1) as f32) as usize;

            let selected_color = colors.iter().min_by_key(|&&color| {
                let dr = rgb[0] as i32 - color.0 as i32;
                let dg = rgb[1] as i32 - color.1 as i32;
                let db = rgb[2] as i32 - color.2 as i32;
                dr * dr + dg * dg + db * db
            }).unwrap();

            row.push_str(&format!(
                "{}{}",
                color::Fg(*selected_color),
                ascii_chars[char_index]
            ));
        }
        row.push_str(&color::Fg(color::Reset).to_string());
        ascii_art.push(row);
    }

    for row in ascii_art {
        println!("{}", row);
    }
}