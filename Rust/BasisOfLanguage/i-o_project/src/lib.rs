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
    build(mut args: impl Iterator<Item = String>) 
    -> Result<Config, &'static str> {
        args.next();

        let query    : String = match args.next() {
            Some(arg) => arg,
            None      => return Err("Não foi possivel obter a Query")
        };

        let file_path: String = match args.next() {
            Some(arg) => arg,
            None      => return Err("Não foi possivel obter o Caminho")
        };

        let ignore_case = match env::var("IGNORE_CASE") {
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
     let mut count  : usize = 1;

    /*
    The original function of the Rust tutorial use only the sequence 
    lines => filter => collect, but I needed modify because the function
    return a tuple (usize, &str) vector.

    I thought about using a filter_map, but the best choise is below
    */
    contents
        .lines()
        .filter(|line: &&str| { line.contains(query)      })
        .map   (|item:  &str| { count += 1; (count, item) })
        .collect()
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