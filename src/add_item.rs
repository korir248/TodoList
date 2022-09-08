use crate::todo_list::{TodoList, TodoItem};

pub fn add_item(mut list: TodoList, item: TodoItem) {
    list.todos.push(item);
}