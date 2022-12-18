use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{
    pages_paths::Route,
    switch
};

pub mod pages;
pub mod components;

#[function_component(App)]
pub fn
Run() -> Html {
    // Mesmo conceito do aplicado no jsx
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch}/>
        </BrowserRouter>
    }
}