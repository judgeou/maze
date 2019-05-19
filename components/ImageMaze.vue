<template>
  <div>
    <input type="text" v-model="imgUrl">
    <img ref="image" :src="imgUrl" alt="">
    <canvas ref="canvas" @click="onCanvasClick"></canvas>
    <button @click="goSolve">solve</button>
    <button @click="clearPaths">clear</button>
  </div>
</template>

<script>
import { solveMaze } from '../lib/matrix'
import Jimp from 'jimp'

export default {
  data () {
    return {
      imgUrl: 'about:blank',
      mapArr: [],
      clickPoints: [],
      width: 0,
      height: 0
    }
  },
  methods: {
    goSolve (startXY, endXY, width, height) {
      const { paths } = solveMaze(this.mapArr, startXY, endXY, width, height)
      const { canvas } = this.$refs
      const context = canvas.getContext('2d')
      context.fillStyle = "#FF0000"
      for (let path of paths) {
        context.fillRect(path[0], path[1], 1, 1)
      }
    },
    onCanvasClick (e) {
      const { clickPoints, width, height } = this
      clickPoints.push([e.offsetX, e.offsetY])
      const { length } = clickPoints
      if (length >= 2) {
        this.goSolve(clickPoints[length - 2], clickPoints[length - 1], width, height)
      }
    },
    async clearPaths () {
      const { image, canvas } = this.$refs
      const { width, height } = image
      canvas.width = width
      canvas.height = height

      const mapArr = new Array(width * height)
      const imageData = await Jimp.read(image.src)
      let i = 0
      imageData.scan(0, 0, imageData.bitmap.width, imageData.bitmap.height, (x, y, idx) => {
        const red = imageData.bitmap.data[idx + 0]
        const green = imageData.bitmap.data[idx + 1]
        const blue = imageData.bitmap.data[idx + 2]
        if (red === 255 && green === 255 && blue === 255) {
          mapArr[i] = 1
        } else {
          mapArr[i] = 0
        }
        i++
      })
      this.mapArr = mapArr

      const context = canvas.getContext('2d')
      context.drawImage(image, 0, 0, width, height);

      this.width = imageData.bitmap.width
      this.height = imageData.bitmap.height

      this.clickPoints = []
    }
  },
  created () {
    window.vm = this
  },
  mounted () {

  }
}
</script>

<style>

</style>
