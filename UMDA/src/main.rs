
use std::collections::HashSet;
use std::fs::File;
use std::io::{self, prelude::*};
use std::process;
use std::error::Error;
use csv::Writer;
use rand::Rng;
use indicatif::{ProgressBar, ProgressStyle};

fn main() { 

    let mut rng = rand::thread_rng();
    let mut n = 100;
    let alpha =0.1;
    let mut values = read_csv_f("../reductos/BasicMatrix.csv");

    let mut initial_distribution:Vec<f32> = Vec::from([0.5; 68]);

    let mut initial_poblation:Vec<Vec<i8>> = vec![];


    let pb = ProgressBar::new(n);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));

    loop{
        let mut cromosome:Vec<Vec<f32>> =vec![];
        let chrom = generate_binary_array_with_n_ones(52);
        let indexes = find_ones_indices(&chrom);
        for (_,k) in values.iter().enumerate(){
            let mut x:Vec<f32>= vec![];
            for i in &indexes{
                x.push(k[*i as usize]);
            }
            cromosome.push(x);
        }
        let result = fitness_functon(&cromosome);
        if result>0.98{
            initial_poblation.push(chrom);
            pb.inc(1);
        }
        if initial_poblation.len()== n as usize{
            break;
        }
    }
    eprintln!("get_distributions() = {:?}", get_distributions(&initial_poblation));
    eprintln!("calculate_marginal_frequencies(![&initial_poblation]) = {:?}", calculate_marginal_frequencies(&initial_poblation));

    let mut VariancesResults: Vec<Vec<u32>> = vec![];
    let mut results: Vec<Vec<f32>> = vec![];
    let pb = ProgressBar::new(68);
    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));

    loop {
        let mut random_index:Vec<u32> = vec![];
        let mut rng = rand::thread_rng();
        while random_index.len()<n{
            let value = rng.gen_range(0..values[0].len()-1) as u32;
            if !random_index.contains(&value){
                random_index.push(value)
            }
        }

        let mut cromosome:Vec<Vec<f32>> =vec![];
        for (_,k) in values.iter().enumerate(){
            let mut x:Vec<f32>= vec![];
            for i in &random_index{
                x.push(k[*i as usize]);
            }
            cromosome.push(x);
        }
        VariancesResults.push(random_index);


        let result = fitness_functon(&cromosome);

        if result > 0.98{
            break
        }else{
            n+=1;
            pb.inc(1);
        }
    }

    let pb = ProgressBar::new(50);

    pb.set_style(ProgressStyle::with_template("[{elapsed_precise}/{eta}] {bar:60.cyan/blue} {pos:>7}/{len:7} {msg}").unwrap()
    .progress_chars(">--"));


    eprintln!("Getting chromosomes");

    while results.len()<50{
        let mut random_index:Vec<u32> = vec![];
        let mut rng = rand::thread_rng();

        while random_index.len()<n{
            let value = rng.gen_range(0..68) as u32;
            if !random_index.contains(&value){
                random_index.push(value)
            }
        }

        let mut cromosome:Vec<Vec<f32>> =vec![];
        for (_,k) in values.iter().enumerate(){
            let mut x:Vec<f32>= vec![];
            for i in &random_index{
                x.push(k[*i as usize]);
            }
            cromosome.push(x);
        }

        let result = fitness_functon(&cromosome);
        if result > 0.98 {
            pb.inc(1);
            random_index.sort();
            let mut vector:Vec<f32> = random_index.iter().map(|x| *x as f32).collect();
            vector.push(result);
            results.push(vector);
        }
    }

    print!("Finished chromosomes");
    to_csv(&results, String::from("GenerationsCromosomes.csv")).expect("Error in CSV");
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

fn read_csv(values: &mut Vec<Vec<u8>>) -> Result<&Vec<Vec<u8>>,Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(io::stdin());
    for result in rdr.records() {
        let mut row:Vec<u8> = vec![];
        for value in result?.into_iter(){
            row.push(value.parse::<u8>().unwrap());
        }
        values.push(row);
    }
    Ok(values)
}


