//Item 1
struct User {
    name: String,
    balance: (f32, String)
}

//Item 2
impl User {
    fn print_user_detail(&self) {
        println!("name: {}, balance: {}, currency: {}", self.name, self.balance.0, self.balance.1);
    }
}

//Item 3
fn accure_interest(user: &mut User, interest: f32, period: i32){
    
    //user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
    
    if period > 1 {
        for _ in 1..period{
            user.balance.0 = user.balance.0 * (1.0 + user.balance.0 * interest / 100.0);
        }
        
    }

    user.print_user_detail();
}



//Item 4
fn main() {
    let mut user = User{
        name: "Lionel".to_owned(),
        balance: (25.00, "USD".to_owned())
    };
    User::print_user_detail(&user);
    accure_interest(&mut user, 10.0, 1);

    //Item 5 (bonus)
    //To call accure_interest a 2nd time, we need to pass user by reference &
    accure_interest(&mut user, 10.0, 5);

}
