using System;
using System.IO;
using System.Linq;
using System.Text.RegularExpressions;
using CommandLine;

namespace data_munging
{
  class Program
  {
    class Args
    {
      [Option('f', "file", Required = true)]
      public string FilePath { get; set; }
    }

    readonly struct Option<T>
    {
      public readonly T Value;
      public readonly bool IsSome;

      Option(T value) { Value = value; IsSome = true; }

      public static Option<T> Some(T value) => new Option<T>(value);
      public static readonly Option<T> None = new Option<T>();
    }

    struct Day
    {
      public uint DayNumber;
      public uint MaxTemperature;
      public uint MinTemperature;

      public uint TemperatureSpread() => MaxTemperature - MinTemperature;

      public static Option<Day> Parse(string s)
      {
        var words = new Regex(@"\s+")
          .Split(s)
          .Where(s => !String.IsNullOrWhiteSpace(s))
          .ToArray();

        if (words.Length < 3) return Option<Day>.None;

        // Declare variables before parsing.
        // Initialize with values to placate the compiler.
        uint dayNumber = 0;
        uint maxTemperature = 0;
        uint minTemperature = 0;

        var success = uint.TryParse(words[0], out dayNumber)
          && uint.TryParse(words[1], out maxTemperature)
          && uint.TryParse(words[2], out minTemperature);

        return success
          ? Option<Day>.Some(new Day
          {
            DayNumber = dayNumber,
            MaxTemperature = maxTemperature,
            MinTemperature = minTemperature
          })
          : Option<Day>.None;
      }
    }

    static void Main(string[] args)
    {
      Parser.Default.ParseArguments<Args>(args).WithParsed<Args>(args =>
      {
        var days = File
          .ReadAllLines(args.FilePath)
          .Select(Day.Parse)
          .Where(o => o.IsSome)
          .Select(o => o.Value);

        // Tuples are ordered by first, then second, then third argument.
        // We use `idx` as a tie-breaker to avoid implementing IComparable for Day.
        var minDay = days
          .Select((day, idx) => (day.TemperatureSpread(), idx, day))
          .ToArray()
          .Min()
          .Item3;

        Console.WriteLine($"Day {minDay.DayNumber} has the smallest temperature spread of {minDay.TemperatureSpread()}.");
      });
    }
  }
}
