var backend = require('../native');

module.exports = function generate() {
  return new Uint8Array(backend.mandelbrot())
}
