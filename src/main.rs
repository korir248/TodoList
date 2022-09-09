use todo::{TodoList, TodoItem,TodoStatus, add_item::add_item, mark_item_as_in_progress::mark_item_as_in_progress, remove_item::remove_item};

fn main() {
    let mut list = TodoList { todos: vec![] };

    let item = TodoItem{
        title: String::from("Go Shopping"),
        description : String::from("Shopping for food items"),
        status:TodoStatus::New, 
        
        
    };

    add_item(&mut list,item);
    println!("{:#?}",list);
    add_item(&mut list, TodoItem { title: String::from("hi"), description: String::from("hi1"), status: TodoStatus::Done });
    mark_item_as_in_progress(&mut list,0);

    remove_item(&mut list,1);
    println!("{:#?}",list);
    


}