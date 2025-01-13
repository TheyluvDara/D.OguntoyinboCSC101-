#[derive(Debug)]
struct Laptops {
    dell: i32,
    hp: i32,
    toshiba: i32,
    ibm: i32,
}

impl Laptops {
    fn total_cost(&self) -> i32 {
        let dell_cost = self.dell * 850_000;
        let hp_cost = self.hp * 650_000;
        let toshiba_cost = self.toshiba * 550_000;
        let ibm_cost = self.ibm * 755_000;

        dell_cost + hp_cost + toshiba_cost + ibm_cost
    }

    fn price_breakdown(&self) -> String {
        let dell_cost = format!("Dell: N{}", self.dell * 850_000);
        let hp_cost = format!("HP: N{}", self.hp * 650_000);
        let toshiba_cost = format!("Toshiba: N{}", self.toshiba * 550_000);
        let ibm_cost = format!("IBM: N{}", self.ibm * 755_000);

        format!("{} \n{} \n{} \n{}", dell_cost, hp_cost, toshiba_cost, ibm_cost)
    }
}

fn main() {

    println!("Laptop devices available:");
    println!();

    let laptops_in_stock = vec!["10 HP at N650,000 each", "6 IBM at N755,000 each", "10 TOSHIBA at N550,000 each", "4 DELL at N850,000 each"];

    for laptop in laptops_in_stock.iter() {
        println!("{}", laptop);
    }

    println!();

    let customer = Laptops {
        dell: 3,
        hp: 3,
        toshiba: 3,
        ibm: 3,
    };

    println!("Customer purchased {:?}", customer);
    println!();
    println!("Price breakdown: \n{}", customer.price_breakdown());
    println!();
    println!("Total cost: N{}", customer.total_cost());
}
