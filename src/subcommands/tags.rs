use crate::fs::{read_dir, read_note};
use anyhow::Result;
use rayon::iter::Either;
use rayon::prelude::*;

pub fn run() -> Result<i32> {
    let (unflattened_tags, errs): (Vec<_>, Vec<_>) =
        read_dir("./")
            .par_bridge()
            .partition_map(|en| match read_note(en.path()) {
                Ok(n) => Either::Left(n.front_matter.tags),
                Err(e) => Either::Right(e),
            });

    let mut tags: Vec<_> = unflattened_tags.into_par_iter().flatten().collect();
    tags.par_sort_unstable();
    tags.dedup();

    let err_count = errs.len();
    for e in errs {
        eprintln!("{:?}", e);
    }

    for t in tags {
        println!("{}", t);
    }

    Ok(if err_count > 0 { 1 } else { 0 })
}
