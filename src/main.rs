fn main() {
    let v = vec![45, 43, 88, 102, 55, 1, 2];
    let mean = mean(&v);
    println!("The mean is: {}", mean);

    let mut med_vec = vec![45, 43, 88, 102, 55, 1, 2];
    let median = median(&mut med_vec);
    println!("The median is: {}", median);

    let mode_vec = vec![45, 43, 88, 102, 102, 102, 55, 1, 2, 1];
    let mode = mode(&mode_vec);
    println!("The mode is: {}", mode);
}

// Given a list of integers, use a vector and return the mean (the average value),
fn mean(vector: &Vec<usize>) -> usize {
    let mut counter = 0;
    for i in vector {
        counter += i
    }
    println!("Counter is: {}", counter);
    println!("Vector length is: {}", vector.len());

    &counter / vector.len()
}

// median (when sorted, the value in the middle position),
fn median(vector: &mut Vec<usize>) -> usize {
    vector.sort();
    println!("{:?}", vector);

    let mid_point = vector.len() / 2;
    println!("{:?}", mid_point);
    vector[mid_point]
}

// and mode (the value that occurs most often; a hash map will be helpful here) of the list.

use std::collections::HashMap;

fn mode(vector: &Vec<usize>) -> usize {
    let mut map = HashMap::new();
    for num in vector {
        let count = map.entry(num).or_insert(0);
        *count += 1;
    }

    let mut count_vec: Vec<_> = map.into_iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!("{:?}", count_vec);

    *count_vec[0].0
}
