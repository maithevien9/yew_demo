// use ethers::prelude::*;
use dotenv_codegen::dotenv;
use gloo::console;
use web3::transports::eip_1193::{Eip1193, Provider};

use yew::{prelude::*, virtual_dom::VNode};

use web3::{
    transports::eip_1193::{self},
    types::H160,
};

const ADDRESS_WALLET: &str = dotenv!("ADDRESS_WALLET");

#[derive(Default)]
pub struct WalletAddress(Option<H160>);

#[function_component(Nav)]
pub fn nav() -> Html {
    const MENU_TITLES: [&str; 4] = ["Market", "Exchange", "Tutorials", "Wallets"];
    // let web3 = web3::Web3::new(Eip1193::new(Provider::default().unwrap().unwrap()));

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

    let connect_account = Callback::from(move |_| {
        let provider = eip_1193::Provider::default().unwrap().unwrap();
        let transport = eip_1193::Eip1193::new(provider);
        let web3 = web3::Web3::new(transport);

        // let web3 = web3.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let accounts = web3.eth().request_accounts().await;
            if let Ok(addr) = accounts {
                if !addr.is_empty() {
                    console::log!(format!("{:#x}", addr[0]).to_string());
                    console::log!(ADDRESS_WALLET)
                }
            }
        })
    });

    html! {
            <nav class="w-full flex md:justify-center justify-between items-center p-4">
                <div class="md:flex-[0.5] flex-initial justify-center items-center">
                    <img src="https://res.cloudinary.com/dfi8bluhn/image/upload/v1659807903/pbl6/1659807903200-bullet_afn5jv.png" alt="logo" class="w-10 cursor-pointer" />
                </div>
                <ul class="text-white md:flex hidden list-none flex-row justify-between items-center flex-initial">
                    {render_nav_items("my-2 text-lg")}
                    <li class="bg-[#2952e3] py-2 px-7 mx-4 rounded-full cursor-pointer hover:bg-[#2546bd]" onclick={connect_account}>
                        {"Connect"}
                    </li>
                </ul>
            </nav>
    }
}
