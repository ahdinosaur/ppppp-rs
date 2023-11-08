use std::collections::{BTreeMap, BTreeSet};

use crate::{author_id::AuthorId, msg::Msg, msg_hash::MsgHash};

pub struct Tangle {
    root_msg_hash: MsgHash,
    max_depth: u64,
    root_msg: Option<Msg>,
    tips: BTreeSet<MsgHash>,
    prev_msg_hashs: BTreeMap<MsgHash, BTreeSet<MsgHash>>,
    depth: BTreeMap<MsgHash, u64>,
    per_depth: BTreeMap<u64, BTreeSet<MsgHash>>,
}

impl Tangle {
    pub fn new(root_msg_hash: MsgHash) -> Self {
        Self {
            root_msg_hash,
            max_depth: 0,
            root_msg: None,
            tips: BTreeSet::new(),
            prev_msg_hashs: BTreeMap::new(),
            depth: BTreeMap::new(),
            per_depth: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, msg_hash: &MsgHash, msg: &Msg) {
        if msg_hash == &self.root_msg_hash && self.root_msg.is_none() {
            self.tips.insert(msg_hash.clone());
            self.per_depth.insert(0, BTreeSet::from([msg_hash.clone()]));
            self.depth.insert(msg_hash.clone(), 0);
            self.root_msg = Some(msg.clone());
            return;
        }

        let tangles = msg.metadata().tangles();
        if msg_hash != &self.root_msg_hash && tangles.contains_key(&self.root_msg_hash) {
            self.tips.insert(msg_hash.clone());

            let tangle = tangles.get(&self.root_msg_hash).unwrap();
            let prev_msg_hashs = tangle.prev_msg_hashs();
            for prev_msg_hash in prev_msg_hashs.clone() {
                self.tips.remove(&prev_msg_hash);
            }
            self.prev_msg_hashs
                .insert(msg_hash.clone(), prev_msg_hashs.clone());

            let tangle = tangles.get(&self.root_msg_hash).unwrap();
            let depth = tangle.depth().clone();
            if depth > self.max_depth {
                self.max_depth = depth;
            }
            self.depth.insert(msg_hash.clone(), depth);

            let at_depth = self.per_depth.entry(depth).or_insert_with(BTreeSet::new);
            at_depth.insert(msg_hash.clone());
        }
    }

    fn get_all_at_depth(&self, depth: u64) -> BTreeSet<MsgHash> {
        self.per_depth
            .get(&depth)
            .cloned()
            .unwrap_or(BTreeSet::new())
    }

    pub fn topo_sort(&self) -> Vec<MsgHash> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return Vec::new();
        }
        let mut sorted = Vec::new();
        for i in 0..=self.max_depth {
            let at_depth = self.get_all_at_depth(i);
            sorted.extend(at_depth.iter().cloned());
        }
        sorted
    }

    pub fn get_tips(&self) -> BTreeSet<MsgHash> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return BTreeSet::new();
        }
        self.tips.clone()
    }

    pub fn get_lipmaa_set(&self, depth: u64) -> BTreeSet<MsgHash> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return BTreeSet::new();
        }
        let lipmaa_depth = lipmaa(depth + 1) - 1;
        self.get_all_at_depth(lipmaa_depth).clone()
    }

    pub fn has(&self, msg_hash: &MsgHash) -> bool {
        self.depth.contains_key(&msg_hash)
    }

    pub fn get_depth(&self, msg_hash: &MsgHash) -> Option<u64> {
        self.depth.get(&msg_hash).cloned()
    }

    pub fn is_feed(&self) -> bool {
        if let Some(root_msg) = &self.root_msg {
            let data = root_msg.data();
            let metadata = root_msg.metadata();
            data.is_null() && metadata.data_size() == &0 && metadata.data_hash().is_none()
        } else {
            eprintln!("Tangle is missing root message");
            false
        }
    }

    pub fn get_feed(&self) -> Option<(AuthorId, String)> {
        if !self.is_feed() {
            None
        } else {
            let root_msg = self.root_msg.as_ref().unwrap();
            let metadata = root_msg.metadata();
            let author_id = metadata.author_id().clone();
            let data_type = metadata.data_type().to_owned();
            Some((author_id, data_type))
        }
    }

    pub fn shortest_path_to_root(&self, msg_hash: &MsgHash) -> Vec<MsgHash> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return Vec::new();
        }
        let mut path = Vec::new();
        let mut current = msg_hash;
        while let Some(prev_msg_hashs) = self.prev_msg_hashs.get(&current) {
            let (min_msg_hash, _) = prev_msg_hashs
                .iter()
                .map(|msg_hash| (msg_hash, self.depth.get(msg_hash).unwrap_or(&u64::MAX)))
                .min_by_key(|&(_, depth)| depth)
                .unwrap();
            path.push(min_msg_hash.clone());
            current = min_msg_hash;
        }
        path
    }

    pub fn precedes(&self, a: &MsgHash, b: &MsgHash) -> bool {
        if a == b || b == &self.root_msg_hash {
            return false;
        }
        let mut to_check = vec![b];
        while let Some(current) = to_check.pop() {
            if let Some(prev_msg_hashs) = self.prev_msg_hashs.get(&current) {
                if prev_msg_hashs.contains(&a) {
                    return true;
                }
                to_check.extend(prev_msg_hashs.iter());
            }
        }
        false
    }

    pub fn size(&self) -> usize {
        self.depth.len()
    }

    pub fn get_max_depth(&self) -> u64 {
        self.max_depth
    }

    pub fn debug(&self) -> String {
        let mut str = String::new();
        for i in 0..=self.max_depth {
            let at_depth = self.get_all_at_depth(i);
            let at_depth_str = at_depth
                .iter()
                .map(|msg_hash| msg_hash.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            str.push_str(&format!("Depth {}: {}\n", i, at_depth_str));
        }
        str
    }
}

fn lipmaa(n: u64) -> u64 {
    let mut m = 1;
    let mut po3 = 3;
    let mut u = n;

    // find k such that (3^k - 1)/2 >= n
    while m < n {
        po3 *= 3;
        m = (po3 - 1) / 2;
    }

    // find longest possible backjump
    po3 /= 3;
    if m != n {
        while u != 0 {
            m = (po3 - 1) / 2;
            po3 /= 3;
            u %= m;
        }

        if m != po3 {
            po3 = m;
        }
    }

    return n - po3;
}