fn fitness_functon(cromosome:&Vec<Vec<f32>>) -> f32 {
    let mut _t:u32 = 0;
    let mut _p:u32 = 0;
    let alpha:f32 = 0.1;
    let mut sum:u32 = 0;

    //get t(x_n)
    for i in cromosome{
        if is_afines(i,alpha) {
            _t += 1;
        }
    }
    //get p(x_n)
    for i in 0..cromosome[0].len()-1{
        if count_vectors_below_threshold(cromosome,i,alpha) > 1.0 as usize{
            _p+=1;
        }
    }
    let ans:f32 = 0.2*((cromosome.len() as f32 -_t as f32)/cromosome.len() as f32) + (1.0-0.2)*((_p as f32/cromosome[0].len() as f32));
    ans
}

fn is_zero(buf: &Vec<u8>) -> bool {
    let (prefix, aligned, suffix) = unsafe { buf.align_to::<u128>() };

    prefix.iter().all(|&x| x == 0)
        && suffix.iter().all(|&x| x == 0)
        && aligned.iter().all(|&x| x == 0)
}


fn filter_uniq(vec: Vec<u8>) -> Vec<u8> {
    vec.into_iter()
        .collect::<HashSet<u8>>()
        .into_iter()
        .collect()
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

fn is_afines(v1: &Vec<f32>,alpha:f32) ->bool{
    v1.iter()
        .all(|x| x<&alpha)
}

fn check_threshold(vec: &Vec<f32>, i: usize, threshold: f32) -> bool {
    let mut new_vec = vec.clone();
    new_vec.remove(i);
    new_vec.iter().all(|&x| x < threshold)
}
fn count_vectors_below_threshold(vectors: &Vec<Vec<f32>>, j: usize, threshold: f32) -> usize {
    vectors.iter().filter(|&v| {
        let mut new_v = v.clone();
        new_v.remove(j);
        new_v.iter().all(|&x| x < threshold)
    }).count()

}

fn get_random_row(distribution:&Vec<f32>)->Vec<i8>{
    let mut chromosome:Vec<i8> = vec![];
    let mut rng = rand::thread_rng();
    for i in 0..distribution.len(){
        let random_number: f32 = rng.gen(); // generate a random number between 0 and 1
        if random_number < distribution[i]{
            chromosome.push(0);
        }else{
            chromosome.push(1);
        }
    }
    chromosome
}
fn get_distributions(poblation:&Vec<Vec<i8>>)->Vec<f32>{
    let mut distribution:Vec<f32> = vec![];
    let poblation_size = poblation.len() as i8;
    eprintln!("poblation_size = {:?}", poblation_size);
    let mut sum = 0;
    for i in 0..(poblation[0].len()-1){
        for j in 0..(poblation_size -1) as usize{
            if poblation[j][i] == 1{
                sum+=1;
            }
        }
        distribution.push(sum as f32/poblation_size as f32);
        sum = 0;
    }
    distribution
}

fn get_features(poblation:&Vec<Vec<i8>>)->Vec<i8>{
    let mut features:Vec<i8> = vec![];
    let poblation_size = poblation.len() as i8;
    for i in 0..(poblation[0].len()-1){
        for j in 0..(poblation_size -1) as usize{
            if poblation[j][i] == 1{
                features.push(1);
            }
        }
    }
    features
}

fn generate_binary_array_with_n_ones(n: usize) -> Vec<i8> {
    let mut rng = rand::thread_rng();
    let mut binary_array = vec![0i8; 68];
    for i in 0..n {
        let mut index: usize;
        loop {
            index = rng.gen_range(0..68);
            if binary_array[index] == 0 {
                break;
            }
        }
        binary_array[index] = 1;
    }
    binary_array
}
fn find_ones_indices(binary_vector: &Vec<i8>) -> Vec<u32> {
    let mut ones_indices = Vec::new();
    for (i, &bit) in binary_vector.iter().enumerate() {
        if bit == 1 {
            ones_indices.push(i as u32);
        }
    }
    ones_indices
}

fn calculate_marginal_frequencies(binary_vectors: &[Vec<i8>]) -> Vec<f64> {
    let num_vectors = binary_vectors.len() as f64;
    let num_bits = binary_vectors[0].len() as f64;
    let mut frequencies = vec![0.0; binary_vectors[0].len()];
    
    for i in 0..binary_vectors[0].len() {
        let mut count = 0.0;
        for j in 0..binary_vectors.len() {
            count += binary_vectors[j][i] as f64;
        }
        frequencies[i] = (count / num_vectors);
    }
    
    frequencies
}
