//item 1
#[derive(Debug,PartialEq)]
enum PaymentType {
    DigitalToken,
    Cash
}

//item 2
struct Seller {
    payment_type: PaymentType,
    price: f32,
    balance: f32
}

//item 3
struct Buyer {
    name: String,
    payment_type: PaymentType,
    balance: f32
}

//item 4
struct BuyerGroup {
    members: Vec<Buyer>,
} 
//items 5-7
impl BuyerGroup {
    fn add_member(&mut self, buyer: Buyer) {
        self.members.push(buyer);
    }

    fn find_buyer(&self, payment_type: &PaymentType) -> i32 {
        //loop through the vector of buyers, for each buyer, compare the payment type,
        //if true, return pos index, if false return -1
        let mut index = 0;
        for member in &self.members {
            if member.payment_type == *payment_type {
                println!("matching buyer of payment type {:?} at index {}", payment_type, index);
                return index;
            }
            index = index + 1;
        }
        println!("matching buyer of payment type {:?} does not exist", payment_type);
        return -1;
    }

    //you need &mut self to pass reference to members vector
    fn buy(&mut self, index: i32, seller: &mut Seller) {
        let buyer = &mut self.members[index as usize];
        while buyer.balance > seller.price{
            buyer.balance -= seller.price;
            seller.balance += seller.price;
        }        
    } 
}

fn main() {
    let buyer1 = Buyer {
        name: "John".to_owned(),
        payment_type: PaymentType::DigitalToken,
        balance: 100.00
    };

    let buyer2 = Buyer {
        name: "Sally".to_owned(),
        payment_type: PaymentType::Cash,
        balance: 100.00
    };

    let mut buyer_group = BuyerGroup {
        members: Vec::new()
    };

    buyer_group.add_member(buyer1);
    buyer_group.add_member(buyer2);

    let mut seller1 = Seller {
        payment_type: PaymentType::Cash,
        price: 10.00,
        balance: 0.00
    };

    let buyer_index = buyer_group.find_buyer(&seller1.payment_type);
    if buyer_index >= 0{
        buyer_group.buy(buyer_index, &mut seller1);
    }
    
}
