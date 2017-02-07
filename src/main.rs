use std::io;

extern crate csv;

fn main() {
    type Row = (i8, String, String, f32);

    let mut rdr = csv::Reader::from_file("./ptdata2.csv").unwrap();
    let rows = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();
    let accepted_rows = Vec<Row>;
    for row in rows {
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
                let chars = input.chars();
                for c in chars {
                    if c.is_alphabetic() {
                        println!("{}", c);
                        let c_str = c.to_string();
                        for row in rows {
                            if c_str == row.1 {
                                accepted_rows.push(row);
                            }
                        }
                                
                        }
                    }
                }

            }
            Err(error) => println!("error: {}", error),
        }
    }
}
