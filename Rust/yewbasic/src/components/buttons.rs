/**
 * This module is used only to create a single type-button<a>
 */

pub mod home { 

    // Yew standards
    use yew::prelude::*;
    use yew_router::prelude::*;

    // Intern
    use crate::pages::pages_paths::ArtworkCodeLayout;
    
    // Home Button Props
    // I will maybe remove the callback
    #[derive(Clone, PartialEq, Properties)]
    pub struct Props {
        pub class_name: String,
        pub name      : String,
        pub route     : ArtworkCodeLayout ,
        pub callback  : fn()
    }

    ///
    /// ```
    ///  Component(HomeButton)
    ///     >> &home::Props
    ///     << Html<Link<ArtworkCodeLayout>>
    /// ```
    #[function_component(HomeButton)]
    pub fn home_button(props: &Props) -> Html {
        html! {
            <Link<ArtworkCodeLayout> 
                classes={classes!(&props.class_name)}
                to={props.route.clone()}
            >
                <div>
                    <h1>{&props.name}</h1>
                </div>
            </Link<ArtworkCodeLayout>>
        }
    }
}