pub fn main(args: Vec<String>) {
    let size = args[1].parse().unwrap();

    let mut temp = helpers::random_arr(size);
    let arr = temp.as_mut_slice();

    sorts::selection_sort(arr);
}

mod helpers {
    use rand::Rng;

    pub fn random_arr(size: u128) -> Vec<u128> {
        let mut arr = Vec::with_capacity(size as usize);
        let mut rng = rand::rng();

        for _ in 0..size {
            let a = rng.random_range(0..100);
            arr.push(a);
        }
        return arr;
    }

    pub fn show_values(array: &[u128]) {
        let n = array.len();
        let mut str_buffer = "[".to_string();

        for (i, value) in array.iter().enumerate() {
            if i < n - 1 {
                str_buffer += &format!("{}, ", value);
            } else {
                str_buffer += &format!("{}]", value);
            }
        }

        println!("{}", str_buffer);
    }
}

mod sorts {
    use super::helpers;

    pub fn selection_sort(array: &mut [u128]) {
        let n = array.len();

        for i in 0..n {
            let mut min = i;
            for j in i..n {
                if array[min] > array[j] {
                    min = j;
                }
            }
            let temp = array[i];
            array[i] = array[min];
            array[min] = temp;
            helpers::show_values(array);
        }
    }
}
