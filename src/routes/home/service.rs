use yew::html;
use yew::prelude::*;
use yew_octicons::Icon;
use yew_octicons::IconKind;

#[derive(Properties, Clone, PartialEq)]
pub struct ServiceCardProps {
    pub color: String,
    pub title: String,
    pub subtitle: String,
    pub icon: Icon,
}

#[function_component(ServiceCard)]
pub fn serviceCard(props: &ServiceCardProps) -> Html {
    html! {
       <div class="flex flex-row justify-start items-start white-glassmorphism p-3 m-2 cursor-pointer hover:shadow-xl">
      <div
        class={format!("w-10 h-10 rounded-full flex justify-center items-center {}", props.color.clone())}
      >
      <span class="text-white">
        {props.icon.clone()}
        </span>
      </div>
      <div class="ml-5 flex flex-col flex-1">
        <h3 class="mt-2 text-white text-lg">{props.title.clone()}</h3>
        <p class="mt-1 text-white text-sm md:w-9/12">{props.subtitle.clone()}</p>
      </div>
    </div>
        }
}

#[function_component(Service)]
pub fn service() -> Html {
    html! {
      <div class="flex w-full justify-center items-center gradient-bg-services">
      <div class="flex mf:flex-row flex-col items-center justify-between md:p-20 py-12 px-4">
        <div class="flex-1 flex flex-col justify-start items-start">
          <h1 class="text-white text-3xl sm:text-5xl py-2 text-gradient">
            {"Services that we"}
            <br />
           {"continue to improve"}
          </h1>
          <p class="text-left my-2 text-white font-light md:w-9/12 w-11/12 text-base">
            {"The best choice for buying and selling your crypto assets, with the
            various super friendly services we offer"}
          </p>
        </div>

        <div class="flex-1 flex flex-col justify-start items-center">
          <ServiceCard
            color={String::from("bg-[#2952E3]")}
            title="Security gurantee"
            subtitle="Security is guranteed. We always maintain privacy and maintain the quality of our products"
            icon={Icon::new(IconKind::CheckCircleFill)}
          />
           <ServiceCard
            color={String::from("bg-[#8945F8]")}
            title="Best exchange rates"
            subtitle="Security is guranteed. We always maintain privacy and maintain the quality of our products"
            icon={Icon::new(IconKind::Browser)}
          />


           <ServiceCard
            color={String::from("bg-[#F84550]")}
            title="Fastest transactions"
            subtitle="Security is guranteed. We always maintain privacy and maintain the quality of our products"
            icon={Icon::new(IconKind::Alert)}
          />
        </div>
      </div>
    </div>
      }
}
