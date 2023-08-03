use yew::prelude::*;
#[function_component]
fn App() -> Html{
    html!{
        <h1>{"hello"}</h1>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
