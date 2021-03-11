use std::sync::{Weak,Arc};

pub trait Owned<T> {
  fn set_owner(&self, owner: Weak<T>);
}

pub trait Owner<T> {
  fn add_item(&self, item: Arc<T>);
}