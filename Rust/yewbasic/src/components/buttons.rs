pub mod home { 

    use yew::prelude::*;
    use yew_router::prelude::*;

    use crate::pages::ArtworkCodeLayout;
    
    #[derive(Clone, PartialEq, Properties)]
    pub struct Props {
        pub class_name: String,
        pub name      : String,
        pub route     : ArtworkCodeLayout ,
        pub callback  : fn()
    }

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