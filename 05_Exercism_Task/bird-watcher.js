// @ts-check
//
// The line above enables type checking for this file. Various IDEs interpret
// the @ts-check directive. It will give you helpful autocompletion when
// implementing this exercise.

/**
 * Calculates the total bird count.
 *
 * @param {number[]} birdsPerDay
 * @returns {number} total bird count
 */
export function totalBirdCount(birdsPerDay) {
      let totalNumberOfBird = 0;
      for(let i = 0; i < birdsPerDay.length; i++){
         totalNumberOfBird += birdsPerDay[i];
      }
   return totalNumberOfBird;
}

/**
 * Calculates the total number of birds seen in a specific week.
 *
 * @param {number[]} birdsPerDay
 * @param {number} week
 * @returns {number} birds counted in the given week
 */
export function birdsInWeek(birdsPerDay, week) {
   let totalBirdsInGivenWeek = 0;
   let weekWindow = 7 * (week - 1);
   for(let i = 0 ;  i < 7; i++){
       totalBirdsInGivenWeek += birdsPerDay[i + weekWindow];
   }
   return totalBirdsInGivenWeek;
}

/**
 * Fixes the counting mistake by increasing the bird count
 * by one for every second day.
 *
 * @param {number[]} birdsPerDay
 * @returns {void} should not return anything
 */
export function fixBirdCountLog(birdsPerDay) {
   for(let i = 0; i < birdsPerDay.length; i+=2){
        birdsPerDay[i]+=1;
   }
}
