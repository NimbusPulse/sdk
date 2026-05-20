use uuid::{Uuid, uuid};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Product {
    pub id: Uuid,
    pub name: &'static str,
    pub hourly_price_cents: i32,
    pub monthly_price_cents: i32,
    pub cpu: i32,
    pub ram: i32,
    pub max_players: i32,
}

impl From<Product> for Uuid {
    fn from(product: Product) -> Self {
        product.id
    }
}

impl From<&Product> for Uuid {
    fn from(product: &Product) -> Self {
        product.id
    }
}

pub const CADET_TIER: Product = Product {
    id: uuid!("01920555-2bd0-70aa-a8ca-9b8387815cab"),
    name: "Cadet Tier",
    hourly_price_cents: 20,
    monthly_price_cents: 2999,
    cpu: 1,
    ram: 10,
    max_players: 5,
};

pub const WINGMAN_TIER: Product = Product {
    id: uuid!("01920555-2bd1-7877-9c08-990b29b5d3af"),
    name: "Wingman Tier",
    hourly_price_cents: 27,
    monthly_price_cents: 3999,
    cpu: 2,
    ram: 12,
    max_players: 10,
};

pub const STRATEGIST_TIER: Product = Product {
    id: uuid!("01920555-2bd1-7877-9c08-990c708ab93f"),
    name: "Strategist Tier",
    hourly_price_cents: 37,
    monthly_price_cents: 5499,
    cpu: 3,
    ram: 16,
    max_players: 20,
};

pub const COMMANDER_TIER: Product = Product {
    id: uuid!("01920555-2bd1-7877-9c08-990d1f20e2b6"),
    name: "Commander Tier",
    hourly_price_cents: 47,
    monthly_price_cents: 6999,
    cpu: 3,
    ram: 20,
    max_players: 40,
};

pub const VETERAN_TIER: Product = Product {
    id: uuid!("01920555-2bd1-7877-9c08-990eaa62a7e4"),
    name: "Veteran Tier",
    hourly_price_cents: 60,
    monthly_price_cents: 8999,
    cpu: 4,
    ram: 24,
    max_players: 60,
};

pub const ACE_TIER: Product = Product {
    id: uuid!("01920555-2bd1-7877-9c08-990f475791ec"),
    name: "Ace Tier",
    hourly_price_cents: 77,
    monthly_price_cents: 11499,
    cpu: 5,
    ram: 32,
    max_players: 100,
};

pub const PRODUCTS: &[Product] = &[
    CADET_TIER,
    WINGMAN_TIER,
    STRATEGIST_TIER,
    COMMANDER_TIER,
    VETERAN_TIER,
    ACE_TIER,
];

pub fn product_by_id(id: Uuid) -> Option<&'static Product> {
    PRODUCTS.iter().find(|product| product.id == id)
}
