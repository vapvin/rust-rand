
// 변수와 가변성 

// fn main() {
//     let x = 5; // 기본적으로 러스트에서의 변수선언은 불변성을 갖는다.
//     println!("{}", x);

//     x = 6;
//     println!("{}", x);
// }

// fn main() {
//     let mut x = 5; // mut 키워드를 통해 변할 수 있게 선언해준다.
//     println!("{}", x);

//     x = 6;
//     println!("{}", x);
// }

// 상수 선언과 불변변수와의 차이

// const 키워드를 통해 선언한다.
// 할당할 값의 타입을 반드시 지정해야한다.
// mut키워드를 사용 할 수 없다. 
// 상수 이름에 대문자만 이용한다.

// const MAX_POINT: i32 = 100;

// Data Types

// 8bit i8 u8
// 16bit i16 u16
// 32bit i32 u32
// 64bit i64 u64
// arch isize usize

// ownerships

// fn main(){ // block 기준으로 소유자를 확인
//     let a = {
//         5
//     }; // 블록이 끝날 시에 메모리 반환
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     first_word("HelloWrodl")
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool
// }

// fn main(){
//     let mut user1 = User {
//         email: String::from("some@example.com"),
//         username: String::from("someuser"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("otheruser@example.com");

// }

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }


    

// 변수와 가변성 

// fn main() {
//     let x = 5; // 기본적으로 러스트에서의 변수선언은 불변성을 갖는다.
//     println!("{}", x);

//     x = 6;
//     println!("{}", x);
// }

// fn main() {
//     let mut x = 5; // mut 키워드를 통해 변할 수 있게 선언해준다.
//     println!("{}", x);

//     x = 6;
//     println!("{}", x);
// }

// 상수 선언과 불변변수와의 차이

// const 키워드를 통해 선언한다.
// 할당할 값의 타입을 반드시 지정해야한다.
// mut키워드를 사용 할 수 없다. 
// 상수 이름에 대문자만 이용한다.

// const MAX_POINT: i32 = 100;

// Data Types

// 8bit i8 u8
// 16bit i16 u16
// 32bit i32 u32
// 64bit i64 u64
// arch isize usize

// ownerships

// fn main(){ // block 기준으로 소유자를 확인
//     let a = {
//         5
//     }; // 블록이 끝날 시에 메모리 반환
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }
//     s.len()
// }

// fn main() {
//     first_word("HelloWrodl")
// }

// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool
// }

// fn main(){
//     let mut user1 = User {
//         email: String::from("some@example.com"),
//         username: String::from("someuser"),
//         active: true,
//         sign_in_count: 1,
//     };

//     user1.email = String::from("otheruser@example.com");

// }

// fn build_user(email: String, username: String) -> User {
//     User{
//         email,
//         username,
//         active: true,
//         sign_in_count: 1,
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }


// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;

fn route(ip_type: IpAddrKind) {}

route(IpAddrKind::V4);
route(IpAddrKind::V6);

struct  IpAddr {
    kind: IpAddrKind,
    address: String
}

let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
}

let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
}

struct Com {
    modelname: String,
    version: String,
}

fn main (){}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn like_boss(){}
    }
}

pub fn eat_at(){
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();
}

pub fn like_that() {
    crate::front_of_house::hosting::like_boss();
}