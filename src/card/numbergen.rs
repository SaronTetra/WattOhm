use rand::Rng;
pub fn generate() -> String{
    let mut rng = rand::thread_rng();
    let mut card:String= String::new();

    // declare number
    let mut number:Vec<u8>=vec![5,3,2,3,1,5]; //532315 - WattOhm Issuer Identification Number (IIN)
    let mut i:i8=0;
    let mut x:u8;

    // fill number with random digits
    while i<9 {
        x=rng.gen_range(0..10);
        number.push(x);
        i+=1;
    }
    // transform number:Vec to String
    for digit in number{
        card += &digit.to_string();
    }

    return card;
}