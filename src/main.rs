use yew::prelude::*;

#[function_component(Model)]
pub fn model() -> Html {
    let value = use_state(|| 0);
    let onclick = {
        let counter = value.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };
    html! {
        <div>
            <button {onclick}>{ "+1" }</button>
            <p>{ *value }</p>
        </div>
    }
}

fn main() {
    yew::start_app::<Model>();
}
