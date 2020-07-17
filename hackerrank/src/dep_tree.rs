use std::{
    any::{Any, TypeId},
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

#[derive(Debug)]
pub struct Node<T> {
    num_parents: usize,
    children: Vec<T>,
    value: T,
}

impl<T> PartialEq for Node<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<T> Eq for Node<T> where T: Eq {}

impl<T> Hash for Node<T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl<T> Node<T> {
    pub fn new(value: T) -> Self {
        Self {
            num_parents: 0,
            children: Vec::new(),
            value,
        }
    }
}

#[derive(Debug)]
pub struct DependencyTree {
    items: HashMap<TypeId, Box<dyn Any>>,
    dep_tree: HashMap<TypeId, Vec<TypeId>>,
    child_tree: HashMap<TypeId, Node<TypeId>>,
    visited: HashSet<TypeId>,
}

impl Default for DependencyTree {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            dep_tree: HashMap::new(),
            child_tree: HashMap::new(),
            visited: HashSet::new(),
        }
    }
}

impl DependencyTree {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: 'static>(&mut self, item: T, deps: &[TypeId]) {
        let type_id = <T as Any>::type_id(&item);
        self.items.insert(type_id, Box::new(item));

        self.dep_tree
            .entry(type_id)
            .or_default()
            .extend_from_slice(deps);
    }

    pub fn add_child<T: 'static>(&mut self, item: T, input: &[TypeId]) {
        let name = <T as Any>::type_id(&item);
        self.items.insert(name, Box::new(item));

        let mut num_parents: usize = 0;
        for &parent in input {
            if parent == name {
                continue;
            }
            self.child_tree
                .entry(parent)
                .or_insert_with(|| Node {
                    value: parent,
                    children: vec![],
                    num_parents: 0,
                })
                .children
                .push(name);
            num_parents += 1;
        }
        self.child_tree
            .entry(name)
            .or_insert_with(|| Node {
                value: name,
                children: Vec::new(),
                num_parents,
            })
            .num_parents = num_parents;
    }

    pub fn topological_sort(mut self) -> Option<Vec<TypeId>> {
        let mut queue = self
            .child_tree
            .iter()
            .filter(|(_, v)| v.num_parents == 0)
            .map(|(k, _)| *k)
            .collect::<Vec<_>>();
        let mut ret = Vec::new();
        while let Some(cur) = queue.pop() {
            for children in self
                .child_tree
                .get_mut(&cur)?
                .children
                .drain(0..)
                .collect::<Vec<_>>()
            {
                println!("visiting child {:?}", children);
                let child = self.child_tree.get_mut(&children)?;
                child.num_parents -= 1;
                if child.num_parents == 0 {
                    queue.push(child.value)
                }
            }
            ret.push(cur);
        }
        println!("{:?}", ret);

        // ret.iter().flat_map(|id| self.items.remove(id)).collect()
        Some(ret)
    }

    pub fn topo_sort(mut self) -> Vec<TypeId> {
        let mut ret = Vec::new();
        println!("{:?}", self.dep_tree);
        println!("{:?}", self.items);
        let keys = self.dep_tree.keys().cloned().collect::<Vec<_>>();
        for node in keys {
            if !self.visited.contains(&node) {
                self.topo_rec(node, &mut ret);
            }
        }
        ret
    }

    pub fn topo_rec(&mut self, node: TypeId, ret: &mut Vec<TypeId>) {
        let children = self.dep_tree.get(&node).unwrap().clone();
        for id in children {
            if !self.visited.contains(&id) {
                self.topo_rec(id, ret);
            }
        }
        self.visited.insert(node);
        ret.push(node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct A;
    struct B;
    struct C;
    struct D;
    struct E;
    struct F;
    struct G;
    struct H;

    #[test]
    fn simple_tree() {
        let mut tree = DependencyTree::new();
        tree.add(A, &[]);
        tree.add(B, &[TypeId::of::<A>()]);
        tree.add(C, &[TypeId::of::<A>()]);
        tree.add(D, &[TypeId::of::<B>(), TypeId::of::<C>()]);
        let deps = tree.topo_sort();
        println!("{:?}", &deps);
        assert_eq!(
            &deps,
            &[
                TypeId::of::<A>(),
                TypeId::of::<B>(),
                TypeId::of::<C>(),
                TypeId::of::<D>()
            ]
        );
    }

    #[test]
    fn simple_tree_stack() {
        println!("A {:?}", TypeId::of::<A>());
        println!("B {:?}", TypeId::of::<B>());
        println!("C {:?}", TypeId::of::<C>());
        println!("D {:?}", TypeId::of::<D>());
        let mut tree = DependencyTree::new();
        tree.add_child(A, &[]);
        tree.add_child(B, &[TypeId::of::<A>()]);
        tree.add_child(C, &[TypeId::of::<A>()]);
        tree.add_child(D, &[TypeId::of::<B>(), TypeId::of::<C>()]);
        let deps = tree.topological_sort().unwrap();
        println!("{:?}", &deps);
        assert_eq!(
            &deps,
            &[
                TypeId::of::<A>(),
                TypeId::of::<C>(),
                TypeId::of::<B>(),
                TypeId::of::<D>()
            ]
        );
    }

    #[test]
    fn simple_tree_two() {
        let mut tree = DependencyTree::new();
        tree.add(A, &[]);
        tree.add(B, &[TypeId::of::<A>()]);
        tree.add(C, &[TypeId::of::<A>()]);
        tree.add(
            D,
            &[TypeId::of::<B>(), TypeId::of::<C>(), TypeId::of::<E>()],
        );
        tree.add(E, &[TypeId::of::<A>(), TypeId::of::<C>()]);
        let deps = tree.topo_sort();
        println!("{:?}", &deps);
        assert_eq!(
            &deps,
            &[
                TypeId::of::<A>(),
                TypeId::of::<B>(),
                TypeId::of::<C>(),
                TypeId::of::<E>(),
                TypeId::of::<D>(),
            ]
        );
    }

    #[test]
    fn simple_tree_two_insertion_order() {
        let mut tree = DependencyTree::new();
        tree.add(
            D,
            &[TypeId::of::<B>(), TypeId::of::<C>(), TypeId::of::<E>()],
        );
        tree.add(A, &[]);
        tree.add(C, &[TypeId::of::<A>()]);
        tree.add(E, &[TypeId::of::<A>(), TypeId::of::<C>()]);
        tree.add(B, &[TypeId::of::<A>()]);
        let deps = tree.topo_sort();
        println!("{:?}", &deps);
        assert_eq!(
            &deps,
            &[
                TypeId::of::<A>(),
                TypeId::of::<B>(),
                TypeId::of::<C>(),
                TypeId::of::<E>(),
                TypeId::of::<D>(),
            ]
        );
    }
}
