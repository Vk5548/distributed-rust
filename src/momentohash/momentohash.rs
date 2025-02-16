// Functionality :
// -> Dense b-array, Replacement Set(R), Lazy Migration(I am still confused about the same)
// -> LookUp function following R

use std::collections::HashMap;
use std::collections::HashSet;

//use jumphash for the initial hashing
use jumphash::JumpHasher;

use siphasher::sip::SipHasher13;
use std::hash::{Hash, Hasher}; // Using SipHAsh for the traditional hashing

/// Represents the MomentoHash State
pub struct MomentoHash {
    pub n: usize,                          // Number of total buckets
    pub l: usize,                          // Last removed bucket
    pub R: HashMap<usize, (usize, usize)>, // Replacement Set {b -> (c, p)}
    pub hasher: JumpHasher,                // JumpHasher instance
}

impl MomentoHash {
    ///Initializes the new MomentoHash instance
    pub fn new(initial_node_count: usize) -> Self {
        Self {
            n: initial_node_count,
            // and if a next bucket is added to the initial state, it would be 'n' and therefore it works out
            l: initial_node_count, // Set last removed bucket to n (buckets are 0-indexed; hence no bucket indexed n yet)
            R: HashMap::new(),     // Empty Replacement Set initially
            hasher: JumpHasher::new(), // Initialize the hasher
        }
    }

    /// Function to remove a bucket
    pub fn remove_bucket(&mut self, b: usize) {
        if b >= self.n {
            // the bucket doesn't exist
            panic!("Invalid bucket index {}. Must be 0 <= b < n.", b); //later will update for another iterative input
                                                                       //until we get it right
        }

        if b == self.n - 1 && self.R.is_empty() {
            // last indexed working bucket to be removed
            // Case 1: If removing last bucket and 'R'is Empty, hence just reduce 'n'
            self.n -= 1;
            //TODO: how does keys migration take place or does it take place?
        } else {
            // Case 2: Remove an arbitrary bucket (not the last one)
            let w = self.n - self.R.len(); //Number of currently working buckets
                                           //Storing replacement of b
            self.R.insert(b, (w - 1, self.l));
            //TODO: how does keys migration take place or does it take place?
        }

        // upadte the last removed bucket in any case
        self.l = b;
    }

    // Add a bucket (either stores last removed bucket or expands n if 'R' is empty)
    pub fn add_bucket(&mut self) -> usize {
        if self.R.is_empty() {
            // Case 1: If 'R' is empty, just expand 'n'
            let b = self.n;
            self.n += 1; //increament 'n' by 1
            self.l = self.n; //update last removed bucket to the new bucket
            b //returning the added bucket; the previous value of n
        } else {
            // Case 2: If 'R' is not empty, pop a bucket from 'R' which is 'l':last removed bucket.
            let b = self.l;
            let (_, p) = self.R.remove(&b).unwrap(); //here 'l' becomes the key for {b->(c,p)}
            self.l = p;
            b //returning the added bucket from R
        }
        //TODO:how does keys migration take place or does it take place?
    }

    /// Function to LookUp a key in the MomentoHash (follows "R" chain)
    pub fn lookup<T: Hash>(&self, key: &T) -> usize {
        //get the jumphash of the key i.e. the first bucket in the range  0 to self.n-1
        //how do I include the jump-hash crate
        //Step 1: get the jumphash of the key
        let mut b = self.hasher.slot(key, self.n as u32) as usize;

        let mut visited = HashSet::new(); //to keep track of visited buckets

        //Step 2: Follow the R chain
        while let Some(&(c, p)) = self.R.get(&b) {
            if !visited.insert(b) {
                eprintln!(
                    "Cycle detected in the replacement chain for Bucket : {}. Exiting.",
                    b
                );
                return b; //return the bucket with cycle to avoid crash
            }

            // if there is a replacement present
            let wb = c; // Step 3: get the working bucket count after removal of 'b'

            //Step 4: compute the traditional hash such that the keys were evenly re-distributed
            let mut sip = SipHasher13::new();
            key.hash(&mut sip); //hash the key
            let h = sip.finish() as usize; //get the hash value

            //Step 5: Compute the new bucket 'd'
            let mut d = h % wb;

            //Step 6-8: Follow the substitution chain until we find a working bucket
            while let Some(&(u, _q)) = self.R.get(&d) {
                if u >= wb {
                    d = u; // Follow the substitution chain; that's how the bucket not in range 0 to wb-1 but still part of the dense array is assigned
                } else {
                    break; // Break the loop if the bucket is in the range 0 to wb-1
                }
            }
            b = d; // Update the bucket to the new working bucket
        }
        b // Return the final bucket
    }
    //finds the target node(server) for the key given
    pub fn get_target_node<T: Hash>(&self, key: &T) -> String {
        let bucket = self.lookup(key); // Step 1: find teh correct bucket ID
        format!("datanode{}", bucket) // Step2: Convert the bucket ID to the node identifier
    }
}
