// TODO: Define a new `Order` type.
//   It should keep track of three pieces of information: `product_name`, `quantity`, and `unit_price`.
//   The product name can't be empty and it can't be longer than 300 bytes.
//   The quantity must be strictly greater than zero.
//   The unit price is in cents and must be strictly greater than zero.
//   Order must include a method named `total` that returns the total price of the order.
//   Order must provide setters and getters for each field.
//
// Tests are located in a different place this time—in the `tests` folder.
// The `tests` folder is a special location for `cargo`. It's where it looks for **integration tests**.
// Integration here has a very specific meaning: they test **the public API** of your project.
// You'll need to pay attention to the visibility of your types and methods; integration
// tests can't access private or `pub(crate)` items.

pub struct Order {
    product_name: String,
    quantity: u32,
    unit_price: u64,
}

impl Order {
    pub fn new(product_name: String, quantity: u32, unit_price: u64) -> Self {
        let product_name = product_name.trim().to_string();

        Self::validate_product_name(&product_name);
        Self::validate_quantity(quantity);
        Self::validate_unit_price(unit_price);

        Self {
            product_name,
            quantity,
            unit_price,
        }
    }

    pub fn total(&self) -> u64 {
        self.unit_price * u64::from(self.quantity)
    }

    // Getters
    pub fn product_name(&self) -> &str {
        &self.product_name
    }

    pub fn quantity(&self) -> &u32 {
        &self.quantity
    }

    pub fn unit_price(&self) -> &u64 {
        &self.unit_price
    }

    //  Setters
    pub fn set_product_name(&mut self, new_product_name: String) {
        let new_product_name = new_product_name.trim().to_string();

        Self::validate_product_name(&new_product_name);
        self.product_name = new_product_name;
    }

    pub fn set_quantity(&mut self, new_quantity: u32) {
        Self::validate_quantity(new_quantity);
        self.quantity = new_quantity;
    }

    pub fn set_unit_price(&mut self, new_unit_price: u64) {
        Self::validate_unit_price(new_unit_price);
        self.unit_price = new_unit_price;
    }

    // Validations
    fn validate_product_name(product_name: &str) {
        assert!(!product_name.trim().is_empty(), "The product name can't be empty");
        assert!(
            product_name.len() <= 300,
            "The product name can't be longer than 300 bytes"
        );
    }

    fn validate_quantity(quantity: u32) {
        assert!(
            quantity > 0,
            "The quantity must be strictly greater than zero"
        );
    }

    fn validate_unit_price(unit_price: u64) {
        assert!(
            unit_price > 0,
            "The unit price must be strictly greater than zero."
        );
    }
}
