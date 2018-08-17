use std::collections::HashMap;

trait Graph<V, E> {
    fn bfs_shortest_path(s: V, t: V) -> Vec<V> {
        Vec::new()
    }
}

struct AdjGraph<V, E> where V: std::hash::Hash {
    adj: HashMap<V, HashMap<V, E>>
}

fn main() {
    println!("Hello, world!");
}
