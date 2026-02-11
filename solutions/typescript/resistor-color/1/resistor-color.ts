export const COLORS = [
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

export const colorCode = (color: string) => {
  if (color == undefined) {
    throw new Error('Color argument not defined!');
  }
  
  let code = COLORS.indexOf(color.toLowerCase());
  
  if ( code < 0 ) {
    throw new Error(`Color not found in map (${color})!`);
  }

  return code;
}


