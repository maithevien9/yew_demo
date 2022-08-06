use yew::prelude::*;

/// Home page
#[function_component(Home)]
pub fn home() -> Html {
    html! {
      <div>
       <p class="text-3xl font-bold underline">{"Test!"}</p>
        <p class={ classes!("bg-red-500") }>{"Test2!"}</p>
      </div>
    }
}
