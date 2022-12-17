
use yew::{Html, html, function_component};

use crate::{components::buttons::{
    home::HomeButton, self},
    pages::ArtworkCodeLayout::{
        CodeRoot, 
        ArtworkRoot}
};

#[function_component(Home)]
pub fn get() -> Html {

    let rhs = buttons::home::Props {
        class_name: String::new(),
        name      : String::from("Code"),
        route     : CodeRoot,
        callback  : || {
            println!("Nada");
        }
    };

    let lhs = buttons::home::Props {
        class_name: String::new(),
        name      : String::from("Artwork"),
        route     : ArtworkRoot,
        callback  : || {
            println!("Nada");
        }
    };


    html! {
        <nav>
            <HomeButton
                class_name={rhs.class_name}
                name={rhs.name}
                route={rhs.route} 
                callback = {rhs.callback}
            />

            <HomeButton
                class_name={lhs.class_name}
                name={lhs.name}
                route={lhs.route} 
                callback = {lhs.callback}
            />
        </nav>
        
    }
}    

