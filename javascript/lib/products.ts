export type Product = {
  id: string;
  name: string;
  hourly_price_cents: number;
  monthly_price_cents: number;
  cpu: number;
  ram: number;
  max_players: number;
};

export const CADET_TIER = {
  id: "01920555-2bd0-70aa-a8ca-9b8387815cab",
  name: "Cadet Tier",
  hourly_price_cents: 20,
  monthly_price_cents: 2999,
  cpu: 1,
  ram: 10,
  max_players: 5,
} as const satisfies Product;

export const WINGMAN_TIER = {
  id: "01920555-2bd1-7877-9c08-990b29b5d3af",
  name: "Wingman Tier",
  hourly_price_cents: 27,
  monthly_price_cents: 3999,
  cpu: 2,
  ram: 12,
  max_players: 10,
} as const satisfies Product;

export const STRATEGIST_TIER = {
  id: "01920555-2bd1-7877-9c08-990c708ab93f",
  name: "Strategist Tier",
  hourly_price_cents: 37,
  monthly_price_cents: 5499,
  cpu: 3,
  ram: 16,
  max_players: 20,
} as const satisfies Product;

export const COMMANDER_TIER = {
  id: "01920555-2bd1-7877-9c08-990d1f20e2b6",
  name: "Commander Tier",
  hourly_price_cents: 47,
  monthly_price_cents: 6999,
  cpu: 3,
  ram: 20,
  max_players: 40,
} as const satisfies Product;

export const VETERAN_TIER = {
  id: "01920555-2bd1-7877-9c08-990eaa62a7e4",
  name: "Veteran Tier",
  hourly_price_cents: 60,
  monthly_price_cents: 8999,
  cpu: 4,
  ram: 24,
  max_players: 60,
} as const satisfies Product;

export const ACE_TIER = {
  id: "01920555-2bd1-7877-9c08-990f475791ec",
  name: "Ace Tier",
  hourly_price_cents: 77,
  monthly_price_cents: 11499,
  cpu: 5,
  ram: 32,
  max_players: 100,
} as const satisfies Product;

export const PRODUCTS = [
  CADET_TIER,
  WINGMAN_TIER,
  STRATEGIST_TIER,
  COMMANDER_TIER,
  VETERAN_TIER,
  ACE_TIER,
] as const satisfies readonly Product[];

export type ProductId = (typeof PRODUCTS)[number]["id"];

export function productById(id: string): Product | undefined {
  return PRODUCTS.find((product) => product.id === id);
}
