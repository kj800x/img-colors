use kmeans::*;
extern crate rand;
use std::env;

fn flatten<T>(nested: Vec<Vec<T>>) -> Vec<T> {
    nested.into_iter().flatten().collect()
}

fn detect_colors(pixels: &Vec<u8>) {
    let sample_dims = 3;
    let k = 3;
    let max_iter = 100;
    let sample_count = pixels.len();

    let kmean = KMeans::new(
        pixels.into_iter().map(|&x| f64::from(x)).collect(),
        sample_count,
        sample_dims,
    );
    let result = kmean.kmeans(
        k,
        max_iter,
        KMeans::init_kmeanplusplus,
        &mut rand::thread_rng(),
    );

    println!("Centroids: {:?}", result.centroids);
    println!("Cluster-Assignments: {:?}", result.assignments);
    println!("Error: {}", result.distsum);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let imageResult = image::open(&args[1]).expect("Expected argument to be a valid image");

    detect_colors(
        &imageResult
            .as_rgb8()
            .expect("Expected to be able to treat as rgb")
            .into_raw(),
    )
}

