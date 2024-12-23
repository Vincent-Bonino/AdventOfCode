use hashbrown::{HashMap, HashSet};
use itertools::Itertools;

use crate::day23::parse::CAPACITY;

pub fn solve_part_one(graph: &HashMap<String, Vec<String>>) -> usize {
    let mut result: HashSet<Vec<&str>> = HashSet::new();

    for (node, neighbours1) in graph.iter() {
        for neighbour1 in neighbours1 {
            for neighbour2 in graph[neighbour1].iter() {
                if graph[neighbour2].contains(node) {
                    let mut subresult: Vec<&str> = vec![node, neighbour1, &neighbour2];
                    subresult.sort();
                    result.insert(subresult);
                }
            }
        }
    }

    result
        .iter()
        .filter(|computer_trio| {
            computer_trio
                .iter()
                .any(|computer| computer.starts_with("t"))
        })
        .count()
}

pub fn solve_part_two(graph: &HashMap<String, Vec<String>>) {
    let mut hashset_graph: HashMap<String, HashSet<String>> = HashMap::new();

    for (key, value) in graph {
        hashset_graph.insert(key.to_string(), HashSet::from_iter(value.iter().cloned()));
    }

    let biggest_clique: HashSet<String> = bron_kerbosch_alg(&hashset_graph);
    let password: String = biggest_clique.iter().sorted().join(",");

    println!("LAN password:");
    println!("{password}");
}

pub fn bron_kerbosch_alg(graph: &HashMap<String, HashSet<String>>) -> HashSet<String> {
    bron_kerbosch_alg_rec(
        graph,
        HashSet::new(),
        graph.keys().cloned().collect(),
        HashSet::new(),
    )
    .expect("No result")
}

/// Implementation of the Bron–Kerbosch algorithm.
///
/// Implementation with pivoting.
/// See https://en.wikipedia.org/wiki/Bron%E2%80%93Kerbosch_algorithm
fn bron_kerbosch_alg_rec(
    graph: &HashMap<String, HashSet<String>>,
    r: HashSet<String>,
    p: HashSet<String>,
    x: HashSet<String>,
) -> Option<HashSet<String>> {
    // Recursion limit
    if p.is_empty() && x.is_empty() {
        return Some(r);
    }

    if p.is_empty() {
        return None;
    }

    let mut p: HashSet<String> = p.clone();
    let mut x: HashSet<String> = x.clone();

    let pivot: &String = p.iter().find_or_first(|_| false)?;
    let pivot_neighbours: &HashSet<String> = graph.get(pivot)?;

    let tmp = p.clone();
    let candidates = tmp.difference(pivot_neighbours).cloned();

    let mut result: Option<HashSet<String>> = None;

    for vertex in candidates.into_iter() {
        let rec_r: HashSet<String> = r.union(&HashSet::from([vertex.clone()])).cloned().collect();
        let rec_p: HashSet<String> = p.intersection(&graph[&vertex]).cloned().collect();
        let rec_x: HashSet<String> = x.intersection(&graph[&vertex]).cloned().collect();

        if let Some(res) = bron_kerbosch_alg_rec(graph, rec_r, rec_p, rec_x) {
            match &result {
                None => result = Some(res),
                Some(cur_res) => {
                    if res.len() > cur_res.len() {
                        result = Some(res)
                    }
                }
            }
        }

        p.remove(&vertex);
        x.insert(vertex);
    }

    result
}
