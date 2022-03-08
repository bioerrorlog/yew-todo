use yew::prelude::*;
use components::header::Header;
use components::list::List;
use components::form::Form;

mod components;

#[function_component(App)]
fn app() -> Html {
    let on_add = {
        Callback::from(move |title: String| {
            log::info!("on_add: {:?}", title);
        })
    };

    html! {
        <>
            <Header />
            <main class="container-fluid">
                <Form {on_add}/>
                <List />
            </main>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
    wasm_logger::init(wasm_logger::Config::default())
}
