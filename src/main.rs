use yew::prelude::*;
use web_sys::*;

#[function_component(Producer)]
fn producer() -> Html {
    let navigator: Navigator = window().unwrap().navigator();
    let media_devices: MediaDevices = navigator.media_devices().unwrap();
    html! {
    <div class="producer">
        <h3>{"Producer"}</h3>
    </div>
    }
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html! {
    <div class="consumer">
        <h3>{"Consumer"}</h3>
    </div>
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <div class="grid">
            <Producer/>
            <Consumer/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
