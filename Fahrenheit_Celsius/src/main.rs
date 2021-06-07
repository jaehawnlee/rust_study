use std::io;

fn main() {

    let mut temper = String::new();
    temper.clear();

    io::stdin().read_line(&mut temper)
        .expect("Failed to read line");
    
    //vec[0] = c or f, vec[0] = temper
    let vec : Vec<&str> = temper.split('_').collect();
    let f_or_c = vec[0];
    let temper : f32 = vec[1].trim().parse().unwrap();

    let mut changed_unit = "";
    let mut result = 0.00;

    if f_or_c == "c"{
        changed_unit = "f";
        result = celsius_to_fahrenhit(temper);
    }
    else if f_or_c == "f"{
        changed_unit = "c";
        result = fahrenhit_to_celsius(temper);
    };
    

    println!("result {} to {}: {:.2}\n", f_or_c, changed_unit, result);
}

fn celsius_to_fahrenhit(c: f32) -> f32{
     c * 1.8 + 32.0
}

fn fahrenhit_to_celsius(f: f32) -> f32{
    (f-32.0)/1.8
}
