use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Define the output file and canvas resolution (800x600 px)
    let root = BitMapBackend::new("math_functions.png", (800, 600)).into_drawing_area();
    
    // 2. Set canvas background color
    root.fill(&WHITE)?;

    // 3. Build the chart configuration: bounds, margins, and label styling
    let mut chart = ChartBuilder::on(&root)
        .caption("Trigonometric & Damped Functions", ("sans-serif", 30).into_font())
        .margin(15)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(-3.14f64..3.14f64, -1.2f64..1.2f64)?;

    // 4. Configure grid and axes
    chart
        .configure_mesh()
        .x_labels(10)
        .y_labels(10)
        .x_desc("x")
        .y_desc("f(x)")
        .axis_desc_style(("sans-serif", 15))
        .draw()?;

    // 5. Plot f(x) = sin(x) in Blue
    chart
        .draw_series(LineSeries::new(
            (-314..=314).map(|x| x as f64 / 100.0).map(|x| (x, x.sin())),
            &BLUE,
        ))?
        .label("sin(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    // 6. Plot f(x) = cos(x) in Red
    chart
        .draw_series(LineSeries::new(
            (-314..=314).map(|x| x as f64 / 100.0).map(|x| (x, x.cos())),
            &RED,
        ))?
        .label("cos(x)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    // 7. Plot f(x) = sin(3x) * exp(-0.5x) in Green (Damped oscillation)
    chart
        .draw_series(LineSeries::new(
            (-314..=314).map(|x| x as f64 / 100.0).map(|x| (x, (3.0 * x).sin() * (-0.5 * x.abs()).exp())),
            &GREEN,
        ))?
        .label("sin(3x) * e^(-0.5|x|)")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &GREEN));

    // 8. Draw the legend box
    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .position(SeriesLabelPosition::UpperRight)
        .draw()?;

    // 9. Flush/write buffers to disk
    root.present()?;
    println!("Graph successfully saved as math_functions.png");

    Ok(())
}