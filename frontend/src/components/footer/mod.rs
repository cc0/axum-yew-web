mod inner_top;
mod inner_bot;
mod mobile_apps_ad;

use crate::routes::Props;
use crate::components::footer::{
    inner_top::InnerTop,
    inner_bot::InnerBot,
    mobile_apps_ad::MobileAppsAd,
};

use crate::components::locales::footer_loc;

use yew::{function_component, html, Html};
use yew_i18n::{use_translation, I18nProvider};


#[function_component(Footer)]
pub fn footer(props: &Props) -> Html {

    let mobile_apps_ad_loc = footer_loc::mobile_apps_ad();
    let top_translations = footer_loc::inner_top_loc();
    let bot_translations = footer_loc::inner_bot_loc();

    html! {
        
        <div class="app-footer">
        
            <div class="footer-wrapper">

                <I18nProvider supported_languages={props.supported_languages.clone()} translations={mobile_apps_ad_loc}>
                    <MobileAppsAd selected_language={props.selected_language.clone()} />
                </I18nProvider>
                
                <footer class="footer">
                
                    <div class="layout">
                    
                        <div class="footer-top">

                            <I18nProvider supported_languages={props.supported_languages.clone()} translations={top_translations}>
                                    <InnerTop selected_language={props.selected_language.clone()} />
                            </I18nProvider>
                            

                            <I18nProvider supported_languages={props.supported_languages.clone()} translations={bot_translations}>
                                    <InnerBot selected_language={props.selected_language.clone()} />
                            </I18nProvider>

                        </div>

                    </div>

                </footer>

            </div>

        </div>

    }
}