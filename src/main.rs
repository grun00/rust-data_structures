use rand::Rng;
use std::time::Instant;
mod sorting;
mod linked_lists;

fn main(){
    let mut rng = rand::thread_rng();

    let vec_size = 100_000;
    let base_vec: Vec<u64> = (0..vec_size).map(|_| rng.gen_range(0, 10000)).collect();
    let mut bubble_vec = base_vec.clone();
    let merge_vec = base_vec.clone();
    let mut quick_vec = base_vec.clone();
    let mut threaded_quick_vec = base_vec.clone();

    println!("=================+ Staring benchmark");

    let start = Instant::now();
    sorting::buble_sort(&mut bubble_vec);
    let duration = start.elapsed();

    println!("Bubble Sort time for {} elements: {:?}", vec_size, duration);

    println!("---------------------------");

    let start = Instant::now();
    sorting::merge_sort(merge_vec);
    let duration = start.elapsed();

    println!("Merge Sort time for {} elements: {:?}", vec_size, duration);

    println!("---------------------------");

    let start = Instant::now();
    sorting::quick_sort(&mut quick_vec);
    let duration = start.elapsed();

    println!("Quick Sort time for {} elements: {:?}", vec_size, duration);

    println!("---------------------------");

    let start = Instant::now();
    sorting::rayon_quick_sort(&mut threaded_quick_vec);
    let duration = start.elapsed();

    println!("Multi Thread Quick Sort time for {} elements: {:?}", vec_size, duration);

    println!("=================+ Ending benchmark");
}
