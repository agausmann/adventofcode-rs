use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::Hash;

struct QueueEntry<K, V>(K, V);

impl<K, V> PartialOrd for QueueEntry<K, V>
where
    K: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<K, V> Ord for QueueEntry<K, V>
where
    K: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<K, V> PartialEq for QueueEntry<K, V>
where
    K: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K, V> Eq for QueueEntry<K, V> where K: Eq {}

pub fn astar<T, N, G, H, P>(
    start: T,
    mut for_neighbors_of: N,
    mut edge_cost: G,
    mut heuristic: H,
    mut is_goal: P,
) -> Option<(u64, Vec<T>)>
where
    T: Clone + Eq + Hash,
    N: FnMut(&T, &mut dyn FnMut(T)),
    G: FnMut(&T, &T) -> u64,
    H: FnMut(&T) -> u64,
    P: FnMut(&T) -> bool,
{
    let mut came_from: HashMap<T, T> = HashMap::new();
    let mut g_scores: HashMap<T, u64> = HashMap::new();
    g_scores.insert(start.clone(), 0);

    // Key needs to have reverse ordering to convert max-heap to min-heap.
    let mut open_set: BinaryHeap<QueueEntry<Reverse<u64>, T>> = BinaryHeap::new();
    open_set.push(QueueEntry(Reverse(heuristic(&start)), start.clone()));
    let mut closed_set: HashSet<T> = HashSet::new();

    while let Some(QueueEntry(Reverse(current_f), current)) = open_set.pop() {
        if closed_set.contains(&current) {
            // Already visited at a lower score -
            continue;
        }
        closed_set.insert(current.clone());

        if is_goal(&current) {
            let mut path = Vec::new();
            let mut node = current;
            while let Some(next) = came_from.remove(&node) {
                path.push(node);
                node = next;
            }
            path.push(node);
            path.reverse();
            return Some((current_f, path));
        }

        for_neighbors_of(&current, &mut |neighbor| {
            let tentative_g = g_scores[&current] + edge_cost(&current, &neighbor);
            let current_g = g_scores.get(&neighbor);
            if current_g.is_none() || tentative_g < *current_g.unwrap() {
                came_from.insert(neighbor.clone(), current.clone());
                g_scores.insert(neighbor.clone(), tentative_g);

                open_set.push(QueueEntry(
                    Reverse(tentative_g + heuristic(&neighbor)),
                    neighbor,
                ));
            }
        })
    }
    None
}
