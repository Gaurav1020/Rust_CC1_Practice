fn main() {
    // create some strings
    let str1 = "Educative is the best platform!";
    let str2 = "Rust";
    let str3 = "Welcome to Edpresso";
    let str4 = "Programming";
  
    // create the matches
    let match1 = "is";
    let match2 = 'R';
    let match3 = "to";
    let match4 = "23";
  
    // find the matches and print byte indices
    println!(" {:?}", str1.find(match1));
    println!(" {:?}", str2.find(match2));
    println!(" {:?}", str3.find(match3));
    println!(" {:?}", str4.find(match4));
  }