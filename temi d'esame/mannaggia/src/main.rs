fn main() {
    let mut data = vec![1, 2, 3, 4, 5];
    data.push(60);
    let mut process_data = || {
        data.push(50);
        let count = data.clone().iter().filter(|&x| x % 2 == 0).count();
        println!("Numero: {:?}", count);
    };
    data.push(40);
    process_data();
    data.push(30);
}