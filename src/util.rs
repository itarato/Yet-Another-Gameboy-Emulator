pub struct Util;

impl Util {
  pub fn in_range<T: PartialOrd>(min: T, max: T, val: T) -> bool {
    min <= val && val < max
  }
}
