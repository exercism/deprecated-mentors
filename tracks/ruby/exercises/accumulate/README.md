### Reasonable Solutions

```ruby
class Array
  def accumulate
    map { |i| yield(i) }
  end
end
```

### Talking Points
- Block syntax (`yield` vs `block.call`)
- `map` vs `each` with an object
