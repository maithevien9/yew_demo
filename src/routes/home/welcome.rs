use serde::{Deserialize, Serialize};
use stdweb::web::Window;
use web_sys::HtmlInputElement;
use yew::html;
use yew::Callback;
use yew_octicons::Icon;
use yew_octicons::IconKind;

use crate::components::input::Input;
use crate::utils::shorten_address::shorten_address;
use uuid::Uuid;
use yew::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct UserInfo {
    pub id: String,
    pub name: String,
    pub age: String,
    pub sex: String,
    pub address: String,
}

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let user_info = use_state(UserInfo::default);
    let users: UseStateHandle<Vec<UserInfo>> = use_state(|| Vec::new());

    let company_common_styles =
  "min-h-[70px] sm:px-0 px-2 sm:min-w-[120px] flex justify-center items-center border-[0.5px] border-gray-400 text-sm font-light text-white";

    let oninput_name = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.name = input.value();
            user_info.set(info);
        })
    };
    let oninput_age = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.age = input.value();
            user_info.set(info);
        })
    };

    let oninput_sex = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.sex = input.value();
            user_info.set(info);
        })
    };

    let oninput_address = {
        let user_info = user_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*user_info).clone();
            info.address = input.value();
            user_info.set(info);
        })
    };

    let on_create = {
        let users = users.clone();
        let user_info = user_info.clone();

        Callback::from(move |ev: MouseEvent| {
            let mut cloned_users = (*users).clone();
            let id = Uuid::new_v4();
            let mut current_user: UserInfo = (*user_info).clone();
            current_user.id = id.to_string();

            cloned_users.push(current_user);
            users.set(cloned_users);
            ev.prevent_default();
        })
    };

    html! {
      <div class="flex flex-col justify-center items-center">
         <div class="flex w-full justify-center items-center">
        <div class="flex mf:flex-row flex-col items-start justify-between md:p-20 py-12 px-4">
        <div class="flex flex-1 justify-start items-start flex-col mf:mr-10">
          <h1 class="text-3xl sm:text-5xl text-white text-gradient py-1">
            {"Send Crypto"} <br /> {"across the world"}
          </h1>
          <p class="text-left mt-5 text-white font-light md:w-9/12 w-11/12 text-base">
            {"Explore the crypto world. Buy and sell cryptocurrencies easily on
            Krypto."}
          </p>

          <div class="grid sm:grid-cols-3 grid-cols-2 w-full mt-10">
            <div class={format!("rounded-tl-2xl {}",company_common_styles)}>
              {"Reliability"}
            </div>
            <div class={company_common_styles}>{"Security"}</div>
            <div class={format!("sm:rounded-tr-2xl {}",company_common_styles)}>
              {"Ethereum"}
            </div>
            <div class={format!("sm:rounded-bl-2xl {}",company_common_styles)}>
             {" Web 3.0"}
            </div>
            <div class={company_common_styles}>{"Low Fees"}</div>
            <div class={format!("rounded-br-2xl {}",company_common_styles)}>
              {"Blockchain"}
            </div>
          </div>
        </div>

        <div class="flex flex-col flex-1 items-center justify-start w-full mf:mt-0 mt-10">
          <div class="p-3 flex justify-end items-start flex-col rounded-xl h-40 sm:w-72 w-full my-5 eth-card .white-glassmorphism">
            <div class="flex justify-between flex-col w-full h-full">
              <div class="flex justify-between items-start">
                <div class="w-10 h-10 rounded-full border-2 border-white flex justify-center items-center">
                 <span class="text-white" >
                  { Icon::new(IconKind::Strikethrough) }
                </span>

                </div>
                <span class="text-white">
                  { Icon::new(IconKind::Alert) }
                </span>
              </div>
              <div>
                <p class="text-white font-light text-sm">
                  {shorten_address(String::from("0x1BA4Cd2a003cF26dd3201C466De2148EB2F76fe5"))}
                </p>
                <p class="text-white font-semibold text-lg mt-1">
                  {"Ethereum"}
                </p>
              </div>
            </div>
          </div>
          <div class="p-5 sm:w-96 w-full flex flex-col justify-start items-center blue-glassmorphism">
            <Input placeholder={String::from("Name")} input_type={String::from("text")} value={user_info.name.clone()} oninput={oninput_name}/>
            <Input placeholder={String::from("Age")} input_type={String::from("text")} value={user_info.age.clone()} oninput={oninput_age}/>
            <Input placeholder={String::from("Sex")} input_type={String::from("text")} value={user_info.sex.clone()} oninput={oninput_sex}/>
            <Input placeholder={String::from("Address")} input_type={String::from("text")} value={user_info.address.clone()} oninput={oninput_address}/>

            <div class="h-[1px] w-full bg-gray-400 my-2" />

            // {isLoading ? (
            //   <Loader />
            // ) : (
              <button
                type="button"
                onclick={on_create}
                class="text-white w-full mt-2 border-[1px] p-2 border-[#3d4f7c] hover:bg-[#3d4f7c] rounded-full cursor-pointer"
              >
              {"Create"}
              </button>
            // )}
          </div>
        </div>
      </div>
      </div>

      <section class="container mx-auto p-6 font-mono">

      <div class="w-full mb-8 overflow-hidden rounded-lg shadow-lg">
        <div class="w-full overflow-x-auto">
          <table class="w-full">
            <thead>
              <tr class="text-md font-semibold tracking-wide text-left text-gray-900 bg-gray-100 uppercase border-b border-gray-600">
                <th class="px-4 py-3">{"Name"}</th>
                <th class="px-4 py-3">{"Age"}</th>
                <th class="px-4 py-3">{"Sex"}</th>
                <th class="px-4 py-3">{"Address"}</th>
                <th class="px-4 py-3">{"Action"}</th>

              </tr>
            </thead>
            <tbody class="bg-white">
              {users.clone().iter()
                .map(|item| {
                    html! {
                      <tr class="text-gray-700">
                    <td class="px-4 py-3 border">
                      <div class="flex items-center text-sm">
                        <div class="relative w-8 h-8 mr-3 rounded-full md:block">
                          <img class="object-cover w-full h-full rounded-full" src="https://images.pexels.com/photos/5212324/pexels-photo-5212324.jpeg?auto=compress&cs=tinysrgb&dpr=3&h=750&w=1260" alt="" loading="lazy" />
                          <div class="absolute inset-0 rounded-full shadow-inner" aria-hidden="true"></div>
                        </div>
                        <div>
                          <p class="font-semibold text-black">{item.name.clone()}</p>
                        </div>
                      </div>
                    </td>
                    <td class="px-4 py-3 text-ms font-semibold border">{item.age.clone()}</td>
                    <td class="px-4 py-3 text-xs border">
                      <span class="px-2 py-1 font-semibold leading-tight text-green-700 bg-green-100 rounded-sm"> {item.sex.clone()} </span>
                    </td>
                    <td class="px-4 py-3 text-sm border">{item.address.clone()}</td>
                  <td class="px-4 py-3 text-ms font-semibold border">

                  <button
                      type="button"
                      class="text-black w-full mt-2 border-[1px] p-2 border-[#3d4f7c] rounded-full cursor-pointer"
                      onclick={
                        let users = users.clone();
                        let id = item.id.clone();
                        Callback::from(move |ev: MouseEvent| {
                          let mut cloned_users = (*users).clone();
                          let index = cloned_users.iter().position(|x| *x.id == id).unwrap();
                          cloned_users.remove(index);

                          users.set(cloned_users);
                          ev.prevent_default();
                        })
                      }
                  >
                    {"Delete"}
                  </button>
                  </td>
                  </tr>
                    }
                })
                .collect::<Html>()}




            </tbody>
          </table>
        </div>
      </div>
    </section>

      </div>


    }
}
