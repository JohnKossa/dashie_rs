use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use crate::shared_global::SharedGlobal;

#[derive(Clone)]
pub struct RequestContext {
	pub global: Arc<SharedGlobal>,
	pub params: HashMap<String, String>,
}

impl RequestContext {
	pub fn param(&self, name: &str) -> Option<&str> {
		self.params.get(name).map(|s| s.as_str())
	}

	pub fn global<T>(&self, key: &str) -> Option<&T>
	where
			T: Any + Send + Sync + 'static,
	{
		self.global.get::<T>(key)
	}
}