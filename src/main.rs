use yew::prelude::*;
use web_sys::*;
use wasm_bindgen::*;
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

    let mut constraints = MediaStreamConstraints::new();
    constraints.video(&JsValue::from(true));

    let devices_query = media_devices
        .get_user_media_with_constraints(&constraints)
        .unwrap();
    
    let device = JsFuture::from(devices_query)
        .await
        .unwrap()
        .unchecked_into::<MediaStream>();

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
