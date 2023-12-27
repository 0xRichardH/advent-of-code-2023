#![allow(dead_code)]

use std::{collections::HashMap, fs::File, io::Write};

use anyhow::{anyhow, bail, Result};
use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::graph::UnGraph};

pub fn process_data(input: &str) -> Result<usize> {
    let graph = parse_graph(input);

    let min_cut_edges_result: Result<Option<(i32, Vec<_>)>, usize> =
        stoer_wagner_min_cut(&graph, |_| Ok(1));
    let Some((_, partitions)) =
        min_cut_edges_result.map_err(|_| anyhow!("failed to perform stoer_wagner_min_cut"))?
    else {
        bail!("No min cut found");
    };

    let total_nodes = graph.node_count();
    let a = partitions.len();
    let b = total_nodes - a;

    Ok(a * b)
}

fn parse_graph(input: &str) -> UnGraph<&str, usize> {
    let mut nodes = HashMap::new();
    input
        .trim()
        .lines()
        .fold(UnGraph::new_undirected(), |mut graph, line| {
            let mut parts = line.trim().split(':');
            let current = parts.next().map(|s| s.trim()).unwrap();
            let current_node = *nodes
                .entry(current)
                .or_insert_with(|| graph.add_node(current));
            parts
                .next()
                .map(|s| s.trim())
                .map(|s| {
                    s.split_whitespace()
                        .map(|s| s.trim())
                        .fold(graph, |mut graph, neighbor| {
                            let neighbor_node = *nodes
                                .entry(neighbor)
                                .or_insert_with(|| graph.add_node(neighbor));
                            graph.add_edge(current_node, neighbor_node, 1);
                            graph
                        })
                })
                .unwrap()
        })
}

fn export_graph(graph: &UnGraph<&str, usize>) -> Result<()> {
    use rustworkx_core::petgraph::dot::{Config, Dot};

    let dot_txt = format!("{:?}", Dot::with_config(&graph, &[Config::EdgeNoLabel]));
    let mut file = File::create("graph.dot")?;
    file.write_all(dot_txt.as_bytes())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_data() {
        let input = "jqt: rhn xhk nvd
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
        assert_eq!(process_data(input).unwrap(), 54);
    }
}
