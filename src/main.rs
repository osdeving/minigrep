use std::{env, fs, process};

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // fn new(args: &[String]) -> Self {
    //     if(args.len() < 3) {
    //         panic!("no enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();
    
    //     Self {query, file_path}
    // }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config {query, file_path})
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    //let config = parse_config(&args);

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("problem parsing args: {}", err);
        process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("in file {}", config.file_path);

    let contents = fs::read_to_string(config.file_path).expect("msg");

    println!("With text:\n{contents}");
}


// fn parse_config(args: &[String]) -> Config {
//     Config::build(args)
// }