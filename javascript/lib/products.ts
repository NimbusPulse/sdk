export type ProductId = string;

export type Product = {
  id: ProductId;
  name: string;
  hourly_price_cents: number;
  monthly_price_cents: number;
  cpu: number;
  ram: number;
  max_players: number;
};

export const CADET_TIER: Product = {
  id: "01920555-2bd0-70aa-a8ca-9b8387815cab",
  name: "Cadet Tier",
  hourly_price_cents: 20,
  monthly_price_cents: 2999,
  cpu: 1,
  ram: 10,
  max_players: 5,
};

export const WINGMAN_TIER: Product = {
  id: "01920555-2bd1-7877-9c08-990b29b5d3af",
  name: "Wingman Tier",
  hourly_price_cents: 27,
  monthly_price_cents: 3999,
  cpu: 2,
  ram: 12,
  max_players: 10,
};

export const STRATEGIST_TIER: Product = {
  id: "01920555-2bd1-7877-9c08-990c708ab93f",
  name: "Strategist Tier",
  hourly_price_cents: 37,
  monthly_price_cents: 5499,
  cpu: 3,
  ram: 16,
  max_players: 20,
};

export const COMMANDER_TIER: Product = {
  id: "01920555-2bd1-7877-9c08-990d1f20e2b6",
  name: "Commander Tier",
  hourly_price_cents: 47,
  monthly_price_cents: 6999,
  cpu: 3,
  ram: 20,
  max_players: 40,
};

export const VETERAN_TIER: Product = {
  id: "01920555-2bd1-7877-9c08-990eaa62a7e4",
  name: "Veteran Tier",
  hourly_price_cents: 60,
  monthly_price_cents: 8999,
  cpu: 4,
  ram: 24,
  max_players: 60,
};

export const ACE_TIER: Product = {
  id: "01920555-2bd1-7877-9c08-990f475791ec",
  name: "Ace Tier",
  hourly_price_cents: 77,
  monthly_price_cents: 11499,
  cpu: 5,
  ram: 32,
  max_players: 100,
};

export const PRODUCTS: readonly Product[] = [
  CADET_TIER,
  WINGMAN_TIER,
  STRATEGIST_TIER,
  COMMANDER_TIER,
  VETERAN_TIER,
  ACE_TIER,
];

export function productById(id: string): Product | undefined {
  return PRODUCTS.find((product) => product.id === id);
}
