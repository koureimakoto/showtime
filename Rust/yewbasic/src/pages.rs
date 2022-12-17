use std::{fmt::{Display, Result, Formatter}, string};

use yew::{Html, html};
use yew_router::prelude::*;

// use crate::page_layout::CodeRoute;

use self::{
    home::Home, 
    error_404::NotFound,
    code::Code,
    artwork::Artwork, 
};

pub mod home;
pub mod code;
pub mod artwork;
pub mod error_404;


#[derive(Clone, Routable, PartialEq, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/*")]
    Layout,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum ArtworkCodeLayout {
    #[at("/code")]
    CodeRoot,
    #[at("/code/*")]
    Code,
    #[at("/artwork")]
    ArtworkRoot,
    #[at("/artwork/*")]
    Artwork,
    #[not_found]
    #[at("/*/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]

pub enum ArtworkRoute {
    #[at("/artwork/2d")]
    TwoD,
    #[at("/artwork/3d")]
    ThreeD,
    #[not_found]
    #[at("/artwork/404")]
    NotFound,
}

#[derive(Clone, Routable, PartialEq, Debug)]

pub enum CodeRoute {
    #[at("/code/web")]
    Web,
    #[not_found]
    #[at("/code/404")]
    NotFound,
}




pub fn switch(routes: Route) -> Html {
    html! {
        {
            match &routes {
                Route::Home     => html! {<Home    />},
                Route::Layout   => html! {
                    <Switch<ArtworkCodeLayout> render={switch_layout} />
                },
                _ => html! {}
            }
        }
    }
    
}

fn switch_layout(route: ArtworkCodeLayout) -> Html {
    html! {
        <>
        <h1>{"Oi dois"}</h1>
        {
        match route {
            ArtworkCodeLayout::CodeRoot    => html! {<Code   />},
            ArtworkCodeLayout::Code        => html! {<h1>{"CodeLayout"}</h1>},
            ArtworkCodeLayout::ArtworkRoot => html! {<Artwork/>},
            ArtworkCodeLayout::Artwork     => html! {<h1>{"ArtworkLayout"}</h1>},
            ArtworkCodeLayout::NotFound    => html! {<NotFound/>},
            _ => html!{}
        }
        }
        </>
    }
}


// fn portfolio_layout(route: &Route) -> Html {
//     html! {
//         <>
//         // Apply de Default Code
//         <h1>{"oi"}</h1>
//         if let Route::Code = route {
//             <Code    />
//         } else 
//         if let Route::Artwork = route {
//             <Artwork />
//         } else 
//         if let Route::NotFound = route {
//             <NotFound/>
//         }
//         </>
//     }
// }