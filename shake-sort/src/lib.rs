
// shake sort
// !!! tests int testing.rs are very brittle, will break on any changes to sorting function

/*******************************************************
//there are log2(x) splits
*******************************************************/

use types::IndexedVal;

pub struct ShakeElement {
	element_id: usize, // the element's initial location
	val: i32, 
	sorted_loc: Option<usize>,
	unsorted_locs: Vec<usize>, //After 1 swap is at index 0, after 2 at 1...
}

// stores a pivot of a partition
	// records how many elements were placed above/below the pivot before the pivot changed or the partition was fully sub-partitioned
#[derive(Clone)]
pub struct Pivot {
	element_id: usize,
	num_previous: usize, // how many pivots came previously when sorting the partition
	num_lower: usize,
	num_higher: usize
}

#[derive(Clone)]
pub struct PartitionSeparator {
	idx: usize,
	lower_size: usize,
	upper_size: usize,
	lower_pivots: Vec<Pivot>,
	upper_pivots: Vec<Pivot>
	// >:< total number of values accounted for in pivots should equal one less than the number of elements 
		// (last pivot is not below or above any other pivots)
}

pub struct ShakeProfile {
	idx: u32,
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
pub fn sort(idx: u32, values: &mut [IndexedVal]) -> ShakeProfile {
	
    let mut elements = initialize_output(&values);
	let separators = partition(values, &mut elements);
    bubble_sort(values);
	
	//record the positions of the variables after the swap
	for i in 0..values.len() {
		elements[values[i].idx].sorted_loc = Some(i);
	}
    
    return ShakeProfile {
		idx: 0,
		separators,
		elements
	};
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
			element_id: idx,
			unsorted_locs: Vec::new()
		});
    }
	
	data
}

