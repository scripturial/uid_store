use rand::Rng;
use std::collections::HashSet;

// Generate unique game item identifiers
pub struct UidStore {
    size: usize,
    items: HashSet<String>,
}

impl UidStore {
    pub fn new(size: usize) -> UidStore {
        UidStore {
            size: size,
            items: HashSet::new(),
        }
    }

    // Generate, remember, and return a UID.
    pub fn next(&mut self) -> &String {
        loop {
            let id = random_string(self.size);
            if !self.items.insert(id.clone()) {
                continue;
            }
            return self.items.get(&id).unwrap();
        }
    }

    // Generate, remember, and return a human readable UID.
    pub fn next_human(&mut self) -> &String {
        loop {
            let id = human_random_string(self.size);
            if !self.items.insert(id.clone()) {
                continue;
            }
            return self.items.get(&id).unwrap();
        }
    }

    // Generate a uid that fits in a u16
    pub fn next_u16(&mut self) -> &String {
        loop {
            let id = random_max_size(u16::MAX as usize);
            if !self.items.insert(id.clone()) {
                continue;
            }
            return self.items.get(&id).unwrap();
        }
    }

    // Generate a uid that fits in a u16
    pub fn next_u32(&mut self) -> &String {
        loop {
            let id = random_max_size(u32::MAX as usize);
            if !self.items.insert(id.clone()) {
                continue;
            }
            return self.items.get(&id).unwrap();
        }
    }

    // next_u64 generates a base62 uid that fits in a u16.
    pub fn next_u64(&mut self) -> &String {
        loop {
            let id = random_max_size(u64::MAX as usize);
            if !self.items.insert(id.clone()) {
                continue;
            }
            return self.items.get(&id).unwrap();
        }
    }

    // Check if an ID is already in use
    pub fn contains(&self, id: &str) -> bool {
        self.items.contains(id)
    }

    // Check if an ID is already in use
    pub fn size(&self) -> usize {
        self.items.len()
    }

    // return an updated UID if this one is not unique.
    pub fn make_unique(&mut self, id: &str) -> Option<&str> {
        if self.items.contains(id) {
            return Some(self.next());
        }
        self.items.insert(id.to_string());
        None
    }
}

pub fn random_string(size: usize) -> String {
    let mut rng = rand::thread_rng();

    let result: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    result
}

pub fn random_number(size: usize) -> String {
    let mut rng = rand::thread_rng();

    let result: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..NUMSET.len());
            NUMSET[idx] as char
        })
        .collect();

    result
}

pub fn random_max_size(size: usize) -> String {
    let mut rng = rand::thread_rng();
    let uid = rng.gen_range(0..size);
    number_to_uid(uid)
}

pub fn number_to_uid(mut uid: usize) -> String {
    let mut result = String::new();
    if uid == 0 {
        return "A".to_string();
    }
    while uid > 0 {
        let next = uid % CHARSET.len();
        println!("-- {} {}  - {}", next, uid, CHARSET.len());
        uid = uid / CHARSET.len();
        result.push(CHARSET[next] as char);
    }
    result
}

pub fn uid_to_number(uid: &str) -> Option<usize> {
    let mut result: usize = 0;
    for c in uid.chars().rev() {
        /* Rust 1.18
        let value = match c {
            'A'..'Z' => c - 'A',
            'a'..'z' => c - 'a' + 26,
            '0'..'9' => c - '0' + 26 + 26,
        };
        */
        let value;
        if c >= 'A' && c <= 'Z' {
            value = (c as usize) - ('A' as usize);
        } else if c >= 'a' && c <= 'z' {
            value = (c as usize) - ('a' as usize) as usize + 26;
        } else if c >= '0' && c <= '9' {
            value = (c as usize) - ('0' as usize) + 26 + 26;
        } else {
            return None;
        }
        println!("== {} {} ", c, value);
        result = result * 62 + value;
    }
    Some(result)
}

pub fn human_random_string(size: usize) -> String {
    let mut rng = rand::thread_rng();

    let result: String = (0..size)
        .map(|_| {
            let idx = rng.gen_range(0..READABLE_CHARSET.len());
            READABLE_CHARSET[idx] as char
        })
        .collect();

    result
}

const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
abcdefghijklmnopqrstuvwxyz\
0123456789";

const READABLE_CHARSET: &[u8] = b"ABCDEFGHJKMNPQRSTUVWXYZ\
abcdefghjkmnpqrstuvwxyz\
123456789";

const NUMSET: &[u8] = b"0123456789";

#[cfg(test)]
mod tests {
    use crate::human_random_string;
    use crate::number_to_uid;
    use crate::random_number;
    use crate::random_string;
    use crate::uid_to_number;
    use crate::UidStore;

    #[test]
    fn test_number_to_uid() {
        assert_eq!(number_to_uid(0), "A");
        assert_eq!(number_to_uid(1), "B");
        assert_eq!(number_to_uid(52), "0");
        assert_eq!(number_to_uid(9902), "sjC");
        assert_eq!(uid_to_number("A"), Some(0));
        assert_eq!(uid_to_number("B"), Some(1));
        assert_eq!(uid_to_number("0"), Some(52));
        assert_eq!(uid_to_number("sjC"), Some(9902));
        assert_eq!(uid_to_number(&number_to_uid(94029)), Some(94029));
        assert_eq!(uid_to_number(&number_to_uid(2294029)), Some(2294029));
        assert_eq!(uid_to_number(&number_to_uid(43494029)), Some(43494029));
    }

    #[test]
    fn test_random_max_size() {
        let mut u = UidStore::new(10);
        for _ in [0..1000] {
            assert!(uid_to_number(u.next_u16()).unwrap() < u16::MAX.into());
            assert!(uid_to_number(u.next_u32()).unwrap() < u32::MAX.try_into().unwrap());
            assert!(uid_to_number(u.next_u64()).unwrap() < u64::MAX.try_into().unwrap());
        }
    }

    #[test]
    fn test_random() {
        let id = random_string(5);
        assert_eq!(id.len(), 5);

        let id2 = random_string(5);
        assert!(id != id2);

        let id3 = random_number(6);
        let id4 = random_number(6);
        assert_eq!(id3.len(), 6);
        assert!(id3 != id4);

        let id5 = human_random_string(5);
        let id6 = human_random_string(5);
        assert_eq!(id5.len(), 5);
        assert!(id5 != id6);
    }

    #[test]
    fn test_unique() {
        let mut u = UidStore::new(10);
        let id: String;
        let id2: String;
        {
            id = u.next().to_string();
            assert_eq!(id.len(), 10, "failed");
            assert!(u.contains(&id));
        };
        assert_eq!(u.size(), 1, "failed");
        {
            id2 = u.next().to_string();
            assert_eq!(id2.len(), 10, "failed");
            assert!(u.contains(&id2));
        };
        assert_eq!(u.size(), 2, "failed");
        assert_ne!(id, id2, "failed");

        let xo = "0123456789";
        {
            let o = u.make_unique(xo);
            assert!(o.is_none(), "failed. Found {:?}", o.unwrap());
        };
        assert_eq!(u.size(), 3, "failed");

        {
            let o = u.make_unique(xo);
            assert!(o.is_some(), "failed. Found {:?}", o);
            assert_ne!(o.unwrap(), xo, "failed");
        };
        assert_eq!(u.size(), 4, "failed");
    }
}
