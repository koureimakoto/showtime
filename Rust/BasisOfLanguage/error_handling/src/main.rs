use std::any::type_name;
use std::fs::{File, self};
use std::io::{self, Read, ErrorKind};
use std::error::Error;

    
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


    //  error_handling::rest::Test::new("test").it("Int Equal").expect(5).to_equal(5);

    let f = 55;
    println!("{}", type_of(f));



    let greet_file = File::open("Tallveis.txt")?;

    Ok(())
}

fn
type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}




#[cfg(test)]
mod rest_test_impl {
    use error_handling::{rest::it};

    #[test]
    fn last_test(){
        
        let lhs : String = String::from("Talles");
        let rhs : String = String::from("Kourei");

        let lhs_num : u64 = 500;
        let rhs_num : u64 = 500;

        let lhs_fl : f32 = f32::MAX;
        let rhs_fl : f32 = f32::MAX;

        // Os testes
        it::Expect("Talles").to_equal("Talles");
        it::Expect(lhs).not_equal(rhs);
        it::Expect(lhs_num).to_equal(rhs_num);
        it::Expect(lhs_fl).to_equal(rhs_fl);
        it::Expect(4 + 4).to_equal(12 - 4);
        it::Expect(String::from("")).is_empty();
        it::Expect(String::from("")).is_empty();

        it::print_test_data()
    }

}


