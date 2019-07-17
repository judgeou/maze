import Heap from 'heap'

class Node {
  constructor ({ x, y, endDistance = 0, startDistance = 0 }) {
    this.x = x
    this.y = y
    this.endDistance = endDistance // 与终点的距离
    this.startDistance = startDistance // 与起点的实际距离
    this.distance = endDistance + startDistance
    this.passed = false // 该节点是否已探索完毕
    this.isQueue = false // 该节点否已经在队列中
    this.parent = null // 记录从哪个节点过来，方便回溯
    this.isPath = false // 是否是路线途径点
    this.nextNodes = null
  }
}

/**
 * 取节点
 *
 * @param {*} m
 * @param {*} x
 * @param {*} y
 * @returns {Node}
 */
function getNode(nodes, x, y) {
  if (x >= 0 && y >= 0 && x < nodes.length && y < nodes[x].length) {
    return nodes[x][y]
  } else {
    return null
  }
}

/**
 * 取两节点距离
 *
 * @param {Node} nodeA
 * @param {Node} nodeB
 */
function getDistance (nodeA, nodeB) {
  const x = Math.abs(nodeB.x - nodeA.x)
  const y = Math.abs(nodeB.y - nodeA.y)
  return (x + y)
}

/**
 * 取下一个移动点集合
 *
 * @param {*} m
 * @param {*} current
 * @returns
 */
function getNextNodes(nodes, { x, y }) {
  const upNode = getNode(nodes, x - 1, y)
  const downNode = getNode(nodes, x + 1, y)
  const leftNode = getNode(nodes, x, y - 1)
  const rightNode = getNode(nodes, x, y + 1)
  
  return [ upNode, downNode, leftNode, rightNode ]
}

/**
 * 根据二维数组构建图结构
 *
 * @param {Number[]} matrix
 * @param {Node} endNode
 * @param {Number} width
 * @param {Number} height
 * @returns {Node[][]}
 */
function buildNodes (matrix, endNode, startNode, width, height) {
  let nodes = new Array(width)

  // 第一次循环插入每个Node，并计算每个节点与终点的距离，但不包含与其他节点的连接
  for (let x = width - 1; x >= 0; x--) {
    let nodeRow = nodes[x] = new Array(height)
    for (let y = height - 1; y >= 0; y--) {
      const v = matrix[y * width + x]
      if (v > 0) {
        const endDistance = getDistance({ x, y }, { x: endNode[0], y: endNode[1] })
        nodeRow[y] = new Node({x, y, endDistance})
      }
    }
  }

  // 第二次循环构建节点路径
  for (let i = nodes.length - 1; i >= 0; i--) {
    let row = nodes[i]
    for (let j = row.length - 1; j >= 0; j--) {
      let node = row[j]
      if (node) {
        node.nextNodes = getNextNodes(nodes, node)
      }
    }
  }
  
  return nodes
}

/**
 * 直接回溯，构建路径
 *
 * @param {Node} endNode
 */
function buildPaths (endNode) {
  let paths = []
  let node = endNode

  while (node) {
    paths.push(node)
    node.isPath = true
    node = node.parent
  }

  return paths.map(node => {
    return [node.x, node.y]
  }).reverse()
}

function solveMaze (matrix, start, end, width, height) {
  let nodes = buildNodes(matrix, end, start, width, height)
  window.nodes = nodes
  let startNode = nodes[start[0]][start[1]]
  let endNode = nodes[end[0]][end[1]]
  
  if (startNode === undefined) {
    throw '起始点未找到'
  }
  if (endNode === undefined) {
    throw '终点不可到达'
  }

  let checkCount = 0
  
  // 等待探索的节点集合
  let queue = new Heap((a, b) => a.distance - b.distance)
  queue.push(startNode)

  while (!queue.empty()) {
    const node = queue.pop()
    checkCount++
    node.passed = true

    let { nextNodes } = node
    for (let i = nextNodes.length - 1; i >= 0; i--) {
      let n = nextNodes[i]
      if (n) {
        if (n.passed === false) {
          n.parent = node
          
          n.startDistance = (node.startDistance + 1) // 更新与起点的距离
          n.distance = n.startDistance + n.endDistance
  
          if (!n.isQueue) {
            queue.push(n)
            n.isQueue = true
          }
  
          if (n === endNode) {
            console.log(`solve! check point count: ${checkCount}`)
            return { 
              paths: buildPaths(endNode),
              nodes,
              checkCount
            }
          }
        }
      }
    }
  }

  throw Error('无法到达终点')
}

export {
  solveMaze
}