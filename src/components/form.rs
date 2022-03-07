use yew::{function_component, html, Html};

#[function_component(Form)]
pub fn form() -> Html {
    html! {
        <form class="mb-5">
            <div class="mb-3">
                <label for="title" class="form-label">{"title"}</label>
                <input type="text" class="form-control" id="title" />
            </div>
            <button type="submit" class="btn btn-primary">{"Add"}</button>
        </form>
    }
}
