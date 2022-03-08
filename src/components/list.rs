use yew::{function_component, html, Html, Properties};
use crate::components::item::Item;
use crate::components::types::Todo;

#[derive(Properties, PartialEq)]
pub struct ListProps {
    pub todo_items: Vec<Todo>,
}

#[function_component(List)]
pub fn list(props: &ListProps) -> Html {
    html! {
        <ul class="list-group">
            {props.todo_items.iter().map(|todo| html! {
                <Item title={todo.title.clone()} completed={todo.completed} />
            }).collect::<Html>()}
        </ul>
    }
}
