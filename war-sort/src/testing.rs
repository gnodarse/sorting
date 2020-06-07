mod testing {
	use crate::*;

	#[test]
	fn no_swaps() {
		//test to check that the output data is correct when there are no swaps
		let initial_values = [IndexedVal {idx: 0, val: 0},
							  IndexedVal {idx: 1, val: 1},
							  IndexedVal {idx: 2, val: 2},
							  IndexedVal {idx: 3, val: 3},
							  IndexedVal {idx: 4, val: 4},
							  IndexedVal {idx: 5, val: 5},
							  IndexedVal {idx: 6, val: 6},
							  IndexedVal {idx: 7, val: 7}
		];
		let mut values = initial_values.clone();
		let output = sort(&mut values);
		assert!(check_initial(&initial_values, &output));
		assert!(check_unsorted(&values, &output, 0));
		assert!(check_unsorted(&values, &output, 1));
		assert!(check_unsorted(&values, &output, 2));
		assert!(check_cross(&values, &output));
		assert!(check_sorted(&values, &output));
	}
	
	//everything causes a swap
	#[test]
	fn unsorted_swaps() {
		let initial_values = [IndexedVal {idx: 0, val: 5},
							  IndexedVal {idx: 1, val: 4},
							  IndexedVal {idx: 2, val: 7},
							  IndexedVal {idx: 3, val: 6},
							  IndexedVal {idx: 4, val: 1},
							  IndexedVal {idx: 5, val: 0},
							  IndexedVal {idx: 6, val: 3},
							  IndexedVal {idx: 7, val: 2}
		];
		let mut values = initial_values.clone();
		let output = sort(&mut values);
		assert!(check_initial(&initial_values, &output));
		
		values = [IndexedVal {idx: 7, val: 2},
					  IndexedVal {idx: 6, val: 3},
					  IndexedVal {idx: 5, val: 0},
					  IndexedVal {idx: 4, val: 1},
					  IndexedVal {idx: 3, val: 6},
					  IndexedVal {idx: 2, val: 7},
					  IndexedVal {idx: 1, val: 4},
					  IndexedVal {idx: 0, val: 5}
		];
		assert!(check_unsorted(&values, &output, 0));
		
		values = [IndexedVal {idx: 4, val: 1},
					  IndexedVal {idx: 5, val: 0},
					  IndexedVal {idx: 6, val: 3},
					  IndexedVal {idx: 7, val: 2},
					  IndexedVal {idx: 0, val: 5},
					  IndexedVal {idx: 1, val: 4},
					  IndexedVal {idx: 2, val: 7},
					  IndexedVal {idx: 3, val: 6}
		];
		assert!(check_unsorted(&values, &output, 1));
		
		values = [IndexedVal {idx: 5, val: 0},
					  IndexedVal {idx: 4, val: 1},
					  IndexedVal {idx: 7, val: 2},
					  IndexedVal {idx: 6, val: 3},
					  IndexedVal {idx: 1, val: 4},
					  IndexedVal {idx: 0, val: 5},
					  IndexedVal {idx: 3, val: 6},
					  IndexedVal {idx: 2, val: 7}
		];
		assert!(check_unsorted(&values, &output, 2));
		assert!(check_cross(&values, &output));
		assert!(check_sorted(&values, &output));
	}
	
	#[test]
	fn cross_and_sorted_swaps() {
		let initial_values = [IndexedVal {idx: 0, val: 6},
							  IndexedVal {idx: 1, val: 4},
							  IndexedVal {idx: 2, val: 2},
							  IndexedVal {idx: 3, val: 0},
							  IndexedVal {idx: 4, val: 1},
							  IndexedVal {idx: 5, val: 3},
							  IndexedVal {idx: 6, val: 5},
							  IndexedVal {idx: 7, val: 7}
		];
		let mut values = initial_values.clone();
		let output = sort(&mut values);
		
		assert!(check_initial(&initial_values, &output));
		assert!(check_unsorted(&initial_values, &output, 0));
		
		values = [IndexedVal {idx: 3, val: 0},
				  IndexedVal {idx: 2, val: 2},
				  IndexedVal {idx: 1, val: 4},
				  IndexedVal {idx: 0, val: 6},
				  IndexedVal {idx: 4, val: 1},
				  IndexedVal {idx: 5, val: 3},
				  IndexedVal {idx: 6, val: 5},
				  IndexedVal {idx: 7, val: 7}
		];
		assert!(check_unsorted(&values, &output, 1));
		assert!(check_unsorted(&values, &output, 2));
		
		values = [IndexedVal {idx: 3, val: 0},
				  IndexedVal {idx: 2, val: 2},
				  IndexedVal {idx: 1, val: 4},
				  IndexedVal {idx: 4, val: 1},
				  IndexedVal {idx: 0, val: 6},
				  IndexedVal {idx: 5, val: 3},
				  IndexedVal {idx: 6, val: 5},
				  IndexedVal {idx: 7, val: 7}
		];
		assert!(check_cross(&values, &output));
		
		values = [IndexedVal {idx: 3, val: 0},
				  IndexedVal {idx: 4, val: 1},
				  IndexedVal {idx: 2, val: 2},
				  IndexedVal {idx: 5, val: 3},
				  IndexedVal {idx: 1, val: 4},
				  IndexedVal {idx: 6, val: 5},
				  IndexedVal {idx: 0, val: 6},
				  IndexedVal {idx: 7, val: 7}
		];
		assert!(check_sorted(&values, &output));
	}
	
	
	//helper functions
	
	fn check_initial(expected: &[IndexedVal], output: &[OutputData]) -> bool {
		for i in 0 .. (&expected).len() {
			if output[expected[i].idx].initial_loc != i {
				return false;
			}
		}
		true
	}
	
	fn check_sorted(expected: &[IndexedVal], output: &[OutputData]) -> bool {
		for i in 0 .. (&expected).len() {
			if let Some(idx) = output[expected[i].idx].sorted_loc{
				if idx != i { return false; }
			} else {
				return false;
			}
		}
		true
	}
	
	fn check_unsorted(expected: &[IndexedVal], output: &[OutputData], unsorted_idx: usize) -> bool {
		for i in 0 .. (&expected).len() {
			if output[expected[i].idx].unsorted_locs[unsorted_idx] != i {
				return false;
			}
		}
		true
	}
	
	fn check_cross(expected: &[IndexedVal], output: &[OutputData]) -> bool {
		for i in 0 .. (&expected).len() {
			if let Some(idx) = output[expected[i].idx].cross_loc{
				if idx != i { return false; }
			} else {
				return false;
			}
		}
		true
	}
}
	