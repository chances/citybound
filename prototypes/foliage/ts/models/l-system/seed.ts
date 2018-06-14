import { getRandom } from '../../utils'
import { GreenNode } from './green-node'
import { Leaf } from './leaf'
import { Node } from './node'
import { OfPlant } from './plant'

export class Seed extends OfPlant {
  static MAX_WETNESS = 10

  private _hydrationThreshold: number
  private _maxDepth: number
  private _oxygenThreshold: number

  constructor(sugarStorage: number, hydrationThreshold = 9, oxygenThreshold = 1) {
    super(Seed.MAX_WETNESS, sugarStorage)

    this._hydrationThreshold = hydrationThreshold
    this._maxDepth = sugarStorage
    this._oxygenThreshold = oxygenThreshold

    this.storeSugar(sugarStorage)
  }

  grow() {
    const plant = this.plant
    if (this.plant == null) {
      return
    }

    const closeEnoughToSurface = plant.depth <= this._maxDepth
    if (!closeEnoughToSurface) {
      plant.die()
      return
    }

    const children = this.children
    if (children.length) {
      super.grow()
      return
    }

    const hydrated = this.hydration > this._hydrationThreshold && this.hydration < Seed.MAX_WETNESS
    const oxygenated = plant.soilOxygen >= this._oxygenThreshold
    // TODO: Add temparature threshold?
    // https://en.wikipedia.org/wiki/Germination#Introduction
    if (hydrated && closeEnoughToSurface && oxygenated && this.canGrow(2)) {
      super.grow()
      this.eat()
      this._germinate()
    }
  }

  private _germinate() {
    this.children.add(new Seedling())
  }
}

// tslint:disable-next-line:max-classes-per-file
export class Seedling extends GreenNode {
  constructor(
    maxSugars = 1,
  ) {
    super(maxSugars)
  }

  grow() {
    if (!this.canGrow(2)) {
      return
    }
    super.grow()

    const plant = this.plant
    const brokenThroughSoil = this.age > plant.seedSoilDepth
    const tallEnoughForDicot = this.age - plant.seedSoilDepth > plant.minLeafSize
    if (plant.isDicot && brokenThroughSoil && tallEnoughForDicot) {
      const maxSize = plant.minLeafSize + (plant.matureAvgLeafSize - plant.minLeafSize)
      const size = getRandom(plant.minLeafSize, maxSize)
      const minSunlight = this.minSunlight + plant.minLeafSize
      this.children.add(new Leaf(0, size, minSunlight))
      this.children.add(new Leaf(180, size, minSunlight))
    }
  }
}
