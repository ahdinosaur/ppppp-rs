use std::collections::{BTreeMap, BTreeSet};

use crate::{author_id::AuthorId, msg::Msg, msg_id::MsgId};

pub struct Tangle {
    root_msg_id: MsgId,
    max_depth: u64,
    root_msg: Option<Msg>,
    tips: BTreeSet<MsgId>,
    prev_msg_ids: BTreeMap<MsgId, BTreeSet<MsgId>>,
    depth: BTreeMap<MsgId, u64>,
    per_depth: BTreeMap<u64, BTreeSet<MsgId>>,
}

impl Tangle {
    pub fn new(root_msg_id: MsgId) -> Self {
        Self {
            root_msg_id,
            max_depth: 0,
            root_msg: None,
            tips: BTreeSet::new(),
            prev_msg_ids: BTreeMap::new(),
            depth: BTreeMap::new(),
            per_depth: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, msg_id: &MsgId, msg: &Msg) {
        if msg_id == &self.root_msg_id && self.root_msg.is_none() {
            self.tips.insert(msg_id.clone());
            self.per_depth.insert(0, BTreeSet::from([msg_id.clone()]));
            self.depth.insert(msg_id.clone(), 0);
            self.root_msg = Some(msg.clone());
            return;
        }

        let tangles = msg.metadata().tangles();
        if msg_id != &self.root_msg_id && tangles.contains_key(&self.root_msg_id) {
            self.tips.insert(msg_id.clone());

            let tangle = tangles.get(&self.root_msg_id).unwrap();
            let prev_msg_ids = tangle.prev_msg_ids();
            for prev_msg_id in prev_msg_ids.clone() {
                self.tips.remove(&prev_msg_id);
            }
            self.prev_msg_ids
                .insert(msg_id.clone(), prev_msg_ids.clone());

            let tangle = tangles.get(&self.root_msg_id).unwrap();
            let depth = tangle.depth().clone();
            if depth > self.max_depth {
                self.max_depth = depth;
            }
            self.depth.insert(msg_id.clone(), depth);

            let at_depth = self.per_depth.entry(depth).or_insert_with(BTreeSet::new);
            at_depth.insert(msg_id.clone());
        }
    }

    fn get_all_at_depth(&self, depth: u64) -> BTreeSet<MsgId> {
        self.per_depth
            .get(&depth)
            .cloned()
            .unwrap_or(BTreeSet::new())
    }

    pub fn topo_sort(&self) -> Vec<MsgId> {
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

    pub fn get_tips(&self) -> BTreeSet<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return BTreeSet::new();
        }
        self.tips.clone()
    }

    pub fn get_lipmaa_set(&self, depth: u64) -> BTreeSet<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return BTreeSet::new();
        }
        let lipmaa_depth = lipmaa(depth + 1) - 1;
        self.get_all_at_depth(lipmaa_depth).clone()
    }

    pub fn has(&self, msg_id: &MsgId) -> bool {
        self.depth.contains_key(&msg_id)
    }

    pub fn get_depth(&self, msg_id: &MsgId) -> Option<u64> {
        self.depth.get(&msg_id).cloned()
    }

    pub fn is_feed(&self) -> bool {
        if let Some(root_msg) = &self.root_msg {
            let content = root_msg.content();
            let metadata = root_msg.metadata();
            content.is_null() && metadata.content_size() == &0 && metadata.content_hash().is_none()
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
            let content_type = metadata.content_type().to_owned();
            Some((author_id, content_type))
        }
    }

    pub fn shortest_path_to_root(&self, msg_id: &MsgId) -> Vec<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return Vec::new();
        }
        let mut path = Vec::new();
        let mut current = msg_id;
        while let Some(prev_msg_ids) = self.prev_msg_ids.get(&current) {
            let (min_msg_id, _) = prev_msg_ids
                .iter()
                .map(|msg_id| (msg_id, self.depth.get(msg_id).unwrap_or(&u64::MAX)))
                .min_by_key(|&(_, depth)| depth)
                .unwrap();
            path.push(min_msg_id.clone());
            current = min_msg_id;
        }
        path
    }

    pub fn precedes(&self, a: &MsgId, b: &MsgId) -> bool {
        if a == b || b == &self.root_msg_id {
            return false;
        }
        let mut to_check = vec![b];
        while let Some(current) = to_check.pop() {
            if let Some(prev_msg_ids) = self.prev_msg_ids.get(&current) {
                if prev_msg_ids.contains(&a) {
                    return true;
                }
                to_check.extend(prev_msg_ids.iter());
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
                .map(|msg_id| msg_id.to_string())
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
