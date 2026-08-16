use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;

const WIDTH: u32 = 1800;
const HEIGHT: u32 = 1350;
const MAX_ITER: usize = 160;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Rendering rational Julia set field lines...");

    // Viewport window framing the structure
    let (x_min, x_max) = (-1.8, 1.8);
    let (y_min, y_max) = (-1.35, 1.35);

    // Wikipedia illustration parameter
    let c = Complex::new(-0.55, -0.55);

    let pixels: Vec<Rgb<u8>> = (0..HEIGHT)
        .into_par_iter()
        .flat_map_iter(|py| {
            (0..WIDTH).map(move |px| {
                let cx = x_min + (px as f64 / WIDTH as f64) * (x_max - x_min);
                // Invert y so positive is upright
                let cy = y_max - (py as f64 / HEIGHT as f64) * (y_max - y_min);
                let z = Complex::new(cx, cy);

                render_pixel(z, c)
            })
        })
        .collect();

    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    for (i, pixel) in pixels.into_iter().enumerate() {
        let px = (i as u32) % WIDTH;
        let py = (i as u32) / WIDTH;
        img.put_pixel(px, py, pixel);
    }

    img.save("rational_julia_fieldlines.png")?;
    println!("Saved crisp output to rational_julia_fieldlines.png");

    Ok(())
}

fn render_pixel(mut z: Complex<f64>, c: Complex<f64>) -> Rgb<u8> {
    let mut sum_angle = 0.0_f64;
    let mut min_dist = 1e10_f64;
    let mut iters_done = 0;

    for i in 0..MAX_ITER {
        // Evaluate: f(z) = (1 - z^3/6) / (z - z^2/2)^2 + c
        let z2 = z * z;
        let z3 = z2 * z;
        let num = Complex::new(1.0, 0.0) - z3 / 6.0;
        let denom_inner = z - z2 / 2.0;
        let denom = denom_inner * denom_inner;

        if denom.norm_sqr() < 1e-12 {
            break;
        }

        let z_next = (num / denom) + c;
        let step_diff = z_next - z;
        let dist = step_diff.norm();

        // Accumulate phase rotation across steps (creates smooth dynamic field rays)
        sum_angle += step_diff.im.atan2(step_diff.re);

        // Track orbit trap distance
        if dist < min_dist {
            min_dist = dist;
        }

        z = z_next;
        iters_done = i;

        // Orbit cycle convergence
        if dist < 1e-5 {
            break;
        }
    }

    // Combine field stream angle, iteration count, and orbit proximity
    let field_stripe = (sum_angle * 1.5).sin();
    let trap_factor = (-min_dist.ln().max(-5.0) / 5.0).clamp(0.0, 1.0);
    let t = (iters_done as f64 * 0.04) + field_stripe * 0.35 + trap_factor * 0.4;

    // Direct chromatic mapping matching the Wikipedia palette:
    // Golden Orange -> Violet / Magenta -> Sky Blue -> Cyan
    let r = ((0.55 + 0.45 * (6.28318 * (t + 0.05)).cos()) * 255.0).clamp(0.0, 255.0) as u8;
    let g = ((0.45 + 0.45 * (6.28318 * (t + 0.30)).cos()) * 255.0).clamp(0.0, 255.0) as u8;
    let b = ((0.60 + 0.40 * (6.28318 * (t + 0.65)).cos()) * 255.0).clamp(0.0, 255.0) as u8;

    Rgb([r, g, b])
}