fn main() {
    const INTERVALS: usize = 15;
    let sample = lib_maxwell_mvs::task_generate_sample().unwrap();
    let histogram = lib_maxwell_mvs::histogram_from_sample(&sample, INTERVALS);
    lib_maxwell_mvs::task_print_histogram(&histogram, 65536);
    //lib_maxwell_mvs::task_print_pdf(lib_maxwell_mvs::maxwell_pdf, 100, 20.0);
    //lib_maxwell_mvs::task_print_windows(&sample, 256, INTERVALS);
}
