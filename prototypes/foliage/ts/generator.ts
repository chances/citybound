import { Color3, Mesh, MeshBuilder, Vector3 } from 'babylonjs'
import { GreenNode, Plant } from './models/foliage'

export interface PlantMesh {
  mesh: Mesh
  color: Color3
}

export function generate(plant: Plant): PlantMesh[] {
  const meshes: PlantMesh[] = []
  const iterator = plant.iterator()
  let res = iterator.next()
  while (!res.done) {
    const node = res.value
    if (node instanceof GreenNode) {
      meshes.push(green(node))
    }
    res = iterator.next()
  }

  return meshes
}

function green(node: GreenNode): PlantMesh {
  const shape = [
    new Vector3(1, 0, 0),
    new Vector3(0, 0, -1),
    new Vector3(-1, 0, 0),
    new Vector3(0, 0, 1),
  ]
  shape.push(shape[0])

  const path = [
    new Vector3(0, 0, 0),
    new Vector3(0, node.age, 0),
  ]

  const extrusion = MeshBuilder.ExtrudeShape(`plantGreen${node.age}`, {
    shape, path,
  })

  return {
    mesh: extrusion,
    color: Color3.Green(),
  }
}
