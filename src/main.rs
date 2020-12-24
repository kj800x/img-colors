use kmeans::*;
extern crate rand;
use std::env;

// fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
//     nested.into_iter().flatten().collect()
// }

fn detect_colors(pixels: Vec<u8>) -> Vec<f64> {
    let sample_dims = 3;
    let k = 3;
    let max_iter = 10000;
    let sample_count = pixels.len() / 3;

    let kmean = KMeans::new(
        pixels.into_iter().map(|x| f64::from(x)).collect(),
        sample_count - 1,
        sample_dims,
    );
    let result = kmean.kmeans(
        k,
        max_iter,
        KMeans::init_kmeanplusplus,
        &mut rand::thread_rng(),
    );

    result.centroids
}

fn print_colors(color_centers: Vec<u64>) {
    println!(
        "#{:02x}{:02x}{:02x}",
        color_centers[0], color_centers[1], color_centers[2]
    );
    println!(
        "#{:02x}{:02x}{:02x}",
        color_centers[3], color_centers[4], color_centers[5]
    );
    println!(
        "#{:02x}{:02x}{:02x}",
        color_centers[6], color_centers[7], color_centers[8]
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let image_result = image::open(&args[1]).expect("Expected argument to be a valid image");

    let rbg = image_result.into_rgb();

    let colors = detect_colors(rbg.into_raw());

    let color_centers: Vec<u64> = colors.into_iter().map(|x| x.round() as u64).collect();

    print_colors(color_centers);
}
