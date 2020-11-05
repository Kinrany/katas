const [fileOpt, fileName] = Deno.args;
if (fileOpt !== "--file") throw "Expected option --file";
if (!fileName) throw "Option --file expects file path";
const filePath = Deno.realPathSync(fileName);

const text = Deno.readTextFileSync(filePath);

const lines = text.split("\n");

type Day = {
  dayNumber: number;
  maxTemperature: number;
  minTemperature: number;
};

const parseDay = (line: string): Day | undefined => {
  const words = line.split(/\s+/).filter(Boolean);
  const dayNumber = parseInt(words[0], 10);
  const maxTemperature = parseFloat(words[1]);
  const minTemperature = parseFloat(words[2]);
  return isNaN(dayNumber) || isNaN(maxTemperature) || isNaN(minTemperature)
    ? undefined
    : { dayNumber, maxTemperature, minTemperature };
};

const days = lines.map(parseDay).filter((x): x is Day => !!x);

const minBy = <T>(arr: T[], fn: (x: T) => number): T | undefined => {
  if (arr.length === 0) return undefined;
  let minValue = arr[0];
  for (const val of arr) {
    if (fn(val) < fn(minValue)) {
      minValue = val;
    }
  }
  return minValue;
};

const temperatureSpread = (day: Day) => day.maxTemperature - day.minTemperature;

const minDay = minBy(days, temperatureSpread);
if (minDay === undefined) throw "No valid lines";
console.log(
  `Day ${minDay.dayNumber} has the smallest temperature spread of ${
    temperatureSpread(minDay)
  }.`,
);
