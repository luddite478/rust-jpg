const jpeg = require('jpeg-js')
const fs = require('fs')
const sizeOf = require('image-size')
const rustJpg = require('../../native')

const dimensions = sizeOf('picture.jpg')
const imgWidth = dimensions.width
const imgHeight = dimensions.height

function base64_encode(file) {
    var bitmap = fs.readFileSync(file);
    return new Buffer.from(bitmap).toString('base64');
}

const base64Raw = base64_encode('./picture.jpg')

// One sec
const start = Date.now()
let now = 0
c = 0
while ((now - start) < 1000) {
	const jpg = rustJpg.rawToJpeg(base64Raw, imgWidth, imgHeight)
	c++
	now = Date.now()
}

console.log(`\n${c} ${imgWidth}x${imgHeight} pictures pec sec\n`)




