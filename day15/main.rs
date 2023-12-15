use std::fs::read_to_string;
use std::time::Instant;
fn main() {
    let start = Instant::now();

    let input = read_to_string("./input").unwrap();
    let input: Vec<Vec<u8>> = input.split(',').map(|row| row.bytes().collect()).collect();
    // println!("{:?}", input);

    let mut total: usize = 0;
    for i in &input {
        // println!("{}", hash(&i));
        total += hash(i);
    }
    println!("{}", total);

    let mut boxes: Vec<Vec<(Vec<u8>, u8)>> = vec![vec![]; 256];
    let mut total: usize = 0;
    for op in &input {
        match op.last().unwrap() {
            b'0'..=b'9' => {
                let label = op[..op.len() - 2].to_vec();
                let focal_len = op.last().unwrap();
                let box_idx = hash(&label);
                let mut found = false;
                for lenses in boxes[box_idx].iter_mut() {
                    if lenses.0 == label {
                        lenses.1 = focal_len - b'0';
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes[box_idx].push((label, focal_len - b'0'));
                }
            }
            b'-' => {
                let label = op[..op.len() - 1].to_vec();
                let box_idx = hash(&label);
                let mut del_idx = usize::MAX;
                for (idx, lenses) in boxes[box_idx].iter().enumerate() {
                    if lenses.0 == label {
                        del_idx = idx;
                    }
                }
                if del_idx != usize::MAX {
                    boxes[box_idx].remove(del_idx);
                }
            }
            _ => {}
        }
    }

    for (box_id, content) in boxes.iter().enumerate() {
        for (lens_id, &(_, focal_len)) in content.iter().enumerate() {
            total += (box_id + 1) * (lens_id + 1) * (focal_len as usize);
        }
    }
    println!("{}", total);
    println!("{:?}", start.elapsed());
}

fn hash(v: &[u8]) -> usize {
    let mut value = 0;
    for &val in v {
        value += val as usize;
        value *= 17;
        value %= 256;
    }
    value
}
