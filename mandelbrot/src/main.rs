use image::{ImageBuffer, Rgb};
use num_complex::Complex;
use rayon::prelude::*;

const WIDTH: u32 = 1200;
const HEIGHT: u32 = 900;
const MAX_ITER: u32 = 500;
const BORDER_THICKNESS: u32 = 12;

#[derive(Clone, Copy)]
enum FractalType {
    Mandelbrot,
    Julia(Complex<f64>),
    Mandelbar, // Conjugated Mandelbrot (Tricorn)
}

/// Compute escape iterations and smooth fractional escape value
fn evaluate_pixel(c: Complex<f64>, fractal: FractalType) -> Option<f64> {
    let (mut z, constant) = match fractal {
        FractalType::Mandelbrot => (Complex::new(0.0, 0.0), c),
        FractalType::Julia(seed) => (c, seed),
        FractalType::Mandelbar => (Complex::new(0.0, 0.0), c)
    };

    for i in 0..MAX_ITER {
        let norm_sq = z.norm_sqr();
        if norm_sq > 4.0 {
            // Continuous potential formula for smooth coloring (no color banding)
            let smooth = i as f64 + 1.0 - (norm_sq.ln() / 2.0).ln() / 2.0_f64.ln();
            return Some(smooth.max(0.0));
        }

        z = match fractal {
            FractalType::Mandelbrot | FractalType::Julia(_) => z * z + constant,
            FractalType::Mandelbar => Complex::new(z.re, -z.im).powi(2) + constant,
        };
    }
    None // Inside the set
}

/// Cosine-based smooth color palette generator
fn palette(t: f64) -> Rgb<u8> {
    let t = t * 0.05; // Adjust cycle frequency
    let r = ((0.5 + 0.5 * (6.28318 * (t + 0.0)).cos()) * 255.0) as u8;
    let g = ((0.5 + 0.5 * (6.28318 * (t + 0.33)).cos()) * 255.0) as u8;
    let b = ((0.5 + 0.5 * (6.28318 * (t + 0.67)).cos()) * 255.0) as u8;
    Rgb([r, g, b])
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Choose your fractal:
    // 1. FractalType::Mandelbrot
    // 2. FractalType::Julia(Complex::new(-0.7, 0.27015))
    // 3. FractalType::Mandelbar
    let fractal_mode = FractalType::Julia(Complex::new( 0.4 , 0.4));
    //  0.285 + 0.01, −0.70176 − 0.3842,  −0.835 − 0.2321, −0.8 + 0.156
    // −0.7269 + 0.1889 ,  0.35 + 0.35, = 0.4 + 0.4
    // Coordinate mapping viewport: (x_min, x_max, y_min, y_max)
    let (x_min, x_max, y_min, y_max) = match fractal_mode {
        FractalType::Mandelbrot => (-2.1, 0.7, -1.2, 1.2),
        FractalType::Julia(_) => (-1.5, 1.5, -1.2, 1.2),
        FractalType::Mandelbar => (-2.2, 1.0, -1.3, 1.3),
    };

    // Parallel row generation with Rayon
    let rows: Vec<Vec<Rgb<u8>>> = (0..HEIGHT)
        .into_par_iter()
        .map(|py| {
            (0..WIDTH)
                .map(|px| {
                    // 1. Draw decorative border frame
                    if px < BORDER_THICKNESS
                        || px >= WIDTH - BORDER_THICKNESS
                        || py < BORDER_THICKNESS
                        || py >= HEIGHT - BORDER_THICKNESS
                    {
                        return Rgb([240, 190, 40]); // Gold frame border
                    }

                    // 2. Map pixel to complex coordinate plane
                    let cx = x_min + (px as f64 / WIDTH as f64) * (x_max - x_min);
                    let cy = y_min + (py as f64 / HEIGHT as f64) * (y_max - y_min);
                    let c = Complex::new(cx, cy);

                    // 3. Determine color
                    match evaluate_pixel(c, fractal_mode) {
                        Some(smooth_iter) => palette(smooth_iter),
                        None => Rgb([12, 12, 28]), // Dark navy interior for the set body
                    }
                })
                .collect()
        })
        .collect();

    // Assemble the flat image buffer and write to disk
    let mut img = ImageBuffer::new(WIDTH, HEIGHT);
    for (py, row) in rows.into_iter().enumerate() {
        for (px, pixel) in row.into_iter().enumerate() {
            img.put_pixel(px as u32, py as u32, pixel);
        }
    }

    img.save("fractal_output.png")?;
    println!("Fractal successfully exported to fractal_output.png");

    Ok(())
}