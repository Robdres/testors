use std::fs::File;
use std::io::{BufReader, Error, Read};
use std::path::Path;
use std::str::FromStr;

fn main() {
    let file_path = "./BinaryMatrixMean.csv";
    let data = read_csv_file(file_path).unwrap();

    let binary_data = data.iter().map(|row| {
    row.iter().map(|&val| val == 1).collect::<Vec<bool>>()}).collect::<Vec<Vec<bool>>>();
    eprintln!("data = {:?}", data);
    let num_cols = data[0].len();
    let target = vec![true; num_cols];
    let testors = lex_algorithm(&binary_data, &target);
    eprintln!("testors = {:?}", testors);
}

fn read_csv_file<P: AsRef<Path>>(file_path: P) -> Result<Vec<Vec<u32>>, Error> {
    let file = File::open(file_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut matrix = Vec::new();
    for line in contents.lines() {
        let row = line
            .split(',')
            .map(|value| u32::from_str(value).unwrap())
            .collect::<Vec<u32>>();
        matrix.push(row);
    }
    Ok(matrix)
}
fn lex_algorithm(data: &[Vec<bool>], target: &[bool]) -> Vec<Vec<usize>> {
    let num_attrs = data[0].len();
    let mut attrs: Vec<usize> = (0..num_attrs).collect();
    attrs.sort();
    let mut testors = Vec::new();
    let mut candidate_testor = Vec::new();
    lex_helper(data, target, &attrs, 0, &mut candidate_testor, &mut testors);
    testors
}

fn lex_helper(
    data: &[Vec<bool>],
    target: &[bool],
    attrs: &[usize],
    start_idx: usize,
    candidate_testor: &mut Vec<usize>,
    testors: &mut Vec<Vec<usize>>,
) {
    // Check if candidate testor is a testor
    if is_testor(data, target, &candidate_testor) {
        testors.push(candidate_testor.clone());
    }
    // Add attributes to candidate testor and backtrack as necessary
    for i in start_idx..attrs.len() {
        candidate_testor.push(attrs[i]);
        lex_helper(data, target, attrs, i + 1, candidate_testor, testors);
        candidate_testor.pop();
    }
}

fn is_testor(data: &[Vec<bool>], target: &[bool], testor: &[usize]) -> bool {
    let mut indiscernibles = Vec::new();
    for i in 0..data.len() {
        let row = &data[i];
        if is_indiscernible(row, testor, &indiscernibles) {
            indiscernibles.push(i);
        }
    }
    is_reduct(&indiscernibles, target)
}

fn is_indiscernible(row: &[bool], testor: &[usize], indiscernibles: &[usize]) -> bool {
    for &idx in indiscernibles {
        let other_row = &row[idx];
        let mut match_count = 0;
        for &attr_idx in testor {
            let attr_value = &row[attr_idx];
            if attr_value == other_row {
                match_count += 1;
            }
        }
        if match_count == testor.len() {
            return true;
        }
    }
    false
}

fn is_reduct(indiscernibles: &[usize], target: &[bool]) -> bool {
    for i in 0..target.len() {
        let mut values = Vec::new();
        for &idx in indiscernibles {
            let value = target[idx];
            if !values.contains(&value) {
                values.push(value);
            }
            if values.len() > 1 {
                return false;
            }
        }
    }
    true
}
