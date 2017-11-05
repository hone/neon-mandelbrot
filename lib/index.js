var backend = require('../native');

module.exports = {
  generate: function(width, height, maxIterations, scalingFactor, panX, panY) {
    return new Uint8Array(backend.mandelbrot(width, height, maxIterations, scalingFactor, panX, panY))
  },
  generate_parallel: function(width, height, maxIterations, scalingFactor, panX, panY) {
    return new Uint8Array(backend.mandelbrot_parallel(width, height, maxIterations, scalingFactor, panX, panY))
  }
}
