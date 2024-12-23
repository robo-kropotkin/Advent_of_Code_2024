pub mod solution {
    use crate::utils::read_lines;

    pub fn total_distance() {
        let data_address = "data/day1.txt";
        let mut distance :i32 = 0;
        let (mut list1, mut list2) = create_lists(&data_address);
        list1.sort();
        list2.sort();
        for i in 0..list1.len() {
            distance += (list1[i] - list2[i]).abs();
        }

        println!("{distance}");
    }

    pub fn similarity_score() {
        use std::collections::HashMap;
        let data_address = "data/day1.txt";
        let mut score_map = HashMap::new();
        let mut score: i32 = 0;
        let (list1, list2) = create_lists(&data_address);
        for element in list1 {
            score_map.insert(element, 0);
        }
        for element in list2 {
            score_map.entry(element).and_modify(|e|{*e += 1});
        }
        for (key, element) in score_map {
            score += key * element;
        }
        println!("Similarity score is {score}");
    }

    fn create_lists(filename: &str) -> ([i32; 1000], [i32; 1000]) {
        let mut list1: [i32; 1000] = [0; 1000];
        let mut list2: [i32; 1000] = [0; 1000];
        let mut i = 0;
    
        let lines = read_lines(&filename).expect("Couldn't read file!");
        for line in lines {
            if let Ok(line) = line {
                if line.len() >= 13 {
                    let n1: i32 = line[..5].trim().parse().unwrap_or(0);
                    let n2: i32 = line[8..13].trim().parse().unwrap_or(0);
                    list1[i] = n1;
                    list2[i] = n2;
                    i += 1;
                }
            }
        }
        (list1, list2)
    }
}