use yew::prelude::*;

pub mod service;
pub mod welcome;

use service::Service;
use welcome::Welcome;

#[function_component(Home)]
pub fn home() -> Html {
    html! {
        <div>
            <Service/>
            <Welcome/>
        </div>
    }
}
