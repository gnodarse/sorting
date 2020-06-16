use rand::Rng;
use std::io;
use std::fs::File;
use std::io::prelude::*;

use war_sort;
use shake_sort;
use types::IndexedVal;

fn main() {
	const ARRAY_LENGTH: usize = 64;
    let mut values = generate_values(ARRAY_LENGTH);
	let output = war_sort::sort(&mut values);
	
	let mut values = generate_values(ARRAY_LENGTH);
	let profile = shake_sort::sort(0, &mut values);
    let elements = profile.elements;
    let separators = profile.separators;
	
	let connection = sqlite::open("shake.db").unwrap();
	initialize_shake_db(&connection);
	
	let mut insert_shake_profile = connection.prepare("insert into ShakeProfile values (?);").unwrap();
	let mut insert_shake_element = connection
		.prepare("insert into ShakeElement (Value, InitialLocation, SortedLocation, ShakeProfileID) values (?, ?, ?, ?);")
		.unwrap();
	let mut insert_unsorted_location = connection
		.prepare("insert into UnsortedLocations (TimesShaken, Location, ShakeElementID) values (?, ?, ?);")
		.unwrap();
	let mut insert_partition_separator = connection
		.prepare("insert into PartitionSeparator (Location, LowerSize, UpperSize, ShakeProfileID) values (?, ?, ?, ?);")
		.unwrap();
	let mut insert_pivot = connection
		.prepare("insert into Pivots (NumberPrevious, NumberLower, NumberHigher, PartitionSeparatorID, ElementUnsortedLocation)
			values (?, ?, ?, ?, ?);")
		.unwrap();
		
	let mut select_last_insert_rowid = connection.prepare("SELECT last_insert_rowid()").unwrap();
	
    insert_shake_profile.bind(1, profile.id as i64).unwrap();
	insert_shake_profile.next();
	
    for elem in elements {
		insert_shake_element.reset();
		insert_shake_element.bind(1, elem.val as i64).unwrap();
		insert_shake_element.bind(2, elem.element_id as i64).unwrap();
		insert_shake_element.bind(3, elem.sorted_loc.unwrap() as i64).unwrap();
		insert_shake_element.bind(4, profile.id as i64).unwrap();
		
		insert_shake_element.next();
		
		// unsorted locations
		select_last_insert_rowid.reset();
		select_last_insert_rowid.next();
		let element_id = select_last_insert_rowid.read::<i64>(0).unwrap();
		
		let mut idx = 0;
		loop {
			if idx >= elem.unsorted_locs.len() {
				break;
			}
			
			let loc = elem.unsorted_locs[idx];
			idx += 1;
			
			insert_unsorted_location.reset();
			insert_unsorted_location.bind(1, idx as i64).unwrap();
			insert_unsorted_location.bind(2, loc as i64).unwrap();
			insert_unsorted_location.bind(3, element_id as i64).unwrap();
			
			insert_unsorted_location.next();
		}
    }
	
    for separator in separators {
	
		insert_partition_separator.reset();
		insert_partition_separator.bind(1, separator.idx as i64).unwrap();
		insert_partition_separator.bind(2, separator.lower_size as i64).unwrap();
		insert_partition_separator.bind(3, separator.upper_size as i64).unwrap();
		insert_partition_separator.bind(4, profile.id as i64).unwrap();
		
		insert_partition_separator.next();
        
        // pivots
		select_last_insert_rowid.reset();
		select_last_insert_rowid.next();
		let separator_id = select_last_insert_rowid.read::<i64>(0).unwrap();
		
        let mut idx = 0;
        loop {
			if idx >= separator.pivots.len() {
				break;
			}
			
			let pivot = &separator.pivots[idx];
			idx += 1;
		
			insert_pivot.reset();
			insert_pivot.bind(1, pivot.num_previous as i64).unwrap();
			insert_pivot.bind(2, pivot.num_lower as i64).unwrap();
			insert_pivot.bind(3, pivot.num_higher as i64).unwrap();
			insert_pivot.bind(4, separator_id as i64).unwrap();
			insert_pivot.bind(5, pivot.element_id as i64).unwrap();
			
			insert_pivot.next();
        }
        
    }
}

fn generate_values(num: usize) -> Vec<IndexedVal> {
	let mut values: Vec<IndexedVal> = Vec::with_capacity(num);
	for i in 0..num {
        values.push( IndexedVal{
			idx: i,
			val: rand::thread_rng().gen_range(1, 1000),
		});
    }
	values
}

fn initialize_shake_db(connection: &sqlite::Connection) {
	connection.execute(
		"create table if not exists ShakeProfile(ID integer not null primary key);"
	).unwrap();
	
	connection.execute(
		"create table if not exists ShakeElement(ID integer not null primary key, Value integer, InitialLocation integer, 
		SortedLocation integer, ShakeProfileID integer);"
	).unwrap();
	
	connection.execute(
		"create table if not exists PartitionSeparator(ID integer not null primary key, Location integer, LowerSize integer, 
        UpperSize integer, ShakeProfileID integer);"
	).unwrap();
	
	connection.execute(
		"create table if not exists UnsortedLocations(ID integer not null primary key, TimesShaken integer, Location integer,  
		ShakeElementID integer);"
	).unwrap(); // >:< shake iteration
	
	connection.execute(
		"create table if not exists Pivots(ID integer not null primary key, NumberPrevious integer, NumberLower integer, NumberHigher integer, PartitionSeparatorID integer, ElementUnsortedLocation integer);"
	).unwrap();
}

/*
fn initialize_data(vals: &[IndexedVal]) -> Vec<ShakeElement> {
	let num = vals.len();
	let mut data: Vec<ShakeElement> = Vec::with_capacity(num);
	for idx in 0..num {
		// >:<
        data.push( ShakeElement {
			val: vals[idx].val, 
			sorted_loc: None,
			initial_loc: idx,
			unsorted_locs: Vec::new(), //After 1 swap is at index 0, after 2 at 1...
			profile: None, // index of the associated ShakeProfile !!! Use lifetimes and a ref
		});
    }
	
	data
} */
