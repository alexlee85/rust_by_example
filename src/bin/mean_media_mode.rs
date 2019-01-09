use std::collections::HashMap;

fn main() {
    let mut numbers: Vec<i32> = vec![2, 2, 3, 4, 5, 6, 2, 3, 1, 4, 5, 4, 7, 3, 1, 5, 9, 9, 9, 9];
    println!("numbers: {:?}, length is: {}", numbers, numbers.len());
    numbers.sort();
    println!("after sort: {:?}", numbers);

    let mut count_map: HashMap<i32, i32> = HashMap::new();
    let mut sum: f32 = 0.0;
    for i in numbers.iter() {
        *count_map.entry(*i).or_insert(0) += 1;

        sum += *i as f32;
    }
    println!("sum is: {}", sum);
    println!("count map is:{:?}", count_map);

    let mean = sum / numbers.len() as f32;
    println!("mean is: {}", mean);

    let length_is_even: bool = numbers.len() % 2 == 0;
    let middle_position = numbers.len() / 2;
    let media = if length_is_even {
        (numbers[middle_position - 1] + numbers[middle_position]) as f32 / 2.0
    } else {
        numbers[middle_position] as f32
    };
    println!("media is: {}", media);

    let mut present_most_number: Option<(i32, i32)> = None;
    for (k, v) in count_map.into_iter() {
        if present_most_number.is_none() || v > present_most_number.unwrap().1 {
            present_most_number = Some((k, v));
        }
    }
    println!("mode is: {}", present_most_number.unwrap().0)
}
