
// shake sort
// !!! tests int testing.rs are very brittle, will break on any changes to sorting function

/*******************************************************
//there are log2(x) splits
*******************************************************/

use types::IndexedVal;

pub struct ShakeElement {
	val: i32, 
	sorted_loc: Option<usize>,
	initial_loc: usize,
	unsorted_locs: Vec<usize>, //After 1 swap is at index 0, after 2 at 1...
}
#[derive(Clone)] // !!! Clone is derived to make cloning PartitionSeparator easier. Do it without deriving clone here for style
pub struct Pivot {
	num_previous: usize,
	num_lower: usize,
	num_higher: usize
}
#[derive(Clone)]
pub struct PartitionSeparator {
	idx: usize,
	lower_size: usize,
	upper_size: usize,
	pivots: Vec<Pivot>
}
pub struct ShakeProfile {
	idx: u32,
	partitions: Vec<PartitionSeparator>
}

pub struct Output {
	profile: ShakeProfile,
	separators: Vec<PartitionSeparator>,
	elements: Vec<ShakeElement>
}

// !!! create tests to make sure output is correct
// !!! randomness and reasonable handling of sorted lists can be accomplished by randomly choosing the first partition
	// and then comparing it to the lowest index and highest index on every other comparison. If the partition does not evenly distribute
	// the elements, check to see if it has partitioned enough of the array that its own index is within the bounds of the highest and
	// lowest unsorted index. If it is still within the bounds, use an adjacent index as the next partition????, if it is outside the bounds
	// average the lowest and highest unsorted index to get the next partition
// !!! is there a way of handling of adverse datasets?
pub fn sort(values: &mut [IndexedVal]) -> Vec<ShakeElement> {
	
    let mut output = initialize_output(&values);
	partition(values, &mut output);
    bubble_sort(values);
	
	//record the positions of the variables after the swap
	for i in 0..values.len() {
		output[values[i].idx].sorted_loc = Some(i);
	}
    
    return output;
}

// !!! make efficient / make better bubble sort (bigger variable length bubbles?)
fn bubble_sort(values: &mut [IndexedVal]){
    
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
}

fn initialize_output(vals: &[IndexedVal]) -> Vec<ShakeElement> {
	let num = vals.len();
	let mut data: Vec<ShakeElement> = Vec::with_capacity(num);
	for idx in 0..num {
        data.push( ShakeElement {
			val: vals[idx].val, 
			sorted_loc: None,
			initial_loc: idx,
			unsorted_locs: Vec::new()
		});
    }
	
	data
}

