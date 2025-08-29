use yew::prelude::*;

#[function_component]
pub fn Hero() -> Html {
    return html! {
        <div class="h-[90vh] overflow-hidden bg-black relative">
            <img class="w-full h-full object-cover absolute top-0 left-0" src="/static/Solarpunk_Melbourne_Sunset_1.png" />
            <div class="relative z-10 flex h-full items-center bg-black/25">
                <div class="bg-slate-900 py-5 px-8 lg:h-max ml-5 mt-10 mr-5 lg:ml-10 rounded-md">
                    <h1 class="text-5xl lg:text-4xl mb-2">{"Hello, i am Alexander Cooper ❤️ 🇦🇺"}</h1>
                    <h1 class="text-3xl lg:text-xl">{"Rust AI model developer / Innovator"}</h1>
                </div>
            </div>
        </div>
    };
}
