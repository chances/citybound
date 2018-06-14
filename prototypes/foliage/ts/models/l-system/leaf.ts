import { GreenNode } from './green-node'

export class Leaf extends GreenNode {
  private _angle: number
  private _size: number

  constructor(
    angle: number,
    size: number,
    maxSugars = GreenNode.MAX_SUGARS,
    maxStoredSugars = 0,
  ) {
    super(maxSugars, maxStoredSugars)

    this._angle = angle
    this._size = size
  }

  get angle() {
    return this._angle
  }

  get size() {
    return this._size
  }
}
