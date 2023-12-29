use anyhow::Result;
use itertools::Itertools;
use pathfinding::directed::edmonds_karp::*;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphNode {
    name: String,
}

#[derive(Debug, Clone)]
struct UndirectedGraph {
    neighbors: HashMap<String, HashMap<String, usize>>,
    nodes: HashSet<GraphNode>,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let mut graph = UndirectedGraph {
        neighbors: HashMap::new(),
        nodes: HashSet::new(),
    };
    let lines = aoc::parse_list::<String>(input)?;
    lines
        .iter()
        .map(|line| {
            let v = line.split(':').collect_vec();

            let n1 = v[0];

            graph.nodes.insert(GraphNode {
                name: n1.to_string(),
            });

            for n2 in v[1].split_whitespace() {
                graph
                    .neighbors
                    .entry(n1.to_string())
                    .or_insert(HashMap::new())
                    .insert(n2.to_string(), 1);

                graph
                    .neighbors
                    .entry(n2.to_string())
                    .or_insert(HashMap::new())
                    .insert(n1.to_string(), 1);

                graph.nodes.insert(GraphNode {
                    name: n2.to_string(),
                });
            }
        })
        .collect_vec();

    // Map node names to node IDs
    let nodes = &graph.nodes.iter().collect_vec();
    let node_ids: HashMap<String, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (node.name.to_string(), i))
        .collect();

    let vertices = node_ids.values().copied().collect_vec();

    // Need a directed graph with edges in both directions
    let edges: Vec<Edge<usize, i32>> = graph
        .neighbors
        .iter()
        .flat_map(|(left, value)| {
            value
                .keys()
                .map(|right| {
                    (
                        (
                            node_ids.get(left).unwrap().to_owned(),
                            node_ids.get(right).unwrap().to_owned(),
                        ),
                        1,
                    )
                })
                .collect_vec()
        })
        .collect_vec();

    let s = &vertices[0];
    for t in vertices.iter().skip(1) {
        let (_, _, mincut) = edmonds_karp_dense(&vertices, s, t, edges.to_vec());
        // println!("mincut: {mincut:?}");

        if mincut.len() == 3 {
            let mincut = mincut
                .iter()
                .map(|((n1, n2), _)| (nodes[*n1].name.to_string(), nodes[*n2].name.to_string()))
                .collect_vec();

            println!("mincut: {mincut:?}");

            let cut_graph = clip(&graph, &mincut);
            let sizes = get_component_sizes(&cut_graph);
            println!("Has {sizes:?} components");

            if sizes.len() == 2 {
                return Ok(sizes[0] * sizes[1]);
            }
        }
    }

    Ok(0)
}

fn clip(graph: &UndirectedGraph, edges: &[(String, String)]) -> UndirectedGraph {
    let mut graph = graph.clone();

    for edge in edges {
        graph.neighbors.get_mut(&edge.0).unwrap().remove(&edge.1);
        graph.neighbors.get_mut(&edge.1).unwrap().remove(&edge.0);
    }

    graph
}

fn get_component_sizes(graph: &UndirectedGraph) -> Vec<usize> {
    let mut sizes = Vec::new();
    let mut seen_all = HashSet::new();
    for node in &graph.nodes {
        if !seen_all.contains(&node.name) {
            let mut seen_local = HashSet::new();
            dfs(graph, &node.name, &mut seen_local);

            sizes.push(seen_local.len());
            seen_all.extend(seen_local);
        }
    }
    sizes
}

fn dfs(graph: &UndirectedGraph, node: &str, seen: &mut HashSet<String>) {
    let mut queue = VecDeque::new();
    queue.push_back(node.to_string());

    while let Some(node) = queue.pop_back() {
        let neighbors = graph.neighbors.get(&node).unwrap();
        for n in neighbors.keys() {
            if !seen.contains(n) {
                seen.insert(n.to_string());
                queue.push_back(n.to_string());
            }
        }
    }
}

fn tests() -> anyhow::Result<()> {
    let input = r"jqt: rhn xhk nvd
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
frs: qnr lhk lsr
";

    let solution = solve(input)?;

    assert_eq!(solution, 54);

    Ok(())
}
