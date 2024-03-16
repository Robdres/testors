use std::error::Error;
use std::fs::File;
use csv::Writer; use indicatif::{ProgressBar, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::str::FromStr;

//type v32 = Vec<f32>;

fn main() {
    let bm = read_csv("./BasicMatrix.csv").expect(&format!("Error reading file {}", "./BasicMatrix.csv"));
    let maxes = column_maxes(&bm);
    let minimus = column_mins(&bm).expect("empty input vector or wrong number of columns");
    eprintln!("minimus = {:?}", minimus);
    eprintln!("maxes = {:?}", maxes);
    let averages = column_means(&bm).expect("empty input vector or wrong number of columns");
    eprintln!("averages = {:?}", averages);
    let binary = binarize_by_column(&bm).expect("empty input vector or wrong number of columns");
    to_csv(&binary,String::from("BinaryMatrixMean.csv")).expect("hello");
    let binaryMedians = binarize_by_column_mediasn(&bm).expect("empty input vector or wrong number of columns");
    to_csv(&binaryMedians,String::from("BinaryMatrixMeadian.csv")).expect("hello");
}

fn read_csv(filename: &str) -> Result<Vec<Vec<f32>>, String> {
    let file = File::open(filename).map_err(|e| e.to_string())?;
    let reader = BufReader::new(file);

    let mut rows = Vec::new();

    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        let row: Result<Vec<f32>, _> = line.split(',').map(|s| f32::from_str(s.trim())).collect();

        rows.push(row.map_err(|e| e.to_string())?);
    }

    Ok(rows)
}

fn to_csv(vector:&Vec<Vec<f32>>,name:String) -> Result<(), Box<dyn Error>> {
    eprintln!("Creating CSV");
    let pb = ProgressBar::new(vector.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));
    let mut wtr = Writer::from_path(name)?;
    for k in vector.into_iter(){
        pb.inc(1);
        wtr.write_record(k.iter().map(|e| e.to_string()))?;
    }
    pb.finish();
    eprintln!("Finished CSV");
    wtr.flush()?;
    Ok(())
}

fn column_maxes(v: &Vec<Vec<f32>>) -> Vec<f32> {
    let mut maxes = vec![0.0; v[0].len()];

    for row in v {
        for (i, &val) in row.iter().enumerate() {
            if val > maxes[i] {
                maxes[i] = val;
            }
        }
    }

    maxes
}
fn column_mins(v: &Vec<Vec<f32>>) -> Option<Vec<f32>> {
    if v.is_empty() {
        return None;
    }
    
    let num_columns = v[0].len();
    let mut mins = vec![f32::MAX; num_columns];

    for row in v {
        if row.len() != num_columns {
            return None;
        }

        for (i, &val) in row.iter().enumerate() {
            if val < mins[i] {
                mins[i] = val;
            }
        }
    }

    Some(mins)
}

fn column_means(v: &Vec<Vec<f32>>) -> Option<Vec<f32>> {
    if v.is_empty() {
        return None;
    }

    let num_columns = v[0].len();
    let mut means = vec![0.0; num_columns];

    for row in v {
        if row.len() != num_columns {
            return None;
        }

        for (i, &val) in row.iter().enumerate() {
            means[i] += val;
        }
    }

    let num_rows = v.len() as f32;
    for mean in &mut means {
        *mean /= num_rows;
    }

    Some(means)
}
fn binarize_by_column(v: &Vec<Vec<f32>>) -> Option<Vec<Vec<f32>>> {
    let means = column_means(v)?;

    let mut result = Vec::with_capacity(v.len());
    for row in v {
        if row.len() != means.len() {
            return None;
        }

        let binarized_row = row.iter().enumerate().map(|(i, &val)| {
            if val < means[i] {
                0.0
            } else {
                1.0
            }
        }).collect();

        result.push(binarized_row);
    }

    Some(result)
}

fn binarize_by_column_mediasn(v: &Vec<Vec<f32>>) -> Option<Vec<Vec<f32>>> {
    let means = column_medians(v)?;

    let mut result = Vec::with_capacity(v.len());
    for row in v {
        if row.len() != means.len() {
            return None;
        }

        let binarized_row = row.iter().enumerate().map(|(i, &val)| {
            if val < means[i] {
                0.0
            } else {
                1.0
            }
        }).collect();

        result.push(binarized_row);
    }

    Some(result)
}



fn column_medians(v: &Vec<Vec<f32>>) -> Option<Vec<f32>> {
    if v.is_empty() {
        return None;
    }

    let num_columns = v[0].len();

    let mut medians = Vec::with_capacity(num_columns);

    for i in 0..num_columns {
        let mut column_values: Vec<f32> = v.iter().map(|row| row[i]).collect();
        column_values.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let num_values = column_values.len();

        if num_values % 2 == 0 {
            // If the number of values is even, take the average of the two middle values
            medians.push((column_values[num_values / 2 - 1] + column_values[num_values / 2]) / 2.0);
        } else {
            // If the number of values is odd, take the middle value
            medians.push(column_values[num_values / 2]);
        }
    }

    Some(medians)
}
