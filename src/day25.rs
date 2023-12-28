use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

use rustworkx_core::petgraph::graph::{NodeIndex, UnGraph};
use rustworkx_core::connectivity::stoer_wagner_min_cut;
use rustworkx_core::Result;

fn get_node_index_or_insert(graph: &mut UnGraph<String, ()>, name: &String) -> NodeIndex {
    if let Some(idx) = graph.node_indices().find(|i| graph[*i] == *name) {
        return idx;
    }
    graph.add_node(name.clone())
}

#[aoc_generator(day25)]
pub fn parse_input(input: &str) -> UnGraph<String, ()> {
    let mut graph = UnGraph::new_undirected();
    for s in input.lines() {
        let mut parts = s.split(": ");
        let left = parts.next().unwrap().to_string();
        let right = parts.next().unwrap().split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>();
        let i1 = get_node_index_or_insert(&mut graph, &left);
        for v in right.iter() {
            let i2 = get_node_index_or_insert(&mut graph, v);
            graph.add_edge(i1, i2, ());
        }
    }
    graph
}

#[aoc(day25, part1)]
pub fn multiply_group_sizes(input: &UnGraph<String, ()>) -> u64 {
    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&input, |_| Ok(1));
    let (min_cut, partition) = min_cut_res.unwrap().unwrap();
    let group1 = partition.len();
    let group2 = input.node_count() - group1;
    assert!(min_cut == 3);
    (group1 * group2) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY25_EXAMPLE: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr";

    #[test]
    fn test_day25_part1() {
        let input = parse_input(DAY25_EXAMPLE);
        assert_eq!(multiply_group_sizes(&input), 54);
    }

}
