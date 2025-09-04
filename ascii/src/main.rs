use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;
use termion::color;

const WIDTH: usize = 100;
const HEIGHT: usize = 100;
const ASPECT: f32 = 2.0;
const FOV: f32 = 50.0;
const NEAR: f32 = 1.0;
const FAR: f32 = 7.0;  // Увеличил дальность для большего диапазона глубины
const Y_OFFSET: f32 = 25.0;
const SYMBOLS: [char; 12] = [' ', '.', ':', '-', '=', '+', '*', '#', '%', '@', '&', '§']; // Добавил больше символов'.', ':', '-', '=', '+', '*', '#', '%', '@'];

const MINI_WIDTH: usize = 30;
const MINI_HEIGHT: usize = 15;
const MINI_OFFSET_X: usize = WIDTH - MINI_WIDTH - 2;
const MINI_OFFSET_Y: usize = 2;

#[derive(Clone, Copy)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Copy)]
struct Triangle {
    vertices: [Vec3; 3],
    color: char,
}

fn rotate_y(v: Vec3, angle: f32) -> Vec3 {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    Vec3 {
        x: cos_a * v.x + sin_a * v.z,
        y: v.y,
        z: -sin_a * v.x + cos_a * v.z,
    }
}

fn rotate_x(v: Vec3, angle: f32) -> Vec3 {
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    Vec3 {
        x: v.x,
        y: cos_a * v.y - sin_a * v.z,
        z: sin_a * v.y + cos_a * v.z,
    }
}

fn project(v: Vec3) -> (f32, f32) {
    let fov_rad = FOV.to_radians();
    let scale = 1.0 / (fov_rad / 2.0).tan();
    
    // Без инверсии по Y (убрал минус перед v.y)
    let x = -v.x / v.z * scale * (WIDTH as f32 / 2.0) * ASPECT + (WIDTH as f32 / 2.0);
    let y = -v.y / v.z * scale * (HEIGHT as f32 / 2.0) + (HEIGHT as f32 / 2.0) + Y_OFFSET;
    (x, y)
}

fn project_mini(v: Vec3) -> (f32, f32) {
    let scale = 0.5;
    let x = v.x * scale + (MINI_WIDTH as f32 / 2.0);
    let y = v.z * scale + (MINI_HEIGHT as f32 / 2.0); // Убрал инверсию и здесь
    (x, y)
}

