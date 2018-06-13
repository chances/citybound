import {
  ArcRotateCamera, Color3, Color4, DirectionalLight, Engine,
  HemisphericLight, Light, MeshBuilder, Scene, ShadowGenerator, StandardMaterial, Vector3,
} from 'babylonjs'

import { radians } from './utils'

class Prototype {
  private _canvas: HTMLCanvasElement
  private _engine: Engine
  private _scene: Scene
  private _camera: ArcRotateCamera
  private _sun: DirectionalLight

  constructor() {
    this._canvas = document.getElementsByTagName('canvas')[0] as HTMLCanvasElement
    this._engine = new Engine(this._canvas, true)
  }

  createScene(): void {
    const groundColor = Color4.FromInts(195, 210, 159, 255)

    // Create a basic BJS Scene object.
    this._scene = new Scene(this._engine)
    this._scene.clearColor = groundColor

    // 45deg + ((90deg - 45deg) / 2deg)
    const cameraTilt = radians(60)
    const cameraRotation = radians(45)
    this._camera = new ArcRotateCamera(
      'camera',
      cameraRotation, cameraTilt, 10,
      new Vector3(0, 0, 0),
      this._scene,
    )
    this._camera.setTarget(Vector3.Zero())
    this._camera.lowerRadiusLimit = 3
    this._camera.upperBetaLimit = radians(88)

    // Attach the camera to the canvas.
    this._camera.attachControl(this._canvas, false)

    const ambient = new HemisphericLight('ambientLight', new Vector3(0, 1, 0), this._scene)
    ambient.specular = Color3.FromInts(226, 237, 255)
    ambient.groundColor = Color3.FromInts(226, 237, 255)
    ambient.intensity = 0.35
    // -30 is a good "high in the sky" Y
    this._sun = new DirectionalLight('sun', new Vector3(6, -20, 6), this._scene)
    this._sun.specular = Color3.FromInts(0, 255 / 2, 255 * 0.75)
    this._sun.intensity = 0.75

    const shadowGenerator = new ShadowGenerator(1024, this._sun)
    shadowGenerator.usePoissonSampling = true

    const solid = new StandardMaterial('solidMaterial', this._scene)
    solid.diffuseColor = Color3.Green()
    solid.specularColor = Color3.Black()
    const sphere = MeshBuilder.CreateSphere('sphere',
      { segments: 16, diameter: 2 }, this._scene)
    sphere.material = solid

    // Move the sphere upward 1/2 of its height.
    sphere.position.y = 1

    shadowGenerator.getShadowMap().renderList.push(sphere)

    const groundMaterial = new StandardMaterial('goundMaterial', this._scene)
    groundMaterial.diffuseColor = new Color3(groundColor.r, groundColor.g, groundColor.b)
    groundMaterial.specularColor = Color3.Black()
    const ground = MeshBuilder.CreateGround('ground',
      { width: 1000, height: 1000, subdivisions: 2 }, this._scene)
    ground.receiveShadows = true
    ground.material = groundMaterial
  }

  doRender(): void {
    this._engine.runRenderLoop(() => {
      this._scene.render()
      const fpsLabel = document.getElementById('fpsLabel')
      fpsLabel.innerHTML = this._engine.getFps().toFixed() + ' fps'
    })

    window.addEventListener('resize', () => {
      this._engine.resize()
    })

    this._canvas.focus()
    this._canvas.click()
  }
}

window.addEventListener('DOMContentLoaded', () => {
  const prototype = new Prototype()
  prototype.createScene()
  prototype.doRender()
})
