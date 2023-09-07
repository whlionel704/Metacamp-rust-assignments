// define struct of UserAccount with field: name (String), and age (Option<u32>)
struct UserAccount {
    name: String,
    age: Option<u32>
}
// define a trait called Balance, and within, function get_balance returning integer of 10
trait Balance {
    fn get_balance(&self) -> u32 {
        10
    }
}
// implement trait Balance to UserAccount struct 
// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
// - if increase amount is <= 10, return an OK containing the get_balance + amount
// - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
// Tip: this function should return a Result<u32, String>

impl Balance for UserAccount {
}

fn increase_balance<T: Balance>(x: &T, amount: u32) -> Result<u32,String> {
        if amount <= 10 {
            Ok(x.get_balance() + amount)
        } else {
            Err("Increase must be less than 10!".to_owned())
        }
    }

fn main() {
    // create user_account, and set his age as Option::None
    // You want to increase the user_account's balance by 11
    // use a match, if the result of increase_balance is
    // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
    // - Err: print the error message returned
    // use an if...let...else statement to print the UserAccount age if it is a Option::Some

    let user_account = UserAccount {
        name: "lionel".to_owned(),
        age: Option::None,
    };

    match increase_balance(&user_account, 11){
        Ok(yes) => println!("UserAccount balance increased to {}", yes),
        Err(e) =>  println!("{}", e),
    }

    if let Some(age) = user_account.age {
        println!("UserAccount age is {}", age);
    }
}
   