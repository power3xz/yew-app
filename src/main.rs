use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[function_component(Secure)]
fn secure() -> Html {
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <Header />
        </div>
    }
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1>},
    }
}

struct Model {
    value: i32,
}

#[function_component(Header)]
pub fn header() -> Html {
    let history = use_history().unwrap();
    let on_click_home = {
        let history = history.clone();
        Callback::from(move |_| history.push(Route::Home))
    };

    let on_click_secure = { Callback::from(move |_| history.push(Route::Secure)) };
    html! {
        <div>
            <button onclick={on_click_home}>{ "Home" }</button>
            <button onclick={on_click_secure}>{ "Secure" }</button>
        </div>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
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
            <Header />
            <button {onclick}>{ "+1" }</button>
            <p>{ state.value }</p>
        </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
