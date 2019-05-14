<template>
  <div>
    <div class="top">
      <button @click="solve">Solve</button>
    </div>

    <div class="div-matrix">
      <div v-for="(row, x) in matrix" :key="x">
        <div class="cell" 
          :class="{ black: p <= 0, path: isPath(x, y) }"
          v-for="(p, y) in row" :key="y" :style="{ left: `${y * 2.5}em`, top: `${x * 2.5}em` }">
          {{ start[0] === x && start[1] === y ? 'S' : '' }}
          {{ end[0] === x && end[1] === y ? 'E' : '' }}
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import { solveMaze } from '../lib/matrix'

export default {
  data () {
    return {
      start: [3, 4],
      end: [7, 9],
      matrix: [],
      paths: [],
    }
  },
  methods: {
    isPath (x, y) {
      const p = this.paths.find(path => path[0] === x && path[1] === y)
      return p
    },
    solve () {
      const { matrix, start, end} = this
      this.paths = solveMaze(matrix, start, end)
    }
  },
  created () {
    const m = this.matrix = [
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    ]
  }
}
</script>

<style>
.top {
  margin-bottom: 1em;
}
.div-matrix {
  position: relative;
}
.cell {
  border: 1px black solid;
  width: 2em;
  height: 2em;
  position:absolute;
  text-align: center;
}
.cell.path {
  border: 1px red solid;
}
.black {
  background: black;
}
</style>
