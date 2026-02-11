const COLORS = [
    "black",
    "brown",
    "red",
    "orange",
    "yellow",
    "green",
    "blue",
    "violet",
    "grey",
    "white"
];

const MULTIPLIERS = [
  "",
  "kilo",
  "mega",
  "giga",
]

export function decodedResistorValue(color_codes: string[]) {
  if (color_codes == undefined || color_codes.length < 3) {
    throw new Error('Color codes array is not well defined');
  }
  function indexString(color: string) {
    return COLORS.indexOf(color).toString();
  }
  let base_value = parseInt(color_codes.slice(0, 2).map(indexString).join(""));
  let exponent = COLORS.indexOf(color_codes[2]);
  
  if (color_codes[1] == "black") {
    base_value /= 10;
    exponent += 1;
  } else if (exponent % 3 != 0 && exponent % 2 == 0) {
    base_value *= 10
    exponent -= 1;
  }
  
  if (exponent < 3) {
    base_value *= Math.pow(10, exponent);
  }
  
  let multiplier = MULTIPLIERS[Math.floor(exponent/3)];

  return `${base_value} ${multiplier}ohms`;
}
