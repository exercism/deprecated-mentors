### Reasonable solutions
#### struct based

```csharp
using System;

public struct Clock
{
    private const int HoursPerDay = 24;
    private const int MinutesPerHour = 60;

    public Clock(int hours, int minutes)
    {
        Hours = Mod((hours * MinutesPerHour + minutes) / (float)MinutesPerHour, HoursPerDay);
        Minutes = Mod(minutes, MinutesPerHour);
    }

    public int Hours { get; }

    public int Minutes { get; }

    public Clock Add(int minutesToAdd) => new Clock(Hours, Minutes + minutesToAdd);

    public Clock Subtract(int minutesToSubtract) => new Clock(Hours, Minutes - minutesToSubtract);

    public override string ToString() => $"{Hours:00}:{Minutes:00}";

    private static int Mod(double x, double y) => (int)(((x % y) + y) % y);
}
```

#### class based
If you implment Clock as a class you have to handle the tricky rules of equality
```csharp
public class Clock
{
    private readonly int timeInMinutes;
    
    public Clock(int hours, int minutes)
    {
        timeInMinutes = AdjustClock(hours * 60 + minutes, 0);
    }

    public int Hours => timeInMinutes / 60;
    public int Minutes => timeInMinutes % 60;
    
    public Clock Add(int minutesToAdd) => new Clock(Hours, Minutes + minutesToAdd);

    public Clock Subtract(int minutesToSubtract) => Add(-minutesToSubtract);

    public override string ToString()
    {
        return $"{Hours:D2}:{Minutes:D2}";
    }

    private int AdjustClock(int minutesIn, int adjustmenttMinutes)
    {
        return (24 * 60 + ( minutesIn + adjustmenttMinutes) % (24 * 60)) % (24 * 60);
    }

    private bool Equals(Clock other)
    {
        return timeInMinutes == other.timeInMinutes;
    }

    public override bool Equals(object obj)
    {
        if (ReferenceEquals(null, obj)) return false;
        if (ReferenceEquals(this, obj)) return true;
        if (obj.GetType() != this.GetType()) return false;
        return Equals((Clock) obj);
    }

    public override int GetHashCode()
    {
        return timeInMinutes;
    }
}
```

### Common suggestions

- The clock can be implemented as a struct or a class.  The main practical advantage of the struct
 in this exercise is that equality is handled automatically for the struct
- To pad the string "00" and "D2" (where D stands for decimal) are equally valid

### Talking points

- The student may not be aware of the algorithm to handle the modulus with negative numbers.  This is not
a C# issue but the algorithm is worth mentioning as it improves the quality of the code
- `private const`s are preferred to `readonly` fields as they are more idiomatic and, not that it matters
much here, more performant
- Converting the time to minutes simpifies processing.  Again, not a particularly C# point
- `IEquatable` does not buy you anything in this example.  IEquatable allows an object to signal
that callers such as a generic collection can use a typed implementation of `Equals()` to test
equality.  In the case of classes this simply saves a call through `Equals(object)'.  In the case of
structs there is also the issue of boxing