<template>
  <div class="div-matrix">
    <div v-for="(row, x) in matrix" :key="x">
      <div class="cell" :class="{ black: p <= 0 }" v-for="(p, y) in row" :key="y" :style="{ left: `${y * 2.5}em`, top: `${x * 2.5}em` }">

      </div>
    </div>
  </div>
</template>

<script>
import { solveMaze } from '../lib/matrix'

const m = [
  [1, 1, 0, 0, 0],
  [0, 1, 1, 1, 0],
  [0, 0, 0, 1, 0],
  [0, 0, 0, 1, 1]
]

export default {
  data () {
    return {
      matrix: []
    }
  },
  created () {
    this.matrix = m
    console.log(
      solveMaze(m, [0, 0], [3, 4])
    )
  }
}
</script>

<style>
.div-matrix {
  position: relative;
}
.cell {
  border: 1px black solid;
  width: 2em;
  height: 2em;
  position:absolute;
}
.black {
  background: black;
}
</style>
