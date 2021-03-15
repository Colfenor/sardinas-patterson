use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main() {
    // pass filename to cmd line args
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() == 1 {
        println!("Please specify the input file as second parameter");
        return;
    }

    let path = Path::new(&args[1]);
    if let Ok(lines) = read_lines(path) {
        for line in lines {
            if let Ok(ip) = line {
                println!("{}", ip);
                
            }
        }
    }


    
}
