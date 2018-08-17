use std::collections::HashMap;
use std::collections::VecDeque;

trait Graph<V, E>
  where V: Clone
{
    fn bfs_shortest_path(&self, s: V, t: V) -> Vec<V>;
    fn count_vertices(&self) -> usize;
    // fn adjacent_vertices<'a>(&'a self, u: V) -> Box<Iterator<Item=V> + 'a>;
}

struct AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq + Clone
{
    adj: HashMap<V, HashMap<V, E>>
}

impl<V, E> Graph<V, E> for AdjGraph<V, E>
  where V: std::hash::Hash + std::cmp::Eq + Clone
{
    fn bfs_shortest_path(&self, s: V, t: V) -> Vec<V> {
        let mut q = VecDeque::new();
        let mut pred: HashMap<V, V> = HashMap::new();
        q.push_back(s.clone());
        while !q.is_empty() {
            let u = q.pop_front().unwrap();
            for v in self.adj.keys() {
                if !pred.contains_key(&v) && *v!=s {
                    pred.insert(v.clone(), u.clone());
                    q.push_back(v.clone());
                }
            }
        }
        let mut inv = Vec::new();
        let mut p = &t;
        while pred.contains_key(&p) {
            inv.push(pred[p].clone());
            p = &pred[p];
        }
        Vec::new()
    }

    fn count_vertices(&self) -> usize {
        self.adj.len()
    }

    // fn adjacent_vertices<'a>(&'a self, u: V) -> Box<Iterator<Item=V> + 'a> {
    //     if self.adj.contains_key(&u) {
    //         let k = self.adj[&u].keys();
    //         Box::new(k)
    //     }
    //     else {
    //         Box::new(std::iter::empty())
    //     }
    // }
}

fn main() {
    println!("Hello, world!");
}
