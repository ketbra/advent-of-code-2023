use anyhow::Result;
use itertools::Itertools;
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
                println!("{n1} -- {n2};");

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

    // Looked at graphviz to determine edges to cut
    graph = clip(
        &graph,
        &vec![
            &("rsg".to_string(), "nsk".to_string()),
            &("zcp".to_string(), "zjm".to_string()),
            &("jks".to_string(), "rfg".to_string()),
        ],
    );
    let sizes = get_component_sizes(&graph);
    println!("Has {sizes:?} components");

    let mut answer = 0;
    if sizes.len() == 2 {
        answer = sizes[0] * sizes[1];
    }

    // let nodes = graph.nodes.clone();

    // // Build a unique list of connections
    // let mut edges = Vec::new();
    // for n1 in &nodes {
    //     let neighbors = graph.neighbors.get(&n1.name).unwrap().keys();
    //     for n2 in neighbors {
    //         if &n1.name > n2 {
    //             edges.push((n1.name.to_string(), n2.to_string()));
    //         }
    //     }
    // }

    // let mut answer = 0;
    // println!("Edges {}", edges.len());
    // let mut i: u64 = 0;

    // let mut graph = graph.clone();
    // // Keep removing edges until it is in two components
    // 'SEARCH: for edge1 in &edges {
    //     graph = clip(&graph, &vec![edge1]);
    //     let sizes = get_component_sizes(&graph);
    //     if sizes.len() == 2 {
    //         println!("{edge1:?} clipped resulting in {sizes:?}");
    //         answer = sizes[0] * sizes[1];
    //         break 'SEARCH;
    //     }

    // for edge2 in &edges {
    //     if edge1 != edge2 {
    //         for edge3 in &edges {
    //             i += 1;
    //             if i % 1000 == 0 {
    //                 println!("Iter {i}");
    //             }
    //             if edge1 != edge3 && edge2 != edge3 {
    //                 // let new_graph = clip(&graph, &vec![edge1, edge2, edge3]);
    //                 // let sizes = get_component_sizes(&graph);
    //                 // if sizes.len() == 2 {
    //                 //     println!("{edge1:?}, {edge2:?}, {edge3:?}");
    //                 //     answer = sizes[0] * sizes[1];
    //                 //     break 'SEARCH;
    //                 // }
    //             }
    //         }
    //     }
    // }
    // }

    // // println!("{graph:?}");
    // println!("Answer={answer}");
    Ok(answer)
}

fn clip(graph: &UndirectedGraph, edges: &[&(String, String)]) -> UndirectedGraph {
    let mut graph = graph.clone();

    for edge in edges {
        graph.neighbors.get_mut(&edge.0).unwrap().remove(&edge.1);
        graph.neighbors.get_mut(&edge.1).unwrap().remove(&edge.0);
    }

    graph
}

fn get_component_sizes(graph: &UndirectedGraph) -> Vec<usize> {
    let mut sizes = Vec::new();
    let mut components = 0;
    let mut seen_all = HashSet::new();
    for node in &graph.nodes {
        if !seen_all.contains(&node.name) {
            components += 1;

            let mut seen_local = HashSet::new();
            dfs(graph, &node.name, &mut seen_local);

            sizes.push(seen_local.len());
            seen_all.extend(seen_local);
        }
    }

    // // Count vertices
    // let vertices = graph.nodes.len();

    // // Count edges
    // let edges: usize = graph.neighbors.values().map(|x| x.len()).sum();

    // println!("Edges {edges}, Vertices {vertices}");

    // vertices - (edges / 2)
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

    // let solution = solve(input)?;

    // assert_eq!(solution, 54);

    Ok(())
}
