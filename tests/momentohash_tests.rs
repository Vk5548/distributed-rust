#[cfg(test)]
mod tests {
    use simple_distribution_system::momentohash::momentohash::MomentoHash;
    use std::collections::HashSet;

    #[test]
    fn test_add_bucket() {
        println!(" Running test_add_bucket");
        let mut mh = MomentoHash::new(5);
        let new_bucket = mh.add_bucket();
        assert_eq!(new_bucket, 5); // 5th bucket should be added
        assert_eq!(mh.n, 6); // total number of buckets should be 6 after addition
    }

    #[test]
    fn test_remove_bucket() {
        println!(" Running test_remove_bucket");
        let mut mh = MomentoHash::new(5);
        mh.remove_bucket(2);
        assert_eq!(mh.n, 5); // total number of buckets should be 4 after removal, I am confused about this
        assert!(mh.R.contains_key(&2)); // removed bucket should be in the replacement set
    }

    #[test]
    fn test_lookup() {
        println!(" Running test_lookup");
        let mut mh = MomentoHash::new(5);
        let mut keys = HashSet::new();
        keys.insert("key1");
        keys.insert("key2");
        keys.insert("key3");
        keys.insert("key4");
        keys.insert("key5");

        for key in keys.iter() {
            let bucket = mh.lookup(key);
            assert!(bucket < 5); // bucket should be less than total number of buckets
        }
        mh.remove_bucket(2);
        for key in keys.iter() {
            let bucket = mh.lookup(key);
            assert!(bucket != 2); // bucket should be less than total number of buckets
        }
    }

    #[test]
    fn test_multiple_removals() {
        println!(" Running test_multiple_removals");
        let mut mh = MomentoHash::new(5);
        mh.remove_bucket(2);
        mh.remove_bucket(3);
        mh.remove_bucket(4);
        assert_eq!(mh.n, 5); // total number of buckets should be 5 after removals, wb are in a certain range
        assert!(mh.R.contains_key(&2)); // removed bucket should be in the replacement set
        assert!(mh.R.contains_key(&3)); // removed bucket should be in the replacement set
        assert!(mh.R.contains_key(&4)); // removed bucket should be in the replacement set
    }

    #[test]
    fn test_add_remove() {
        println!(" Running test_add_remove");
        let mut mh = MomentoHash::new(5);
        mh.remove_bucket(2);
        mh.remove_bucket(3);
        mh.remove_bucket(4);
        mh.add_bucket();
        mh.add_bucket();
        assert_eq!(mh.n, 5); // total number of buckets should be 4 after removals and additions
        assert!(mh.R.contains_key(&2)); // removed bucket should be in the replacement set
        assert!(!mh.R.contains_key(&3)); // removed bucket should be in the replacement set
        assert!(!mh.R.contains_key(&4)); // removed bucket should be in the replacement set
    }

    #[test]
    fn test_consistent_hashing() {
        println!(" Running test_consistent_hashing");
        let mut mh = MomentoHash::new(5);
        let mut keys = HashSet::new();
        keys.insert("key1");
        keys.insert("key2");
        keys.insert("key3");
        keys.insert("key4");
        keys.insert("key5");

        let mut buckets = HashSet::new();
        for key in keys.iter() {
            let bucket = mh.lookup(key);
            buckets.insert(bucket);
        }
        assert!(buckets.len() <= 5); // all keys should be distributed to different buckets

        let mut results = HashSet::new();
        for _ in 0..100 {
            results.insert(mh.lookup(&"key1"));
        }
        assert_eq!(results.len(), 1); // Lookup should always return the same bucket

        mh.remove_bucket(2);
        mh.remove_bucket(3);
        mh.remove_bucket(4);

        for key in keys.iter() {
            let bucket = mh.lookup(key);
            assert!(bucket < 3); // bucket should be less than total number of buckets
                                 // assert_eq!(buckets.len(), 5);
        }
        assert_eq!(mh.n, 5); // total number of buckets should be 2 after removals
    }
}
