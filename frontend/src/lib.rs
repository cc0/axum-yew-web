mod components;
mod views;
mod stores;
mod routes;

use crate::{routes::{switch, Route}, stores::language::LangSelector};


use yew::{Html, html, function_component};
use yew_router::{BrowserRouter, Switch};


#[function_component(App)]
pub fn app() -> Html {
    
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}