fn draw_triangle(buffer: &mut [String], zbuffer: &mut [f32], tri: Triangle, is_mini: bool) {
    let mut projected = [(0.0, 0.0); 3];
    let mut depths = [0.0; 3];
    
    for i in 0..3 {
        let v = tri.vertices[i];
        if is_mini {
            projected[i] = project_mini(v);
            depths[i] = v.y;
        } else {
            if v.z <= NEAR { return; }
            projected[i] = project(v);
            depths[i] = v.z;
        }
    }

    // Backface culling
    let v1 = Vec3 {
        x: projected[1].0 - projected[0].0,
        y: projected[1].1 - projected[0].1,
        z: depths[1] - depths[0],
    };
    let v2 = Vec3 {
        x: projected[2].0 - projected[0].0,
        y: projected[2].1 - projected[0].1,
        z: depths[2] - depths[0],
    };
    let normal_z = v1.x * v2.y - v1.y * v2.x;
    if normal_z <= 0.0 { return; }

    let (width, height) = if is_mini {
        (MINI_WIDTH, MINI_HEIGHT)
    } else {
        (WIDTH, HEIGHT)
    };

    let (min_x, max_x) = (
        projected.iter().map(|p| p.0.floor() as i32).min().unwrap().max(0),
        projected.iter().map(|p| p.0.ceil() as i32).max().unwrap().min(width as i32 - 1),
    );
    let (min_y, max_y) = (
        projected.iter().map(|p| p.1.floor() as i32).min().unwrap().max(0),
        projected.iter().map(|p| p.1.ceil() as i32).max().unwrap().min(height as i32 - 1),
    );

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let idx = if is_mini {
                (y as usize + MINI_OFFSET_Y) * WIDTH + (x as usize + MINI_OFFSET_X)
            } else {
                y as usize * WIDTH + x as usize
            };
            
            let p = Vec3 {
                x: x as f32 + 0.5,
                y: y as f32 + 0.5,
                z: 0.0,
            };
            let v0 = Vec3 {
                x: projected[0].0,
                y: projected[0].1,
                z: 0.0,
            };
            let v1 = Vec3 {
                x: projected[1].0,
                y: projected[1].1,
                z: 0.0,
            };
            let v2 = Vec3 {
                x: projected[2].0,
                y: projected[2].1,
                z: 0.0,
            };
            
            let d00 = (v1.x - v0.x) * (v1.x - v0.x) + (v1.y - v0.y) * (v1.y - v0.y);
            let d01 = (v1.x - v0.x) * (v2.x - v0.x) + (v1.y - v0.y) * (v2.y - v0.y);
            let d11 = (v2.x - v0.x) * (v2.x - v0.x) + (v2.y - v0.y) * (v2.y - v0.y);
            let d20 = (p.x - v0.x) * (v1.x - v0.x) + (p.y - v0.y) * (v1.y - v0.y);
            let d21 = (p.x - v0.x) * (v2.x - v0.x) + (p.y - v0.y) * (v2.y - v0.y);
            let denom = d00 * d11 - d01 * d01;
            if denom.abs() < 1e-6 { continue; }
            
            let u = (d11 * d20 - d01 * d21) / denom;
            let v = (d00 * d21 - d01 * d20) / denom;
            let w = 1.0 - u - v;
            
            if u >= 0.0 && v >= 0.0 && w >= 0.0 {
                let depth = u * depths[0] + v * depths[1] + w * depths[2];
                let depth_range = if is_mini { 5.0 } else { FAR - NEAR };
                let normalized_depth = if is_mini {
                    (depth + 2.5) / 5.0
                } else {
                    (depth - NEAR) / depth_range
                };
                
                if normalized_depth >= 0.0 && normalized_depth <= 1.0 {
                    let symbol_idx = ((1.0 - normalized_depth) * (SYMBOLS.len() - 1) as f32).round() as usize;
                    let symbol = SYMBOLS[symbol_idx];
                    
                    let color = match tri.color {
                        'R' => color::Fg(color::Rgb(255, 50, 50)),
                        'B' => color::Fg(color::Rgb(50, 50, 255)),
                        'G' => color::Fg(color::Rgb(50, 255, 50)),
                        'W' => color::Fg(color::Rgb(200, 200, 200)),
                        _ => color::Fg(color::Rgb(200, 200, 200)),
                    };
                    
                    if depth < zbuffer[idx] {
                        zbuffer[idx] = depth;
                        buffer[idx] = format!("{}{}", color, symbol);
                    }
                }
            }
        }
    }
}

fn main() {
    let s = 0.6;
    let vertices = [
        Vec3 { x: 0.0, y: s, z: 0.0 },
        Vec3 { x: -s * 0.707, y: -s / 1.732, z: -s * 0.408 },
        Vec3 { x: s * 0.707, y: -s / 1.732, z: -s * 0.408 },
        Vec3 { x: 0.0, y: -s / 1.732, z: s * 0.816 },
    ];

    let triangles = [
        Triangle { vertices: [vertices[0], vertices[1], vertices[2]], color: 'R' },
        Triangle { vertices: [vertices[0], vertices[2], vertices[3]], color: 'B' },
        Triangle { vertices: [vertices[0], vertices[3], vertices[1]], color: 'G' },
        Triangle { vertices: [vertices[1], vertices[3], vertices[2]], color: 'W' },
    ];

    let mut buffer = vec![String::from(" "); WIDTH * HEIGHT];
    let mut zbuffer = vec![FAR; WIDTH * HEIGHT];
    let mut angle = 0.0;

    loop {
        buffer.fill(String::from(" "));
        zbuffer.fill(FAR);

        let mut rotated_triangles = triangles;
        for tri in rotated_triangles.iter_mut() {
            for v in tri.vertices.iter_mut() {
                *v = rotate_y(*v, angle);
                v.z += 3.0;
            }
            draw_triangle(&mut buffer, &mut zbuffer, *tri, false);
            
            let mut mini_tri = *tri;
            for v in mini_tri.vertices.iter_mut() {
                *v = rotate_x(*v, std::f32::consts::PI / 2.0);
            }
            draw_triangle(&mut buffer, &mut zbuffer, mini_tri, true);
        }

        print!("\x1B[2J\x1B[1;1H");
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                print!("{}", buffer[y * WIDTH + x]);
            }
            println!("{}", color::Fg(color::Reset));
        }
        stdout().flush().unwrap();

        angle += 0.05;
        thread::sleep(Duration::from_millis(40));
    }
}