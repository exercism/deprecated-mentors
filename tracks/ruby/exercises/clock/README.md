**Note:** This exercise changes a lot depending on which version the person has solved.

### Reasonable Solutions

```ruby
class Clock
  MINS_PER_HOUR = 60
  MINS_PER_DAY = 1440

  def initialize(hour: 0, minute: 0)
    @time = (hour * MINS_PER_HOUR + minute) % MINS_PER_DAY
  end

  def to_s
    "%02d:%02d" % time.divmod(MINS_PER_HOUR)
  end

  def +(other)
    Clock.new(minute: time + other.time)
  end

  def -(other)
    Clock.new(minute: time - other.time)
  end

  def ==(other)
    time == other.time
  end
  alias :eql? :==

  protected
  attr_reader :time
end
```

### Common Suggestions

- Clocks are minute-trackers - hours only matter when formatting for human reading. Use a single value for storing time, rather than storing hours and minutes seperately. (note: to be more precise, clocks track seconds - think `epoch` - but for this exercise we don't care about seconds)
- Using `protected` to allow you to access another instance’s `time` or `minutes` in `==` without making the attributes public.
- Using constants instead of magic numbers-
- (Advanced) Make Clock a [value object](https://www.sitepoint.com/value-objects-explained-with-ruby/) - they are instances that are ideally immutable (use `freeze` in the initializer) and do equality through their value, not their identity. Also consider supporting [hash equality](https://www.belighted.com/blog/overriding-equals-equals).

### Talking Points

- `+()`, `-()`, `==()`
- `alias`
- Compound constructors
- Number formatting (`"%02d:%02d"` etc)
- Constants (too many, too few, etc)
- `divmod()`
