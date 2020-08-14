use crate::note::Note;
use petgraph::dot::Dot;
use petgraph::graph::UnGraph;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

pub fn run() -> Result<(), Box<dyn Error>> {
    let mut graph = UnGraph::<&str, &str>::new_undirected();
    let mut notes = Vec::new();
    let mut paths_to_nodes = HashMap::new();

    for entry in fs::read_dir(".")? {
        let path = entry?.path();

        match Note::read_from_file(&path) {
            Ok(mut note) => {
                // We don't reference note content here, so there's no reason to keep it in memory
                note.content = String::new();
                notes.push(note);
            }
            Err(_) => continue,
        };
    }

    for note in notes.iter() {
        paths_to_nodes.insert(
            note.path.as_ref().unwrap().canonicalize()?,
            graph.add_node(&note.front_matter.title),
        );
    }

    for note in notes.iter() {
        let node_a = paths_to_nodes
            .get(&note.path.as_ref().unwrap().canonicalize()?)
            .unwrap();
        for path in note.front_matter.links.iter() {
            let node_b = match paths_to_nodes.get(&path.canonicalize()?) {
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
