use std::{fs::File, io::BufWriter};
use std::io::Write;

use rand::distributions::Distribution;
use statrs::{StatsError, distribution::Normal};

fn magnitude(v: &[f32]) -> f32 {
    v.iter().map(|x| x * x).sum::<f32>().sqrt()
}

fn generate_sample(n: usize) -> Result<Vec<f32>, StatsError> {
    let mut rng = rand::thread_rng();
    let normal_dist = Normal::new(0.0, 1.0)?;
    let normal_vec: Vec<f32> = normal_dist.sample_iter(&mut rng)
        .take(3 * n).map(|x| x as f32).collect();
    let maxwell_vec: Vec<f32> = normal_vec.chunks(3).map(magnitude).collect();
    Ok(maxwell_vec)
}

pub fn histogram_from_sample(data: &[f32], intervals: usize) -> Vec<(f32, u32)> {
    let max = data.iter().fold(0.0f32, |a, &b| a.max(b));
    //eprintln!("Max: {}", max);
    let mut histogram: Vec<(f32, u32)> = (0..intervals)
        .map(|i| (i as f32 * max / intervals as f32, 0)).collect();
    for x in data {
        let i = (x * (intervals - 1) as f32 / max) as usize;
        histogram[i].1 += 1;
    }
    histogram
}

pub fn task_generate_sample() -> Result<Vec<f32>, StatsError> {
    generate_sample(65536)
}

// pub fn task_plot_histogram(data: &[f32]) { use plotters::prelude::*; let root =
//         SVGBackend::new("histogram.svg", (640, 480)).into_drawing_area();
// 
//     root.fill(&WHITE).unwrap();
// 
//     let mut chart = ChartBuilder::on(&root)
//         .x_label_area_size(35)
//         .y_label_area_size(40)
//         .margin(5)
//         .build_cartesian_2d((0u32..50u32).into_segmented(), 0u32..10000u32)
//         .unwrap();
// 
//     chart.draw_series(
//         Histogram::vertical(&chart)
//             .style(RED.mix(0.5).filled())
//             .data(data.iter().map(|&x| ((x * 10.0) as u32, 1))),
//     ).unwrap();
// }


fn print_histogram<W: Write>(w: &mut W, histogram: &[(f32, u32)], n: usize) {
    //eprintln!("Mode: {}", histogram.iter().max_by_key(|xy| xy.1).map(|xy| xy.0).unwrap());
    for (x, y) in histogram {
        writeln!(w, "{} {}", x, *y as f32 / n as f32).unwrap();
    }
}

pub fn task_print_histogram(histogram: &[(f32, u32)], n: usize) {
    eprintln!("Mode: {}", histogram.iter().max_by_key(|xy| xy.1).map(|xy| xy.0).unwrap());
    for (x, y) in histogram {
        println!("{} {}", x, *y as f32 / n as f32);
    }
}

pub fn maxwell_pdf(x: f32) -> f32 {
    use core::f32::consts::PI;
    const a: f32 = 2.5;
    (2.0 / PI).sqrt() * (x*x * (-x*x / (2.0*a*a)).exp()) / (a * a * a)
}

pub fn task_print_pdf<F: Fn(f32) -> f32>(f: F, n: usize, max: f32) {
    for i in 0..n {
        let x = i as f32 * max / n as f32;
        println!("{} {}", x, f(x));
    }
}

pub fn task_print_windows(data: &[f32], width: usize, intervals: usize) {
    data.windows(width).enumerate().for_each(|(i, window)| {
        let histogram = histogram_from_sample(&window, intervals);
        let mut file = BufWriter::new(
            File::create(format!("windows/window{}.dat", i)).unwrap()
        );
        print_histogram(&mut file, &histogram, width);
    });
}
