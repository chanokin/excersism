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
]
export function decodedValue(color_codes: string[]) {
  if (color_codes == undefined || color_codes.length < 2) {
    throw new Error('undefined list of values');
  }
  function indexString(key: string) {
    return (COLORS.indexOf(key)).toString();
  }
  let color_values = color_codes.slice(0, 2).map(indexString);
  
  return parseInt(color_values.join(""));
}
