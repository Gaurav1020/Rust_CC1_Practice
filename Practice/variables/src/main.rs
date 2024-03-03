const STARTING_MISSILES:i32 = 8;
const READY_AMOUNT:i32 = 2;
fn main() {
    let (mut missiles, ready):(i32,i32)= (STARTING_MISSILES, READY_AMOUNT);
    // const STARTING_MISSILES:i32 = 8; //Declaring const here works as well as const variables are processed during compile time in their respective scopes
    // const READY_AMOUNT:i32 = 2; // 
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles-ready; 
    println!("{} missiles left", missiles);
}
