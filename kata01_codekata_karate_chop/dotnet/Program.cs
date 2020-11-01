using System;

namespace karate_chop
{
  class Program
  {
    static int Chop(int x, int[] arr)
    {
      int left = 0;
      int right = arr.Length;
      Func<int> count = () => right - left;

      while (count() > 1)
      {
        int center = (left + right) / 2;
        if (arr[center] > x)
        {
          right = center;
        }
        else if (arr[center] < x)
        {
          left = center;
        }
        else
        {
          return center;
        }
      }

      return left;
    }

    static void Main()
    {
      Console.WriteLine($"chop(5, [1, 3, 5, 7]): {Chop(5, new int[] { 1, 3, 5, 7 })}");
    }
  }
}
