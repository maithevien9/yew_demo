use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::footer::Footer;
use crate::components::nav::Nav;
use crate::routes::{switch, AppRoute};

/// Root app component
#[function_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
             <div class="gradient-bg-welcome flex flex-col justify-between">
                <Nav />
                <div class="min-h-[calc(100vh-294px)]">
                    <Switch<AppRoute> render={Switch::render(switch)} />
                </div>
                <Footer/>
            </div>
        </BrowserRouter>
    }
}
