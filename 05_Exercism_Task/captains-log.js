// @ts-check

/**
 * Generates a random starship registry number.
 *
 * @returns {string} the generated registry number.
 */
export function randomShipRegistryNumber() { 
  let min = 1000; 
  let max = 9999; 
  // The logic for generating a random number in the range [1000, 9999] inclusive is correct
   let id = Math.floor(Math.random() * (max - min + 1)) + min; 
  return `NCC-${id}`; 
}
/**
 * Generates a random stardate.
 *
 * @returns {number} a stardate between 41000 (inclusive) and 42000 (exclusive).
 */
export function randomStardate() {
    let startDate = 41000.0 
    let endDate = 42000.0
    return Math.random()*(endDate - startDate) + startDate;
}

/**
 * Generates a random planet class.
 *
 * @returns {string} a one-letter planet class.
 */
export function randomPlanetClass() {
  let planet = ['D', 'H', 'J', 'K', 'L', 'M', 'N', 'R', 'T','Y']
  let max = planet.length;
  let randomIndex = Math.floor(Math.random() * max);
  return planet[randomIndex];
}
