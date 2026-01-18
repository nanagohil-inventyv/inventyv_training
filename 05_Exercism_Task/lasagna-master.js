/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Implement the functions needed to solve the exercise here.
 * Do not forget to export them so they are available for the
 * tests. Here an example of the syntax as reminder:
 *
 * export function yourFunction(...) {
 *   ...
 * }
 */

export function cookingStatus(remainingTime){
  if(remainingTime === 0){
     return 'Lasagna is done.';
  }else if(remainingTime === undefined){
    return 'You forgot to set the timer.';
  }else{
    return 'Not done, please wait.';
  }
}

export function preparationTime(layers , averageTime = 2){
   let noOfLayers = layers.length;
   return noOfLayers * averageTime;
}

export function quantities(layers) {
  //each layer 50g noodles 
  // sause layer 0.2 L
  let totalNoodles = 0;
  let totalSause = 0;
  let layersLen = layers.length;
  for(let i = 0; i < layersLen; i++){
    if(layers[i] === 'sauce'){
      totalSause = Number(totalSause) + 0.2;
    }else if(layers[i] === 'noodles'){
      totalNoodles = Number(totalNoodles) + 50;
    }
  }
  console.log(totalNoodles, totalSause)
   const result =  {
    noodles:totalNoodles,
    sauce:totalSause 
  }
  return result;
}

export function addSecretIngredient(friendsList , myList) {
    myList.push(friendsList[friendsList.length - 1])
}

export function scaleRecipe(recipe , noOfPortions) {
  const requiredRecipe = {}
  for(let key in recipe){
    requiredRecipe[key] = recipe[key]/2 * noOfPortions;
  }
  return  requiredRecipe;
  
}