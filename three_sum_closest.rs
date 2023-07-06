fn three_sum_closest(arr: &[i32], target: i32) -> (i32, i32, i32, i32) {
    let n = arr.len();
    let mut min_combination: (i32, i32, i32, i32) = (-1, -1, -1, -1);

    if n < 3 {
        return min_combination;
    }

    let mut best_combo: i32= i32::MAX;
    let mut sum: i32 = 0;

    for i in 0..n {
        for j in (i + 1)..n {
            for k in (j + 1)..n {
            
                sum = arr[i] + arr[j] + arr[k];

                let diff = (target - sum).abs();

                if diff <= best_combo {
                
                    best_combo = diff;
                    
                    min_combination = (arr[i], arr[j], arr[k], best_combo);

                }
            }
        }
    }

    return min_combination;
}

fn main() {
    //let numbers = [7, 5, 12, 6, 8, 14, 3, 17, 9];
    let numbers = [-1,2,1,-4];
    let target = 1;

    let min_combination = three_sum_closest(&numbers, target);
    
    println!("-->{} {} {} {}", 
            min_combination.0, min_combination.1, 
            min_combination.2, min_combination.3);
}