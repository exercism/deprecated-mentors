### Intro
TwoFer -> Hamming -> RainDrops -> Difference of Squares -> **Grains** -> Scrabble Score 

The instructions for the students suggest(!) that the first solution would use an iteration, and that the next step would be optimizing (with maths) for performance and/or readability.
At this point, students should have used 'basic' and 'advanced' Enumberable methods for iterations in previous exercises (core: Hamming, Difference of Squares, side: Series).  


### Reasonable solutions

The use of a custom exception, the choice of variable names that relate to the story not the maths (`position` rather than `number`), and the use of `cover?` all make this an excellent solution.

```ruby
module Grains
  SQUARES = (1..64)

  def self.square(position)
    raise BoardPositionError unless SQUARES.cover? position
    2 ** (position - 1)
  end

  def self.total
    square(SQUARES.max) * 2 - 1
  end
end

class BoardPositionError < ArgumentError
  def initialize(message = "This position doesn't exist on this board")
    super
  end
end
```

### Common suggestions 
- With a compound conditional `(< 1 || > 64)` -> Extract the compound conditional in a separate method
- With a range conditional -> `cover?` has better performance than something like `between` (because `cover?` doesn't iterate over the range but calculates).

- With a non-maths solution -> `inject` or `reduce` are sub-optimal, suggest `sum` (Ruby 2.4+) 


### Talking points
- Naming things is so relevant for this exercise. What does the magic number 64 means? Parameter name: is it a 'number' or...? 
- Constants (and naming them)

