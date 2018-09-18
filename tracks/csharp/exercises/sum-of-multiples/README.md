### Reasonable solutions

```csharp
using System.Linq;
using System.Collections.Generic;

public static class SumOfMultiples
{
    public static int Sum(IEnumerable<int> inputs, int max)
        => inputs.SelectMany(
            input => Enumerable.Range(1, (max - 1) / input).Select(numTimes => numTimes * input)
            , (numtimes, multiple) => multiple).Distinct().Sum();
}
```

If the student is not yet ready for / comfortable with full-on Linq then something like the following would be appropriate:

```csharp
using System.Linq;
using System.Collections.Generic;

public static class SumOfMultiples
{
    public static int Sum(IEnumerable<int> inputs, int max)
    {
        var hs = new HashSet<int>();
        foreach (var input in inputs)
        {
            for (int ii = (max - 1) / input * input; ii > 0 ; ii -= input)
            {
                hs.Add(ii);
            }
        }

        return hs.Sum();
    }
 }
 ```

### Common suggestions

- There are dozens of equally valid linq constructions that can be used.
- expression bodied members are not to everybody's taste and should be optional

### Talking points

- Students often submit a non-performant solution.  Even if you think
that this is allowed under the premature optimisation tenet it is
still worth chalenging the student to produce a solution that will perform
against a test such as `SumOfMultiples.Sum(new[] { 1_999_999_999 }, 2_000_000_000)`
To handle large sets of multiples a more efficient algorithm may be required -
probably not of great interest to a csharp learner but perhaps worth mentioning.
- Submissions including the ones above are not robust in that they will fail for values
where large numbers of multiples are generated.  The "checked" modifier
can be mentioned to show how Sum() handles overflows and there may be scope
to position this earlier in the solution for a fast and more informative fail.  Alternatively long or perhaps
 BigInteger types can be employed to handle issues with values greater
 than int.MaxValue.

TODO a bullet proof solution for signed ints.