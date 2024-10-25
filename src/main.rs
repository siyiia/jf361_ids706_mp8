use csv::ReaderBuilder;
use ndarray::{s, Array2, Axis};
use std::error::Error;
use std::fs::File;
use std::time::Instant;
use sysinfo::{ProcessExt, System, SystemExt};

fn analyse_data(file_path: &str) -> Result<(Vec<String>, Array2<f64>), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new().has_headers(true).from_reader(file);

    let headers: Vec<String> = reader.headers()?.iter().map(|h| h.to_string()).collect();

    let records: Vec<f64> = reader
        .records()
        .flat_map(|result| {
            result
                .ok()
                .map(|record| {
                    record
                        .iter()
                        .filter_map(|x| x.parse::<f64>().ok())
                        .collect::<Vec<f64>>()
                })
                .unwrap_or_default()
        })
        .collect();

    let num_rows = records.len() / headers.len();
    let num_cols = headers.len();
    let data = Array2::from_shape_vec((num_rows, num_cols), records)?;

    let means = data.mean_axis(Axis(0)).unwrap();
    let std_devs = data.std_axis(Axis(0), 1.0);

    let mut stats_data = Array2::zeros((2, num_cols));
    stats_data.slice_mut(s![0, ..]).assign(&means);
    stats_data.slice_mut(s![1, ..]).assign(&std_devs);

    Ok((headers, stats_data))
}

fn display_stats(headers: &[String], stats_data: &Array2<f64>) {
    print!("{:<10}", "");
    for header in headers {
        print!("{:<15}", header);
    }
    println!();

    let row_labels = ["mean", "std"];

    for (i, &label) in row_labels.iter().enumerate() {
        print!("{:<10}", label);
        for &value in stats_data.row(i) {
            print!("{:<15.2}", value);
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = "StudentPerformanceFactors.csv";

    let mut system = System::new_all();
    system.refresh_processes();

    let process_id = sysinfo::get_current_pid().unwrap();
    let initial_memory = system.process(process_id).unwrap().memory();

    let start_time = Instant::now();

    let (headers, stats_data) = analyse_data(file_path)?;

    let elapsed_time = start_time.elapsed().as_secs_f64();

    system.refresh_all();
    let final_memory = system.process(process_id).unwrap().memory();
    let memory_used = final_memory - initial_memory;

    display_stats(&headers, &stats_data);

    println!("\nTime taken: {:.4?} seconds", elapsed_time);
    println!("Memory used: {} KB", memory_used);

    Ok(())
}
