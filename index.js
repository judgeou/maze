import Vue from 'vue'
import App from './App.vue'

new Vue({
  el: '#app',
  data () {
    return {
      native: {}
    }
  },
  created () {
    let vm = this
    // 加载 wasm 模块
    import('./pkg/index.js').then(m => {
      vm.native = m
    })
  },
  render: h => h(App)
})