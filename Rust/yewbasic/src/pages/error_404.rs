use yew::{function_component, Html, html};


#[function_component(NotFound)]
pub fn get() -> Html {
    html! {
        <h1>
            {"Error 404"}
        </h1>
    }
}