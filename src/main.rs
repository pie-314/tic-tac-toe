use std::io;


//this function is jsut responsible for drawing the grid
fn draw(
    state1: &str,
    state2: &str,
    state3: &str,
    state4: &str,
    state5: &str,
    state6: &str,
    state7: &str,
    state8: &str,
    state9: &str){
   println!(" {} | {} | {} \n {} | {} | {} \n {} | {} | {} \n  ",state1, state2 ,state3,state4,state5,state6,state7,state8,state9);
}


// stream lines shit
fn main() {
    println!("Welcome to tic-tac-toe");
    println!("Make a choice");
    let mut state1 = "1";
    let mut state2 = "2";
    let mut state3 = "3";
    let mut state4 = "4";
    let mut state5 = "5";
    let mut state6 = "6";
    let mut state7 = "7";
    let mut state8 = "8";
    let mut state9 = "9";
    draw(&state1, &state2 ,&state3,&state4,&state5,&state6,&state7,&state8,&state9);
    loop{
        let out = player1();
        if (out == "1"){
            state1 = "X";
        } else if (out == "2"){
            state2 = "X"
        } else if (out == "3"){
            state3 = "X"
        } else if (out == "4"){
            state4 = "X"
        } else if (out == "5"){
            state5 = "X"
        } else if (out == "6"){
            state6 = "X"
        } else if (out == "7"){
            state7 = "X"
        } else if (out == "8"){
            state8 = "X"
        } else if (out == "9"){
            state9 = "X"
        } else {
            println!("Enter a valid input ");
        } 
        draw(&state1, &state2 ,&state3,&state4,&state5,&state6,&state7,&state8,&state9);
    }
}

//player1 - takes input from user -> position
fn player1() -> String {
    println!("player 1 : ");
    let mut player1_choice = String::new();
    io::stdin().read_line(&mut player1_choice).expect("failed");
    player1_choice
}

//player2 - takes input from user -> position
fn player2() -> String {
    println!("player 2 : ");
    let mut player2_choice = String::new();
    io::stdin().read_line(&mut player2_choice).expect("failed");
    player2_choice
}
//state : -> returns the state of grid, in hashmap format

//player2 computer minimax function : -> position integer

//decider :-> i32 (0 for player1, 1 for player 2 )

