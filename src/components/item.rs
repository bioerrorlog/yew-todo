use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ItemProps {
    pub title: String,
    pub completed: bool,
}

#[function_component(Item)]
pub fn item(props: &ItemProps) -> Html {
    html! {
        <li class="list-group-item">
            <input class="form-check-input" type="checkbox" checked={props.completed} />
            {&props.title}
        </li>
    }
}
