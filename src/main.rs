use yew::prelude::*;

#[function_component]
fn App() -> Html {
    let count = use_state(|| 0);
    let onclick = {
        let count = count.clone();
        move |_| {
            count.set(*count + 1);
        }
    };

    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *count }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
