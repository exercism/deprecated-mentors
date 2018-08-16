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

- Use a single value for storing time, rather than storing hours and minutes seperately

### Talking Points

- `+()`, `-()`, `==()`
- `alias`
- Compound constructors
- Constants (too many, too few, etc)
- `divmod()`
