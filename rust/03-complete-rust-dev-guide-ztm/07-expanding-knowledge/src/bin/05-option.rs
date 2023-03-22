struct Survey {
    q1: Option<i32>,
    q2: Option<bool>,
    q3: Option<String>
}

fn main() {
    let response = Survey {
        q1: None,
        q2: Some(true),
        q3: Some("John Smith".to_owned())
    };

    match response.q1 {
        Some(answer) => println!("Question 1: {:?}", answer),
        None => println!("Question 1: no response")
    }

    match response.q2 {
        Some(answer) => println!("Question 2: {:?}", answer),
        None => println!("Question 2: no response")
    }

    match response.q3 {
        Some(answer) => println!("Question 3: {:?}", answer),
        None => println!("Question 3: no response")
    }
}
