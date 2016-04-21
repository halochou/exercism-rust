use std::collections::BTreeMap;

#[derive(Debug)]
pub struct School (BTreeMap<i32, Vec<String>>);

impl School {
	pub fn new() -> School {
		School(BTreeMap::new())
	}
    pub fn grades(&self) -> Vec<i32> {
    	self.0.keys().cloned().collect()
    }
    pub fn add(&mut self, k: i32, v: &str) {
    	let mut names = self.0.entry(k).or_insert(vec![]);
    	names.push(v.to_string());
    	names.sort();
    }
    pub fn grade(&self, g: i32) -> Option<&Vec<String>> {
    	self.0.get(&g)
    }
}