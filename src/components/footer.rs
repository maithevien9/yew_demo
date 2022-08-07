use yew::prelude::*;

#[function_component(Footer)]
pub fn footer() -> Html {
    html! {
        <div class="w-full flex md:justify-center justify-between items-center flex-col p-4 gradient-bg-footer">
          <div class="w-full flex sm:flex-row flex-col justify-between items-center my-4">
            <div class="flex flex-[0.5] justify-center items-center">
               <img src="https://res.cloudinary.com/dfi8bluhn/image/upload/v1659807903/pbl6/1659807903200-bullet_afn5jv.png" alt="logo" class="w-10 cursor-pointer" />
            </div>
            <div class="flex flex-1 justify-evenly items-center flex-wrap sm:mt-0 mt-5 w-full">
              <p class="text-white text-base text-center mx-2 cursor-pointer font-semibold">
                {"Market"}
              </p>
              <p class="text-white text-base text-center mx-2 cursor-pointer font-semibold">
                {"Exchange"}
              </p>
              <p class="text-white text-base text-center mx-2 cursor-pointer font-semibold">
                {"Tutorials"}
              </p>
              <p class="text-white text-base text-center mx-2 cursor-pointer font-semibold">
                {"Wallets"}
              </p>
            </div>
          </div>

          <div class="flex justify-center items-center flex-col mt-5">
            <p class="text-white text-sm text-center font-semibold">
              {"Come join us and hear for the unexpected miracle"}
            </p>
            <p class="text-white text-sm text-center font-medium mt-2 font-bold">
             {"info@google.com"}
            </p>
          </div>

          <div class="sm:w-[90%] w-full h-[0.25px] bg-gray-400 mt-5"/>

          <div class="sm:w-[90%] w-full flex justify-between items-center mt-3">
            <p class="text-white text-left text-xs">{"@kryptomastery2022"}</p>
            <p class="text-white text-right text-xs">{"All rights reserved"}</p>
          </div>
        </div>
    }
}
