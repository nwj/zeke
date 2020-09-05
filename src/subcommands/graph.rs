use crate::note::Note;
use anyhow::Result;
use path_clean::PathClean;
use petgraph::dot::Dot;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;

pub fn run() -> Result<()> {
    let mut graph = UnGraph::<&str, &str>::new_undirected();
    let mut notes = Vec::new();
    let mut paths_to_nodes = HashMap::new();

    for entry in fs::read_dir(".")? {
        let p = entry?.path();

        if p.is_dir() {
            continue;
        }

        if p.extension().unwrap_or_default() != OsStr::new("md") {
            continue;
        }

        match Note::read_from_file(&p.clean()) {
            Ok(note) => {
                notes.push(note);
            }
            Err(_) => continue,
        };
    }

    for note in notes.iter() {
        paths_to_nodes.insert(
            note.path.as_ref().unwrap(),
            graph.add_node(&note.front_matter.title),
        );
    }

    for note in notes.iter() {
        let node_a = paths_to_nodes.get(&note.path.as_ref().unwrap()).unwrap();

        for path in note.front_matter.links.iter() {
            let node_b = match paths_to_nodes.get(path) {
                Some(node) => node,
                None => continue,
            };
            graph.update_edge(*node_a, *node_b, "");
        }

        for path in note.content.get_note_links().iter() {
            let node_b = match paths_to_nodes.get(path) {
                Some(node) => node,
                None => continue,
            };
            graph.update_edge(*node_a, *node_b, "");
        }
    }

    // render graph to dot file
    println!("{}", Dot::new(&graph));

    Ok(())
}
