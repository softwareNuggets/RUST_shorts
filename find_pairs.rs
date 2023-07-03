fn find_pairs(arr: &[i32], target: i32) -> Vec<(i32, i32)> {

    let n = arr.len();
    let mut pairs = Vec::new();
    
    if n < 2 {
        return pairs; 
    }
    
    for i in 0..n {
        for j in (i + 1)..n {
            if arr[i] + arr[j] == target {
                pairs.push((arr[i], arr[j]));
            }
        }
    }
    
    pairs
}

fn main() {

    let numbers = [1, 5, 2, 6, 8];
    let target = 7;
    
    let pairs = find_pairs(&numbers, target);
    
    if pairs.is_empty() {
        println!("No pairs found.");
    } else {
        println!("Pairs found:");
        for (num1, num2) in pairs {
            println!("{} and {}", num1, num2);
        }
    }
}