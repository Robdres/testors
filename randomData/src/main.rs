use std::error::Error;
use std::io;
use std::path::Path;
use csv::Writer;
use rand::Rng;

fn main() {
    let rows:u32= 1000;
    let atributtes:u32 = 100;

    let mut values:Vec<Vec<u8>> = vec![];

    let mut rng = rand::thread_rng(); 

    for _ in 0..rows{
        let mut row: Vec<u8> = vec![];
        for _ in 0..atributtes{
            row.push(rng.gen_range(0..2)); 
        }
        values.push(row);
    }

    to_csv(&values).expect("Error");
    
}
fn to_csv(vector:&Vec<Vec<u8>>) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_writer(io::stdout());
    for k in vector.into_iter(){
        wtr.write_record(k.iter().map(|e| e.to_string()))?;
    }
    wtr.flush()?;
    Ok(())
}
