use std::io;

extern crate csv;

fn main() {
    type Row = (i8, String, String, f32);

    let mut rdr = csv::Reader::from_file("./ptdata2.csv").unwrap();
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let mut accepted_rows: Vec<Row> = vec![];
    for row in &rows {
        println!("Element {}: {}, {} weighs {}", row.0, row.1, row.2, row.3);
    }
    
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                input = input.trim().to_string();
                println!("{}", input);
                if input == "exit" {
                    break;
                }

                for row in &rows {
                    let v: Vec<_> = input.match_indices(&row.1).collect();
                    println!("{:?}", v);
                }
            }
            Err(error) => println!("error: {}", error),
        }
    }
}

