var backend = require('../native');

module.exports = function generate(width, height, maxIterations, scalingFactor, panX, panY) {
  return new Uint8Array(backend.mandelbrot(width, height, maxIterations, scalingFactor, panX, panY))
}
