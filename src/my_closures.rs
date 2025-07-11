pub fn start() {
    let closure = |x| x + 1;
    let result = closure(5);
    println!("Result: {}", result);
    
    let nums: Vec<i32> = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = nums.iter().map(|&x| x * 2).collect();
    println!("----------------------------------");
    println!("Original: {:?}", nums);
    println!("Doubled: {:?}", doubled);
}