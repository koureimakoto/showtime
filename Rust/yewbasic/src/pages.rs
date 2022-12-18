use yew::{Html, html};
use yew_router::prelude::*;

use self::{
    home::Home, 
    error_404::NotFound,
    code::Code,
    artwork::Artwork, 
    pages_paths::{ArtworkCodeLayout, Route}, 
};

pub mod home;
pub mod code;
pub mod artwork;
pub mod error_404;

#[path ="pages_paths.rs"]
pub mod pages_paths;


pub fn
switch(routes: Route) -> Html {
    html! {{
        match &routes {
            Route::Home     => html! {<Home    />},
            Route::Layout   => html! {
                <Switch<ArtworkCodeLayout> render={switch_layout} />
            },
            Route::NotFound => html! {<NotFound/>}
    }}}
}

fn
switch_layout(route: ArtworkCodeLayout) -> Html {
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
        }}
        </>
    }
}
