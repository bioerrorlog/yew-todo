use yew::prelude::*;
use components::header::Header;
use components::list::List;
use components::form::Form;

mod components;

#[function_component(App)]
fn app() -> Html {
    html! {
        <>
            <Header />
            <main class="container-fluid">
                <Form />
                <List />
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
