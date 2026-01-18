// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Determines how long it takes to prepare a certain juice.
 *
 * @param {string} name
 * @returns {number} time in minutes
 */
export function timeToMixJuice(name) {
    let timeToMix = 2.5;
   switch(name){
     case "Pure Strawberry Joy":
        timeToMix = 0.5;
        break;
     case "Energizer":
         timeToMix = 1.5;
        break;
      case "Green Garden":
         timeToMix = 1.5;
        break;
      case "Tropical Island":
        timeToMix = 3;
        break;
      case "All or Nothing":
         timeToMix = 5;
        break;
     default:
         timeToMix = 2.5;
         break;
   }
   return timeToMix;
}

/**
 * Calculates the number of limes that need to be cut
 * to reach a certain supply.
 *
 * @param {number} wedgesNeeded
 * @param {string[]} limes
 * @returns {number} number of limes cut
 */
export function limesToCut(wedgesNeeded, limes) {
   let requiredCut = 0;
   let i = 0;
   let limesLen = limes.length;
   while(i < limesLen){
      if(requiredCut >= wedgesNeeded){
         break;
      }
      if(limes[i] == 'small'){
        requiredCut += 6;
      }else if(limes[i] == 'large'){
         requiredCut += 8;
      }else {
         requiredCut += 10;
      }
     i++;
   }
  return i;
    
}

/**
 * Determines which juices still need to be prepared ll after the end of the shift.
 *
 * @param {number} timeLeft
 * @param {string[]} orders
 * @returns {string[]} remaining orders after the time is up
 */
export function remainingOrders(timeLeft, orders) {
     let ordersSz = orders.length;
     let i = 0;
     while(i < ordersSz){
        let timeToProcessOrder = timeToMixJuice(orders[i]);
        if(timeLeft <= 0){
           break;
        }
        timeLeft -= timeToProcessOrder;
        i++;
     }
     return orders.slice(i);
   
}
