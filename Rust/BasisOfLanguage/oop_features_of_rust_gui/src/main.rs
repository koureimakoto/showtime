use oop_features_of_rust_gui::{Screen, Button, Draw};

struct
SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        todo!()
    }
}


fn main() {
    
    let screen: Screen = Screen { 
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10, 
                options: vec![
                    String::from("No"),
                    String::from("Yes"),
                    String::from("Maybe")
                ]
            }), 
            
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("Send")
            })
        ]
    };


    screen.run();

}
