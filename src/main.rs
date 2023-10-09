use yew::prelude::*;
use web_sys::*;
use wasm_bindgen::*;
#[function_component(Producer)]
fn producer() -> Html {
    html! {
        <div class="producer">
            <h3>{"Producer"}</h3>
            <div class="black-box"></div> 
        </div>
    }
}

#[function_component(Consumer)]
fn consumer() -> Html {
    html! {
        <div class="consumer">
            <h3>{"Consumer"}</h3>
            <div class="black-box"></div>
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
