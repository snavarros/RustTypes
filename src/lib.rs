#[test]
fn iterator_demostration() {
    /*
    let v1 = vec![1,2,3];

    let mut v1_iter = v1.iter();
    assert_eq!(v1_iter.next(), Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);
    */

    let v1 = vec!["fear", "anger", "hate", "suffering"];

    let mut v1_iter = v1.into_iter();

    assert_eq!(v1_iter.next(), Some("fear"));
    assert_eq!(v1_iter.next(), Some("anger"));
    assert_eq!(v1_iter.next(), Some("hate"));
    assert_eq!(v1_iter.next(), Some("suffering"));
    assert_eq!(v1_iter.next(), None);
}

#[test]
fn iterator_sum(){
  let v1 = vec![1,2,3];

  let v1_iterator = v1.iter();

  let total :i32 = v1_iterator.sum();

  assert_eq!(total,6);
}





/*
pub trait Iterator {
  type Item;

  fn next(&mut self) -> Option<Self::Item>;
}
*/
