use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    #[wasm_bindgen(js_namespace = console)]
    fn log( s: &str );

    #[wasm_bindgen( js_namespace = console, js_name = log )]
    fn log_u32( a: u32 );

    #[wasm_bindgen( js_namespace = console, js_name = log )]
    fn log_many( a: &str, b: &str );

}

macro_rules! console_log {
    ( $($t:tt)* ) => (log(&format_args!($($t)*).to_string()))
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert( &format!("Hello, {}", name) )
}

#[wasm_bindgen]
pub fn exec() {
    console_log!( "Hello Console" );
}