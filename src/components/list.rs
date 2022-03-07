use yew::{function_component, html, Html};
use crate::components::item::Item;
use crate::components::types::Todo;

#[function_component(List)]
pub fn list() -> Html {
    let todo = Todo {
        id: 1,
        title: "Learn yew".to_string(),
        completed: false,
    };
    html! {
        <ul class="list-group">
            <Item title={todo.title} completed={todo.completed} />
        </ul>
    }
}
