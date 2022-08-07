use yew::{prelude::*, virtual_dom::VNode};

#[function_component(Nav)]
pub fn nav() -> Html {
    const MENU_TITLES: [&str; 4] = ["Market", "Exchange", "Tutorials", "Wallets"];

    fn render_nav_items(class_props: &str) -> VNode {
        MENU_TITLES
            .iter()
            .map(|item| {
                html! {
                  <li class={format!("mx-4 cursor-pointer font-semibold {}", class_props)}>{item}</li>
                }
            })
            .collect::<Html>()
    }

    html! {
       <nav class="w-full flex md:justify-center justify-between items-center p-4">
            <div class="md:flex-[0.5] flex-initial justify-center items-center">
                <img src="https://res.cloudinary.com/dfi8bluhn/image/upload/v1659807903/pbl6/1659807903200-bullet_afn5jv.png" alt="logo" class="w-10 cursor-pointer" />
            </div>
            <ul class="text-white md:flex hidden list-none flex-row justify-between items-center flex-initial">
                {render_nav_items("my-2 text-lg")}
                <li class="bg-[#2952e3] py-2 px-7 mx-4 rounded-full cursor-pointer hover:bg-[#2546bd]">
                    {"Login"}
                </li>
            </ul>
    </nav>
    }
}
