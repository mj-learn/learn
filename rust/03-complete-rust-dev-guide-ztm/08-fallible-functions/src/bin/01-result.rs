#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit
}

fn print_choice(choice: &MenuChoice) {
    println!("choice = {:?}", choice);
}

fn get_choice(choice: &str) -> Result<MenuChoice, String> {
    match choice {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("menu choice not found".to_owned())
    }
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

fn main() {
    let choice = get_choice("start");
    match choice {
        Ok(choice) => print_choice(&choice),
        Err(e) => println!("error = {:?}", e)
    }

    println!();

    let choice = pick_choice("mainmenu");
    println!("choice value = {:?}", choice);

    println!();

    match pick_choice("quit") {
        Ok(_) => (),
        Err(e) => println!("choice 3 value = {:?}", e)
    };
}
