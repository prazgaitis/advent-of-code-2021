const fs = require('fs');

function readlines(path) {
	const input = fs.readFileSync(path, 'utf8');
	return input.split('\n');
}

module.exports = { readlines };