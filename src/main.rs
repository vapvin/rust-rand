use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let mut user: String = String::new();

    println!("---------- 숫자 찾기 게임에 오신것을 환영합니다. ----------");
    println!("---------- 이름을 입력해주세요 ----------");

    io::stdin().read_line(&mut user)
    .expect("입력에 실패하였습니다.");

    let input_value: u32 = user_input(user);
    let secret_value: u32 = create_random_number();

    println!("{} {}", input_value, secret_value);

    match input_value.cmp(&secret_value){
        Ordering::Less => println!("입력한 숫자가 정답보다 낮습니다."),
        Ordering::Greater => println!("입력한 숫자가 정답보다 큽니다."),
        Ordering::Equal => println!("입력한 숫자와 정답이 일치합니다."),

    }
}

fn user_input(user: String) -> u32{
    println!("---------- {}님 반갑습니다. ----------", user);
    println!("---------- 랜덤한 숫자가 설정되었습니다. ----------");
    println!("---------- 정답을 입력하여 주세요. ----------");

    let mut guess: String = String::new();
    
    io::stdin().read_line(&mut guess)
    .expect("입력에 실패하였습니다.");

    let guess: u32 = guess.trim().parse().expect("입력값이 잘못되었습니다.");

    println!("입력한 값: {}", guess);
    return guess;
}

fn create_random_number() -> u32{
    let secret_number: u32 = rand::thread_rng().gen_range(1, 101);
    println!("{}", secret_number);
    return secret_number;
}