fn partition(values: &mut [IndexedVal], output: &mut Vec<ShakeElement>) {
	
	// !!! let mut helper_vec: Vec<IndexedVal> = Vec::with_capacity(values.len());
		// !!! place values in second array or use only the same array as quick sort does?
	
	// !!! use a vector to store indexes of cutoffs. It may or may not be beneficial to retain cutoff positions for bubbling step
	// !!! or consider storing and iterating through cutoffs in a way to promote memory locality
	
	
	let mut partitions: Vec<PartitionSeparator> = Vec::with_capacity(values.len() / 2);
	let mut partition = PartitionSeparator {
		idx: 0,
		lower_size: 0,
		upper_size: values.len(),
		pivots: Vec::new(),
	};
	
	let mut partition_idx: usize = 0;
	
	
	//loop until all partitions are less than or equal to 3 in size
	loop {
		
		//lower and upper half
		for half in 1 .. 3 {
			if ((half == 1) && (partition.lower_size > 3)) || ((half == 2) && (partition.upper_size > 3)){
				
				let mut lower_idx;
				let mut upper_idx;
				let mut num_higher;
				let mut num_lower;
				let mut pivot;
				
				if half == 1 {
					lower_idx = partition.idx - partition.lower_size;	//partition index - first half size
					upper_idx = partition.idx - 1; //partition index - 1
				} else {
					lower_idx = partition.idx; // partion index
					upper_idx = partition.idx + partition.upper_size - 1 ; // partition index - 1 + second half size
				}
				
				// !!! whole pivot is copied to match test output, could be left in place. Also, storing val directly would be faster
				pivot = values[lower_idx].clone();
				lower_idx += 1;
				
				// compare to first element
				// !!! make the first pivot a median of 3
				if values[lower_idx].val >= pivot.val {
					let temp = values[upper_idx].clone();
					values[upper_idx] = values[lower_idx].clone();
					values[lower_idx] = temp;
					upper_idx -= 1;
					num_higher = 1;
					num_lower = 0;
				} else {
					values[lower_idx - 1] = values[lower_idx].clone();
					lower_idx += 1;
					num_higher = 0;
					num_lower = 1;
				}
				
				// split around a pivot. if the pivot is determined to be far from the center, switch pivot
				// !!! different check times than 2,4,7,10,13... ???
				// !!! pivot itself goes in upper or lower half or neither? does it matter?
					// Probably better to leave in lower partition so extra comparisons/swaps don't occur for an element that can go either way
				loop {
					
					if lower_idx == upper_idx {
						// compare the last value before break
						// pivot is always considered in lower partition
						if values[upper_idx].val >= pivot.val {
							upper_idx -= 1;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
						}
						
						values[upper_idx] = pivot.clone(); // !!! put in opposite half of latest element for evening out short partitions
						break;
					}
					
					// !!! for values equal to the pivot, should they go all the same way or on the side with fewer elements?
					if num_higher == 1 {
						if values[lower_idx].val >= pivot.val {
							values[lower_idx - 1] = pivot.clone();
							pivot = values[lower_idx].clone();
							lower_idx += 1;
							num_lower = 1;
							num_higher = 0; 
							continue;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
							num_lower = 1;
							num_higher = 1;
						}
					} else {
						if values[upper_idx].val >= pivot.val {
							upper_idx -= 1;
							num_higher = 1;
							num_lower = 1;
						} else {
							let temp = values[upper_idx].clone();
							values[upper_idx] = pivot.clone();
							pivot = temp;
							upper_idx -= 1;
							num_higher = 1;
							num_lower = 0;
							continue;
						}
					}
					
					
					loop {
						if lower_idx == upper_idx {
							break;
						}
						
						if (num_lower + num_higher) % 3 == 0 {
							if num_lower == (num_lower + num_higher) / 3 {
								if values[lower_idx].val >= pivot.val {
									values[lower_idx - 1] = pivot.clone();
									pivot = values[lower_idx].clone();
									lower_idx += 1;
									num_lower = 1;
									num_higher = 0; 
									break;
								} else {
									values[lower_idx - 1] = values[lower_idx].clone();
									lower_idx += 1;
									num_lower += 1;
									continue;
								}
							} else if num_higher == (num_lower + num_higher) / 3 {
								if values[upper_idx].val >= pivot.val {
									upper_idx -= 1;
									num_higher += 1;
									continue;
								} else {
									let temp = values[upper_idx].clone();
									values[upper_idx] = pivot.clone();
									pivot = temp;
									upper_idx -= 1;
									num_higher = 1;
									num_lower = 0;
									break;
								}
							}
						} 
						if values[lower_idx].val >= pivot.val {
							let temp = values[upper_idx].clone();
							values[upper_idx] = values[lower_idx].clone();
							values[lower_idx] = temp;
							upper_idx -= 1;
							num_higher += 1;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
							num_lower += 1;
						}
					}
					
					
				}
				
				
				//by here, all elements in indices below lower_idx are lower partition, 
					//and all elements above upper_idx are upper partition, with no overlap
				let lower_size;
				let upper_size;
				
				if half == 1 {
					upper_size = partition.idx - lower_idx;
					lower_size = partition.lower_size - upper_size;
				} else {
					lower_size = lower_idx - partition.idx;
					upper_size = partition.upper_size - lower_size;
				}
				
				// record the positions of the variables after the swap
				for i in lower_idx - lower_size .. lower_idx + upper_size {
					output[values[i].idx].unsorted_locs.push(i);
				}
				
				partitions.push( PartitionSeparator {
					idx: lower_idx,
					lower_size,
					upper_size,
					pivots: Vec::new(), // >:<
				}); 
			
			}
		}
		
		
		if partitions.len() == partition_idx {
			break;
		} else {
			partition = partitions[partition_idx].clone();
			partition_idx += 1;
			continue;
		}
		
	}
}

//TESTS
#[cfg(test)]
mod testing;

