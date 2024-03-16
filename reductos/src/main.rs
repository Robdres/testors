use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{self, prelude::*};
use csv::ReaderBuilder;
use csv::Writer; use indicatif::{ProgressBar, ProgressStyle};

fn main() {
    let alpha = 0.15;
    let file_path = "./datos/Faces_subset.csv";
    let mut data = read_csv_f(file_path);
    data.remove(0);

    let classes = extract_last_element(&mut data);
    let first_elements = remove_first_element(data);
    let final_data = convert_to_vectors_of_3(first_elements);

    let pb = ProgressBar::new(classes.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}")
    .unwrap().progress_chars(">--"));

    let mut dv :Vec<Vec<f32>> =vec![];

    eprintln!("Creating Discernibility Matrix");
    // //BDM
    // //discenibility vectors
    for (i,j) in classes.iter().enumerate() {
        pb.inc(1);
        for (l,k) in classes[i+1..].iter().enumerate(){
            if k != j {
                let result = euclidean_distances(&final_data[i], &final_data[i+l+1]);
                dv.push(result);
            }
        }
    }
    pb.finish();

    to_csv(&dv, String::from("DicernibilityMatrix.csv")).expect("Error in Creating CSV");
    eprintln!("Finished Dicernibility Matrix");
    
    let mut bm: Vec<Vec<f32>> = dv.clone();
    let mut rows:Vec<usize> = vec![];

    let pb = ProgressBar::new(dv.len() as u64+ rows.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));


    eprintln!("Checking for zeros");
    for (i,j) in dv.iter().enumerate(){
        pb.inc(1);
        if is_afines(j,alpha) {
            rows.push(i);
            continue;
       }
    }
    for j in rows.into_iter().rev() {
        bm.remove(j);
    }
    pb.finish();
    eprintln!("Finished checking for afines");


    let pb = ProgressBar::new(bm.len() as u64);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));

    eprintln!("Creating basic matrix");
    let mut rows:Vec<u8> = vec![];

    'loop1:for (i,j) in bm.iter().enumerate(){
        pb.inc(1);
        for (k,l) in bm[i+1..].iter().enumerate(){
            if is_less(j, l){
                rows.push((k+i+1) as u8);
            }else if less(&l,&j){
                rows.push(i as u8);
                continue 'loop1
            }
        }
    }
    pb.finish();

    let mut rows = filter_uniq(rows);
    rows.sort();
    for j in rows.into_iter().rev() {
        bm.remove(j as usize);
    }
    eprintln!("Finished basic matrix");

    to_csv(&bm,String::from("BasicMatrix.csv")).expect("Error");
}

fn read_csv(values: &mut Vec<Vec<String>>) -> Result<&Vec<Vec<String>>,Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(io::stdin()); //reading whole
    for result in rdr.records() {
        let mut row:Vec<String> = vec![];
        for value in result?.into_iter(){
            row.push(value.to_string());
        }
        values.push(row);
    }
    Ok(values)
}

// fn compare(row1:&Vec<String>,row2:&Vec<String>) -> Vec<u8>{
//     let mut x:Vec<u8> = vec![];
//     for (i,j) in row1.into_iter().enumerate(){
//         if j == &row2[i] {x.push(0)} else {x.push(1)};
//     }
//     x
// }

// fn metric(row1:Vec<f32>,row2:Vec<f32>) -> Vec<Vec<f32>>{
//     let mut x:Vec<Vec<f32>> = vec![];
//     for (i,j) in row1.into_iter().enumerate(){
//         if j == row2[i] {x.push(euclidean_distancs(&row1, &row2))};
//     }
//     x
// }

// much faster
fn is_zero(buf: &Vec<u8>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}

fn less(row1:&Vec<f32>,row2:&Vec<f32>) -> bool {
    let mut _x:u32 = 0;
    for (i,j) in row1.iter().enumerate(){
        if j > &row2[i]{
            _x +=1;
            break
        }else{
            continue
        }
    }
    if _x!=0 {false} else {true}
}

fn filter_uniq(vec: Vec<u8>) -> Vec<u8> {
    vec.into_iter()
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect()
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

fn extract_last_element(vectors: &mut Vec<Vec<f32>>) -> Vec<f32> {
    let mut last_elements = Vec::new();
    for vector in vectors.iter_mut() {
        if let Some(last) = vector.pop() {
            last_elements.push(last);
        }
    }
    last_elements
}

fn read_csv_file(file_path: &str) -> Result<Vec<Vec<f32>>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .from_reader(file);

    let mut vectors = Vec::new();
    for result in reader.records() {
        let record = result?;
        let mut vector = Vec::new();
        for field in record.iter() {
            let value = field.parse::<f32>()?;
            vector.push(value);
        }
        vectors.push(vector);
    }
    Ok(vectors)
}

fn read_csv_f(file_path: &str) -> Vec<Vec<f32>> {
    let file = File::open(file_path).unwrap();
    let lines = io::BufReader::new(file).lines();

    let mut data = Vec::new();
    for line in lines {
        let row = line.unwrap()
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect::<Vec<f32>>();
        data.push(row);
    }
    data
}
fn convert_to_vectors_of_3(data: Vec<Vec<f32>>) -> Vec<Vec<Vec<f32>>> {
    data.into_iter()
        .map(|inner_vec| {
            inner_vec
                .chunks(3)
                .map(|chunk| chunk.to_vec())
                .collect()
        })
        .collect()
}
fn remove_first_element(data: Vec<Vec<f32>>) -> Vec<Vec<f32>> {
    data.into_iter()
        .map(|mut inner_vec| {
            inner_vec.remove(0);
            inner_vec
        })
        .collect()
}

//metric
fn euclidean_distance(v1: &Vec<f32>, v2: &Vec<f32>) -> f32 {
    v1.iter()
        .zip(v2.iter())
        .map(|(&x1, &x2)| (x1 - x2).powi(2))
        .sum::<f32>()
        .sqrt()
}

fn euclidean_distances(
    data1: &Vec<Vec<f32>>,
    data2: &Vec<Vec<f32>>,
) -> Vec<f32> {
    assert_eq!(data1.len(), data2.len());
    let num_samples = data1.len();

    let mut distances = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let dist = euclidean_distance(&data1[i], &data2[i]);
        distances.push(dist);
    }
    distances
}

fn is_less(v1: &Vec<f32>, v2: &Vec<f32>) -> bool {
    if v1.len() != v2.len() {
        panic!("Vectors must be of equal length");
    }
    for i in 0..v1.len() {
        if v1[i] >= v2[i] {
            return false;
        }
    }
    true
}

fn is_afines(v1: &Vec<f32>,alpha:f32) ->bool{
    v1.iter()
        .all(|x| x<&alpha)
}

fn apply_threshold(vectors: &Vec<Vec<f32>>, alpha: f32) -> Vec<Vec<f32>> {
    // Create a new vector to store the output
    let mut result = Vec::new();
    // Loop over each input vector
    for vector in vectors {
        // Create a new vector to store the thresholded values
        let mut thresholded = Vec::new();

        // Loop over each element in the input vector
        for &value in vector {
            // Apply the threshold and append the result to the output vector
            if value < alpha {
                thresholded.push(0.0);
            } else {
                thresholded.push(1.0);
            }
        }
        // Append the thresholded vector to the output
        result.push(thresholded);
    }

    // Return the output vector
    result
}
fn create_alpha_basic_matrix(alpha:f32, data:&Vec<Vec<f32>>) {

}
