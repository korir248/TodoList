use crate::todo_list::{TodoList, TodoItem};

pub fn add_item(list: &mut TodoList, item: TodoItem) {
    list.todos.push(item);
}

