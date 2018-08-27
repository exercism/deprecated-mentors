### Reasonable Solutions

```
ALPHABET_SIZE = 26

def pangram?(sentence)
  sentence.downcase.scan(/[a-z]/).uniq.size == ALPHABET_SIZE
end
```

```
ALPHABET_26 = ("a".."z").to_a

def self.pangram?(sentence)
  text = sentence.downcase.chars
  (ALPHABET_26 - text).empty?
end

```

### Common suggestions

- Use `scan` instead of `gsub`.
- Use array substraction


### Talking points
- The naming of the Alphabet. The English language has 26 letters, but, as I learned yesterday, the Italian and Hawaiian has not. I found it hard to come up with a better name that wasn't ambiguous or unclear ('Latin', 'English', 'ISO Basic Latin', 'Western European' et cetera). Hence: `Alphabet_26` in solution 2. The naming in this exercise offers opportunities to raise awareness about culture biased code. 

