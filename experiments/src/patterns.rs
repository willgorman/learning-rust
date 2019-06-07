#[cfg(test)]
mod tests {
  #[test]
  fn basic() {
      let x = 'n';
      match x {
        'c' => println!("one", ),
        _ => println!("other", ),
      }
  }

  #[test]
  fn structs() {
      struct Point {
        x: i32,
        y: i32,
      }

      let o = Point { x: 123, y: 321 };
      
      match o {
        Point { x, .. } => println!("{}", x)
      }
  }

  #[test]
  fn partial_move() {
      let tuple: (u32, String) = (5, String::from("five"));
      let (x, _) = tuple;
      println!("Tuple {:?}", tuple)
  }

  #[test]
  fn ref_pattern() {
      let mut x = 6;
      match x {
        ref mut mr => println!("{}", mr)
      }
  }

  #[test]
  fn ranges() {
      let x = 1;
      let y = match x {
        1 ... 5 => 10,
        _ => 11,
      };
      println!("y: {}", y)
  }
}
