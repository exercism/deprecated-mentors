# Scrabble Score

This is the first more complex task. There are several ways to solve this and benchmarks are 
one important criteria for a good solution. Another important criteria is simple and readable code.

## Reasonable solutions

Solutions have a wide range here. A solution should be in the range of the solutions below to be approved. That means not 
overly complicated / verbose and not slower than the second solution. Allocations are not necessary and should be avoided.

_Unformatted or unlinted code is a reason not to approve._

Fast solution:
```
// Score impelents scrabble score
func Score(s string) int {
	var score int
	for _, r := range s {
		switch unicode.ToUpper(r) {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T':
			score += 1
		case 'D', 'G':
			score += 2
		case 'B', 'C', 'M', 'P':
			score += 3
		case 'F', 'H', 'V', 'W', 'Y':
			score += 4
		case 'K':
			score += 5
		case 'J', 'X':
			score += 8
		case 'Q', 'Z':
			score += 10
		}
	}

	return score
}
```

Very readable solution but ~3 times slower:
```
var m = map[rune]int{
	'A': 1, 'E': 1, 'I': 1, 'O': 1, 'U': 1, 'L': 1, 'N': 1, 'R': 1, 'S': 1, 'T': 1,
	'D': 2, 'G': 2,
	'B': 3, 'C': 3, 'M': 3, 'P': 3,
	'F': 4, 'H': 4, 'V': 4, 'W': 4, 'Y': 4,
	'K': 5,
	'J': 8, 'X': 8,
	'Q': 10, 'Z': 10,
}

// Score impelents scrabble score
func Score(s string) int {
	var score int
	for _, r := range s {
		score += m[unicode.ToUpper(r)]
	}

	return score
}
```

## Common suggestions

The most common feedback revolves around:

* Simplicity and readability
* Speed and allocations

**Simplicity and Readability**

* are they using a lot of if statements instead of a switch? Suggest to use a switch for readability.

Sometimes we have to make compromises between speed on the one side and simplicity / readability on the other. In this exercise this is not the case. My suggestion is to make this exercise mostly about speed and the solution will become simple and readable.

**Speed and alloctions**
* are they using `strings.ToLower`/`strings.ToUpper` before or even inside the for loop? Suggest using `unicode.ToLower`/`unicode.ToUpper` as it is faster and removes allotions.
* are they using a `map[string]int` and maybe even a `strings.Contains`? Suggest a `map[rune]int` for direct lookup of a rune to its value. Runes are they way to go here as they are the result of a `for .. range` over a string.
* are they defining the map inside the function? Suggest to move it outside to package level.
* a `switch` can be suggested if they are using a `map`. It is approx. 3 times faster than the map lookups.
* are they using go routines? They are probably very excited about the easy of use or think it will be super fast: It isn't. Go routines make this exercise super slow. Point to benchmarking the exercise with and without goroutines.

A criterium can be the memory allocations: there shouldn't be any memory allocations.

## Talking points

* How to use and read benchmarks in go.
* `rune`s vs `byte`s. What are runes? What is the difference? `Rune`s are not necessary here (we are in ascii space) but they are faster because we need no extra type conversions.
* If they used go routines talk about why the go routines don't make sense here. They add too much overhead to be faster than a simple switch + count.
