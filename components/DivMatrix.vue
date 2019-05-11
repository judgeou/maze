<template>
  <div>
    Hello div
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
  created () {
    console.log(
      solveMaze(m, [0, 0], [3, 4])
    )
  }
}
</script>

<style>

</style>
