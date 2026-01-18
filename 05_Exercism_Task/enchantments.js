/// <reference path="./global.d.ts" />
// @ts-check

/**
 * Get the first card in the given deck
 *
 * @param {Card[]} deck
 *
 * @returns {Card} the first card in the deck
 */
export function getFirstCard(deck) {
  const [a] = deck;
  return a;
}

/**
 * Get the second card in the given deck
 *
 * @param {Card[]} deck
 *
 * @returns {Card} the second card in the deck
 */
export function getSecondCard(deck) {
  const [, b] = deck;
  return b;
}

/**
 * Switch the position of the two cards
 *
 * @param {[Card, Card]} deck
 *
 * @returns {[Card, Card]} new deck with the 2 cards swapped
 */
export function swapTwoCards(deck) {
  const [c1,c2] = deck;
  return [c2 ,c1];
}

/**
 * Rotate (shift) the position of the three cards (by one place)
 *
 * @param {[Card, Card, Card]} deck
 *
 * @returns {[Card, Card, Card]} new deck with the 3 cards shifted by one position
 */
export function shiftThreeCardsAround(deck) {
  const [c1 ,c2 , c3] = deck;
  return [c2 , c3 , c1];
}

/**
 * Grab the chosen pile from the available piles
 *
 * @param {{ chosen: Card[], disregarded: Card[] }} piles
 *
 * @returns {Card[]} the pile named chosen
 */
export function pickNamedPile(piles) {
  const {chosen , disregarded} = piles;
  return chosen;
  // 🚨 Do NOT use piles.chosen or piles.disregarded.
  
}

/**
 * Swap the chosen pile for the disregarded pile and the disregarded pile for the chosen pile
 *
 * @param {{ chosen: Card[], disregarded: Card[] }} piles
 * @returns {{ chosen: Card[], disregarded: Card[] }} new piles where the two piles are swapped
 */
export function swapNamedPile(piles) {
     return {
        chosen:piles.disregarded,
        disregarded:piles.chosen
     }
}

// @ts-check

/**
 * Retrieve card from cards array at the 0-based position
 *
 * @param {number[]} cards
 * @param {number} position
 *
 * @returns {number} the card
 */
export function getItem(cards, position) {
   return  cards[position];
}

/**
 * Exchange card with replacementCard at the 0-based position
 *
 * @param {number[]} cards
 * @param {number} position
 * @param {number} replacementCard
 *
 * @returns {number[]} the cards with the change applied
 */
export function setItem(cards, position, replacementCard) {
   cards.splice(position ,1 , replacementCard);
   return cards;
}

/**
 * Insert newCard at the end of the cards array
 *
 * @param {number[]} cards
 * @param {number} newCard
 *
 * @returns {number[]} the cards with the newCard applied
 */
export function insertItemAtTop(cards, newCard) {
    cards.push(newCard);
    return cards;
}

/**
 * Remove the card at the 0-based position
 *
 * @param {number[]} cards
 * @param {number} position
 *
 * @returns {number[]} the cards without the removed card
 */
export function removeItem(cards, position) {
    cards.splice(position,1);
    return cards;
}

/**
 * Remove card from the end of the cards array
 *
 * @param {number[]} cards
 *
 * @returns {number[]} the cards without the removed card
 */
export function removeItemFromTop(cards) {
  cards.pop();
  return cards;
}

/**
 * Insert newCard at beginning of the cards array
 *
 * @param {number[]} cards
 * @param {number} newCard
 *
 * @returns {number[]} the cards including the new card
 */
export function insertItemAtBottom(cards, newCard) {
     cards.unshift(newCard);
     return cards;
}

/**
 * Remove card from the beginning of the cards
 *
 * @param {number[]} cards
 *
 * @returns {number[]} the cards without the removed card
 */
export function removeItemAtBottom(cards) {
   cards.shift();
   return cards;
}

/**
 * Compare the number of cards with the given stackSize
 *
 * @param {number[]} cards
 * @param {number} stackSize
 *
 * @returns {boolean} true if there are exactly stackSize number of cards, false otherwise
 */
export function checkSizeOfStack(cards, stackSize) {
    return cards.length === stackSize;
}

// @ts-check

/**
 * Determine how many cards of a certain type there are in the deck
 *
 * @param {number[]} stack
 * @param {number} card
 *
 * @returns {number} number of cards of a single type there are in the deck
 */
export function cardTypeCheck(stack, card) {
  // 🚨 Use .forEach
  let noOfCards = 0;
  stack.forEach((c,index)=>{
    if(c == card){
      noOfCards++;
    }
  })
  return noOfCards;
}

/**
 * Determine how many cards are odd or even
 *
 * @param {number[]} stack
 * @param {boolean} type the type of value to check for - odd or even
 * @returns {number} number of cards that are either odd or even (depending on `type`)
 */
export function determineOddEvenCards(stack, type) {
   let totalCards = 0;
   for(const card of stack){
     if(type){
       if(!(card & 1)){
         totalCards++;
       }
     }else{
        if(card & 1){
          totalCards++;
        }
     }
   }
  return totalCards;
}


// @ts-check

/**
 * Double every card in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with every card doubled
 */
export function seeingDouble(deck) {
  return deck.map((value)=>value<<1)
}

/**
 *  Creates triplicates of every 3 found in the deck.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with triplicate 3s
 */
export function threeOfEachThree(deck) {
   return deck.map((card)=> card === 3 ? [3,3,3]: card).flat();
}

/**
 * Extracts the middle two cards from a deck.
 * Assumes a deck is always 10 cards.
 *
 * @param {number[]} deck of 10 cards
 *
 * @returns {number[]} deck with only two middle cards
 */
export function middleTwo(deck) {
   let n = deck.length;
   let startIndex = n/2 - 1;
   let endIndex = n/2 + 1;
   return deck.slice(startIndex , endIndex);
}

/**
 * Moves the outside two cards to the middle.
 *
 * @param {number[]} deck with even number of cards
 *
 * @returns {number[]} transformed deck
 */

export function sandwichTrick(deck) {
    let top = deck[0];
    let bottom = deck[deck.length - 1]
    let middelIndex = deck.length /2;
    let middle = [bottom , top];
    const remaining = deck.slice(1 , -1);
    return [...remaining.slice(0 , middelIndex - 1) , ...middle , ...remaining.slice(middelIndex - 1)] 
  
}

/**
 * Removes every card from the deck except 2s.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} deck with only 2s
 */
export function twoIsSpecial(deck) {
   return deck.filter((value)=>value === 2);
}

/**
 * Returns a perfectly order deck from lowest to highest.
 *
 * @param {number[]} deck shuffled deck
 *
 * @returns {number[]} ordered deck
 */
export function perfectlyOrdered(deck) {
   return deck.sort((a,b)=>a - b);
}

/**
 * Reorders the deck so that the top card ends up at the bottom.
 *
 * @param {number[]} deck
 *
 * @returns {number[]} reordered deck
 */
export function reorder(deck) {
     return deck.reverse();
}
