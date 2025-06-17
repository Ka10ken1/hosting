use yew::prelude::*;

use my_site::components::page::Page;

#[function_component]
fn App() -> Html {
    html!(
        <div>
            <Page/>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
