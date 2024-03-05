use std::fs;

pub fn read_data() -> i32 {
    // let file = File::open("output.txt");
    // let mut buf_reader = BufReader::new(file);
    // let mut data = vec![];
    let data = fs::read_to_string("output.txt").expect("Can't read");

    // Using the parse() method
    match data[12..].trim().parse::<i32>() {
        Ok(number) => {
            println!("Parsed number: {}", number);
            drop(data);
            return number;
        }
        Err(_) => {
            println!("Data: {}", &data[12..]);
            println!("No number");
            return 0;
        }
    }
}
