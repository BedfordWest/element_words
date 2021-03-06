use std::io;
use std::cmp::Ordering;

extern crate csv;
extern crate rustc_serialize;

fn main() {
    
    #[derive(RustcDecodable, Debug, Clone)]
    struct Row {
        number: i8,
        symbol: String,
        full_name: String,
        weight: f32,
    }

    impl Eq for Row {}
        
    impl Ord for Row {
        fn cmp(&self, other: &Row) -> Ordering {
            self.number.cmp(&other.number)
        }
    }

    impl PartialOrd for Row {
        fn partial_cmp(&self, other: &Row) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl PartialEq for Row {
        fn eq(&self, other: &Row) -> bool {
            self.number == other.number
        }
    }

    let mut rdr = csv::Reader::from_file("./ptdata2.csv").unwrap();
    let rows: Vec<Row> = rdr.decode().collect::<csv::Result<Vec<Row>>>().unwrap();

    println!("Welcome to Element Speller!");
    println!("Please enter a word to spell with elements from the periodic table.");
    println!("Enter 'exit' to exit!");
    
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let mut accepted_rows: Vec<Row> = vec![];
                input = input.trim().to_string();
                input = input.to_lowercase();
                if input == "exit" {
                    break;
                }

                for row in &rows {
                    let new_v: Vec<_> = input.match_indices(&row.symbol.to_lowercase()).collect();
                    if new_v != [] {
                        for matched_symbol in &new_v {
                            let mut new_row: Row = row.clone();
                            new_row.number = matched_symbol.0 as i8;
                            accepted_rows.push(new_row);
                        }
                    }
                }
                accepted_rows.sort();
                let mut symbols: String = String::from("");
                let mut names: String = String::from("");
                for element in &accepted_rows {
                    symbols.push_str(&element.symbol);
                    names.push_str(&element.full_name);
                }

                println!("{}", symbols);
                println!("{}", names);
            }
            Err(error) => println!("error: {}", error),
        }
        
    }
}


