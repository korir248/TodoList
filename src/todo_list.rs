#[derive(Debug)]
pub enum TodoStatus {
    Done,
    InProgress,
    New,
}


#[derive(Debug)]
pub struct TodoItem {
    pub title: String,
    pub description: String,
    pub status: TodoStatus,
}

#[derive(Debug)]
pub struct TodoList {
  pub todos: Vec<TodoItem>
}