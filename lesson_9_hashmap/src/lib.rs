pub mod hash_learner {
    pub fn create_hash_map() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();

        scores.insert(String::from("tom"), 60);
        scores.insert(String::from("jerry"), 80);

        for key in scores.keys() {
            println!("key -> {}", key);

            match scores.get(key) {
                Some(value) => {
                    println!("value -> {}", value);
                }
                None => {
                    println!("None");
                }
            }
        }

        for key in scores.into_iter() {
            let k = key.0;
            let v = key.1;

            println!("k -> {}, v -> {}", k, v);
        }
    }

    pub fn build_with_collect() {
        use std::collections::HashMap;

        let students = vec![String::from("Tom"), String::from("Jerry")];
        let initial_scores = vec![90, 80];

        let score_map: HashMap<_, _> = students
            .into_iter()
            .zip(initial_scores.into_iter())
            .collect();

        for key in score_map.into_iter() {
            let k = key.0;
            let v = key.1;

            println!("k -> {}, v -> {}", k, v);
        }
    }

    pub fn ownership() {
        use std::collections::HashMap;

        let field_name = String::from("Favorite color");
        let field_value = String::from("Blue");
        let mut map = HashMap::new();
        map.insert(&field_name, field_value);

        let item = field_name;
    }

    pub fn read() {
        use std::collections::HashMap;

        let mut scores = HashMap::new();
        let tom = String::from("tom");
        let jerry = String::from("jerry");

        scores.insert(&tom, 60);
        scores.insert(&jerry, 80);

        let tom_score = scores.get(&tom);
        let jerry_score = scores.get(&jerry);

        println!("tom -> {:?}, jerry -> {:?}", tom_score, jerry_score);

        for (key, value) in &scores {
            println!("key -> {}, value -> {}", key, value);
        }
    }

    pub fn update() {
        use std::collections::HashMap;
        let tom = String::from("Tom");
        let jerry = String::from("jerry");
        let timi = String::from("Timi");
        let mut scores = HashMap::new();

        scores.insert(&tom, 50);
        scores.insert(&tom, 60);

        println!("overwrite -> {:#?}", scores);

        scores.insert(&jerry, 60);
        scores.entry(&jerry).or_insert(50);
        scores.entry(&timi).or_insert(80);

        println!("overwrite -> {:#?}", scores);

        // let text = "hello world wonderful world";
        // let mut map = HashMap::new();
        // for word in text.split_whitespace() {
        //     let count = map.entry(word).or_insert(0);
        //     *count += 1;
        // }
        // println!("{:?}", map);
    }
}
