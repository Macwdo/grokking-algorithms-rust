pub fn main(args: Vec<String>) {
    let size = args[1].parse().unwrap();

    let arr = helpers::random_arr(size);
    sorts::selection_sort(1, &arr);
}

mod helpers {
    use rand::Rng;

    pub fn random_arr(size: u128) -> Vec<u128> {
        let mut rng = rand::rng();
        let mut arr = Vec::with_capacity(size as usize);

        for _ in 0..size {
            let a = rng.random_range(0..100);
            arr.push(a);
        }
        return arr;
    }
}

mod sorts {
    pub fn selection_sort(target: u128, array: &[u128]) {
        let n = array.len() - 1;
        for i in 0..n {
            for j in i..n {
                println!("{} {}", i, j)
            }
        }
    }
}
