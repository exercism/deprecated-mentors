### Reasonable solutions

The use of a custom exception, the choice of variable names that relate to the story not the maths (`postion` rather than `number`), and the user of `cover?` all make this an excellent solution.
```
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
