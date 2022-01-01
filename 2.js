const fs = require('fs');
const input = fs.readFileSync('inputs/two.txt', 'utf8');

const directions = input
	.split('\n')
	.map(line => line.split(' '))
	.map(([ direction, distance ]) => ({ direction, distance: Number(distance) }));

let depth = 0;
let horiz = 0;

for (const { direction, distance } of directions) {
	switch (direction) {
		case 'up':
			depth -= distance;
			break;
		case 'down':
			depth += distance;
			break;
		case 'forward':
			horiz += distance;
			break;
	}
}

console.log({ depth, horiz, mult: depth * horiz });


