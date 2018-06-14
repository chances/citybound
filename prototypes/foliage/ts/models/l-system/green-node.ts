import { Node } from './node'
import { OfPlant } from './plant'

export abstract class GreenNode extends OfPlant {
  static MAX_SUNLIGHT = Number.MAX_SAFE_INTEGER
  static MAX_SUGARS = Number.MAX_SAFE_INTEGER

  private _sunlight = 0
  private _sugars = 0
  private _maxSugars: number

  constructor(
    maxSugars = GreenNode.MAX_SUGARS,
    maxStoredSugars = 0,
    maxHydration = Node.MAX_HYDRATION,
  ) {
    super(maxHydration, maxStoredSugars)

    this._maxSugars = maxSugars
  }

  get rootGreenery() {
    return this._rootGreenNode(this)
  }

  get maxHydration() {
    if (this === this.rootGreenery) {
      return super.maxHydration
    }
    return this.rootGreenery.maxHydration
  }

  get minSunlight() {
    const plant = this.plant
    if (plant != null) {
      const heightAboveGround = this.depth - plant.depth
      return plant.minLeafSize + (heightAboveGround > 0 ? heightAboveGround : 0)
    }
    return 1.0
  }

  get sunlight() {
    return this._sunlight
  }

  get sugars() {
    return this._sugars
  }

  /**
   * Health of plant green node as a percentage where 0% is dead and 100% is perfectly healthy.
   *
   * A healthy plant green node is well watered and is well lit by the sun in the daytime.
   */
  get health() {
    const overwaterFactor = super.health
    const overlitFactor = Node.minMaxHealthFactor(
      this.minSunlight, GreenNode.MAX_SUNLIGHT, this._sunlight,
    )

    return 1.0 - ((overwaterFactor + overlitFactor) / 2)
  }

  eat(amount = 1) {
    if (this._sugars >= amount) {
      const storedSugarsToEat = this._sugars > amount ? amount - this._sugars : 0
      super.eat(storedSugarsToEat)
      this._sugars -= amount - storedSugarsToEat
    } else {
      super.eat(amount)
    }
  }

  canGrow(amount = 1) {
    return this._sugars + this.storedSugars >= amount
  }

  photosynthesize() {
    this._sunlight++
    if (this._sugars < this._maxSugars) {
      this._sugars++
    } else if (this.parent != null) {
      this.parent.storeSugar()
    }
  }

  night() {
    if (this._sunlight > 0) {
      this._sunlight--
    }
  }

  private _rootGreenNode(node: GreenNode): GreenNode {
    if (node == null) {
      return null
    }
    const parentIsNullAndIsGreenNode = node.parent == null && node instanceof GreenNode
    const parentNotNullAndIsNotGreenNode = node.parent != null &&
      !(node.parent instanceof GreenNode)
    if (parentIsNullAndIsGreenNode || parentNotNullAndIsNotGreenNode) {
      return node
    } else if (node.parent instanceof GreenNode) {
      return this._rootGreenNode(node.parent)
    }
  }
}
