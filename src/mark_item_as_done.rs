use crate::todo_list::{TodoList, TodoStatus};

pub fn mark_item_as_done(list: &mut TodoList, item_index: usize) {
    list.todos[item_index].status = TodoStatus::Done;
}