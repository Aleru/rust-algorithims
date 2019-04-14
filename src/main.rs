//hello it works!
struct ArrAndCount{
	vec: Vec<i32>,
	count: usize,
}

fn build_ArrAndCount(vec: Vec<i32>, count: usize)->ArrAndCount{
ArrAndCount{
	vec:vec,
	count:count
}
}

fn sort_and_count_inv(mut v1: Vec<i32>) -> ArrAndCount{
	let mut n = v1.len();
	
	if n < 2 {
		 build_ArrAndCount(v1,0)
	
	}
	else{
		
		let mut left_inv = sort_and_count_inv(v1[0..n/2].to_vec());
		let mut right_inv = sort_and_count_inv(v1[n/2..n].to_vec());
		let mut merged_inv = merge_and_count_split_inv(left_inv.vec,right_inv.vec);

		build_ArrAndCount(merged_inv.vec, merged_inv.count + left_inv.count+right_inv.count)


		
	}
	
	
	

	}



fn merge_and_count_split_inv( v1:  Vec<i32>, v2: Vec<i32>) ->   ArrAndCount  {
	let (mut i, mut j, mut split_inv) = (0,0,0);
	let n = v1.len() * 2;
	let mut v3: Vec<i32> = Vec::new();

	for _x in 0..n{
		


		if v1[i] < v2[j] {
			v3.push(v1[i]);
			i = i + 1;
			if i == v1.len() {
			for _y in j.. n/2{
				v3.push(v2[j]);
				j = j + 1;
			}
			
			break;

		}
			
		}
		else{
			v3.push(v2[j]);
			split_inv = split_inv + (n/2 - i);
			j = j +1;
			if j == v1.len() {

			for _y in i.. n/2{
			v3.push(v1[i]);
			i = i + 1; }
			break;

		}
		}
		}

	ArrAndCount {
		vec: v3,
		count: split_inv,
	}


	}



fn main() {
let  vec1 = vec![1,2,3];
let  vec2 = vec![4,5,6];



let arr2 = merge_and_count_split_inv(vec1,vec2);
println!("{:?}  {}", arr2.vec, arr2.count);


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
