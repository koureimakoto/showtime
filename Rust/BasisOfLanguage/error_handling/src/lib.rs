pub mod rest {
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
        to_equal_str(&self, other: T) -> Test where T: PartialEq {
            assert!(self.eq(other));
            self.new_test()
        }

        pub fn
        not_equal_str(&self, other: T) -> Test where T: PartialEq {
            assert!(self.ne(other));
            self.new_test()
        }

    }
    
    
    pub fn type_n<T>(_: &T) -> &str {
        std::any::type_name::<T>()
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

    // impl<T: Debug> TestThis<T> {
    //     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), Error> {
    //         f.debug_struct("Result")
    //          .field("To Analisy", &self.to_analisy)
    //          .finish()
    //     }
    // }



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
   
    pub mod it {
        
        static mut COUNT: u32 = 0;

        fn
        increse() {
            unsafe {
                COUNT += 1
            }
        }

        pub fn
        print_test_data() {
            unsafe {
                print!("This test ran {} times :: ", COUNT)
            }
        }
   
        #[derive(Debug, PartialEq, Copy, Clone)]
        pub struct Expect<T>(pub T);
    
        impl Expect<&str> {
            pub fn to_equal(&self, rhs: &str) {
                let int_right: &Expect<&str> = &Expect(rhs);
                assert_eq!( self, int_right)
            }
        }

        


        impl Expect<String> {
            pub fn to_equal(self, rhs: String) {
                let int_right: Expect<String> = Expect(rhs);
                assert_eq!( self, int_right)
            }

            pub fn not_equal(self, rhs: String) {
                let int_right: Expect<String> = Expect(rhs);
                assert_ne!( self, int_right)
            }

            pub fn is_empty(self) {
                let int_rhs: Expect<String> = Expect(String::from(""));
                assert_eq!(self, int_rhs)
            }

            pub fn is_not_empty(self) {
                let int_rhs: Expect<String> = Expect(String::from(""));
                assert_ne!(self, int_rhs)
            }
        }

        impl Expect<u64> {
            pub fn to_equal(self, rhs: u64) {
                increse();
                let int_right: Expect<u64> = Expect(rhs);
                assert_eq!( self, int_right)
            }

            pub fn not_equal(self, rhs: u64) {
                let int_right: Expect<u64> = Expect(rhs);
                assert_ne!( self, int_right)
            }
        }

        impl Expect<f32> {
            pub fn to_equal(self, rhs: f32) {
                let int_right: Expect<f32> = Expect(rhs);
                assert_eq!( self, int_right)
            }

            pub fn not_equal(self, rhs: f32) {
                let int_right: Expect<f32> = Expect(rhs);
                assert_ne!( self, int_right)
            }
        }

        impl Expect<f64> {
            pub fn to_equal(self, rhs: f64) {
                let int_right: Expect<f64> = Expect(rhs);
                assert_eq!( self, int_right)
            }

            pub fn not_equal(self, rhs: f64) {
                let int_right: Expect<f64> = Expect(rhs);
                assert_ne!( self, int_right)
            }
        }

    }

}



#[macro_export]
macro_rules! test_env {
    ( 
        
        $vis:vis fn $test_name:ident () {$body:block}
        $($i:tt)*
    ) => {
        test_env! {
            $($i)*
        }
    };
}





#[macro_export]
macro_rules! test_env3 {
    ( 
        $(#[$outr:meta])* 
        #[cfg(tests)]
        mod testing { 
            $(
                $(#[$inr:meta])*
                #[test]
                fn $test_name:ident ( $args_name:ident : &str )
                    $body:block

            )*
            $($t:tt)*
        }     
    ) => {

        test_env! {
            mod testing{
                $($t)*
            }
        }
    };()=>{};
}

#[macro_export]
macro_rules! test_env2 {
    ($describe:ident { $($i:tt)* } expands to { ($e:tt)*} ) => {
        use error_handling::{rest::it};
        #[cfg(test)]
        pub mod $describe() {
            expands to { $(e)* }
        }; () => {};
    };
}
