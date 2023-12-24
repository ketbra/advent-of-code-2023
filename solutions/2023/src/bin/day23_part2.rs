use anyhow::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq)]
struct GraphHike {
    location: String,
    length: usize,
    seen: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct GraphNode {
    name: String,
}

#[derive(Debug, Clone)]
struct UndirectedGraph {
    neighbors: HashMap<String, HashMap<String, usize>>,
    nodes: HashSet<GraphNode>,
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Pos {
    row: usize,
    col: usize,
}

#[aoc::main]
fn solve(input: &str) -> Result<usize> {
    let map = aoc::parse_list::<String>(input)?
        .iter()
        .map(|x| x.chars().collect_vec())
        .to_owned()
        .collect_vec();

    let height = map.len();
    let width = map[0].len();

    let start_node = "n1_0".to_string();
    let end_node = format!("n{}_{}", width - 2, height - 1);

    println!("Traversing from {start_node} to {end_node}");

    let graph = map_to_graph(&map);
    let graph = simplify_graph(
        &graph,
        vec![start_node.to_string(), end_node.to_string()]
            .into_iter()
            .collect(),
    );

    let answer = find_longest_hike_graph(&graph, &start_node, &end_node);
    Ok(answer)
}

fn simplify_graph(graph: &UndirectedGraph, ignore_nodes: HashSet<String>) -> UndirectedGraph {
    let mut graph = graph.clone();
    loop {
        let mut changes = 0;
        let names = graph
            .nodes
            .iter()
            .map(|node| node.name.to_string())
            .collect_vec();
        for name in names {
            if let Some(neighbors) = graph.neighbors.get(&name).cloned() {
                // let neighbors = neighbors.clone();
                if neighbors.len() == 2 && !ignore_nodes.contains(&name) {
                    if let Some(((n1, c1), (n2, c2))) = neighbors.iter().collect_tuple() {
                        if !ignore_nodes.contains(n1) && !ignore_nodes.contains(n2) {
                            graph.neighbors.get_mut(n1).map(|m| {
                                m.insert(n2.to_string(), c1 + c2);
                                m.remove(&name);
                            });
                            graph.neighbors.get_mut(n2).map(|m| {
                                m.insert(n1.to_string(), c1 + c2);
                                m.remove(&name);
                            });

                            graph.neighbors.remove(&name);
                            graph.nodes.remove(&GraphNode { name });

                            changes += 1;
                        }
                    } else {
                        panic!("There are two neighbors, but couldn't deconstruct");
                    }
                }

                // if graph.neighbors.contains_key(&name) {
                //     if let Some(
                //     if (scalar(keys %{$phNode->{'neighbors'}}) == 2) {
                //         # Remove node
                //         my ($n1, $n2) = keys %{$phNode->{'neighbors'}};

                //         delete $hNodes{$sNodeName};
                //         delete $hNodes{$n1}->{'neighbors'}->{$sNodeName};
                //         delete $hNodes{$n2}->{'neighbors'}->{$sNodeName};
                //         $hNodes{$n1}->{'neighbors'}->{$n2} = 1;
                //         $hNodes{$n2}->{'neighbors'}->{$n1} = 1;
                //         $iChanges++;
                //     }

                // for update in updates {
                //     graph
                //         .neighbors
                //         .get_mut(&update.0)
                //         .unwrap()
                //         .insert(update.1.to_string(), update.2.clone());
                //     // // let neighbors = graph.neighbors.get("name").unwrap();
                //     // let mut x = graph.neighbors.get("Bar").unwrap();
                //     // x.insert("foo".to_string(), 10);
                // }
            }
        }

        if changes == 0 {
            break;
        }
    }

    graph
}

fn map_to_graph(map: &[Vec<char>]) -> UndirectedGraph {
    let mut graph = UndirectedGraph {
        neighbors: HashMap::new(),
        nodes: HashSet::new(),
    };

    for (j, row) in map.iter().enumerate() {
        for (i, c) in row.iter().enumerate() {
            if *c != '#' {
                let node_name = format!("n{i}_{j}");
                graph.nodes.insert(GraphNode {
                    name: node_name.to_string(),
                });
                graph
                    .neighbors
                    .insert(node_name.to_string(), HashMap::new());

                let mut neighbor_map = HashMap::new();
                let neighbors = neighbors(map, Pos { row: j, col: i });
                for neighbor in neighbors {
                    neighbor_map.insert(format!("n{}_{}", neighbor.col, neighbor.row), 1);
                }
                graph.neighbors.insert(node_name.to_string(), neighbor_map);
            }
        }
    }

    graph
}

fn neighbors(map: &[Vec<char>], pos: Pos) -> Vec<Pos> {
    let mut neighbors = Vec::new();
    let height = map.len();
    let width = map[0].len();

    let possible_neighbors = [[0, 1], [0, -1], [1, 0], [-1, 0]];

    for [di, dj] in possible_neighbors {
        let (new_row, new_col) = (pos.row as isize + dj, pos.col as isize + di);
        if new_row >= 0 && new_col >= 0 {
            let new_row = new_row as usize;
            let new_col = new_col as usize;
            let new_pos = Pos {
                col: new_col,
                row: new_row,
            };
            if new_row < height && new_col < width && map[new_row][new_col] != '#' {
                neighbors.push(new_pos);
            }
        }
    }

    neighbors
}

fn find_longest_hike_graph(graph: &UndirectedGraph, start: &str, end: &str) -> usize {
    // Create a queue of hikes to consider and add one at the starting
    // position
    let mut longest = 0;
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    seen.insert(start.to_string());
    queue.push_back(GraphHike {
        seen,
        length: 0,
        location: start.to_string(),
    });

    while let Some(hike) = queue.pop_back() {
        if let Some(m) = graph.neighbors.get(&hike.location) {
            m.iter().for_each(|(neighbor, cost)| {
                if !hike.seen.contains(neighbor) {
                    let mut new_hike = hike.clone();
                    new_hike.location = neighbor.to_string();
                    new_hike.length += cost;
                    new_hike.seen.insert(neighbor.to_string());

                    if neighbor == end {
                        if new_hike.length > longest {
                            longest = new_hike.length;
                        }
                    } else {
                        queue.push_back(new_hike);
                    }
                }
            });
        }
    }

    longest
}

fn tests() -> anyhow::Result<()> {
    let input = r"#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    let solution = solve(input)?;

    assert_eq!(solution, 154);

    Ok(())
}
