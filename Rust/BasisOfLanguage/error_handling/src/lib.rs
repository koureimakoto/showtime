pub mod rest {
    use std::io::{self, Read, ErrorKind};
    use std::fmt::{self, Error};
    use std::fmt::Debug;



    // impl<T> Debug for TestThis<T> where T: Debug {
    //     fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), Error> {
    //         f.debug_struct("Result")
    //          .field("To Analisy", &self.to_analisy)
    //          .finish()
    //     }
    // }

    pub trait Normalize: PartialEq {

    } 

    #[derive(Debug, Clone, Copy)]
    pub struct TestThis <T>{
        to_analisy : T,
    }

    impl <T> TestThis<T> {
        fn new_test(&self) -> Test {
            Test{}
        }

        pub fn
        to_equal(&self, other: T) -> Test where T: PartialEq  {
            assert!(self.to_analisy == other );
            self.new_test()
        }

        pub fn
        not_equal(&self, other: T) -> Test where T: PartialEq  {
            assert!(self.to_analisy != other );
            self.new_test()
        }

        pub fn
        to_cmp(&self, other: T) -> Test where T: PartialEq {
            assert!(self.eq(other));
            self.new_test()
        }

        // pub fn
        // cmp_str(&self, other: T) -> Test where T: PartialEq{
        //     assert_eq!(format!("{:?}", self.to_analisy), self,other);
        //     self.new_test()
        // }


    }
    
    
    pub fn type_n<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    impl<T> AsRef<T> for TestThis<String> {
        fn as_ref(&self) -> &T {
            self.as_ref()
        }
    }


    impl<T: PartialEq> TestThis<T>{
        fn 
        eq(&self, other: T ) -> bool {
            self.to_analisy == other 
        }

        fn 
        ne(&self, other: T ) -> bool {
            self.to_analisy != other
        }
    }


    // impl<T: PartialEq> TestThis<T> {
        

    // }

    impl<T: Debug> TestThis<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), Error> {
            f.debug_struct("Result")
             .field("To Analisy", &self.to_analisy)
             .finish()
        }

        // fn fmt(&self) -> String {
        //     format!("{:?}", self)
        // }
    }



    pub struct  Test {}

    impl Test {
        
        pub fn
        title(title: &str ) -> Self {
            println!("Inicialize: {}", title);
            Test{}
        }

        pub fn
        it( self, title: &str) -> Self {
            println!(" --- {}", title);
            self
        }

        pub fn
        next(self, title: &str) -> Self {
            println!(" --- next : {}", title);
            self
        }
        
        pub fn
        expect<E>(self, element: E) -> TestThis<E> {
            TestThis {
                to_analisy: element,
            }
        }
 
    }
    
}


