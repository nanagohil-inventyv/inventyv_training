/**
 * Calculate the price of a pizza with optional extras.
 *
 * @param {string} pizza
 * @param  {...string} extras
 * @returns {number}
 */
export function pizzaPrice(pizza, ...extras) {
  // Base prices
  const prices = {
    Margherita: 7,
    Caprese: 9,
    Formaggio: 10
  };

  // Extra prices
  const extraPrices = {
    ExtraSauce: 1,
    ExtraToppings: 2
  };

  // Start with the pizza base price
  let total = prices[pizza] ?? 0;

  // Add any extras
  for (const extra of extras) {
    if (extraPrices[extra] !== undefined) {
      total += extraPrices[extra];
    }
  }

  return total;
}

/**
 * Calculate the total price of multiple pizza orders.
 *
 * @param {Array<{pizza: string, extras: string[]}>} pizzaOrders
 * @returns {number}
 */
export function orderPrice(pizzaOrders) {
  let total = 0;

  for (const order of pizzaOrders) {
    const { pizza, extras } = order;
    total += pizzaPrice(pizza, ...(extras || []));
  }

  return total;
}
