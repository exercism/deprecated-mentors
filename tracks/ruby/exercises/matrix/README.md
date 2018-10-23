# Matrix

## Reasonable Solutions

```ruby
class Matrix
  attr_reader :rows, :columns
  def initialize(input)
    @rows = split_input(input)
    @columns = rows.transpose
  end

  private

  def split_input(string)
    string.lines.map { |line| split_line(line) }
  end

  def split_line(line)
    line.split.map(&:to_i)
  end
end
```

Many solutions will not split out the input parsing into methods like this. We don't yet have consensus among mentors if this is important or not. If the methods are split, the above example does demonstrate how the methods could be flagged as private. Some Ruby projects insist on marking as private methods that are not part of the public, tested interface.

## Common Suggestions

There's no need to overload a student with all of these suggestions, so here is a list ordered roughly by most important suggestions first:

* For students who wrote code that interleaves string manipulation with data persistance, encourage them to do those steps sequentially
* Suggest the student look at other methods on `String` that could simplify their code if they don't use `String#lines`
* More generally, for any solutions that has matching based on a regular expression or explicitly decleared string or substring, encourage them to find a solution that does not require either
* For solutions that do not iterate over the output of `String#lines`, suggestions common to the Hamming exercise may also work well
* For solutions that use `Array#transpose`, you can challenge the student to solve the problem without it, and vice versa
