use yew::prelude::*;

use crate::components::icon::Icon;

#[derive(PartialEq)]
pub enum SocialType {
    Email,
    GitHub,
    YouTube,
    BlueSky,
    Instagram,
    Twitch,
}

#[derive(Properties, PartialEq)]
pub struct SocialLinkProps {
    pub social_type: SocialType,
    pub text: String,
    pub href: String,
}

#[function_component]
pub fn SocialLink(props: &SocialLinkProps) -> Html {
    let (slug, color) = match props.social_type {
        SocialType::Email => ("protonmail", "6D4AFF"),
        SocialType::GitHub => ("github", "FFFFFF"),
        SocialType::YouTube => ("youtube", "FF0000"),
        SocialType::BlueSky => ("bluesky", "0285FF"),
        SocialType::Instagram => ("instagram", "FF0069"),
        SocialType::Twitch => ("twitch", "9146FF"),
    };

    return html! {
        <a class={format!("w-full lg:w-max px-4 py-4 lg:py-2 mx-2 lg:ml-2 mb-4 lg:mb-2 border-2 rounded-md border-[#{}] flex items-center", color)} href={props.href.clone()}>
            <Icon slug={slug} color={color}/>
            <h1 class="pl-4 lg:pl-2 text-5xl lg:text-xl">{props.text.clone()}</h1>
        </a>
    };
}
