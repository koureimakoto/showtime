use yew_router::Routable;

#[derive(Clone, Routable, PartialEq, Debug)]
pub enum CodeRoute {
    #[at("/")]
    Home,
    #[at("/*path")]
    Misc { path: String }
}