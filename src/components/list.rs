use yew::{function_component, html, Html};
use crate::components::item::Item;
use crate::components::types::Todo;

#[function_component(List)]
pub fn list() -> Html {
    let todos = vec![
        Todo {
            id: 1,
            title: "Learn Rust".to_string(),
            completed: false,
        },
        Todo {
            id: 2,
            title: "Learn Yew".to_string(),
            completed: true,
        },
        Todo {
            id: 3,
            title: "Learn Motoko".to_string(),
            completed: false,
        },
    ];

    html! {
        <ul class="list-group">
            {todos.iter().map(|todo| html! {
                <Item title={todo.title.clone()} completed={todo.completed} />
            }).collect::<Html>()}
        </ul>
    }
}
