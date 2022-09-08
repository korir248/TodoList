use crate::todo_list::{TodoList, TodoStatus};

pub fn mark_item_as_in_progress(mut list: TodoList, item_index: usize) {
    list.todos[item_index].status = TodoStatus::InProgress;
}