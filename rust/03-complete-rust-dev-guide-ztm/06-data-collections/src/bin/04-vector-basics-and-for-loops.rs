struct Test {
    score: i32,
}

fn main() {
    let my_scores = vec![
        Test { score: 99 },
        Test { score: 88 },
        Test { score: 75 },
        Test { score: 32 },
        Test { score: 74 },
    ];

    for test in my_scores {
        println!("Score = {:?}", test.score);
    }
}
