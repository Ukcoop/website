use yew::prelude::*;

mod components;
mod sections;

use sections::{hero::Hero, socials::Socials};

#[function_component]
fn App() -> Html {
    html! {
        <div class="">
            <Hero/>
            <Socials/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
