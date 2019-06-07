



#[cfg(test)]
mod tests {
  use std::rc::Rc;
  #[test]
  fn nope() {
      let s: Rc<String> = Rc::new("foo".to_string());
      let t: Rc<String> = s.clone();
      let u: Rc<String> = s.clone();

      
  }
}
