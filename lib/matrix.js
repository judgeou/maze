class Node {
  constructor ({ x, y, distance = 0 }) {
    this.x = x
    this.y = y
    this.distance = distance // 与终点的距离
    this.passed = false // 该节点是否已探索完毕
    this.isQueue = false // 该节点否已经在队列中
    this.parent = null // 记录从哪个节点过来，方便回溯
    this.nextNodes = []
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
  return (x * x + y * y)
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
  
  return [ upNode, downNode, leftNode, rightNode ].filter(n => n)
}

/**
 * 根据二维数组构建图结构
 *
 * @param {Number[][]} matrix
 * @param {Node} endNode
 * @returns {Node[][]}
 */
function buildNodes (matrix, endNode) {
  let nodes = []

  // 第一次循环插入每个Node，并计算每个节点与终点的距离，但不包含与其他节点的连接
  for (let x = 0; x < matrix.length; x++) {
    let row = matrix[x]
    let nodeRow = nodes[x] = []
    for (let y = 0; y < row.length; y++) {
      if (row[y] > 0) {
        const distance = getDistance({ x, y }, { x: endNode[0], y: endNode[1] })
        nodeRow[y] = new Node({ x, y, distance })
      }
    }
  }

  // 第二次循环构建节点路径
  for (let row of nodes) {
    for (let node of row) {
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
    paths.unshift(node)
    node = node.parent
  }
  
  return paths.map(node => {
    return [node.x, node.y]
  })
}

function solveMaze (matrix, start, end) {
  let nodes = buildNodes(matrix, end)
  let startNode = nodes[start[0]][start[1]]
  let endNode = nodes[end[0]][end[1]]
  let checkCount = 0
  
  // 等待探索的节点集合
  let queue = [ startNode ]

  while (queue.length) {
    const node = queue.pop()
    console.log(node)
    checkCount++
    node.passed = true

    for (let n of node.nextNodes) {
      if (n.passed === false) {
        n.parent = node
        
        if (!n.isQueue) {
          queue.unshift(n)
          queue.sort((a, b) => b.distance - a.distance)
          n.isQueue = true
        }

        if (n === endNode) {
          console.log(`solve! check point count: ${checkCount}`)
          return buildPaths(endNode)
        }
      }
    }
  }
}

export {
  solveMaze
}