#![allow(dead_code)]
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

type Link<K, V> = Option<Rc<RefCell<Node<K, V>>>>;

#[derive(Debug, Clone)]
struct Node<K, V> {
    key: K,
    data: V,
    next: Link<K, V>,
    prev: Link<K, V>,
}

#[derive(Debug, Clone)]
struct LruCache<K: std::hash::Hash + std::cmp::Eq, V> {
    capacity: usize,
    map: HashMap<K, Link<K, V>>,
    head: Link<K, V>,
    tail: Link<K, V>,
}

impl<K: std::hash::Hash + std::cmp::Eq + Clone, V: Clone> LruCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            head: None,
            tail: None,
        }
    }

    pub fn get(&mut self, key: K) -> Option<V> {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone().unwrap();
            let data = node.borrow().data.clone();
            self.remove_node(Some(node.clone()));
            self.push_front(Some(node.clone()));
            Some(data)
        } else {
            None
        }
    }

    pub fn put(&mut self, key: K, value: V) {
        if let Some(node) = self.map.get(&key) {
            let node = node.clone().unwrap();
            node.borrow_mut().data = value;
            self.remove_node(Some(node.clone()));
            self.push_front(Some(node.clone()));
            return;
        }

        if self.map.len() >= self.capacity {
            if let Some(tail) = self.tail.clone() {
                let tail_key = tail.borrow().key.clone();
                self.map.remove(&tail_key);
                self.remove_node(Some(tail.clone()));
            }
        };

        let new_node = Rc::new(RefCell::new(Node {
            key: key.clone(),
            data: value,
            next: None,
            prev: None,
        }));

        self.push_front(Some(new_node.clone()));
        self.map.insert(key, Some(new_node.clone()));
        
    }

    fn remove_node(&mut self, node: Link<K, V>) {
        if let Some(node) = node {
            let prev = node.borrow().prev.clone();
            let next = node.borrow().next.clone();

            if let Some(prev) = &prev {
                prev.borrow_mut().next = next.clone();
            } else {
                self.head = next.clone();
            }

            if let Some(next) = &next {
                next.borrow_mut().prev = prev.clone();
            } else {
                self.tail = prev.clone();
            }
            
            drop(next);
            drop(prev);
        }
    }

    fn push_front(&mut self, node: Link<K, V>) {
        if let Some(node) = &node {
            node.borrow_mut().next = self.head.clone();
            node.borrow_mut().prev = None;

            if let Some(head) = &self.head {
                head.borrow_mut().prev = Some(node.clone());
            }

            self.head = Some(node.clone());

            if self.tail.is_none() {
                self.tail = Some(node.clone());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = LruCache::new(2);
        cache.put(1, 1);
        cache.put(2, 2);
        assert_eq!(cache.get(1), Some(1));
        cache.put(3, 3);
        assert_eq!(cache.get(2), None);
        cache.put(4, 4);
        assert_eq!(cache.get(1), None);
        assert_eq!(cache.get(3), Some(3));
        assert_eq!(cache.get(4), Some(4));
    }
}
