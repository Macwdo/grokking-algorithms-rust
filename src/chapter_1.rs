pub fn main(args: Vec<String>) {
    let binary_path = &args[0];
    println!("BINARY PATH: {}", binary_path);

    let target: u128 = args[1].parse().unwrap();
    let arr_size: u128 = args[2].parse().unwrap();

    let arr = (1..arr_size + 1).collect::<Vec<_>>();
    let (b_res, b_count) = search::binary_search(target, &arr);
    helpers::display_search_info(b_res, b_count, "Binary Search");

    let (s_res, s_count) = search::simple_search(target, &arr);
    helpers::display_search_info(s_res, s_count, "Simple Search");
}
mod helpers {
    pub fn display_search_info(res: Option<usize>, count: u128, search_type: &str) {
        let i = match res {
            None => "Not Found".to_string(),
            Some(res) => res.to_string(),
        };

        let info_str = format!(
            "ID: {}.\nCOUNT: {}.\nSEARCH TYPE: {}\n",
            i, count, search_type
        );

        println!("{}", info_str);
    }
}

mod search {

    pub fn simple_search(target: u128, array: &[u128]) -> (Option<usize>, u128) {
        let n = array.len() - 1;

        let mut i = 0;
        let mut count = 0;

        while n >= i {
            count += 1;

            let current = array[i];
            if target == current {
                return (Some(i), count);
            }
            i += 1;
        }

        return (None, count);
    }

    pub fn binary_search(target: u128, array: &[u128]) -> (Option<usize>, u128) {
        let n = array.len();

        let mut low = 0;
        let mut high = n - 1;

        let mut count = 0;
        while low <= high {
            count += 1;
            let middle = (low + high) / 2;
            let current = array[middle as usize];

            if target == current {
                return (Some(middle), count);
            }

            if target > current {
                low = middle + 1;
            }

            if target < current {
                high = middle - 1;
            }
        }

        return (None, count);
    }
}
