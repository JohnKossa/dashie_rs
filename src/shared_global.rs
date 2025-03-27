use std::{any::Any, collections::HashMap, sync::Arc};

#[derive(Default)]
pub struct SharedGlobal {
	pub data: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl SharedGlobal {
	pub fn new() -> Self {
		SharedGlobal {
			data: HashMap::new(),
		}
	}

	pub fn register<T>(&mut self, key: &str, value: T)
	where
			T: Any + Send + Sync + 'static,
	{
		self.data.insert(key.to_owned(), Arc::new(value));
	}

	pub fn get<T>(&self, key: &str) -> Option<&T>
	where
			T: Any + Send + Sync + 'static,
	{
		self.data.get(key).and_then(|arc_any| arc_any.downcast_ref::<T>())
	}
}