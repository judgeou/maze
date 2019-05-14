<template>
  <div>
    <canvas width="500" height="400" id="mazeCanvas"></canvas>
  </div>
</template>

<script>
import { fabric } from 'fabric'

export default {
  created () {
  },
  mounted () {
    const can = new fabric.Canvas('mazeCanvas')

    window.fabric = fabric
  }
}
</script>

<style>

</style>
