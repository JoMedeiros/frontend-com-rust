use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! { 
        <>
            <h1> {"Título" } </h1>
            <p> {"hello world"} </p>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}

