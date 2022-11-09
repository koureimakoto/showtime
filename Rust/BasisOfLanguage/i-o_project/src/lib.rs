use std::{ fs, error::Error };


/**
 * The struct stores the terminal args
 */
pub struct
Config {
    pub query    : String,
    pub file_path: String
}

// Implement Config Struct
impl Config {
    /**
     * Build the Config only if args lenght is more than 2
     */
    pub fn
    build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Número insulficiente de argumentos");
        }
        
        let query    : String = args[1].clone();
        let file_path: String = args[2].clone();
        
        Ok(Config { query, file_path })
    }
}

/**
 * Run Config args to simulate the Linux 'grep'
 */
pub fn
run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    // println!("Text: \n{contents}");   

    // let result: &str = search(&config.query, &contents);
    
    for line in search(&config.query, &contents) {
        println!("[{}]: {}", line.0, line.1); 
    }

    Ok(())
}

// Rust Original function
// pub fn
// search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();

//     for line in contents.lines() {
//         if line.contains(query) {
//             results.push(line);
//         }
//     }

//     results
// }


// Modify by me and slower :: OLD
// pub fn
// search(query: &str, contents: &str) -> Vec<String> {
//     let mut results = Vec::new();
//     let mut count  : usize = 1;

//     for line in contents.lines() {
//         if line.contains(query) {
//              let string = String::from(format!("[{}] : {}", count, line));
//             results.push(string);
//         }
//         count += 1;
//     }

//     results
// }

// Modify by me and slower :: NEW
pub fn
search<'a>(query: &str, contents: &'a str) -> Vec<(usize, &'a str)> {
    let mut results: Vec<(usize, &str)> = Vec::new();
    let mut count  : usize = 1;

    for line in contents.lines() {
        if line.contains(query) {
            results.push((count, line));
        }
        count += 1;
    }

    results
}


// ----- TESTS -----

#[cfg(test)]
mod testes {

    use super::*;

    #[test]
    fn correct_build() {

    }

    #[test]
    fn one_result() {
        let query  : &str = "duct";
        let content: &str = "Rust:\nsafe, fast, productive.\nPick three";
        let result : Vec<(usize, &str)> = search(query, content);
        assert_eq!("safe, fast, productive.", result[0].1);    
    }
    
}