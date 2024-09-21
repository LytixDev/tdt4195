use std::fs::File;
use std::io::{BufRead, BufReader, Error};

// Read file in its entirty
// Parse line by line
// Based on first char, act
// Two vectors, vertices and indices

pub fn load_obj(file_path: &str) -> Result<(Vec<f32>, Vec<u32>), Error> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    
    let mut vertices: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let items = line.split_whitespace().collect::<Vec<&str>>();
        if items.len() == 0 {
            continue;
        }

        match items[0] {
            "#" => continue,
            "v" => {
                for item in items.iter().skip(1) {
                    vertices.push(item.parse::<f32>().unwrap());
                }
            }
            "f" => {
                for item in items.iter().skip(1) {
                    let vertex_index = item.split('/').next().unwrap_or("");
                    // -1 because obj indices are 1 based
                    if !vertex_index.is_empty() {
                        indices.push(vertex_index.parse::<u32>().unwrap() - 1);
                    }
                }
            }
            _ => println!("Unrecognized start of line: {}", items[0]),
        }
    }

    Ok((vertices, indices))
}
