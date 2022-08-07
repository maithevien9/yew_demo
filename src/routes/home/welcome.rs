use yew::html;
use yew_octicons::Icon;
use yew_octicons::IconKind;

use crate::components::input::Input;
use crate::utils::shorten_address::shorten_address;
use yew::prelude::*;

#[function_component(Welcome)]
pub fn welcome() -> Html {
    let company_common_styles =
  "min-h-[70px] sm:px-0 px-2 sm:min-w-[120px] flex justify-center items-center border-[0.5px] border-gray-400 text-sm font-light text-white";

    html! {
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
        //   {!currentAccount && (
        //     <button
        //       type='button'
        //       // onClick={connectWallet}
        //       class='flex flex-row justify-center items-center my-5 bg-[#2952e3] p-3 rounded-full cursor-pointer hover:bg-[#2546bd]'
        //     >
        //       <AiFillPlayCircle class='text-white mr-2' />
        //       <p class='text-white text-base font-semibold'>
        //         Connect Wallet
        //       </p>
        //     </button>
        //   )}

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
            <Input placeholder={String::from("Address To")} input_type={String::from("text")} value={String::from("")}/>
            <Input placeholder={String::from("Amount (ETH)")} input_type={String::from("text")} value={String::from("")}/>
            <Input placeholder={String::from("Keyword (Gif)")} input_type={String::from("text")} value={String::from("")}/>
            <Input placeholder={String::from("Enter Message")} input_type={String::from("text")} value={String::from("")}/>


            // <Input
            //   placeholder="Address To"
            //   name="addressTo"
            //   type='text'
            //   handleChange={handleChange}
            // />
            // <Input
            //   placeholder='Amount (ETH)'
            //   name='amount'
            //   type='number'
            //   handleChange={handleChange}
            // />
            // <Input
            //   placeholder='Keyword (Gif)'
            //   name='keyword'
            //   type='text'
            //   handleChange={handleChange}
            // />
            // <Input
            //   placeholder='Enter Message'
            //   name='message'
            //   type='text'
            //   handleChange={handleChange}
            // />

            <div class="h-[1px] w-full bg-gray-400 my-2" />

            // {isLoading ? (
            //   <Loader />
            // ) : (
            //   <button
            //     type='button'
            //     onClick={handleSubmit}
            //     class='text-white w-full mt-2 border-[1px] p-2 border-[#3d4f7c] hover:bg-[#3d4f7c] rounded-full cursor-pointer'
            //   >
            //     Send now
            //   </button>
            // )}
          </div>
        </div>
      </div>
    </div>

    }
}
