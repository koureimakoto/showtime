use std::{ fs, error::Error, env };


/**
 * The struct stores the terminal args
 */
pub struct
Config {
    pub query      : String,
    pub file_path  : String,
    pub ignore_case: bool
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

        let ignore_case = match env::var("IGNORE_CASE"){
            Ok(var)  => if var == "1" { true } else { false },
            Err(_) => false
        };

        println!("Case sensitive: {}", ignore_case );
        
        Ok(Config { query, file_path, ignore_case })
    }
}

/**
 * Run Config args to simulate the Linux 'grep'
 */
pub fn
run(config: Config) -> Result<(), Box<dyn Error>>{
    let contents = fs::read_to_string(config.file_path)?;
    
    let result: Vec<(usize, &str)> = 
    if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    
    for line in result {
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

pub fn
search_case_insensitive<'a>(query: &str, contents: &'a str) ->Vec<(usize, &'a str)> {
    let mut results: Vec<(usize, &str)> = Vec::new();
    let mut count  : usize = 1;

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            results.push((count, line));
        }
        count += 1;
    }

    results
}




/*
 * --- TESTS ---
 */
#[cfg(test)]
mod testes {

    use super::*;

    // Setups
    fn search_setup<'a>() -> &'a str {
        "Rust:\nsafe, fast, productive.\nPick three\nTrust me"
    }

    #[test]
    fn case_sensitive(){
        let test_it : &str = "safe, fast, productive.";
        let query   : &str = "duct";
        let contents: &str = search_setup();

        assert_eq!(test_it, search(query, contents)[0].1);        
    }

    #[test]
    fn case_insensitive() {
        let test_it : Vec<&str> = vec!["Rust:", "Trust me."];
        let query   : &str = "rUsT";
        let contents: &str = search_setup();

        let mut id: usize = 0;
        for item in search_case_insensitive(query, contents) {
            assert_eq!(test_it[id], item.1);
            id += 1;
        }
    }
    
}