fn partition(values: &mut [IndexedVal], elements: &mut Vec<ShakeElement>) -> Vec<PartitionSeparator> {
	
	// !!! let mut helper_vec: Vec<IndexedVal> = Vec::with_capacity(values.len());
		// !!! place values in second array or use only the same array as quick sort does?
	
	// !!! use a vector to store indexes of cutoffs. It may or may not be beneficial to retain cutoff positions for bubbling step
	// !!! or consider storing and iterating through cutoffs in a way to promote memory locality
	
	
	let mut partitions: Vec<PartitionSeparator> = Vec::with_capacity(values.len() / 2);
	partitions.push(PartitionSeparator {
		idx: 0,
		lower_size: 0,
		upper_size: values.len(),
		lower_pivots: Vec::new(),
		upper_pivots: Vec::new()
	});
	
	let mut partition_idx: usize = 0;
	let mut partition = partitions[partition_idx].clone();
	
	//loop until all partitions are less than or equal to 3 in size
	loop {
		
		//lower and upper half
		for half in 1 .. 3 {
			if ((half == 1) && (partition.lower_size > 3)) || ((half == 2) && (partition.upper_size > 3)){
				
				let mut lower_idx;
				let mut upper_idx;
				let mut pivot;
				let mut pivot_data;
				let pivots; // alias to the partition's pivot
				
				if half == 1 {
					lower_idx = partition.idx - partition.lower_size;	//partition index - first half size
					upper_idx = partition.idx - 1; //partition index - 1
					pivots = &mut partition.lower_pivots;
				} else {
					lower_idx = partition.idx; // partion index
					upper_idx = partition.idx + partition.upper_size - 1 ; // partition index - 1 + second half size
					pivots = &mut partition.upper_pivots;
				}
				
				pivot_data = Pivot {
					element_id: values[lower_idx].idx,
					num_previous: 0,
					num_lower: 0,
					num_higher: 0
				};
				pivot = values[lower_idx].clone();
				lower_idx += 1;
				
				// compare to first element
				if values[lower_idx].val >= pivot.val {
					let temp = values[upper_idx].clone();
					values[upper_idx] = values[lower_idx].clone();
					values[lower_idx] = temp;
					upper_idx -= 1;
					pivot_data.num_higher = 1;
					pivot_data.num_lower = 0;
				} else {
					values[lower_idx - 1] = values[lower_idx].clone();
					lower_idx += 1;
					pivot_data.num_higher = 0;
					pivot_data.num_lower = 1;
				}
				
				// split around a pivot. if the pivot is determined to be far from the center, switch pivot
				// !!! different check times than 2,4,7,10,13... ???
				// !!! pivot itself goes in upper or lower half or neither? does it matter?
					// Probably better to leave in lower partition so extra comparisons/swaps don't occur for an element that can go either way
				loop {
					
					if lower_idx == upper_idx {
						// compare the last value before break
						if values[upper_idx].val >= pivot.val {
							upper_idx -= 1;
							pivot_data.num_higher += 1;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
							pivot_data.num_lower += 1;
						}
						pivots.push(pivot_data);
						
						// pivot is always considered in lower partition
							// !!! put in opposite half of latest element for evening out short partitions?
						values[upper_idx] = pivot.clone(); 
						break;
					}
					
					// check if the first 2 elements fell on either side of the pivot. if not, change pivot and continue
					// !!! for values equal to the pivot, should they go all the same way or on the side with fewer elements?
					if pivot_data.num_higher == 1 {
						if values[lower_idx].val >= pivot.val {
							values[lower_idx - 1] = pivot.clone();
							pivot = values[lower_idx].clone();
							lower_idx += 1;
							pivots.push(pivot_data.clone());
							pivot_data.element_id = pivot.idx;
							pivot_data.num_previous += 1;
							pivot_data.num_lower = 1;
							pivot_data.num_higher = 0; 
							continue;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
							pivot_data.num_lower = 1;
							pivot_data.num_higher = 1;
						}
					} else {
						if values[upper_idx].val >= pivot.val {
							upper_idx -= 1;
							pivot_data.num_higher = 1;
							pivot_data.num_lower = 1;
						} else {
							let temp = values[upper_idx].clone();
							values[upper_idx] = pivot.clone();
							pivot = temp;
							upper_idx -= 1;
							pivots.push(pivot_data.clone());
							pivot_data.element_id = pivot.idx;
							pivot_data.num_previous += 1;
							pivot_data.num_higher = 1;
							pivot_data.num_lower = 0;
							continue;
						}
					}
					
					// All subsequent checks for inequality will happen periodically. 
					// Loop until there is a pivot change or the partition is sorted
					loop {
						if lower_idx == upper_idx {
							break;
						}
						
						if (pivot_data.num_lower + pivot_data.num_higher) % 3 == 0 {
							if pivot_data.num_lower == (pivot_data.num_lower + pivot_data.num_higher) / 3 {
								if values[lower_idx].val >= pivot.val {
									values[lower_idx - 1] = pivot.clone();
									pivot = values[lower_idx].clone();
									lower_idx += 1;
									pivots.push(pivot_data.clone());
									pivot_data.element_id = pivot.idx;
									pivot_data.num_previous += 1;
									pivot_data.num_lower = 1;
									pivot_data.num_higher = 0; 
									break;
								} else {
									values[lower_idx - 1] = values[lower_idx].clone();
									lower_idx += 1;
									pivot_data.num_lower += 1;
									continue;
								}
							} else if pivot_data.num_higher == (pivot_data.num_lower + pivot_data.num_higher) / 3 {
								if values[upper_idx].val >= pivot.val {
									upper_idx -= 1;
									pivot_data.num_higher += 1;
									continue;
								} else {
									let temp = values[upper_idx].clone();
									values[upper_idx] = pivot.clone();
									pivot = temp;
									upper_idx -= 1;
									pivots.push(pivot_data.clone());
									pivot_data.element_id = pivot.idx;
									pivot_data.num_previous += 1;
									pivot_data.num_higher = 1;
									pivot_data.num_lower = 0;
									break;
								}
							}
						} 
						if values[lower_idx].val >= pivot.val {
							let temp = values[upper_idx].clone();
							values[upper_idx] = values[lower_idx].clone();
							values[lower_idx] = temp;
							upper_idx -= 1;
							pivot_data.num_higher += 1;
						} else {
							values[lower_idx - 1] = values[lower_idx].clone();
							lower_idx += 1;
							pivot_data.num_lower += 1;
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
					elements[values[i].idx].unsorted_locs.push(i);
				}
				
				partitions.push( PartitionSeparator {
					idx: lower_idx,
					lower_size,
					upper_size,
					lower_pivots: Vec::new(),
					upper_pivots: Vec::new()
				});
			
			}
		}
		
		partition_idx += 1;
		if partitions.len() == partition_idx {
			break;
		} else {
			partition = partitions[partition_idx].clone();
			continue;
		}
		
	}
	
	return partitions;
}

//TESTS
#[cfg(test)]
mod testing;

