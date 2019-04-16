use std::fs;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::io::BufReader;


struct VecAndCount{
	vec: Vec<i32>,
	count: usize,
}

// constructor
fn build_VecAndCount(vec: Vec<i32>, count: usize)->VecAndCount{
VecAndCount{
	vec:vec,
	count:count
}
}


fn sort_and_count_inv(mut v1: Vec<i32>) -> VecAndCount{
	let mut n = v1.len();    // number of elements in passed vec
	
	if n < 2 {   					// base case returns the vec b.c already sorted and 0 because there has to be 
									// 0 inversions
		 build_VecAndCount(v1,0)
	
	}
	else{
		
		let mut left_inv = sort_and_count_inv(v1[0..n/2].to_vec()); // recursively sort left half of the array
		let mut right_inv = sort_and_count_inv(v1[n/2..n].to_vec());	// recursively right half of hte array
		let mut merged_inv = merge_and_count_split_inv(left_inv.vec,right_inv.vec); // 

		build_VecAndCount(merged_inv.vec, merged_inv.count + left_inv.count+right_inv.count)


		
	}
	
	
	

	}


//takes two sorted vectors, counts split inversions and returns 
fn merge_and_count_split_inv( v1:  Vec<i32>, v2: Vec<i32>) ->   VecAndCount  {
	let (mut l_ctr,mut r_ctr,mut split_inv) = (0,0,0);
	let mut v3: Vec<i32> = Vec::new();

	while l_ctr < v1.len() && r_ctr < v2.len(){
		
		if v1[l_ctr] < v2[r_ctr]{
			v3.push(v1[l_ctr]);
			l_ctr = l_ctr + 1;
		}

		else{
			split_inv = split_inv + (v1.len() - l_ctr);
			v3.push(v2[r_ctr]);
			r_ctr = r_ctr + 1;

		}}

/*

		WHEN THIS INVLADE CODE SUCH AS THE BELOW IS RAN COMPILER NEVER GETS TO THE RETURN STATEMENT AND IT COMPLAINS ABOUT NOT HAVING A RETURN VALUE INSTEAD OF THIS INVLAID CODE
		v3.push(v1[l_ctr:v1.len()]); 


*/


		for _x in l_ctr.. v1.len(){
			v3.push(v1[_x]);
		}

		for _x in r_ctr..v2.len(){
			v3.push(v2[_x]);
		}
		build_VecAndCount(v3,split_inv)

		


	}
	

// make function to open a file and put numbers  into vector

fn file_into_i32_vec(s: &str) -> Result<Vec<i32>,io::Error>{

	let f = File::open(s)?;

	let reader = BufReader::new(f);

	let mut vec: Vec<i32> = Vec::new();

	for line in reader.lines() {
		let l = match line{
			Ok(str) => str,
			Err(error) => {
				panic!("there was an error parsing the line in the opened file {:?}", error)
			}
			};
		let input: i32 = l.trim().parse().expect("attempted to parse non numeric");
		 vec.push(input);

		};

	Ok(vec)


}



fn main()  {

	let mut vec = file_into_i32_vec("algo.txt").unwrap();

 	 let vec_and_count_to_parse = sort_and_count_inv(vec);
	println!("{}", vec_and_count_to_parse.count);
			  
   

 }
  




// unit tests for merge function
/*#[cfg(test)]
mod tests{
	use crate::merge_and_count_split_inv; 
	#[test]
	fn check_msci(){
		let  vec1 = vec![1,2,3];
		let  vec2 = vec![4,5,6];

		let arr = merge_and_count_split_inv(vec2,vec1);
		assert_eq!(arr.vec, [1,2,3,4,5,6]);
	}
}

*/


