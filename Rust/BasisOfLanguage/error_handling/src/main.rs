use std::fs::{File, self};
use std::io::{self, Read, ErrorKind};
use std::error::Error;
use error_handling;

fn read_username_from_file(path: &str) -> Result<String, io::Error> {
    let uname_result = File::open(path);

    let mut
    uname_file: File = match uname_result {
        Ok(file) => file,
        Err(e)  => return Err(e)
    };

    let mut uname: String = String::new();

    match uname_file.read_to_string(&mut uname) {
        Ok(_)  => Ok(uname),
        Err(e) => Err(e)
    }
}

fn read_uname_from_file(path: &str) -> Result<String, io::Error> {
    let mut uname_file: File   = File::open(path)?;
    let mut uname     : String = String::new();
    uname_file.read_to_string(&mut uname)?;
    Ok(uname)
}

// Esta ultima implementação aqui é muito legal
fn read_uname_from_file_short(path: &str) -> Result<String, io::Error> {
    let mut uname: String = String::new();
    File::open(path)?.read_to_string(&mut uname)?;
    Ok(uname)
}

fn read_uname_from_file_caralho_que_enxuto(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}



fn last_char_of_first_line(txt: &str) -> Option<char> {
    txt.lines().next()?.chars().last()
}


fn main() -> Result<(), Box<dyn Error>> {
    /*
     * Recoverable Error with Result<T, E>
     */
    
    // let file_result = File::open("bonitinho.txt");

    // let file = match file_result {
    //     Ok(fd)  => fd,
    //     Err(e) => panic!("Problema ao abrir o arquivo:")
    // };

    let path = "bonitinho.txt";
    let _file = File::open(path).unwrap_or_else(
        |error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(path).unwrap_or_else(
                    |error| {
                        panic!("Problemas na criação do arquivo {:?}", error);
                    }
                )
            } else {
                panic!("Problema na abertura do arquivo {:?}", error);
            }
        }
    );

    /*
     * Shortcuts for Panic on Error: unwrap and excpet 
     */

    // let path = "bonitinho2.txt";
    // let file2 = File::open(path).unwrap();

    /*
     * Propagating Error
     */

    println!("UserName: {:?}", read_username_from_file(path)   );
    println!("UserName: {:?}", read_uname_from_file(path)      );
    println!("UserName: {:?}", read_uname_from_file_short(path));
    println!("UserName: {:?}", read_uname_from_file_caralho_que_enxuto(path)    );
    println!("UserName: {:?}", read_uname_from_file_caralho_que_enxuto("oi.txt"));

    /*
     * Where the ? Operator Can Be Used 
     */

     println!("Last Char: {:?}", last_char_of_first_line("Makoto\nWorld"));
     println!("Last Char: {}", match last_char_of_first_line("Makoto\nWorld") {Some(x) => x, None => '_'} );

     //let hello_file = File::open("nothing_to_open.txt")?;


     //error_handling::rest::Test.it(5).equal(5);

     Ok(())
}
