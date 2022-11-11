use std::{ env, process::exit  };
use i_o_project::{ Config, run };


fn main() {
    let args: Vec<String> = env::args().collect();

    let conf: Config = Config::build(&args).unwrap_or_else(
        |err: &str| {
            eprintln!("Problema no parsing dos argumentos: {err}");
            exit(1)
        }
    );
    
    println!("Minha busca    : {}", conf.query    );
    println!("Em qual arquivo: {}", conf.file_path);
    println!();

    /*
    There is a value to return together with Error
    run(conf).unwrap_or_else(
        |err| {
            println!("Problema no parsing dos argumentos: {err}");
            exit(1)
        }
    )
    */

    // /\ or \/, but there is a small difference 

    // There is no return value other than Error
    if let Err(e) = run(conf) {
        eprintln!("Problema no parsing dos argumentos: {e}");
        exit(1)
    }

}
