use crate::todo_list::TodoList;

pub fn remove_item(list: &mut TodoList,item_index: usize) {
    list.todos.remove(item_index);
}