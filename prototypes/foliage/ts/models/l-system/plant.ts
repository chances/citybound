import { Node } from './node'

export class Plant extends Node {
  private _seedSoilDepth: number
  private _isDicot: boolean
  private _minLeafSize: number
  private _matureAvgLeafSize: number
  private _soilOxygen: number
  private _airOxygen: number
  private _isDead = false

  constructor(
    seedSoilDepth: number,
    minLeafSize: number, matureAvgLeafSize: number,
    soilOxygen: number, airOxygen: number,
    isDicot = false,
  ) {
    super()

    this._seedSoilDepth = seedSoilDepth
    this._isDicot = isDicot
    this._minLeafSize = minLeafSize
    this._matureAvgLeafSize = matureAvgLeafSize
    this._soilOxygen = soilOxygen
    this._airOxygen = airOxygen
  }

  get seedSoilDepth() {
    return this._seedSoilDepth
  }

  get isDicot() {
    return this._isDicot
  }

  get minLeafSize() {
    return this._minLeafSize
  }

  get matureAvgLeafSize() {
    return this._matureAvgLeafSize
  }

  get soilOxygen() {
    return this._soilOxygen
  }

  get airOxygen() {
    return this._airOxygen
  }

  get health() {
    if (this._isDead) {
      return 0.0
    }

    return super.health
  }

  die(): void {
    this._isDead = true
  }
}

// tslint:disable-next-line:max-classes-per-file
export class OfPlant extends Node {
  get plant(): Plant {
    const root = this.root
    if (root != null && root instanceof Plant) {
      return root as Plant
    }
    return null
  }
}
