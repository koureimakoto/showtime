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


     Ok(())
}

#[cfg(test)]
mod tests {
    use error_handling::{rest::it};

    
    // #[test]
    // fn rest_lib_test() {
    //     Test::title("AST Equality")
    //         .it("5 == 5").expect(5).to_equal(5)
    //         .it("7 == 7").expect(7).to_equal(7);
    // }

    // #[test]
    // fn rest_clean_test() {
    //     Test::title("AST Equality Simple")
    //         .expect(5).to_equal(5)
    //         .expect(7).not_equal(8);
    // }

    // #[test]
    // fn rest_string_test() {
    //     Test::title("AST String")
    //         .expect(String::from("World")).not_equal_str(String::from("Worlds"))
    //         .expect(String::from("World")).to_equal_str(String::from( "World"));
    // }

    // #[test]
    // fn rust_assert_test() {
    //     assert!(5 == 5);
    //     assert!(7 == 7);
    // }

    /**
     * A implementação Padrão 
     */
    #[test]
    fn string_test() {
        let lhs : String = String::from("Talles");
        let rhs : String = String::from("Kourei");

        // Os testes 
        assert_eq!("Talles", "Talles");
        assert_ne!(lhs, rhs);
    }

    /**
     * A cópia do Jest
     */
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
    }

}