
// bracket sort
use types::IndexedVal;

// bracket sort in place where the same elements may be compared with each other multiple times
pub fn redundant_sort<T: PartialOrd>(values: &mut [T]){
    let len = values.len();
    let mut num_iterations = 1;
    while (1 << num_iterations <= len){
       num_iterations += 1;
    }
    num_iterations -= 1;
    
    // >:< not accounting for array lengths that are not multiples of 2
    for iteration in 0 .. num_iterations {
        let min_split_length = len / (1 << iteration); // always larger than 1
        let mut loser_idx = 0; // idx of the next iteration's split for the current split's loser elements
        let mut winner_idx; // idx of the next iteration's split for the current split's winner elements
        
        for split in 0 ..= iteration { // split# == number of wins each element in the split has
        
            let mut split_length = 1; // split_length = (iteration# choose split#) * smallest split length
            
            // calculate split length
            let difference = iteration - split;
            if difference > split {
                let mut multiplier = difference + 1;
                while multiplier <= iteration {
                    split_length *= multiplier;
                    multiplier += 1;
                }
                let mut divider = split;
                while divider > 1 {
                    split_length /= divider;
                    divider -= 1;
                }
            } else {
                let mut multiplier = split + 1;
                while multiplier <= iteration {
                    split_length *= multiplier;
                    multiplier += 1;
                }
                let mut divider = difference;
                while divider > 1 {
                    split_length /= divider;
                    divider -= 1;
                }
            }
            split_length *= min_split_length;
            
            winner_idx = loser_idx + split_length / 2;
            for _ in 0 .. split_length / 2 {
                if values[loser_idx] > values[winner_idx] {
                    values.swap(loser_idx, winner_idx);
                }
                
                loser_idx += 1;
                winner_idx += 1;
            }
            
            loser_idx = winner_idx;
        }
    }
}
