use aoc::{Counter, parse_lines_with};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution;

impl aoc::Solution for Solution {
    fn solve_1(&self, input: String) -> String {
        let graph: HashMap<String, Vec<String>> = parse_lines_with(&input, parse_line).collect();
        let mut q = vec![("you".to_string(), 1)];
        let mut counts = Counter::new();
        while let Some((node, n)) = q.pop() {
            counts.count_n(node.clone(), n);
            if let Some(children) = graph.get(&node) {
                for child in children {
                    q.push((child.clone(), n));
                }
            }
        }
        counts.get("out").to_string()
    }

    fn solve_2(&self, input: String) -> String {
        let graph: HashMap<String, Vec<String>> = parse_lines_with(&input, parse_line).collect();
        let mut q = vec!["svr".to_string()];
        let mut incoming: HashMap<String, HashSet<String>> = HashMap::new();
        incoming.insert("svr".to_string(), HashSet::new());

        while let Some(node) = q.pop() {
            if node == "out" {
                continue;
            }
            for child in &graph[&node] {
                let child_incoming = incoming.entry(child.clone()).or_default();
                if !child_incoming.insert(node.clone()) {
                    continue;
                }
                q.push(child.clone());
            }
        }

        log::info!("Finished building incoming graph");

        let svr_to_dac = paths(&graph, &incoming, "svr", "dac");
        let svr_to_fft = paths(&graph, &incoming, "svr", "fft");
        let dac_to_fft = paths(&graph, &incoming, "dac", "fft");
        let fft_to_dac = paths(&graph, &incoming, "fft", "dac");
        let fft_to_out = paths(&graph, &incoming, "fft", "out");
        let dac_to_out = paths(&graph, &incoming, "dac", "out");

        ((svr_to_dac * dac_to_fft * fft_to_out) + (svr_to_fft * fft_to_dac * dac_to_out))
            .to_string()
    }
}

fn paths(
    graph: &HashMap<String, Vec<String>>,
    incoming: &HashMap<String, HashSet<String>>,
    start: &str,
    end: &str,
) -> u64 {
    log::info!("Calculating paths from {start} to {end}");
    let mut queue = VecDeque::new();
    queue.push_back(start.to_string());
    let mut queued = HashSet::new();
    queued.insert(start.to_string());
    let mut paths = HashMap::new();
    paths.insert(start.to_string(), 1);
    while let Some(node) = queue.pop_front() {
        log::debug!("Visiting node {node} queue size: {}", queue.len());

        if node != start {
            let parents = &incoming[&node];

            if parents.is_empty() {
                log::debug!("  No parents for {node}, unreachable");
                paths.insert(node.clone(), 0);
            }

            let unknown_parents: Vec<_> = parents
                .iter()
                .filter(|it| !paths.contains_key(*it))
                .cloned()
                .collect();
            if !unknown_parents.is_empty() {
                log::debug!("  Not all parents known yet, re-queueing. {unknown_parents:?} ");
                for p in unknown_parents {
                    if !queued.contains(&p) {
                        log::debug!("  Queueing parent {p} ");
                        queue.push_back(p.clone());
                        queued.insert(p);
                    }
                }
                queue.push_back(node);
                continue;
            }
            let total_paths = parents.iter().map(|it| paths[it]).sum::<u64>();
            paths.insert(node.clone(), total_paths);
            log::debug!("  Total paths to {node}: {total_paths}");
        }
        if node == "out" {
            continue;
        }
        for child in &graph[&node] {
            log::debug!("  Considering child {child} ");
            let k = child.clone();
            if queued.contains(&k) {
                continue;
            }
            if paths.contains_key(&k) {
                continue;
            }
            log::debug!("  Queueing child {child}");
            queue.push_back(k.clone());
            queued.insert(k);
        }
    }

    let paths = *paths.get(end).unwrap();
    log::info!("Total paths from {start} to {end}: {paths}");
    paths
}

fn parse_line(line: &str) -> (String, Vec<String>) {
    let (name, outputs) = line.split_once(": ").unwrap();
    let outputs = outputs.split_ascii_whitespace().map(String::from).collect();
    (name.to_string(), outputs)
}
