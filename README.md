# rust_stooge_sort
 Rust is a blazingly fast language; too fast, some might say. Perhaps it needs a bit of a slowdown... 

 This is an implementation of Stooge Sort in Rust. Stooge Sort is a very simple but ultimately very inefficient sorting algorithm. It has a time complexity of O(n^2.7095..), making it slower than even simple sorts like bubble sort or insertion sort. It thus finds no use in any modern (or past) systems. However, it's simplistic nature and unique time complexity make it useful for educational purposes. Stooge is available as a crate [here](https://crates.io/crates/stooge).

## Usage:
------
```rust
extern crate stooge;

fn main() {
	let mut v: Vec<i32> = vec![1, 5, 4, 3];
	stooge::sort(&mut v);
	return v; // [1, 3, 4, 5]
}
```
I wrote stooge in order to learn Rust. To my knowledge, I am the first person to take up this task in the language. Clearly we are treading historic ground here.
