//use std::fmt::Debug;

/*
Stooge Sort is a very simple but ultimately 
very inefficient sorting algorithm. It has a time complexity of 
O(n^2.7095..), making it slower than even simple sorts
like bubble sort or insertion sort. It thus finds no 
use in any modern (or past) systems. However, it's simplistic 
nature and unique time complexity make it useful for educational
purposes.

Tyler Reimold, 2017
*/
pub fn sort<T: PartialOrd>(data: &mut [T]){
	let n = data.len();
	if data[0] > data[n - 1] {
		data.swap(0, n - 1);
	}
	if n > 2 {
		let m = ((n) as f32/3.0).ceil();
		let m2 = (n as f32 - m).ceil() as usize;
		sort(&mut data[0..m2]);
		sort(&mut data[m2 - 1..n]);
		sort(&mut data[0..m2]);
	}
}

//A function for printing a vec<i32>, used for debugging.
/*
fn print_vec<T: Debug>(v: &mut [T]) {
	for i in v.iter() {
		print!("{:?} ", i);
	}
	println!("")
}
*/

//Tests
fn main() {
	//1
	let mut v1: Vec<i32> = vec![2, 1, 3, 5];
	sort(&mut v1);
	assert_eq!(v1,[1, 2, 3, 5]);
	//2
	let mut v2: Vec<&str> = vec!["apple", "orange", "banana", "dragonfruit"];
	sort(&mut v2);
	assert_eq!(v2, ["apple", "banana", "dragonfruit", "orange"]);
	//3
	let mut v3:Vec<i32> = vec![2,1];
	sort(&mut v3);
	assert_eq!(v3, [1,2]);
	//4
	let mut v3:Vec<i32> = vec![2,1,2];
	sort(&mut v3);
	assert_eq!(v3, [1,2,2]);
	//5
	let mut v3:Vec<i32> = vec![2,1,3];
	sort(&mut v3);
	assert_eq!(v3, [1,2,3]);
	//6
	let mut v3:Vec<i64> = vec![2,1,3];
	sort(&mut v3);
	assert_eq!(v3, [1,2,3]);
	//7
	let mut v1: Vec<i32> = vec![5, 4, 3, 2, 1];
	sort(&mut v1);
	assert_eq!(v1,[1, 2, 3, 4, 5]);
	println!("All tests completed successfully.")
}