# Scrabble Score

This is the first more complex task. There are several ways to solve this and benchmarks are 
one important criteria for a good solution. Another important criteria is simple and readable code.

## Reasonable solutions

Solutions have a wide range here. A solution should be in the range of the solutions below to be approved. That means not 
overly complicated / verbose and not (much) slower than the second solution. Allocations are not necessary and should be avoided.

Unformatted or unlinted code is a reason not to approve.

Fast solution:
```
// Score impelents scrabble score
func Score(s string) int {
	var score int
	for _, r := range s {
		switch r {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T', 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't':
			score += 1
		case 'D', 'G', 'd', 'g':
			score += 2
		case 'B', 'C', 'M', 'P', 'b', 'c', 'm', 'p':
			score += 3
		case 'F', 'H', 'V', 'W', 'Y', 'f', 'h', 'v', 'w', 'y':
			score += 4
		case 'K', 'k':
			score += 5
		case 'J', 'X', 'j', 'x':
			score += 8
		case 'Q', 'Z', 'q', 'z':
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
* 

**Speed and alloctions**
* are they using `strings.ToLower`/`strings.ToUpper`? Suggest using `unicode.ToLower`/`unicode.ToUpper` as it is faster and removes allotions.
* are they using a `map[string]int` and maybe even a `strings.Contains`? Suggest a `map[rune]int` for direct lookup of a rune.
* are they defining the map inside the function? Suggest to move it outside to package level.
* a `switch` can be suggested if they are using a `map`.
* there shouldn't be any memory allocations on this task. Use above suggestions to get rid of them.

## Talking points

* trade-off between `fmt.Sprintf` and `+`, `fmt` is very common and idiomatic, but `+` is shorter and avoids and import.
