use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct IconProps {
    pub slug: String,
    pub color: String,
}

#[function_component]
pub fn Icon(props: &IconProps) -> Html {
    return html! {
        <div class="">
            <img height="32" width="32" src={format!("https://cdn.simpleicons.org/{}/{}", props.slug, props.color)} />
        </div>
    };
}
