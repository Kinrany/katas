function chop(x: number, arr: number[]): number {
  let left = 0;
  let right = arr.length;
  const count = () => right - left;

  while (count() > 1) {
    const center = Math.floor((left + right) / 2);
    if (arr[center] > x) {
      right = center;
    } else if (arr[center] < x) {
      left = center;
    } else {
      return center;
    }
  }

  return left;
}

console.log(`chop(5, [1, 3, 5, 7]): ${chop(5, [1, 3, 5, 7])}`);
