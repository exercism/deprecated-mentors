### Reasonable Solutions

```
ALPHABET_SIZE = 26

def pangram?(sentence)
  sentence.downcase.scan(/[a-z]/).uniq.size == ALPHABET_SIZE
end
```

### Common suggestions

- Use `scan` instead of `gsub`.
