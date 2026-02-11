export function format(name: string, _number: number): string {
  let str_number:string = _number.toString();
  if (str_number.length == 0 || _number < 1) {
    return "";
  }

  let number_ordinal: string = "th";
  
  if (str_number.endsWith("1")) {
    number_ordinal = "st";
  } else if (str_number.endsWith("2")) {
    number_ordinal = "nd";
  } else if (str_number.endsWith("3")) {
    number_ordinal = "rd";
  }

  if (str_number.length > 1 && str_number.slice(-2, -1) == "1") {
    number_ordinal = "th";
  }

  return `${name}, you are the ${_number}${number_ordinal} customer we serve today. Thank you!`;
}
