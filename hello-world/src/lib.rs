pub fn hello(name: Option<&str>) -> String {
  match name {
      Some(name) => "Hello, ".to_string() + name + "!",
      None => "Hello, World!".to_string()
  }
}