use std::collections::HashMap;
use std::collections::VecDeque;
use std::iter::FromIterator;

trait Graph<V, E> {
    fn bfs_shortest_path<'g>(&'g self, s: &'g V, t: &'g V) -> Option<Vec<&'g V>>;
    fn count_vertices(&self) -> usize;
}

trait FlowEdge<F> {
    fn capacity(&self) -> F;
    fn flow(&self) -> F;
}

trait Flow<V, E, F>
  where E: FlowEdge<F>
{
    fn inner_graph<'g>(&'g self) -> &'g Graph<V, E>;
    fn edmonds_karp_maxflow<'g>(&'g self, s: &'g V, t: &'g V) -> Self;
}

struct AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq
{
    adj: HashMap<V, HashMap<V, E>>
}

impl<V, E> AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq
{
    fn new() -> AdjGraph<V, E> {
        AdjGraph { adj: HashMap::new() }
    }
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

struct FlowEdgeI32 {
    cap: i32,
    flow: i32
}

impl FlowEdge<i32> for FlowEdgeI32 {
    fn capacity(&self) -> i32 {
        return self.cap;
    }

    fn flow(&self) -> i32 {
        return self.flow;
    }
}

struct AdjFlowI32<V, E>
  where V: std::hash::Hash + std::cmp::Eq, E: FlowEdge<i32>
{
    g: AdjGraph<V, E>
}

#[test]
fn test_empty_graph() {
    let g: AdjGraph<i32, ()> = AdjGraph::new();
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
    // path.iter().map(|v| v.to_string()).collect::<Vec<String>>().join("->")
}

fn main() {
    let f: AdjFlowI32<i32, FlowEdgeI32> = AdjFlowI32{ g: AdjGraph::new() };
}
