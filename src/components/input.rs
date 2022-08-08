use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub placeholder: String,
    pub input_type: String,
    pub value: String,
    pub oninput: Callback<InputEvent>,
}

#[function_component(Input)]
pub fn check(props: &Props) -> Html {
    html! {
      <input
        placeholder={props.placeholder.clone()}
        type={props.input_type.clone()}
        step="0.0001"
        value={props.value.clone()}
        oninput={props.oninput.clone()}
        class="my-2 w-full rounded-sm p-2 outline-none bg-transparent text-white border-none text-sm white-glassmorphism border-transparent focus:border-transparent focus:ring-0"
    />
      }
}
