use yew::prelude::*;
use web_sys::*;
use wasm_bindgen::JsCast;

#[function_component(Producer)]
fn producer() -> Html {
    let navigator = window().unwrap().navigator();
    let media_devices = navigator.media_devices().unwrap();
    let video_element = window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("webcam")
        .unwrap()
        .unchecked_into::<web_sys::HtmlVideoElement>();

    html! {
    <div class="producer">
        <h3>{"Producer"}</h3>
        <video autoplay=true id="webcam"></video>
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
