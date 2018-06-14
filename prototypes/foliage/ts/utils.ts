/**
 * Convert a value from degrees to radians.
 * @param deg Value in degrees to convert to radians
 */
export function radians(deg: number) {
  return deg * Math.PI / 180
}

/**
 * Convert a value from radians to degrees.
 * @param rad Value in radians to convert to degrees
 */
export function degrees(rad: number) {
  return rad * 180 / Math.PI
}

/**
 * Returns a random number between min (inclusive) and max (exclusive)
 * @param min Minimum value to randomly select from
 * @param max Maximum value to randomly select from (exclusive)
 */
export function getRandom(min: number, max: number) {
  return Math.random() * (max - min) + min
}
