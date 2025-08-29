use yew::prelude::*;

use crate::components::social_link::{SocialLink, SocialType};

#[function_component]
pub fn Socials() -> Html {
    return html! {
        <div class="">
            <h1 class="text-5xl lg:text-4xl mx-5 py-4">{"Socials"}</h1>
            <div class="h-0 border-2 border-slate-600 mx-5 mb-2"></div>
            <div class="flex flex-wrap px-4 py-2">
                <SocialLink social_type={SocialType::Email} text="coop@alexandercoop.com" href="mailto:coop@alexandercoop.com"/>
                <SocialLink social_type={SocialType::GitHub} text="@Ukcoop" href="https://github.com/Ukcoop"/>
                <SocialLink social_type={SocialType::YouTube} text="@Ukcoop562" href="https://www.youtube.com/@Ukcoop562"/>
                <SocialLink social_type={SocialType::BlueSky} text="@alexandercoop.com" href="https://bsky.app/profile/alexandercoop.com"/>
                <SocialLink social_type={SocialType::Instagram} text="@ukcoop562" href="https://www.instagram.com/ukcoop562/"/>
                <SocialLink social_type={SocialType::Twitch} text="@ukcoop562" href="https://www.twitch.tv/ukcoop562"/>
            </div>
        </div>
    };
}
