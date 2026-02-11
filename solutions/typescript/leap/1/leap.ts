export function isLeap(year: number) {
  year = Math.floor(year);
  
  let is_div_by_4 = (year % 4) == 0;
  
  
  if (is_div_by_4) {
    let is_div_by_100 = (year % 100) == 0;
    
    if (is_div_by_100) {
      if (year % 400 == 0) {
        return true;
      }
      
      return false;
    }
    
    return true;
  }

  return false;
}
