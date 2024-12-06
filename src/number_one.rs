use std::collections::HashMap;

// Given a list of integers,
// use a vector and return the median (when sorted, the value in the middle position)
// and mode (the value that occurs most often; a hash map will be helpful here) of the list.
pub fn median_mode(list: &mut [i32]) -> Vec<f32> {
    list.sort();
    println!("{:?}", list);
    let mid_index = list.len() / 2;
    println!("{mid_index}");
    let mut mid_value: f32 = list[mid_index] as f32;
    if mid_index % 2 == 0 {
        mid_value += list[mid_index - 1] as f32;
        mid_value = mid_value / 2.0;
    }
    println!("Mid: {mid_value}");
    let mut values_hash_map: HashMap<String, i32> = HashMap::new();
    for item in list {
        let check_present = values_hash_map.get(&item.to_string());
        let mut count = 0;
        match check_present {
            Some(value) => {
                count = value.to_owned();
                count += 1;
            }
            None => count = 1,
        }
        values_hash_map.insert(item.to_string(), count);
    }
    println!("{values_hash_map:?}");
    let mut frequency = 0;
    let mut mode: String = String::new();
    for (key, value) in values_hash_map {
        if value >= frequency {
            frequency = value;
            mode = key;
        }
    }
    println!("Mode: {mode}. Freq: {frequency}");
    let mut returned_vector: Vec<f32> = Vec::new();
    returned_vector.push(mid_value);
    returned_vector.push(mode.parse().unwrap());
    returned_vector
}
