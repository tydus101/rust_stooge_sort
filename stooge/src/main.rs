use std::fmt::Debug;
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

fn print_vec<T>(v: &mut [T]) where T: Debug{
	for i in v.iter() {
		print!("{:?} ", i);
	}
	println!("")
}


fn main() {
	let mut v: Vec<i32> = vec![1337 ,1738, 69, 420, 9001];
	sort(&mut v);
}