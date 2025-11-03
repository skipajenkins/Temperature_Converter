fn main() {

    let _a = convert_to_celsius(0.0);

    let _b = convert_to_fahrenheit(0.0);
}

fn convert_to_celsius(temp:f32){
   
    const CONST_A : f32 = 32.0;
    const CONST_B : f32 = 0.55555555555;

    let temp : f32 = (temp- CONST_A) * (CONST_B);
    println!("Temperature in celsius: {temp}");
}

fn convert_to_fahrenheit(temp:f32){

    
    const CONST_A : f32 = 32.0;
    const CONST_C : f32 = 1.8;
    
    let temp : f32 = temp*(CONST_C)+ CONST_A;
    println!("Temperature in fahrenheit: {temp}");
}

