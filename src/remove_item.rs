use crate::todo_list::TodoList;

pub fn remove_item(mut list: TodoList,item_index: usize) {
    list.todos.remove(item_index);
}