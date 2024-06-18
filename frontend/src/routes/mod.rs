use yew::{html, Html};
use yew_router::Routable;

use crate::views::{
    home::Home,
    about_us::AboutUs,
    support::Support
};

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/support")]
    Support,
    #[at("/about-us")]
    AboutUs
}

pub fn switch(route: Route) -> Html {
    match route {
        Route::Home => { html! { <Home /> } },
        Route::Support => { html! { <Support /> } },
        Route::AboutUs => { html! { <AboutUs /> } }
    }
}