// list of items, with a type associated
enum Payment {
    Cash(f32),
    CreditCard(String, f32),
    DebitCard(DebitData),
}

struct DebitData {
    pub card_number: u64,
    pub card_holder: String,
    pub amount: f32,
}

fn main() {
    let cash_payment = Payment::Cash(29.99);
    process_payment(cash_payment);

    let cc_payment = Payment::CreditCard("CC_payment".to_string(), 33.45);
    process_payment(cc_payment);

    let debit_payment = Payment::DebitCard(DebitData {
        card_number: 3456,
        card_holder: "Mr. John Doe".to_string(),
        amount: 39.99,
    });
    process_payment(debit_payment);
}

fn process_payment(some_payment: Payment) {
    match some_payment {
        Payment::Cash(amt) => {
            println!("Paying with cash : {}", amt);
        }
        Payment::CreditCard(_, some_amount) => {
            println!("Paying with credit card : {}", some_amount);
        }
        Payment::DebitCard(data) => {
            println!(
                "Paying with debit card : number: {}, holder: {}, amount: {}",
                data.card_number, data.card_holder, data.amount
            );
        }
    }
}
