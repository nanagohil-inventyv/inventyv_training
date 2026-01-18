// @ts-check

import { ElectronicDevice } from './lib.js';

/**
 * Checks if input is a boolean.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isBoolean(value) {
  return typeof value === 'boolean';
}

/**
 * Checks if input is a finite number or bigint.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isNumber(value) {
  if (typeof value === 'number') {
    return Number.isFinite(value);
  }
  return typeof value === 'bigint';
}

/**
 * Checks if a value is an object.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isObject(value) {
  return typeof value === 'object' && value !== null;
}

/**
 * Checks if a value is a numeric string.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isNumericString(value) {
  return (
    typeof value === 'string' &&
    value.trim() !== '' &&
    Number.isFinite(Number(value))
  );
}

/**
 * Checks if an object is an instance of the `ElectronicDevice` class or one of its children.
 *
 * @param {object} object
 * @returns {boolean}
 */
export function isElectronic(object) {
  return object instanceof ElectronicDevice;
}

/**
 * Checks if a value is a non empty array.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isNonEmptyArray(value) {
  return Array.isArray(value) && value.length > 0;
}

/**
 * Checks if a value is an empty array.
 *
 * @param {unknown} value
 * @returns {boolean}
 */
export function isEmptyArray(value) {
  return Array.isArray(value) && value.length === 0;
}

/**
 * Checks if a value has a "type" property or method.
 *
 * @param {object} object
 * @returns {boolean}
 */
export function hasType(object) {
  return (
    object != null &&
    (typeof object.type !== 'undefined' ||
      typeof object.type === 'function')
  );
}

/**
 * Throws an error if an object is missing an "id" property or method.
 *
 * @param {object} object
 * @returns {void}
 */
export function assertHasId(object) {
    if( 'id' in object) return 
    throw new Error();
}

/**
 * Checks if a value has an "id" property.
 *
 * @param {object} object
 * @returns {boolean}
 */
export function hasIdProperty(object) {
  return Object.hasOwn(object,"id")
}

/**
 * Checks if a value has a defined "type" property.
 *
 * @param {object} object
 * @returns {boolean}
 */
export function hasDefinedType(object) {
    return  Object.hasOwn(object , "type") && typeof(object.type) !== 'undefined'
}
