import { Nullable } from 'babylonjs'

export abstract class Node {
  static MAX_HYDRATION = Number.MAX_SAFE_INTEGER

  private _parent: Node = null
  private _children: Node[] = []
  private _age = 0
  private _hydration = 0
  private _maxHydration: number
  // Insuluable plant energy stores: Oils, Fats, and Starch
  private _storedSugars = 0
  private _maxStoredSugars: number

  constructor(maxHydration: number = Node.MAX_HYDRATION, maxStoredSugars = 0) {
    this._maxHydration = maxHydration
    this._maxStoredSugars = maxStoredSugars
  }

  get parent() {
    return this._parent
  }

  get root() {
    return this._root(this)
  }

  get children() {
    return {
      add: (child: Node) => this._add(child),
      remove: (child: Node) => this._remove(child),
      grow: () => this._children.forEach(n => n.grow()),
      length: this._children.length,
      first: this._children.length ? this._children[0] : null,
      raw: this._children,
    }
  }

  get isLeaf() {
    return this._children.length > 0
  }

  get depth() {
    return this._depth(this)
  }

  get age() {
    return this._age
  }

  get previousSibling(): Nullable<Node> {
    if (this._parent == null) {
      return null
    }
    const index = this._parent._children.indexOf(this)
    if (index > 0) {
      return this._parent._children[index - 1]
    }

    return null
  }

  get nextSibling(): Nullable<Node> {
    if (this._parent == null) {
      return null
    }
    const index = this._parent._children.indexOf(this)
    if (index < this._parent._children.length - 2) {
      return this._parent._children[index - 1]
    }

    return null
  }

  get maxHydration() {
    return this._maxHydration
  }

  get hydration() {
    return this._hydration
  }

  get storedSugars() {
    return this._storedSugars
  }

  /**
   * Health of plant node as a percentage where 0% is dead and 100% is perfectly healthy.
   *
   * A healthy plant node is watered but not overwatered.
   */
  get health() {
    if (this._hydration === 0) {
      return 0.1
    }

    const overwaterFactor = Node.minMaxHealthFactor(0.0, this._maxHydration, this._hydration)
    return 1.0 - overwaterFactor
  }

  hydrate() {
    this._hydration++
  }

  storeSugar(amount = 1) {
    if (this._storedSugars < this._maxStoredSugars) {
      this._storedSugars += amount
    }
  }

  eat(amount = 1) {
    if (this._storedSugars >= amount) {
      this._storedSugars -= amount
    }
  }

  canGrow(amount = 1) {
    return this._storedSugars >= amount
  }

  grow() {
    this._age++
    this.children.grow()
  }

  static minMaxHealthFactor(min: number, max: number, value: number) {
    if (value > min && value <= max) {
      return 1.0
    }

    const healthFactor = Math.abs(max - value) / max
    return healthFactor <= 1.0 ? healthFactor : 1.0
  }

  private _root(node: Node): Nullable<Node> {
    if (node == null) {
      return null
    }
    if (node.parent == null) {
      return node
    }
    return this._root(node.parent)
  }

  private _depth(node: Node) {
    if (node == null) {
      return 0
    }
    if (node.parent == null) {
      return 1
    }
    return 1 + this._depth(node.parent)
  }

  private _add(child: Node) {
    this._children.push(child)
    child._parent = this
  }

  private _remove(child: Node) {
    if (this._children.includes(child)) {
      this._children = this._children.filter(n => n !== child)
      child._parent = null
    }
  }
}
