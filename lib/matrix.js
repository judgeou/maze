/**
 * 取点的数值
 *
 * @param {*} m
 * @param {*} x
 * @param {*} y
 * @returns
 */
function getPoint(m, x, y) {
  if (x >= 0 && y >= 0 && x < m.length && y < m[x].length) {
    return m[x][y]
  } else {
    return 0
  }
}

/**
 * 取下一个移动点
 *
 * @param {*} m
 * @param {*} current
 * @param {*} lastPoint
 * @returns
 */
function getNextPoint(m, current, lastPoint) {
  let [x, y] = current
  let nextPoint = [
    [x - 1, y], [x + 1, y], [x, y - 1], [x, y + 1]
  ].find(p => {
    let r1 = getPoint(m, p[0], p[1]) > 0
    let r2 = !isSamePoint(p, lastPoint)
    return r1 && r2
  })
  return nextPoint
}

function isSamePoint (p1, p2) {
  return p1[0] === p2[0] && p1[1] === p2[1]
}

function solveMaze (matrix, start, end) {
  // 记录走过的路径
  let path = []

  // 当前点
  let current = start
  path.push(current)

  // 上次走过的路
  let lastPoint = start

  while (1) {
    // 终点判断
    if (isSamePoint(current, end)) {
      break
    }

    let validPoint = getNextPoint(matrix, current, lastPoint)

    if (validPoint) {
      path.push(validPoint)
      lastPoint = current
      current = validPoint
    } else {
      break
    }
  }
  return path

  return [
    [0, 0], [0, 1], [1, 2], [1, 3], [2, 3], [3, 3], [3, 4]
  ]
}

export {
  solveMaze
}