use std::collections::{HashMap, HashSet};

use lipmaa_link::lipmaa;

use crate::{AccountId, MootDetails, Msg, MsgId};

#[derive(Clone, Debug, thiserror::Error)]
#[error("tangle is missing root message: {root_msg_id}")]
pub struct TangleMissingRootMessageError {
    pub root_msg_id: MsgId,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TangleType {
    Feed,
    Account,
    Weave,
}

#[derive(Clone, Debug)]
pub struct Tangle {
    root_msg_id: MsgId,
    root_msg: Option<Msg>,
    tips: HashSet<MsgId>,
    prev_msg_ids: HashMap<MsgId, HashSet<MsgId>>,
    depth: HashMap<MsgId, u64>,
    per_depth: HashMap<u64, HashSet<MsgId>>,
    max_depth: u64,
}

impl Tangle {
    pub fn new(root_msg_id: MsgId) -> Self {
        Self {
            root_msg_id,
            root_msg: None,
            tips: HashSet::new(),
            prev_msg_ids: HashMap::new(),
            depth: HashMap::new(),
            per_depth: HashMap::new(),
            max_depth: 0,
        }
    }

    pub fn add(&mut self, msg_hash: &MsgId, msg: &Msg) {
        if msg_hash == &self.root_msg_id && self.root_msg.is_none() {
            self.tips.insert(*msg_hash);
            self.per_depth.insert(0, HashSet::from([*msg_hash]));
            self.depth.insert(*msg_hash, 0);
            self.root_msg = Some(msg.clone());
            return;
        }

        let tangles = msg.metadata().tangles();
        if msg_hash != &self.root_msg_id && tangles.contains_key(&self.root_msg_id) {
            self.tips.insert(*msg_hash);

            let tangle = tangles.get(&self.root_msg_id).unwrap();
            let prev_msg_ids = tangle.prev_msg_ids();
            for prev_msg_hash in prev_msg_ids.clone() {
                self.tips.remove(&prev_msg_hash);
            }
            self.prev_msg_ids.insert(*msg_hash, prev_msg_ids.clone());

            let tangle = tangles.get(&self.root_msg_id).unwrap();
            let depth = tangle.depth();
            if depth > self.max_depth {
                self.max_depth = depth;
            }
            self.depth.insert(*msg_hash, depth);

            let at_depth = self.per_depth.entry(depth).or_default();
            at_depth.insert(*msg_hash);
        }
    }

    fn get_all_at_depth(&self, depth: u64) -> HashSet<MsgId> {
        self.per_depth
            .get(&depth)
            .cloned()
            .unwrap_or(HashSet::new())
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

    pub fn get_tips(&self) -> HashSet<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return HashSet::new();
        }
        self.tips.clone()
    }

    pub fn get_lipmaa_set(&self, depth: u64) -> HashSet<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return HashSet::new();
        }
        let lipmaa_depth = lipmaa(depth + 1) - 1;
        self.get_all_at_depth(lipmaa_depth).clone()
    }

    pub fn has(&self, msg_hash: &MsgId) -> bool {
        self.depth.contains_key(msg_hash)
    }

    pub fn get_depth(&self, msg_hash: &MsgId) -> Option<u64> {
        self.depth.get(msg_hash).cloned()
    }

    pub fn is_feed(&self) -> bool {
        let Some(ref root_msg) = self.root_msg else {
            eprintln!("Tangle is missing root message");
            return false;
        };
        root_msg.is_moot(None, None)
    }

    pub fn get_id(&self) -> &MsgId {
        &self.root_msg_id
    }

    pub fn get_moot_details(&self) -> Option<MootDetails> {
        if !self.is_feed() {
            return None;
        }
        let Some(ref root_msg) = self.root_msg else {
            eprintln!("Tangle is missing root message");
            return None;
        };
        let metadata = root_msg.metadata();
        Some(MootDetails {
            account_id: metadata.account_id().clone(),
            domain: metadata.domain().clone(),
            id: self.root_msg_id,
        })
    }

    pub fn get_type(&self) -> Result<TangleType, TangleMissingRootMessageError> {
        let Some(ref root_msg) = self.root_msg else {
            return Err(TangleMissingRootMessageError {
                root_msg_id: self.root_msg_id,
            });
        };
        if self.is_feed() {
            Ok(TangleType::Feed)
        } else if root_msg.metadata().account_id() == &AccountId::SelfIdentity {
            Ok(TangleType::Account)
        } else {
            Ok(TangleType::Weave)
        }
    }

    pub fn get_root(&self) -> Result<Msg, TangleMissingRootMessageError> {
        let Some(ref root_msg) = self.root_msg else {
            return Err(TangleMissingRootMessageError {
                root_msg_id: self.root_msg_id,
            });
        };
        Ok(root_msg.clone())
    }

    pub fn shortest_path_to_root(&self, msg_hash: &MsgId) -> Vec<MsgId> {
        if self.root_msg.is_none() {
            eprintln!("Tangle is missing root message");
            return Vec::new();
        }
        let mut path = Vec::new();
        let mut current = msg_hash;
        while let Some(prev_msg_ids) = self.prev_msg_ids.get(current) {
            let (min_msg_hash, _) = prev_msg_ids
                .iter()
                .map(|msg_hash| (msg_hash, self.depth.get(msg_hash).unwrap_or(&u64::MAX)))
                .min_by_key(|&(_, depth)| depth)
                .unwrap();
            path.push(*min_msg_hash);
            current = min_msg_hash;
        }
        path
    }

    pub fn get_minimum_among(&self, msg_ids: Vec<MsgId>) -> Vec<MsgId> {
        let mut minimum: HashSet<MsgId> = HashSet::from_iter(msg_ids.iter().cloned());
        for a in &msg_ids {
            for b in &msg_ids {
                if self.precedes(a, b) {
                    minimum.remove(b);
                }
            }
        }
        minimum.into_iter().collect()
    }

    pub fn precedes(&self, a: &MsgId, b: &MsgId) -> bool {
        if a == b || b == &self.root_msg_id {
            return false;
        }
        let mut to_check = vec![b];
        while let Some(current) = to_check.pop() {
            if let Some(prev_msg_ids) = self.prev_msg_ids.get(current) {
                if prev_msg_ids.contains(a) {
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
                .map(|msg_hash| msg_hash.to_string())
                .collect::<Vec<String>>()
                .join(", ");
            str.push_str(&format!("Depth {}: {}\n", i, at_depth_str));
        }
        str
    }
}
