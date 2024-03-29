use crate::views::{
    home::Home,
    cabinet::Cabinet,
    auth::Auth,
    not_found::NotFound
};

use crate::stores::language::LangSelector;

use strum::IntoEnumIterator;
use yew::{html, Html, Properties};
use yew_router::{components::Redirect, Routable};

use gloo::storage::{LocalStorage, Storage};


pub const LANGUAGE_KEY: &str = "Language";

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/cabinet")]
    Cabinet,
    #[at("/auth")]
    Auth,
    #[not_found]
    #[at("/404")]
    NotFound
}


#[derive(Properties, PartialEq)]
pub struct Props {
    pub selected_language: String,
    pub supported_languages: Vec<&'static str>,
}

#[derive(Properties, PartialEq)]
pub struct Prop {
    pub selected_language: String,
}

pub fn switch(route: Route) -> Html {
    
    // Load LocalStorage Langauge variable, and contribute it to children (create if not exists)
    let selected_language = match LocalStorage::get::<LangSelector>(LANGUAGE_KEY) {
        Err(_) => {
            let _ = LocalStorage::set(LANGUAGE_KEY, LangSelector::default());
            LangSelector::default().to_string()
        },
        Ok(language) => language.to_string()
    };
    
    // Get all the possible langauges and parse them as &str
    let supported_languages = LangSelector::iter()
        .map(|language| language.as_str())
        .collect::<Vec<&'static str>>();


    match route {
        Route::Home => html! { <Home selected_language={selected_language.clone()} supported_languages={supported_languages.clone()} /> },
        Route::Cabinet => html! { <Cabinet /> },
        Route::Auth => html! { <Auth /> },
        Route::NotFound => html! { 
            <Redirect<Route> to={Route::Home} />
         }
    }
}