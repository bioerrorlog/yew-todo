use yew::{function_component, html, Html, use_state, Callback, InputEvent};

#[function_component(Form)]
pub fn form() -> Html {
    let title = use_state(|| "".to_string());

    let oninput = {
        let title = title.clone();
        Callback::from(move |e: InputEvent| {
            let value = e.data();

            match value {
                Some(value) => {
                    title.set((*title).clone() + &value);
                }
                None => {
                    title.set("".to_string());
                }
            }
        })
    };

    html! {
        <form class="mb-5">
            <div class="mb-3">
                <label for="title" class="form-label">{"title"}</label>
                <input type="text" class="form-control" id="title" value={(*title).clone()} {oninput}/>
            </div>
            <div class="mb-3">
                {&*title}
            </div>
            <button type="submit" class="btn btn-primary">{"Add"}</button>
        </form>
    }
}
