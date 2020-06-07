
// War sort

use types::IndexedVal;

pub struct OutputData {
    sorted_loc: Option<usize>,
	initial_loc: usize,
	unsorted_locs: Vec<usize>,	//After 1 swap is at index 0, after 2 at 1...
    cross_loc: Option<usize>,
}

pub fn sort(values: &mut [IndexedVal]) -> Vec<OutputData>{
	
	let mut output = initialize_output(values.len());
	
	battle(values, &mut output);
	cross(values, &mut output);
	bubble_sort(values, &mut output);
	
	return output;
}

fn initialize_output(num: usize) -> Vec<OutputData> {
	let mut output: Vec<OutputData> = Vec::with_capacity(num);
	for idx in 0..num {
        output.push( OutputData{
			sorted_loc: None,
			initial_loc: idx,
			unsorted_locs: Vec::new(),
			cross_loc: None,
		});
    }
	output
}

fn battle(values: &mut [IndexedVal], output: &mut Vec<OutputData>){
	
	let log2_of_length: usize = std::mem::size_of::<usize>() * 8 - (values.len().leading_zeros() as usize) - 1;
	
	//split the necessary amount of times
	for i in 0..log2_of_length {
	
		let num_sections = 2_usize.pow(i as u32);
		let section_length = values.len() / num_sections;
		
		//for each section that needs to be split
		for i in 0..num_sections {
			
			let mut lower_idx = i * section_length;
			let mut upper_idx = lower_idx + section_length - 1;
			
			//split the sections into lower and upper halves
			while lower_idx < upper_idx {
				if values[lower_idx].val > values[upper_idx].val {
					let temp = values[lower_idx].clone();
					values[lower_idx] = values[upper_idx].clone();
					values[upper_idx] = temp;
				}
				lower_idx += 1;
				upper_idx -= 1;
			}
			
			if lower_idx == upper_idx {
				panic!("lower and upper indexes equal");
			}
			
		}
		
		//record the positions of the variables after the swap
		for i in 0..values.len(){
			output[values[i].idx].unsorted_locs.push(i);
		}
	}
}

fn cross(values: &mut [IndexedVal], output: &mut Vec<OutputData>){
	let mut i: usize = 2;
	while i < values.len(){
		if values[i].val < values[i-1].val {
			let temp = values[i].clone();
			values[i] = values[i-1].clone();
			values[i-1] = temp;
		}
		i += 2;
	}
	
	for i in 0..values.len() {
		output[values[i].idx].cross_loc = Some(i);
	}
}

fn bubble_sort(values: &mut [IndexedVal], output: &mut Vec<OutputData>){
    let length = values.len();

    for _ in 0 .. length {
        for j in 0 .. length - 1 {
            if values[j].val > values[j+1].val {
                let temp = values[j+1].clone();
                values[j+1] = values[j].clone();
                values[j] = temp;
            }
        }
    }
	
	//record the positions of the variables after the swap
	for i in 0..values.len() {
		output[values[i].idx].sorted_loc = Some(i);
	}
}

//TESTS
#[cfg(test)]
mod testing;