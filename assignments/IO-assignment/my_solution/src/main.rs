use std::error::Error;
use std::fs::File;
use std::io::{
    prelude::{BufRead, Read},
    BufReader, Seek, SeekFrom,
};
use url::Url;

const DEFAULT_FILENAME: &str = "src/data/content.txt";

fn main() -> Result<(), Box<dyn Error>> {
    let filename = std::env::args()
        .nth(1)
        .unwrap_or_else(|| DEFAULT_FILENAME.into());

    println!("[[Opening and reading file]] {}", filename);
    let maybe_file = File::open(filename);

    // TASK 1
    match maybe_file {
        Ok(mut file) => {
            // TASK 2
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            println!("[[All file contents]]\n{}", contents);



            // RESET FILE
            file.seek(SeekFrom::Start(0))?;

            // TASK 3.1 3.2
            let mut ix = 1;
            for maybe_line in BufReader::new(&file).lines() {
                println!("[[line {}]] {}", ix, maybe_line?);
                ix += 1;
            }



            // RESET FILE
            file.seek(SeekFrom::Start(0))?;

            // TASK 3.1 3.3
            let lines = BufReader::new(&file).lines();
            println!("[[line count]] {}", &lines.count());





            // RESET FILE
            file.seek(SeekFrom::Start(0))?;

            // TASK 3.1 3.5
            for (ix, maybe_line) in BufReader::new(&file).lines().enumerate() {
                let line = maybe_line?;
                if !line.is_empty() {
                    println!("[[line {}]] {}", ix + 1, line);
                }
            }




            // RESET FILE
            file.seek(SeekFrom::Start(0))?;

            // TASK 4
            for line in BufReader::new(&file).lines() {
                if let Ok(url) = Url::parse(&line?) {
                    println!("[[URL found]] {}", url);
                }
            }
        }
        Err(err) => println!("Error: {}", err),
    };

    Ok(())
}
