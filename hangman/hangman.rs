//
// Wesley Shaw
// hangman yaarrr, 2 player or single player game
// guess the letters in a secret word or get a short drop and a sudden stop, yarrrr
//
use std::io;
use std::{thread, time};


fn main(){

    let mut user_name = String::new();
    let mut magic_word = String::new();

    println!("\nEnter your name: ");
    io::stdin().read_line(&mut user_name)
        .expect("Failed to read line");

    print!("\nWell well well, you are a brave one, {}",user_name);
    sleep();

    println!("Let's get started...\n");
    sleep();

    println!("\n**the magic word is**: ");
    io::stdin().read_line(&mut magic_word)
        .expect("Failed to read line");

    play_game(&magic_word);

}
//
//function sleeps program for 2500 miliseconds (1 second)
//
fn sleep(){
    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();
    thread::sleep(ten_millis);
    assert!(now.elapsed() >= ten_millis);
}
//
//2 player version
//Takes in a word type string to be secret hangman word.
//
fn play_game(magic_word : &str){
    //char_count, type int, is number of chars in magic word
    //count() is macro that returns how many chars
    let mut game_over : bool = false;
    //note: something wrong here, had to -2 to make it loop correct amount of times. FIX!
    let mut char_count =  magic_word.chars().count();
    char_count -= 2;

    loop{
        print!("\n\n");
        //vector is a growable array
        //It’s also important to note that you must index with the usize type
        let mut vector = vec!["_"; char_count];
        let i : usize = 0;
        for i in &mut vector {
            print!("{} ", i);

            game_over = true;
        }
        print_hangman();

        if game_over == true{
            break;
        }
    }//end of loop

}

fn print_hangman(){
    print!("\n\n");
    println!("______________________");
    println!("| |                 |");
    println!("| |                 O");
    println!("| |               --|--");
    println!("| |                 |");
    println!("| |                / \\ ");
    println!("| |_______________________");




}
