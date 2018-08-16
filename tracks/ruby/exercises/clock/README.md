### Reasonable Solutions

```ruby
class Clock
  MINS_PER_HOUR = 60
  HOURS_PER_DAY = 24
  DAILY_MINS = HOURS_PER_DAY * MINS_PER_HOUR

  def initialize(hour: 0, minute: 0,
                 time: (hour * MINS_PER_HOUR + minute))
    @time = time % DAILY_MINS
  end

  def to_s
    "%02d:%02d" % time.divmod(MINS_PER_HOUR)
  end

  def +(clock)
    Clock.new(time: time + clock.time)
  end

  def -(clock)
    Clock.new(time: time - clock.time)
  end

  def ==(clock)
    time == clock.time
  end
  alias :eql? :==

  protected
  attr_reader :time
end
```

### Common Suggestions

- Clocks are minute-trackers - hours only matter when formatting for human reading. Use a single value for storing time, rather than storing hours and minutes seperately.
- Value objects - instances that are ideally immutable and do equality through their value, not their identity
- Using `protected` to allow you to access another instance’s `time` or `minutes` in `==` without making the attributes public
- Using constants instead of magic numbers

### Talking Points

- `+()`, `-()`, `==()`
- `alias`
- Compound constructors
- Number formatting (`"%02d:%02d"` etc)
- Constants (too many, too few, etc)
- `divmod()`
