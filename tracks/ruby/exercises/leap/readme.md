_Leap_ is one of the first side exercises, unlocked by _TwoFer_. 

## Reasonable Solutions

```ruby
class Leap
  year % 4 == 0 && (year % 100 != 0 || year % 400 == 0)
end
```
Variants: `module` instead of `class`; instantiate class.  


## Common suggestions
Copied from the Python notes by @yawpitch :
- There are just two cases that return true:
  - a year is a multiple of 4 *and not** 100
  - a year is a multiple of 4, 100, and 400
- _Order of operations_ matter:
  - 75% of all years *cannot* be leap years because they are not mulitples of 4; test `year % 4 == 0` first
  - 98.97% of all years that are multiples of 4 are not multiples of 100; test `year % 100 != 0` second
  - 1.03% of all years that are multiples of 4 are also multiples of 100 and 400; test `year % 400 == 0` third
- _Order of evaluation_ matters:
  ```ruby
  year % 4 == 0 && year % 100 != 0 || year % 400 == 0
  ```
  _looks_ right, but will force a year like 999 to be checked for being a multiple of 400 unnecessarily
- Eliminate duplicate work; no year should ever have to be checked multiple times for the same condition

## Talking points
* Logical operators `&&` `||` are more idiomatic than `return ... if ...`
