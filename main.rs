fn main() {
 let data:[[&str;3];3] = [["c","b","a"],["d","a","f"],["g","h","i"]] ; 

  for indx in 0..3 {
    println!("is this column alphabetically ordered:  {:?}", check_order(indx, &data));
  }
}
fn check_order(i: usize, arr: &[[&str;3];3]) -> bool {
      println!("colmn: {}-{}-{}", arr[0][i],arr[1][i],arr[2][i]);
  arr[i][0]>arr[i][2] && arr[i][1]>arr[i][2]
}
