use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;

const WIDTH: u32 = 2400;
const HEIGHT: u32 = 1800;
const MAX_ITER: u32 = 500;
const OUTPUT_FILE: &str = "wiki_field_lines.png";
// Source wikipedia
// https://en.wikipedia.org/wiki/Julia_set

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating field lines for the rational map...");

    let (x_min, x_max, y_min, y_max) = (-2.5, 2.5, -1.875, 1.875);
    let c = Complex::new(0.3, 0.6);


    let pixels: Vec<Rgb<u8>> = (0..HEIGHT)
        .into_par_iter()
        .flat_map_iter(|py| {
            (0..WIDTH).map(move |px| {
                let cx = x_min + (px as f64 / WIDTH as f64) * (x_max - x_min);
                let cy = y_min + (py as f64 / HEIGHT as f64) * (y_max - y_min);
                let z = Complex::new(cx, cy);

                let result = iterate_rational_map(z, c);
                calculate_color(result)
            })
        })
        .collect();

    // Image Buffer -> I was getting this wrong earlier
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    for (i, pixel) in pixels.into_iter().enumerate() {
        let px = (i as u32) % WIDTH;
        let py = (i as u32) / WIDTH;
        img.put_pixel(px, py, pixel);
    }

    img.save(OUTPUT_FILE)?;
    //println!("Image successfully saved to: {}", OUTPUT_FILE);

    Ok(())
}

fn iterate_rational_map(mut z: Complex<f64>, c: Complex<f64>) -> (u32, Complex<f64>) {
    for i in 0..MAX_ITER {
        let numerator = Complex::new(1.0, 0.0) - (z.powi(3) / 6.0);
        let denom_inner = z - (z.powi(2) / 2.0);
        let denominator = denom_inner.powi(2);

        if denominator.norm_sqr() < 1e-12 {
            return (i, Complex::new(1e10, 1e10));
        }

        z = (numerator / denominator) + c;

        if z.norm_sqr() > 16.0 {
            return (i, z);
        }
    }
    (MAX_ITER, Complex::new(0.0, 0.0))
}

/// Generates a two-tone Blue & White color map based on escape dynamics and field lines
fn calculate_color(data: (u32, Complex<f64>)) -> Rgb<u8> {
    let (iters, z_final) = data;

    // Interior points: Deep midnight blue
    if iters == MAX_ITER {
        return Rgb([5, 15, 45]);
    }

    let smooth_iters = iters as f64;
    let escape_angle = z_final.im.atan2(z_final.re);

    // Compute the phase stripe pattern from the field lines
    let phase_stripes = ((smooth_iters * 0.12) + (escape_angle / std::f64::consts::PI * 3.0)).sin();
    let field_intensity = 0.5 + 0.5 * phase_stripes;

    // Normalized blend factor oscillating smoothly between 0.0 and 1.0
    let factor = (0.5 + 0.5 * (smooth_iters * 0.08 + field_intensity * 2.5).sin()).clamp(0.0, 1.0);

    // Palette endpoints
    let blue = [10.0, 50.0, 180.0];  // Deep cobalt blue
    let white = [255.0, 255.0, 255.0]; // Pure white

    // Linear interpolation: blue -> white
    let r = (blue[0] + (white[0] - blue[0]) * factor) as u8;
    let g = (blue[1] + (white[1] - blue[1]) * factor) as u8;
    let b = (blue[2] + (white[2] - blue[2]) * factor) as u8;

    Rgb([r, g, b])
}

// fn calculate_color(data: (u32, Complex<f64>)) -> Rgb<u8> {
//     let (iters, z_final) = data;

//     if iters == MAX_ITER {
//         return Rgb([20, 0, 40]);
//     }

//     let smooth_iters = iters as f64;
//     let escape_angle = z_final.im.atan2(z_final.re);

//     let phase_stripes = ((smooth_iters * 0.1) + (escape_angle / std::f64::consts::PI * 2.0)).sin();
//     let field_intensity = 0.5 + 0.5 * phase_stripes;

//     let t = (smooth_iters + field_intensity * 8.0) * 0.03;

//     let r = ((0.5 + 0.5 * (6.28318 * (t + 0.00)).cos()) * 255.0) as u8;
//     let g = ((0.5 + 0.5 * (6.28318 * (t + 0.15)).cos()) * 255.0) as u8;
//     let b = ((0.5 + 0.5 * (6.28318 * (t + 0.50)).cos()) * 255.0) as u8;

//     Rgb([r, g, b])
// }