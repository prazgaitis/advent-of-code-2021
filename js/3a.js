const { readlines } = require('./util.js');


function get_common_bit(arr) {
	// console.log(arr)
	const ones = arr.filter(bit => bit === '1').length;
	const zeros = arr.filter(bit => bit === '0').length;
	
	return ones > zeros ? '1' : '0';
}

function get_least_common_bit(arr) {
	// console.log(arr)
	const ones = arr.filter(bit => bit === '1').length;
	const zeros = arr.filter(bit => bit === '0').length;

	return zeros > ones ? '1' : '0';
}

function find_gamma_rate(input) {
	// get gammma rate (most common bit at position i)
	// iterate through each column
	let gamma_rate = "";
	for (let i = 0; i < input[0].length; i++) {
		// get all bits at position i
		const bits = input.map(line => line[i]);
		gamma_rate += get_common_bit(bits);
	}

	return gamma_rate;
}

function main() {
	const input = readlines('inputs/three.txt');

	const gamma_rate = find_gamma_rate(input);
	const epsilon_rate = find_epsilon_rate();
	console.log({ gamma_rate, binary: Number.parseInt(gamma_rate, 2) });
	// get epsilon rate

	// get power consumption rate ( gamma * epsilon )

}

main();