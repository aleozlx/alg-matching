use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

trait Graph<V, E> {
    fn bfs_shortest_path<'g>(&'g self, s: &'g V, t: &'g V) -> Option<Vec<&'g V>>;
    fn count_vertices(&self) -> usize;
}

struct AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq
{
    adj: HashMap<V, HashMap<V, E>>
}

impl<V, E> Graph<V, E> for AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq
{
    fn bfs_shortest_path<'g>(&'g self, s: &'g V, t: &'g V) -> Option<Vec<&'g V>> {
        if !self.adj.contains_key(s) { return None; }
        if s == t { return Some(vec![s]); }
        let mut q = VecDeque::new();
        let mut pred = HashMap::new();
        q.push_back(s);
        while let Some(u) = q.pop_front() {
            if let Some(adj) = self.adj.get(u) {
                for v in adj.keys() {
                    if !pred.contains_key(v) && v!=s {
                        pred.insert(v, u);
                        q.push_back(v);
                    }
                }
            }
        }
        let mut ret = std::collections::LinkedList::new();
        ret.push_front(*pred.get(t)?);
        while let Some(pre) = pred.get(ret.front().unwrap()) {
            ret.push_front(pre);
        }
        ret.push_back(t);
        Some(Vec::from_iter(ret))
    }

    fn count_vertices(&self) -> usize {
        self.adj.len()
    }
}

macro_rules! unweighted_graph(
    { $($u:expr => [ $($v:expr),+ ]),+ } => {
        {
            let mut g = AdjGraph { adj: HashMap::new() };
            $(
                g.adj.insert($u, [$($v),+].iter().cloned().map(|v| (v, ())).collect());
            )+
            g
        }
    };
);

#[test]
fn test_empty_graph() {
    let g: AdjGraph<i32, ()> = AdjGraph { adj: HashMap::new() };
    assert_eq!(g.bfs_shortest_path(&0, &0), None);
}

#[test]
fn test_no_path() {
    let g = unweighted_graph! {
        0 => [3, 2],
        4 => [1]
    };
    assert_eq!(g.bfs_shortest_path(&0, &1), None);
}

#[test]
fn test_trivial_path1() {
    let g = unweighted_graph! {
        0 => [0]
    };
    assert_eq!(g.bfs_shortest_path(&0, &0), Some(vec![&0]));
}

#[test]
fn test_trivial_path2() {
    let g = unweighted_graph! {
        0 => [1]
    };
    assert_eq!(g.bfs_shortest_path(&0, &0), Some(vec![&0]));
}

#[test]
fn test_trivial_path3() {
    let g = unweighted_graph! {
        0 => [1]
    };
    assert_eq!(g.bfs_shortest_path(&0, &1), Some(vec![0, 1].iter().collect()));
}

#[test]
fn test_unique_path() {
    let g = unweighted_graph! {
        0 => [3, 2],
        3 => [4],
        4 => [1]
    };
    assert_eq!(g.bfs_shortest_path(&0, &1), Some(vec![0, 3, 4, 1].iter().collect()));
}

#[test]
fn test_optimal_path() {
    let g = unweighted_graph! {
        0 => [3, 2],
        2 => [1],
        3 => [4],
        4 => [1]
    };
    assert_eq!(g.bfs_shortest_path(&0, &1), Some(vec![0, 2, 1].iter().collect()));
}

fn main() {
    let g = unweighted_graph! {
        0 => [3, 2],
        2 => [1],
        3 => [4],
        4 => [1]
    };
    if let Some(path) = g.bfs_shortest_path(&0, &1){
        println!("{}", path.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("->"));
    }
    else {
        println!("No path.");
    }
}
