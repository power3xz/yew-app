use yew::prelude::*;

struct Model {
    value: i32,
}

#[function_component(App)]
pub fn app() -> Html {
    let state = use_state(|| Model { value: 0 });
    let onclick = {
        let counter = state.clone();
        Callback::from(move |_| {
            counter.set(Model {
                value: counter.value + 1,
            })
        })
    };
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ state.value }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
