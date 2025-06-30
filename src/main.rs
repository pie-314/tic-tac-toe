use std::io;
use std::collections::HashMap;


//this function is jsut responsible for drawing the grid
fn draw(state: &HashMap<&str,String>){
   println!(" {} | {} | {} \n {} | {} | {} \n {} | {} | {} \n  ",
       state.get("1").unwrap(),
       state.get("2").unwrap(),
       state.get("3").unwrap(),
       state.get("4").unwrap(),
       state.get("5").unwrap(),
       state.get("6").unwrap(),
       state.get("7").unwrap(),
       state.get("8").unwrap(),
       state.get("9").unwrap(),);
}


// stream lines shit
fn main() {
    println!("Welcome to Tic-Tac-Toe");
    println!("Make a choice");
    let mut state = HashMap::from([
        ("1","1".to_string()),
        ("2","2".to_string()),
        ("3","3".to_string()),
        ("4","4".to_string()),
        ("5","5".to_string()),
        ("6","6".to_string()),
        ("7","7".to_string()),
        ("8","8".to_string()),
        ("9","9".to_string()),
    ]);
    draw(&state);
    //game player
    for _i in 0..5{
        loop{
            println!("Player 1 : ");
            let play1 = player();
            match state.get_mut(play1.as_str().trim()) {
                Some(num) => {
                    *num = String::from("X");
                    draw(&state);
                    break
                }
                None => {
                    println!("invalid position");
                    draw(&state);
                }
            }
        }
        decider(&state);
        loop{
            println!("Player 2 : ");
            let play2 = player();
            match state.get_mut(play2.as_str().trim()) {
                Some(num) => {
                    *num = String::from("O");
                    draw(&state);
                    break
                }
                None => {
                    println!("invalid position");
                    draw(&state);
                }
            }
        }
    }
}



//player - takes input from user -> position
fn player() -> String {
    let mut player_choice = String::new();
    io::stdin().read_line(&mut player_choice).expect("failed");
    player_choice
}

//state : -> returns the state of grid, in hashmap format

//player2 computer minimax function : -> position integer

//decider :-> i32 (0 for player1, 1 for player 2 )
fn decider(cur_state: &HashMap<&str, String>)-> i32{
    let wins = vec![
        vec![1,2,3],vec![4,5,6], vec![7,8,9],   //horizontal
        vec![1,4,7],vec![2,5,8], vec![3,6,9],   //vertical
        vec![1,5,9], vec![3,5,7],               //cross
    ];
    let mut values_vec = vec![];
    values_vec.push(cur_state.get("1").unwrap());
    values_vec.push(cur_state.get("2").unwrap());
    values_vec.push(cur_state.get("3").unwrap());
    values_vec.push(cur_state.get("4").unwrap());
    values_vec.push(cur_state.get("5").unwrap());
    values_vec.push(cur_state.get("6").unwrap());
    values_vec.push(cur_state.get("7").unwrap());
    values_vec.push(cur_state.get("8").unwrap());
    values_vec.push(cur_state.get("9").unwrap());
    println!("{:?}",values_vec);
    return 3;
}
