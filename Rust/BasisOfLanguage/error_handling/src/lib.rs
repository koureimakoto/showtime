

pub mod rest {
    use std::io::{self, Read, ErrorKind};
    use std::fmt::{self, Error};

    pub struct  Test <T>{
        describe: String,
        to_analisy: T
    }

    impl<T> fmt::Debug for Test<T> where T: fmt::Debug, {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let buffer = self.to_analisy;

            f.debug_struct("Test")
                .field("describe", &self.to_analisy)
                .field("to_analist", &self.to_analisy)
                .finish()
        }
    }

    impl<T> Test<T> {
        
        pub fn
        it<EXT>(&self, generic: EXT ) -> Test<EXT> {
            Test {
                describe: self.describe,
                to_analisy: generic
            }
        }
        
        pub fn 
        equal(&self, test_this: T) {
            assert_eq!( self.to_analisy, test_this )
        }        
    }
    



    // impl Test{
    //     fn describe(&self, ext_describe: String) -> Self {
    //             self.describe = ext_describe;
    //             self
    //     }
    
    //     fn
    //     it<i32>(&self, generic: T ) -> Self {
    //         self.to_analisy = Some(generic);
    //         self
    //     }
    // }
}