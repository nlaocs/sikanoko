use rand::seq::SliceRandom;

fn enter_end() {
    let mut input = String::new();
    println!("エンターを入力して終了...");
    std::io::stdin().read_line(&mut input).unwrap();
}

fn main() {
    let mut rng = rand::thread_rng();
    let assets = vec!['し', 'か', 'の', 'こ', 'の', 'こ', 'の', 'こ', 'こ', 'し', 'た', 'ん', 'た', 'ん'];

    let mut count: u32 = 0;
    let start = std::time::Instant::now();
    loop {
        count += 1;
        let elements: Vec<_> = assets.choose_multiple(&mut rng, assets.len()).collect();
        let result: String = elements.iter().copied().collect();
        if count % 5000 == 0 {
            println!("{}, {}", result, count);
        }
        if result == "しかのこのこのここしたんたん" {
            println!("{}, {}", result, count);
            break;
        }
    }

    let end = start.elapsed();
    println!("{}.{:03}秒、{}回で出ました！", end.as_secs(), end.subsec_millis(), count);
    enter_end();
}
