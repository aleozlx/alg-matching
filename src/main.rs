use std::collections::HashMap;
use std::collections::VecDeque;

trait Graph<V, E> {
    fn bfs_shortest_path<'g>(&'g self, s: &'g V, t: &'g V) -> Vec<&'g V>;
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
    fn bfs_shortest_path<'g>(&'g self, s: &'g V, t: &'g V) -> Vec<&'g V> {
        let mut q = VecDeque::new();
        let mut pred: HashMap<&V, &V> = HashMap::new();
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
        let mut inv = vec![t];
        let mut p = t;
        while let Some(&pre) = pred.get(p) {
            inv.push(pre);
            p = pre;
        }
        inv.reverse();
        return inv;
    }

    fn count_vertices(&self) -> usize {
        self.adj.len()
    }
}

fn main() {
    let mut g: AdjGraph<i32, ()> = AdjGraph { adj: HashMap::new() };
    g.adj.insert(0, [3, 2].iter().cloned().map(|v| (v, ())).collect::<HashMap<_, _>>());
    g.adj.insert(2, [1].iter().cloned().map(|v| (v, ())).collect::<HashMap<_, _>>());
    g.adj.insert(3, [4].iter().cloned().map(|v| (v, ())).collect::<HashMap<_, _>>());
    g.adj.insert(4, [1].iter().cloned().map(|v| (v, ())).collect::<HashMap<_, _>>());
    let p = g.bfs_shortest_path(&0, &1);
    for i in p {
        println!("{}", i);
    }
}
