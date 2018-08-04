### Intro
This is the first core exercise. 

### Reasonable solutions

```ruby
class TwoFer
  def self.two_fer(name = 'you')
    "One for #{name}, one for me."
  end
end
```

### Common suggestions
- Suggest using a default value instead of any form of conditionals. 
- People often set the default of `name` to be `nil`. Ask if there something better they could do with the default value to avoid the conditional and make their code simpler.
- Suggest to remove 'return'

### Talking points
- Ruby's implicit returns
- Ruby default values
- Style preferences. With every thing else in order, this is a good opportunity to talk about style conventions like indentation, parameter parenthesis in method declarations and removing redundant comments.
https://github.com/rubocop-hq/ruby-style-guide   

### Mentoring notes
- A friendly standard answer about how this can be done in one line, and a 'hint: use a different default value' will be  all you need for maybe 90% of the submissions. 
- Most mentors seem to ignore the use of either `self.two_fer` or `class << self`. That is appropriate to where we are in the track. 
