
mod testing{
	use crate::*;
	use std::io;
	
	// check first val less,
	// higher and lower after 1
	// higher and lower after 3+
	// higher and lower for (3+)%3 == 0 in both too many above and below

	// check a single partitioning of shifting_partition_sort()
	#[test]
	fn check_single() {
		let initial_values = [
			IndexedVal {idx: 0, val: 373},
			IndexedVal {idx: 1, val: 77},
			IndexedVal {idx: 2, val: 52},
			IndexedVal {idx: 3, val: 144},
			IndexedVal {idx: 4, val: 19},
			IndexedVal {idx: 5, val: 22},
			IndexedVal {idx: 6, val: 910},
			IndexedVal {idx: 7, val: 640},
			IndexedVal {idx: 8, val: 170},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 19, val: 227},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 29, val: 999},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 35, val: 766},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 55, val: 933},
			IndexedVal {idx: 56, val: 516},
			IndexedVal {idx: 57, val: 490},
			IndexedVal {idx: 58, val: 813},
			IndexedVal {idx: 59, val: 36},
			IndexedVal {idx: 60, val: 536},
			IndexedVal {idx: 61, val: 840},
			IndexedVal {idx: 62, val: 41},
			IndexedVal {idx: 63, val: 180},
		];
		
		let mut values = initial_values.clone();
		let output = sort(0, &mut values).elements;
		
		
		// PIVOT LOC: 28
		let values = [
			IndexedVal {idx: 1, val: 77},
			IndexedVal {idx: 2, val: 52},
			IndexedVal {idx: 3, val: 144},
			IndexedVal {idx: 4, val: 19},
			IndexedVal {idx: 5, val: 22},
			IndexedVal {idx: 59, val: 36},
			IndexedVal {idx: 62, val: 41},
			IndexedVal {idx: 8, val: 170},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 19, val: 227},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 57, val: 490},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 29, val: 999},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 35, val: 766},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 55, val: 933},
			IndexedVal {idx: 56, val: 516},
			IndexedVal {idx: 58, val: 813},
			IndexedVal {idx: 7, val: 640},
			IndexedVal {idx: 60, val: 536},
			IndexedVal {idx: 6, val: 910},
			IndexedVal {idx: 61, val: 840},
			IndexedVal {idx: 63, val: 180},
			IndexedVal {idx: 0, val: 373},
		];
		
		assert!(check_unsorted(&values, &output, 0));
		
	}
	
	
	// check a second, recursive partitioning of shifting_partition_sort()
	#[test]
	fn check_recursive() {
		// same initial values as check_results
		let initial_values = [
			IndexedVal {idx: 0, val: 373},
			IndexedVal {idx: 1, val: 707},
			IndexedVal {idx: 2, val: 916},
			IndexedVal {idx: 3, val: 331},
			IndexedVal {idx: 4, val: 241},
			IndexedVal {idx: 5, val: 814},
			IndexedVal {idx: 6, val: 564},
			IndexedVal {idx: 7, val: 17},
			IndexedVal {idx: 8, val: 741},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 19, val: 227},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 29, val: 999},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 35, val: 766},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 55, val: 913},
			IndexedVal {idx: 56, val: 903},
			IndexedVal {idx: 57, val: 237},
			IndexedVal {idx: 58, val: 148},
			IndexedVal {idx: 59, val: 655},
			IndexedVal {idx: 60, val: 508},
			IndexedVal {idx: 61, val: 434},
			IndexedVal {idx: 62, val: 146},
			IndexedVal {idx: 63, val: 901},
		];
		
		
		let mut vals = initial_values.clone();
		let output = sort(0, &mut vals).elements;
		
		
		// PIVOT LOC: 26
		let vals = [ 
			IndexedVal {idx: 0, val: 373},
			IndexedVal {idx: 62, val: 146},
			IndexedVal {idx: 3, val: 331},
			IndexedVal {idx: 4, val: 241},
			IndexedVal {idx: 58, val: 148},
			IndexedVal {idx: 57, val: 237},
			IndexedVal {idx: 7, val: 17},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 19, val: 227},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 61, val: 434},//________________________________
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 29, val: 999},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 35, val: 766},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 55, val: 913},
			IndexedVal {idx: 56, val: 903},
			IndexedVal {idx: 8, val: 741},
			IndexedVal {idx: 6, val: 564},
			IndexedVal {idx: 59, val: 655},
			IndexedVal {idx: 5, val: 814},
			IndexedVal {idx: 60, val: 508},
			IndexedVal {idx: 2, val: 916},
			IndexedVal {idx: 63, val: 901},
			IndexedVal {idx: 1, val: 707},
		];
		
		assert!(check_unsorted(&vals, &output, 0));
		
		let vals = [
			IndexedVal {idx: 62, val: 146},
			IndexedVal {idx: 3, val: 331},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 58, val: 148},
			IndexedVal {idx: 57, val: 237},
			IndexedVal {idx: 7, val: 17},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 19, val: 227}, // PIVOT 2L
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 4, val: 241},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 0, val: 373},
			IndexedVal {idx: 61, val: 434}, // PIVOT 1
			IndexedVal {idx: 1, val: 707},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 60, val: 508},
			IndexedVal {idx: 59, val: 655},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 6, val: 564},
			IndexedVal {idx: 8, val: 741},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 35, val: 766}, // PIVOT 2U
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 55, val: 913},
			IndexedVal {idx: 56, val: 903},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 5, val: 814},
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 2, val: 916},
			IndexedVal {idx: 63, val: 901},
			IndexedVal {idx: 29, val: 999},
		];
		
		assert!(check_unsorted(&vals, &output, 1));
	}
	
	//check the resulting output of partition sort
	#[test]
	fn check_results() -> Result<(),String> {
		// same initial values as check recursive
		let mut values = [
			IndexedVal {idx: 0, val: 373},
			IndexedVal {idx: 1, val: 707},
			IndexedVal {idx: 2, val: 916},
			IndexedVal {idx: 3, val: 331},
			IndexedVal {idx: 4, val: 241},
			IndexedVal {idx: 5, val: 814},
			IndexedVal {idx: 6, val: 564},
			IndexedVal {idx: 7, val: 17},
			IndexedVal {idx: 8, val: 741},
			IndexedVal {idx: 9, val: 722},
			IndexedVal {idx: 10, val: 294},
			IndexedVal {idx: 11, val: 64},
			IndexedVal {idx: 12, val: 994},
			IndexedVal {idx: 13, val: 866},
			IndexedVal {idx: 14, val: 76},
			IndexedVal {idx: 15, val: 204},
			IndexedVal {idx: 16, val: 891},
			IndexedVal {idx: 17, val: 178},
			IndexedVal {idx: 18, val: 954},
			IndexedVal {idx: 19, val: 227},
			IndexedVal {idx: 20, val: 301},
			IndexedVal {idx: 21, val: 916},
			IndexedVal {idx: 22, val: 741},
			IndexedVal {idx: 23, val: 808},
			IndexedVal {idx: 24, val: 795},
			IndexedVal {idx: 25, val: 847},
			IndexedVal {idx: 26, val: 197},
			IndexedVal {idx: 27, val: 494},
			IndexedVal {idx: 28, val: 884},
			IndexedVal {idx: 29, val: 999},
			IndexedVal {idx: 30, val: 842},
			IndexedVal {idx: 31, val: 265},
			IndexedVal {idx: 32, val: 534},
			IndexedVal {idx: 33, val: 300},
			IndexedVal {idx: 34, val: 427},
			IndexedVal {idx: 35, val: 766},
			IndexedVal {idx: 36, val: 99},
			IndexedVal {idx: 37, val: 217},
			IndexedVal {idx: 38, val: 992},
			IndexedVal {idx: 39, val: 864},
			IndexedVal {idx: 40, val: 583},
			IndexedVal {idx: 41, val: 562},
			IndexedVal {idx: 42, val: 423},
			IndexedVal {idx: 43, val: 119},
			IndexedVal {idx: 44, val: 101},
			IndexedVal {idx: 45, val: 476},
			IndexedVal {idx: 46, val: 545},
			IndexedVal {idx: 47, val: 723},
			IndexedVal {idx: 48, val: 217},
			IndexedVal {idx: 49, val: 618},
			IndexedVal {idx: 50, val: 745},
			IndexedVal {idx: 51, val: 536},
			IndexedVal {idx: 52, val: 383},
			IndexedVal {idx: 53, val: 885},
			IndexedVal {idx: 54, val: 68},
			IndexedVal {idx: 55, val: 913},
			IndexedVal {idx: 56, val: 903},
			IndexedVal {idx: 57, val: 237},
			IndexedVal {idx: 58, val: 148},
			IndexedVal {idx: 59, val: 655},
			IndexedVal {idx: 60, val: 508},
			IndexedVal {idx: 61, val: 434},
			IndexedVal {idx: 62, val: 146},
			IndexedVal {idx: 63, val: 901},
		];
		
		let output = sort(0, &mut values).elements; // >:< 
		
		let elem = ShakeElement {
			val: 373, sorted_loc: Some(22), element_id: 0, unsorted_locs: vec!(0, 25, 22, 21), 
		};
		check_element(&elem, &output[0])?;
		
		let elem = ShakeElement {
			val: 707, sorted_loc: Some(38), element_id: 1, unsorted_locs: vec!(63, 27, 27, 37), 
		};
		check_element(&elem, &output[1])?;
		
		let elem = ShakeElement {
			val: 916, sorted_loc: Some(58), element_id: 2, unsorted_locs: vec!(61, 61, 59, 59), 
		};
		check_element(&elem, &output[2])?;
		
		let elem = ShakeElement {
			val: 331, sorted_loc: Some(21), element_id: 3, unsorted_locs: vec!(2, 1, 16, 16, 15), 
		};
		check_element(&elem, &output[3])?;
		
		let elem = ShakeElement {
			val: 241, sorted_loc: Some(16), element_id: 4, unsorted_locs: vec!(3, 18, 17, 17, 17), 
		};
		check_element(&elem, &output[4])?;
		
		let elem = ShakeElement {
			val: 814, sorted_loc: Some(47), element_id: 5, unsorted_locs: vec!(59, 59, 54, 50, 49), 
		};
		check_element(&elem, &output[5])?;
		
		let elem = ShakeElement {
			val: 564, sorted_loc: Some(34), element_id: 6, unsorted_locs: vec!(57, 36, 35, 34, 33), 
		};
		check_element(&elem, &output[6])?;
		
		let elem = ShakeElement {
			val: 17, sorted_loc: Some(0), element_id: 7, unsorted_locs: vec!(6, 5, 4, 3, 2, 1), 
		};
		check_element(&elem, &output[7])?;
		
		let elem = ShakeElement {
			val: 741, sorted_loc: Some(41), element_id: 8, unsorted_locs: vec!(56, 37, 38, 39, 42), 
		};
		check_element(&elem, &output[8])?;
		
		let elem = ShakeElement {
			val: 722, sorted_loc: Some(39), element_id: 9, unsorted_locs: vec!(53, 40, 39, 38, 38), 
		};
		check_element(&elem, &output[9])?;
		
		let elem = ShakeElement {
			val: 294, sorted_loc: Some(18), element_id: 10, unsorted_locs: vec!(9, 20, 19, 18, 20), 
		};
		check_element(&elem, &output[10])?;
		
		let elem = ShakeElement {
			val: 64, sorted_loc: Some(1), element_id: 11, unsorted_locs: vec!(10, 9, 8, 1, 0, 0), 
		};
		check_element(&elem, &output[11])?;
		
		let elem = ShakeElement {
			val: 994, sorted_loc: Some(62), element_id: 12, unsorted_locs: vec!(51, 50, 61, 60, 63), 
		};
		check_element(&elem, &output[12])?;
		
		let elem = ShakeElement {
			val: 866, sorted_loc: Some(51), element_id: 13, unsorted_locs: vec!(47, 48, 48, 51, 51), 
		};
		check_element(&elem, &output[13])?;
		
		let elem = ShakeElement {
			val: 76, sorted_loc: Some(3), element_id: 14, unsorted_locs: vec!(13, 12, 3, 2, 1, 4), 
		};
		check_element(&elem, &output[14])?;
		
		let elem = ShakeElement {
			val: 204, sorted_loc: Some(11), element_id: 15, unsorted_locs: vec!(14, 13, 13, 13, 13), 
		};
		check_element(&elem, &output[15])?;
		
		let elem = ShakeElement {
			val: 891, sorted_loc: Some(54), element_id: 16, unsorted_locs: vec!(43, 49, 63, 58), 
		};
		check_element(&elem, &output[16])?;
		
		let elem = ShakeElement {
			val: 178, sorted_loc: Some(9), element_id: 17, unsorted_locs: vec!(16, 15, 1, 9, 7), 
		};
		check_element(&elem, &output[17])?;
		
		let elem = ShakeElement {
			val: 954, sorted_loc: Some(60), element_id: 18, unsorted_locs: vec!(42, 52, 60, 61, 60), 
		};
		check_element(&elem, &output[18])?;
		
		let elem = ShakeElement {
			val: 227, sorted_loc: Some(14), element_id: 19, unsorted_locs: vec!(18, 16, 15, 14, 16), 
		};
		check_element(&elem, &output[19])?;
		
		let elem = ShakeElement {
			val: 301, sorted_loc: Some(20), element_id: 20, unsorted_locs: vec!(19, 19, 18, 22), 
		};
		check_element(&elem, &output[20])?;
		
		let elem = ShakeElement {
			val: 916, sorted_loc: Some(59), element_id: 21, unsorted_locs: vec!(41, 55, 58, 57), 
		};
		check_element(&elem, &output[21])?;
		
		let elem = ShakeElement {
			val: 741, sorted_loc: Some(42), element_id: 22, unsorted_locs: vec!(36, 35, 40, 46, 44), 
		};
		check_element(&elem, &output[22])?;
		
		let elem = ShakeElement {
			val: 808, sorted_loc: Some(46), element_id: 23, unsorted_locs: vec!(35, 58, 55, 49, 48), 
		};
		check_element(&elem, &output[23])?;
		
		let elem = ShakeElement {
			val: 795, sorted_loc: Some(45), element_id: 24, unsorted_locs: vec!(33, 32, 43, 43, 43), 
		};
		check_element(&elem, &output[24])?;
		
		let elem = ShakeElement {
			val: 847, sorted_loc: Some(49), element_id: 25, unsorted_locs: vec!(32, 31, 44, 44, 47), 
		};
		check_element(&elem, &output[25])?;
		
		let elem = ShakeElement {
			val: 197, sorted_loc: Some(10), element_id: 26, unsorted_locs: vec!(25, 2, 14, 12), 
		};
		check_element(&elem, &output[26])?;
		
		let elem = ShakeElement {
			val: 494, sorted_loc: Some(28), element_id: 27, unsorted_locs: vec!(30, 29, 28, 27, 27, 27), 
		};
		check_element(&elem, &output[27])?;
		
		let elem = ShakeElement {
			val: 884, sorted_loc: Some(52), element_id: 28, unsorted_locs: vec!(27, 60, 51, 54, 53, 52), 
		};
		check_element(&elem, &output[28])?;
		
		let elem = ShakeElement {
			val: 999, sorted_loc: Some(63), element_id: 29, unsorted_locs: vec!(28, 63, 62, 62, 62), 
		};
		check_element(&elem, &output[29])?;
		
		let elem = ShakeElement {
			val: 842, sorted_loc: Some(48), element_id: 30, unsorted_locs: vec!(29, 28, 47, 47, 46), 
		};
		check_element(&elem, &output[30])?;
		
		let elem = ShakeElement {
			val: 265, sorted_loc: Some(17), element_id: 31, unsorted_locs: vec!(24, 24, 20, 19, 18), 
		};
		check_element(&elem, &output[31])?;
		
		let elem = ShakeElement {
			val: 534, sorted_loc: Some(30), element_id: 32, unsorted_locs: vec!(31, 30, 29, 28, 34), 
		};
		check_element(&elem, &output[32])?;
		
		let elem = ShakeElement {
			val: 300, sorted_loc: Some(19), element_id: 33, unsorted_locs: vec!(23, 23, 21, 20, 19), 
		};
		check_element(&elem, &output[33])?;
		
		let elem = ShakeElement {
			val: 427, sorted_loc: Some(25), element_id: 34, unsorted_locs: vec!(22, 22, 23, 25), 
		};
		check_element(&elem, &output[34])?;
		
		let elem = ShakeElement {
			val: 766, sorted_loc: Some(44), element_id: 35, unsorted_locs: vec!(34, 47, 46, 45, 45), 
		};
		check_element(&elem, &output[35])?;
		
		let elem = ShakeElement {
			val: 99, sorted_loc: Some(4), element_id: 36, unsorted_locs: vec!(21, 7, 6, 5, 4, 3), 
		};
		check_element(&elem, &output[36])?;
		
		let elem = ShakeElement {
			val: 217, sorted_loc: Some(12), element_id: 37, unsorted_locs: vec!(20, 8, 10, 11), 
		};
		check_element(&elem, &output[37])?;
		
		let elem = ShakeElement {
			val: 992, sorted_loc: Some(61), element_id: 38, unsorted_locs: vec!(37, 57, 57, 63, 61), 
		};
		check_element(&elem, &output[38])?;
		
		let elem = ShakeElement {
			val: 864, sorted_loc: Some(50), element_id: 39, unsorted_locs: vec!(38, 56, 56, 48, 50), 
		};
		check_element(&elem, &output[39])?;
		
		let elem = ShakeElement {
			val: 583, sorted_loc: Some(35), element_id: 40, unsorted_locs: vec!(39, 38, 36, 35), 
		};
		check_element(&elem, &output[40])?;
		
		let elem = ShakeElement {
			val: 562, sorted_loc: Some(33), element_id: 41, unsorted_locs: vec!(40, 39, 34, 33, 32), 
		};
		check_element(&elem, &output[41])?;
		
		let elem = ShakeElement {
			val: 423, sorted_loc: Some(24), element_id: 42, unsorted_locs: vec!(17, 17, 25, 24), 
		};
		check_element(&elem, &output[42])?;
		
		let elem = ShakeElement {
			val: 119, sorted_loc: Some(6), element_id: 43, unsorted_locs: vec!(15, 14, 2, 7, 9), 
		};
		check_element(&elem, &output[43])?;
		
		let elem = ShakeElement {
			val: 101, sorted_loc: Some(5), element_id: 44, unsorted_locs: vec!(12, 11, 7, 6, 6), 
		};
		check_element(&elem, &output[44])?;
		
		let elem = ShakeElement {
			val: 476, sorted_loc: Some(27), element_id: 45, unsorted_locs: vec!(44, 43, 31, 30, 29, 28), 
		};
		check_element(&elem, &output[45])?;
		
		let elem = ShakeElement {
			val: 545, sorted_loc: Some(32), element_id: 46, unsorted_locs: vec!(45, 44, 30, 29, 28, 31), 
		};
		check_element(&elem, &output[46])?;
		
		let elem = ShakeElement {
			val: 723, sorted_loc: Some(40), element_id: 47, unsorted_locs: vec!(46, 45, 45, 40, 40), 
		};
		check_element(&elem, &output[47])?;
		
		let elem = ShakeElement {
			val: 217, sorted_loc: Some(13), element_id: 48, unsorted_locs: vec!(11, 10, 9, 8, 8), 
		};
		check_element(&elem, &output[48])?;
		
		let elem = ShakeElement {
			val: 618, sorted_loc: Some(36), element_id: 49, unsorted_locs: vec!(48, 46, 37, 36), 
		};
		check_element(&elem, &output[49])?;
		
		let elem = ShakeElement {
			val: 745, sorted_loc: Some(43), element_id: 50, unsorted_locs: vec!(49, 42, 41, 42, 41), 
		};
		check_element(&elem, &output[50])?;
		
		let elem = ShakeElement {
			val: 536, sorted_loc: Some(31), element_id: 51, unsorted_locs: vec!(50, 41, 33, 32, 31, 30), 
		};
		check_element(&elem, &output[51])?;
		
		let elem = ShakeElement {
			val: 383, sorted_loc: Some(23), element_id: 52, unsorted_locs: vec!(8, 21, 24, 23), 
		};
		check_element(&elem, &output[52])?;
		
		let elem = ShakeElement {
			val: 885, sorted_loc: Some(53), element_id: 53, unsorted_locs: vec!(52, 51, 50, 55, 54, 53), 
		};
		check_element(&elem, &output[53])?;
		
		let elem = ShakeElement {
			val: 68, sorted_loc: Some(2), element_id: 54, unsorted_locs: vec!(7, 6, 5, 4, 3, 2), 
		};
		check_element(&elem, &output[54])?;
		
		let elem = ShakeElement {
			val: 913, sorted_loc: Some(57), element_id: 55, unsorted_locs: vec!(54, 53, 52, 53, 56), 
		};
		check_element(&elem, &output[55])?;
		
		let elem = ShakeElement {
			val: 903, sorted_loc: Some(56), element_id: 56, unsorted_locs: vec!(55, 54, 53, 52, 55, 55), 
		};
		check_element(&elem, &output[56])?;
		
		let elem = ShakeElement {
			val: 237, sorted_loc: Some(15), element_id: 57, unsorted_locs: vec!(5, 4, 12, 15, 14), 
		};
		check_element(&elem, &output[57])?;
		
		let elem = ShakeElement {
			val: 148, sorted_loc: Some(8), element_id: 58, unsorted_locs: vec!(4, 3, 11, 10), 
		};
		check_element(&elem, &output[58])?;
		
		let elem = ShakeElement {
			val: 655, sorted_loc: Some(37), element_id: 59, unsorted_locs: vec!(58, 34, 42, 41, 39), 
		};
		check_element(&elem, &output[59])?;
		
		let elem = ShakeElement {
			val: 508, sorted_loc: Some(29), element_id: 60, unsorted_locs: vec!(60, 33, 32, 31, 30, 29), 
		};
		check_element(&elem, &output[60])?;
		
		let elem = ShakeElement {
			val: 434, sorted_loc: Some(26), element_id: 61, unsorted_locs: vec!(26, 26, 26, 26), 
		};
		check_element(&elem, &output[61])?;
		
		let elem = ShakeElement {
			val: 146, sorted_loc: Some(7), element_id: 62, unsorted_locs: vec!(1, 0, 0, 0, 5), 
		};
		check_element(&elem, &output[62])?;
		
		let elem = ShakeElement {
			val: 901, sorted_loc: Some(55), element_id: 63, unsorted_locs: vec!(62, 62, 49, 56, 52, 54), 
		};
		check_element(&elem, &output[63])?;
		
		Ok(())
	}
	
	//helper functions
	
	
	// !!! check if the values are equal, not just the indexes
	fn check_element(expected: &ShakeElement, actual: &ShakeElement) -> Result<(),String> {
		
		let mut err_msg = String::from(format!("Error for expected element {}: ", expected.element_id));
		let mut is_error = false;
		
		if expected.val != actual.val {
			err_msg.push_str(&format!("Values {} and {} don't match", expected.val, actual.val));
			is_error = true;
		}
		if expected.element_id != actual.element_id {
			err_msg.push_str(&format!("Initial locations {} and {} don't match", expected.element_id, actual.element_id));
			is_error = true;
		}
		if expected.sorted_loc != actual.sorted_loc {
			let expected_loc = expected.sorted_loc.unwrap();
			let actual_loc;
			match actual.sorted_loc {
				Some(loc) => {
					actual_loc = loc;
					
					// !!! use a sure way to account for duplicates ending in interchangeable positions
					if (actual_loc > expected_loc && actual_loc - expected_loc < 2)
					|| (expected_loc > actual_loc && expected_loc - actual_loc < 2)
					{}
					else{
						err_msg.push_str(&format!("Sorted locations {:?} and {:?} don't match", expected.sorted_loc, actual.sorted_loc));
						is_error = true;
					}
				},
				None => {
					err_msg.push_str(&format!("Sorted location is None"));
					is_error = true;
				}
			}
		}
		if expected.unsorted_locs.len() != actual.unsorted_locs.len()
		{
			err_msg.push_str(&format!("Unsorted location lengths {} and {} don't match", 
				expected.unsorted_locs.len(), actual.unsorted_locs.len()));
			is_error = true;
		}
		for i in 0 .. expected.unsorted_locs.len() {
			if expected.unsorted_locs[i] != actual.unsorted_locs[i] {
				err_msg.push_str(&format!(
					"Unsorted locations {} and {} don't match", expected.unsorted_locs[i], actual.unsorted_locs[i]
				));
				is_error = true;
			}
		}
		
		if is_error{
			return Err(err_msg);
		}
		
		Ok(())
	}
	
	
	fn check_unsorted(expected: &[IndexedVal], output: &[ShakeElement], unsorted_idx: usize) -> bool {
		for i in 0 .. (&expected).len() {
			// >:<
			let idx = if output[expected[i].idx].unsorted_locs.len() <= unsorted_idx {
				output[expected[i].idx].unsorted_locs.len() - 1
			} else {
				unsorted_idx
			};
			
			if output[expected[i].idx].unsorted_locs[idx] != i {
				return false;
			}
		}
		true
	}
	
	
	// follow the swapping of shifting partition sort, taking user expected values and matching them against 
		// (presumably) correct ones
	fn follow_shifting_partition (i_vals: &mut[IndexedVal]) {
		let mut pivot = i_vals[0].clone();
		let mut lower_idx = 1;
		let mut upper_idx = i_vals.len() - 1;
		let mut num_lower = 0;
		let mut num_higher = 0;
		
		if i_vals[lower_idx].val >= pivot.val {
			let tmp = i_vals[upper_idx].clone();
			i_vals[upper_idx] = i_vals[lower_idx].clone();
			i_vals[lower_idx] = tmp.clone();
			upper_idx -= 1;
			num_higher += 1;
		} else {
			i_vals[lower_idx - 1] = i_vals[lower_idx].clone();
			lower_idx += 1;
			num_lower += 1;
		}
		
		
		while lower_idx < upper_idx {
			// print the 3 values.
			println!("P:{} IS > {} AND < {} --- LI:{}:{}, UI:{}:{}"
				,pivot.val, num_lower, num_higher
				,lower_idx, i_vals[lower_idx].val
				,upper_idx, i_vals[upper_idx].val
			);
			
			// accept a command that will place a value
			let mut input = String::new();
			let command: Command;
			#[derive(PartialEq)]
			enum Command {PL, PU, SL, SU};
			loop {
				// check for a valid command between place lower_idx, place upper_idx, swap with lower_idx, swap with upper_idx
				match io::stdin().read_line(&mut input) {
					Ok(val) => {
						input = input.trim().to_string();
						if input == "PL" {
							command = Command::PL;
						} else if input == "PU" {
							command = Command::PU;
						} else if input == "SL" {
							command = Command::SL;
						} else if input == "SU" {
							command = Command::SU;
						} else {
							println!("invalid command: {}", input);
							input = String::new();
							continue;
						}
						break;
					}
					Err(_) => {
						println!("Couldn't read line");
						continue;
					}
				}
				
			}
			
			// check if command matches the correct command
			if num_lower + num_higher == 1 || (num_lower + num_higher) % 3  == 0 {
				// too few elements lower
				if num_lower == (num_lower + num_higher) / 3 {
					if i_vals[lower_idx].val >= pivot.val {
						if command != Command::SL {
							println!("Expected SL");
							continue;
						}
					} else {
						if command != Command::PL {
							println!("Expected PL");
							continue;
						}
					}
				} 
				// too few elements higher
				else if num_higher == (num_lower + num_higher) / 3 {
					if i_vals[upper_idx].val >= pivot.val {
						if command != Command::PU {
							println!("Expected PU");
							continue;
						}
					} else {
						if command != Command::SU {
							println!("Expected SU");
							continue;
						}
					}
				}
			} else {
				if command != Command::PL {
					println!("Expected PL");
					continue;
				}
			}
			
			match command {
				Command::SL => {
					i_vals[lower_idx - 1] = pivot.clone();
					pivot = i_vals[lower_idx].clone();
					lower_idx += 1;
					num_lower = 1;
					num_higher = 0;
				} 
				Command::PL => {
					if i_vals[lower_idx].val >= pivot.val {
						let tmp = i_vals[upper_idx].clone();
						i_vals[upper_idx] = i_vals[lower_idx].clone();
						i_vals[lower_idx] = tmp.clone();
						upper_idx -= 1;
						num_higher += 1;
					} else {
						i_vals[lower_idx - 1] = i_vals[lower_idx].clone();
						lower_idx += 1;
						num_lower += 1;
					}
				}
				Command::PU => {
					upper_idx -= 1;
					num_higher += 1;
				} 
				Command::SU => {
					let tmp = i_vals[upper_idx].clone();
					i_vals[upper_idx] = pivot.clone();
					pivot = tmp.clone();
					upper_idx -= 1;
					num_lower = 0;
					num_higher = 1;
				}
			}
			
		}
		
		// compare pivot with the last value at upper_idx == lower_idx
		if i_vals[upper_idx].val >= pivot.val {
			i_vals[upper_idx - 1] = pivot.clone();
			println!("PIVOT LOC: {}", upper_idx - 1);
		} else {
			i_vals[upper_idx - 1] = i_vals[upper_idx].clone();
			i_vals[upper_idx] = pivot.clone();
			println!("PIVOT LOC: {}", upper_idx);
		}
	}
}