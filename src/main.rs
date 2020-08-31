struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}
fn main(){
    let user1: User = User{
        email: String::from("vins@vins.dev"),
        username: String::from("vins"),
        active: true,
        sign_in_count: 1,
    };
};