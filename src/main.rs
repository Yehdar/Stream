use yew::prelude::*;

#[function_component(Producer)]
fn producer() -> Html {
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

fn main() {
    yew::Renderer::<Producer>::new().render();
    yew::Renderer::<Consumer>::new().render();
}
