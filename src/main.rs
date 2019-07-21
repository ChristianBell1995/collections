fn main() {
    let v = vec![45, 43, 88, 102, 55, 1, 2];
    let mean = mean(&v);
    println!("The mean is: {}", mean);


}

fn mean(vector: &Vec<usize>) -> usize {
    let mut counter = 0;
    for i in vector {
        counter += i
    }
    println!("Counter is: {}", counter);
    println!("Vector length is: {}", vector.len());

    &counter / vector.len()
}
// Given a list of integers, use a vector and return the mean (the average value),
// median (when sorted, the value in the middle position),
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
