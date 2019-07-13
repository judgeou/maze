<template>
  <div>
    <p>
      <b>{{ msg }}</b>
    </p>
    <div>
      <input type="file" accept="image/*" @change="onFile">    <button @click="clearPaths">重置</button>
    </div>
    <img v-show="false" ref="image" :src="imgUrl" alt="">

    <div>
      <canvas ref="canvas" @click="onCanvasClick"></canvas>
    </div>
  </div>
</template>

<script>
import { solveMaze } from '../lib/matrix'
import Jimp from 'jimp'
import { rgb2lab, deltaE } from '../lib/color'
import { setTimeout } from 'timers';

function colorDiff (c1, c2) {
  return deltaE(rgb2lab([ c1.r, c1.g, c1.b ]), rgb2lab([ c2.r, c2.g, c2.b ]))
}

export default {
  data () {
    return {
      msg: '',
      imgUrl: '',
      mapArr: [],
      clickPoints: [],
      width: 0,
      height: 0
    }
  },
  methods: {
    async onFile ({ target }) {
      const { files } = target
      const vm = this
      if (files.length) {
        vm.msg = '正在处理图片。。。'
        vm.imgUrl = URL.createObjectURL(files[0])
        setTimeout(() => {
          vm.clearPaths()
          vm.msg = '图片处理完毕，点击图片任意位置决定起点与终点'
        }, 1000)
      }
    },
    goSolve (startXY, endXY, width, height) {
      const { paths, checkCount } = solveMaze(this.mapArr, startXY, endXY, width, height)
      const { canvas } = this.$refs
      const context = canvas.getContext('2d')
      context.fillStyle = "#FF0000"
      for (let path of paths) {
        context.fillRect(path[0], path[1], 1, 1)
      }
      return {
        paths,
        checkCount
      }
    },
    genMapArr (imageData, mapArr, walkColor, stopColor) {
      console.log( colorDiff(walkColor, stopColor) )
      let i = 0
      imageData.scan(0, 0, imageData.bitmap.width, imageData.bitmap.height, (x, y, idx) => {
        const red = imageData.bitmap.data[idx + 0]
        const green = imageData.bitmap.data[idx + 1]
        const blue = imageData.bitmap.data[idx + 2]
        const colorDistance = colorDiff(walkColor, { r: red, g: green, b: blue })

        if (colorDistance < 8) {
          mapArr[i] = 1
        } else {
          mapArr[i] = 0
        }
        i++
      })
    },
    async onCanvasClick (e) {
      const { clickPoints, width, height } = this
      const { image } = this.$refs
      clickPoints.push([e.offsetX, e.offsetY])
      const { length } = clickPoints
      if (length >= 2) {
        this.msg = '正在识别可通行节点。。。'
        const mapArr = new Array(width * height)
        const imageData = await Jimp.read(image.src)

        const startPoint = clickPoints[length - 2]
        const endPoint = clickPoints[length - 1]

        const startColor = Jimp.intToRGBA(imageData.getPixelColor(startPoint[0], startPoint[1]))
        const endColor = Jimp.intToRGBA(imageData.getPixelColor(endPoint[0], endPoint[1]))

        this.genMapArr(imageData, mapArr, startColor, endColor)
        this.mapArr = mapArr
        this.msg = '正在计算最短路径。。。'
        try {
          const { paths, checkCount } = this.goSolve(clickPoints[length - 2], clickPoints[length - 1], width, height)
          this.msg = `成功计算最短路径，实际距离 ${paths.length}，探索了节点数 ${checkCount}`
        } catch (err) {
          this.msg = err
        }
      }
    },
    clearPaths () {
      const { image, canvas } = this.$refs
      const { width, height } = image
      canvas.width = width
      canvas.height = height

      const context = canvas.getContext('2d')
      context.drawImage(image, 0, 0, width, height);

      this.width = width
      this.height = height

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
