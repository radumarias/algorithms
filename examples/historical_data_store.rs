use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::io;
use std::io::Read;
use std::rc::{Rc, Weak};
use std::slice::{Iter, IterMut};
use std::vec::IntoIter;

pub struct TreeNode<T> {
    pub value: T,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
    parent: Weak<RefCell<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Rc<RefCell<TreeNode<T>>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            children: Vec::new(),
            parent: Weak::new(),
        }))
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        self.parent.upgrade()
    }

    pub fn iter(&mut self) -> Iter<'_, Rc<RefCell<TreeNode<T>>>> {
        self.children.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<'_, Rc<RefCell<TreeNode<T>>>> {
        self.children.iter_mut()
    }

    pub fn into_iter(self) -> IntoIter<Rc<RefCell<TreeNode<T>>>> {
        self.children.into_iter()
    }
}

pub struct Tree<T> {
    root: Option<Rc<RefCell<TreeNode<T>>>>,
}

impl<T> Tree<T> {
    pub fn new() -> Self {
        Tree { root: None }
    }
    pub fn set_root(&mut self, root: Rc<RefCell<TreeNode<T>>>) {
        self.root = Some(root);
    }

    pub fn get_root(&self) -> Option<Rc<RefCell<TreeNode<T>>>> {
        self.root.clone()
    }

    pub fn push_child(&self, parent: &Rc<RefCell<TreeNode<T>>>, child: &Rc<RefCell<TreeNode<T>>>) {
        parent.borrow_mut().children.push(child.clone());
        child.borrow_mut().parent = Rc::downgrade(parent);
    }

    pub fn remove_child(
        &self,
        parent: &Rc<RefCell<TreeNode<T>>>,
        child: &Rc<RefCell<TreeNode<T>>>,
    ) {
        parent
            .borrow_mut()
            .children
            .retain(|c| !Rc::ptr_eq(c, &child));
        child.borrow_mut().parent = Weak::new();
    }
}

#[derive(Default, Debug)]
struct Value<V: Sized + Ord + Debug> {
    ver: u64,
    value: V,
}

#[derive(Default, Debug)]
struct HistoricalDataStore<K: Sized + Ord + Debug, V: Sized + Ord + Debug> {
    map: BTreeMap<K, Value<V>>,
}

impl<K: Sized + Ord + Debug, V: Sized + Ord + Debug> HistoricalDataStore<K, V> {
    fn record(&mut self, key: K, value: V, time: u64) {
        let entry = self.map.entry(key).or_default();

        if let Some(existing_value) = entry.get_mut(&time) {
            *existing_value = value; // Update the existing value
        } else {
            entry.insert(time, value); // Insert the new value if it doesn't exist
        }
    }

    fn retrieve<Q: Borrow<K> + Ord>(&self, key: Q, mut time: u64) -> Option<&V> {
        // optimize with binary search
        loop {
            // lookup by key
            if let Some(map) = self.map.get(key.borrow()) {
                // lookup closest smaller version
                if let Some(v) = map.get(&time) {
                    return Some(v);
                }
            } else {
                return None;
            }
            time -= 1;
            if time == 0 {
                return None;
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut values: Vec<(String, String, u64)> = vec![];
    let mut in_test = false;
    let mut in_test_in = false;
    let mut in_test_out = false;
    let mut test_key = "".to_string();
    let mut test_ver = 0;
    let mut test_val = "".to_string();

    input.lines().for_each(|x| {
        if x.trim().is_empty() {
            in_test = true;
            in_test_in = true;
            return;
        }
        if !in_test {
            // input line
            let parts: Vec<&str> = x.split(" ").collect();
            values.push((
                parts[0].to_string(),
                parts[1].to_string(),
                parts[2].parse().unwrap(),
            ));
        } else {
            // test lines
            if in_test_in {
                let parts: Vec<&str> = x.split(" ").collect();
                test_key = parts[0].to_string();
                test_ver = parts[1].parse::<u64>().unwrap();
                in_test_in = false;
                in_test_out = true;
            } else if in_test_out {
                test_val = x.to_string();
            }
        }
    });

    let mut store = HistoricalDataStore::default();

    for (key, value, time) in values {
        store.record(key, value, time);
    }

    if let Some(v) = store.retrieve(test_key, test_ver) {
        println!("{v}")
    } else {
        println!("None")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let mut store = HistoricalDataStore::default();

        store.record("report".to_string(), "initial".to_string(), 2); // save "report" with "initial" at time 2.
        store.record("report".to_string(), "updated".to_string(), 5); // save "report" with "updated" at time 5.
        store.record("report".to_string(), "final".to_string(), 7); // save "report" with "final" at time 7.

        store.record("report".to_string(), "initial2".to_string(), 2);

        assert_eq!(
            Some(&"updated".to_string()),
            store.retrieve("report".to_string(), 6)
        ); // return "updated" (since the closest prior time is 5).
        assert_eq!(
            Some(&"initial2".to_string()),
            store.retrieve("report".to_string(), 2)
        ); // return "initial"
        assert_eq!(
            Some(&"final".to_string()),
            store.retrieve("report".to_string(), 8)
        ); // return "initial"
    }

    #[test]
    fn test_fail() {
        let mut store = HistoricalDataStore::default();

        store.record("report".to_string(), "initial".to_string(), 2); // save "report" with "initial" at time 2.
        store.record("report".to_string(), "updated".to_string(), 5); // save "report" with "updated" at time 5.
        store.record("report".to_string(), "final".to_string(), 7); // save "report" with "final" at time 7.

        assert_ne!(
            Some(&"updated2".to_string()),
            store.retrieve("report".to_string(), 6)
        ); // return "updated" (since the closest prior time is 5).
        assert_ne!(
            Some(&"initial2".to_string()),
            store.retrieve("report".to_string(), 2)
        ); // return "initial"
        assert_ne!(
            Some(&"final2".to_string()),
            store.retrieve("report".to_string(), 8)
        ); // return "initial"
    }
}
