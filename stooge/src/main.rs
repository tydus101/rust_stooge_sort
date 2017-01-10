use std::fmt::Debug;
pub fn sort<T: PartialOrd >(data: &mut [T]) where T: Debug{
	print_vec(data);
	let n = data.len();
	if data[0] < data[n - 1] {
		data.swap(0, n - 1);
	}

}

fn print_vec<T>(v: &mut [T]) where T: Debug{
	for i in v.iter() {
		print!("{:?} ", i);
	}
	println!("")
}


fn main() {
	let mut v: Vec<i32> = vec![1,3,4,2,5];
	sort(&mut v);
}