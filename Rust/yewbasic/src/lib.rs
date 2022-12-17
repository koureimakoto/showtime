use yew::prelude::*;
use yew_router::prelude::*;

use crate::pages::{Route, switch, };

pub mod app;
pub mod pages;
pub mod components;
// pub mod page_layout;


#[function_component]
pub fn
Run() -> Html {
    // Mesmo conceito do aplicado no jsx
    // <Switch<Route>     render={switch}/>
    //             <Switch<PageRoute> render={switch_page_route}/>
    html! {
        <>
            <BrowserRouter>
                <Switch<Route>     render={switch}/>
            </BrowserRouter>
            <h1>{"Oi"}</h1>
        </>
    }
}