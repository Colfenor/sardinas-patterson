use std::env;
use std::path::Path;
use std::fs::File;
use std::io::prelude::Read;

/* sardinas patterson algorithm for testing unique decipherability */
// reference: IEEE TRANSACTIONS ON INFORMATION THEORY, VOL. IT-28, NO. 4, JULY 1982

fn read_in_file(path: &Path) -> Vec<String> {
    
    let display = path.display();
    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display, why),
        Ok(file) => file,
    };

    let mut parsed_content = String::new();
    match file.read_to_string(&mut parsed_content) {
        Err(why) => panic!("couldn't read {}: {}", display, why),
        Ok(_) => print!("{} contains:{} \n", display, parsed_content),
    }

    parsed_content.split(",").map(str::to_string).collect()
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
    
    let test = read_in_file(path);

    for x in test {
        println!("{}\n", x)
    }
}
