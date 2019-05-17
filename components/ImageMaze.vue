<template>
  <div>
    Hello
    <img ref="image" src="../assets/maze.gif" alt="">
    <canvas ref="canvas" @click="onCanvasClick"></canvas>
    <button @click="goSolve">solve</button>
  </div>
</template>

<script>
import { solveMaze } from '../lib/matrix'

export default {
  data () {
    return {
      mapArr: []
    }
  },
  methods: {
    goSolve (endXY = [164, 10]) {
      const { paths } = solveMaze(this.mapArr, [13, 50], endXY)
      const { canvas } = this.$refs
      const context = canvas.getContext('2d')
      context.fillStyle = "#FF0000"
      for (let path of paths) {
        context.fillRect(path[0], path[1], 1, 1)
      }
    },
    onCanvasClick (e) {
      this.goSolve([e.offsetX, e.offsetY])
    }
  },
  mounted () {
    const { image, canvas } = this.$refs
    const { width, height } = image
    canvas.width = width
    canvas.height = height
    const context = canvas.getContext('2d')
    context.drawImage(image, 0, 0, width, height);
    
    const mapArr = []
    for (let x = 0; x < width; x++) {
      const row = mapArr[x] = []
      for (let y = 0; y < height; y++) {
        const uint = context.getImageData(x, y, 1, 1).data
        if (uint[0] === 255 && uint[1] === 255) {
          row[y] = 1
        } else {
          row[y] = 0
        }
      }
    }
    this.mapArr = mapArr
  }
}
</script>

<style>

</style>
