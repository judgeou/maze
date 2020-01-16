<template>
  <div>
    <div>
      <input type="file" accept="image/*" @change="onFile">    <button @click="clearPaths">重置</button>
      <b>{{ msg }}</b>
    </div>

    <img v-if="imgUrl" v-show="false" ref="image" :src="imgUrl" @load="imgReady" alt="">

    <div>
      <canvas v-if="imgUrl" ref="canvas" @click="onCanvasClick"></canvas>
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

async function readFile (file) {
  return new Promise((resolve, reject) => {
    let reader = new FileReader()
    reader.readAsArrayBuffer(file)
    reader.onloadend = () => {
      resolve(reader.result)
    }
  })
}

async function fetchFile (url) {
  let res = await fetch(url)
  let buffer = await res.arrayBuffer()
  return new Uint8Array(buffer)
}

export default {
  data () {
    return {
      msg: '',
      imgUrl: require('../assets/maze.jpg'),
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
      }
    },
    imgReady () {
      this.clearPaths()
      vm.msg = '点击图片任意位置决定起点与终点'
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
    async genMapArr (startPoint, endPoint) {
      let buffer = await fetchFile(this.imgUrl)
      let result = vm.$root.native.gen_map_array(
        buffer,
        startPoint,
        endPoint
      )
      return result
    },
    async onCanvasClick (e) {
      const { clickPoints, width, height } = this
      const { image } = this.$refs
      clickPoints.push([e.offsetX, e.offsetY])
      const { length } = clickPoints
      if (length >= 1) {
        this.msg = `起点: (x: ${e.offsetX}, y: ${e.offsetY})`
      }
      if (length >= 2) {
        this.msg = `终点: (x: ${e.offsetX}, y: ${e.offsetY}), 正在扫描可通行区域`
        // this.msg = '正在识别可通行节点。。。'
        const mapArr = new Array(width * height)

        const startPoint = clickPoints[length - 2]
        const endPoint = clickPoints[length - 1]

        try {
          this.mapArr = await this.genMapArr(startPoint, endPoint)
          this.msg = '正在计算最短路径。。。'
          await this.$nextTick()
          const { paths, checkCount } = this.goSolve(clickPoints[length - 2], clickPoints[length - 1], width, height)
          this.msg = `成功计算最短路径，实际距离 ${paths.length}，探索了节点数 ${checkCount}`
        } catch (err) {
          this.msg = err
          throw err
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
      this.msg = ''
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
