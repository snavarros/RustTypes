fn main() {
    //let v1 = vec![1,2,3];
    //let v1_iter = v1.iter();

  //let dark_side = vec!["fear","anger","hate","suffering"];

  /* Sin iterators
  let mut i = 0;
  while i < dark_side.len() {
    println!("i: {}, item: {}", i , dark_side[i]);
    i +=1;
  }
  */

  //Con iteratora
  //let dark_side_iter = dark_side.iter();

  /*
  for item in dark_side_iter{
    println!("item: {}", item);
  }
  */

  let v1 : Vec<i32> = vec![1,2,3];

  let x: Vec<i32> = v1.iter()
    .map(|x| x+1)
    .filter(|x| *x > 1)
    .collect();

  //let y: Vec<i32> = x.collect();

  println!("{:?}", x);
  
}