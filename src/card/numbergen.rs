use rand::Rng;
use crate::card::inttovec::number_to_vec;

pub fn generate() -> String{
    let mut rng = rand::thread_rng();

    let mut card:String= String::new();
    let mut number:Vec<u8>=vec![5,3,2,3,1,5]; //532315 - WattOhm Issuer Identification Number (IIN)

    // fill number with random digits
    let mut i:u8=0;
    let mut x:u8;
    while i<9 {
        x=rng.gen_range(0..10);
        number.push(x);
        i+=1;
    }

    //Luhn
    i=2;
    x=0;
    let mut y:u8;
    for &digit in &number {
        y = digit * i;

        if y >= 10 {
            for s in number_to_vec(y) {
                x += s;
            }
        }
        else {
        x += y;
        }

        if i==2{
            i=1;
        }
        else if i==1{
            i=2
        }
    }
    y=10-(x%10);
    number.push(y);

    // transform number:Vec to String
    for digit in number{
        card += &digit.to_string();
    }

    return card;
}