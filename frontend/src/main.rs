use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
    <section class="section">
        <div class="container">
            <h1 class="button is-primary">
                {"Hello World"}
            </h1>
            <p class="subtitle is-warning">
                {"My first website with "} <strong>{"Bulma"}</strong> {"!"}
            </p>
        </div>
    </section>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
