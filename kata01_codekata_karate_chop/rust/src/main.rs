fn chop<T>(x: T, slice: &[T]) -> usize where T: Ord {
  let mut left = 0;
  let mut right = slice.len();
  let count = |left, right| right - left;

  while count(left, right) > 1 {
    let center = (left + right) / 2;
    if slice[center] > x {
      right = center
    }
    else if slice[center] < x {
      left = center
    }
    else {
      return center;
    }
  }

  left
}

fn main() {
  println!("chop(5, [1, 3, 5, 7]): {}", chop(5, &[1, 3, 5, 7]));
}
