use yew_router::Routable;

/**
 * Home    : / root
 * Layout  : / Any
 * NotFound: / Any Path Error at this level.
*/
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

/**
 * CodeRoot   : /code 
 * Code       : /code/ Any
 * ArtworkRoot: /artwork 
 * Artwork    : /artwork/ Any
 * NotFound   : /artwork/ Any Path Error at this level.
 */
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

/**
 * TwoD   or 2D: /artwork/2d > Only 2D Artwork as Painting, Drawing, Texture.
 * ThreeD or 3D: /artwork/3d > Only 3D Artwork as Modeling, Sculping, Shaders
 * NotFound    : /artwork/ Any Path error at Artwork Level
 */
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

/**
 * Web     : /code/web  > Webpages, Wasm app, or WebGL app
 * Games   : /code/games> Any Game
 * NotFound: /code/ Any Path error at Code Level
 */
#[derive(Clone, Routable, PartialEq, Debug)]
pub enum CodeRoute {
    #[at("/code/web")]
    Web,
    #[at("/code/games")]
    Games,
    #[not_found]
    #[at("/code/404")]
    NotFound,
}

