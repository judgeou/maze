<template>
  <div>
    <div class="top">
      <button @click="solve">Solve</button>
    </div>

    <div class="div-matrix">
      <div v-for="(row, x) in matrix" :key="x">
        <div class="cell"
          :class="{ black: p <= 0, path: isPath(x, y), checked: isPassed(x, y) }"
          v-for="(p, y) in row" :key="y" :style="{ left: `${y * 2.5}em`, top: `${x * 2.5}em` }"
          @mouseenter="onMouseIn(x, y)" @click="onClick(x, y)">
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
      start: [3, 0],
      end: [7, 9],
      matrix: [],
      paths: [],
      nodes: [],
      pointNode: {}
    }
  },
  methods: {
    onMouseIn (x, y) {
      const pn = this.pointNode = this.nodes[x][y]
      console.log(pn)
    },
    onClick (x, y) {
      this.end = [x, y]
      this.solve()
    },
    isPath (x, y) {
      const { nodes } = this
      const row = nodes[x]
      if (row) {
        const node = row[y]
        if (node) {
          return node.isPath
        }
      }
      return false
    },
    isPassed (x, y) {
      const { nodes } = this
      const row = nodes[x]
      if (row) {
        const node = row[y]
        if (node) {
          return node.passed
        }
      }
      return false
    },
    solve () {
      const { matrix, start, end } = this
      const { paths, nodes } = solveMaze(matrix, start, end)
      this.paths = paths
      this.nodes = nodes
    }
  },
  created () {
    const m = this.matrix = [
      [1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
      [1, 0, 0, 0, 0, 0, 1, 0, 0, 1],
      [1, 1, 1, 1, 1, 0, 1, 1, 1, 1],
      [1, 1, 1, 1, 1, 0, 1, 0, 0, 1],
      [1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
      [1, 1, 1, 1, 1, 0, 1, 1, 0, 1],
      [1, 0, 0, 0, 0, 0, 1, 0, 1, 1],
      [1, 1, 1, 1, 1, 1, 1, 0, 1, 1],
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
  background: #3498db;
}
.cell.checked {
  border: 2px red solid;
}
.black {
  background: black;
}
</style>
