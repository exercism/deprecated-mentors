### Reasonable solutions

```javascript
module.exports = {
  isLeap(year) {
        if (this.year % 4 !== 0) return false

        if (this.year % 100 !== 0) return true

        if (this.year % 100 === 0 && this.year % 400 === 0) return true

        return false
  }
}
```

### Common suggestions

- For solutions that have more than one exit point, suggest trying to use a single
exit point while maintaining legibility. It could be done by aggregating all
condintionals in one and creating a temp var with a legible name for each.

- Creating a helper function that checks if a number is divisible might also be
a good idea. This would help with the first point and keep the logic readable.
[Here is a good example of it](https://exercism.io/tracks/javascript/exercises/leap/solutions/caa4742c2be14884848044f9bcfbb775)

### Talking points

- Although this exercise is quite simple it is worth talking about the dangers of
having functions with a lot of branching and specially when they lead to exit points.

- If there is a boolean check that returns `true` or `false` then just returning the
result of the boolean operation might achieve the same result.
