use std::env::{args, Args};



fn main() {
    let mut args: Args = args();

    let first_arg = args.nth(1).unwrap();
    let operator = args.nth(0).unwrap().chars().next().unwrap();
    let second_arg = args.nth(0).unwrap();

    let first_number: f32 = first_arg.parse().unwrap();
    let second_number: f32 = first_arg.parse().unwrap();
    let result = operate(operator, first_number, second_number);
    let result_2 = operate_two(operator, first_number, second_number);
    println!("{:?}",output_formatter(operator, first_number, second_number, result));
    println!("{:?}",output_formatter(operator, first_number, second_number, result_2));
}


fn operate(operator: char, first_number: f32, second_number: f32) -> f32{
 if operator == '+' {
    first_number + second_number
 }else if operator == '-' {
    first_number - second_number
 }else if operator == '*' {clear
    first_number * second_number
 }else if operator == '/' {
    first_number / second_number
 }else {
     0.0
 }
}
 fn operate_two(operator: char, first_number: f32, second_number: f32)-> f32{
     match operator{
         '+' => first_number + second_number,
         '-' => first_number - second_number,
         '/' => first_number / second_number,
         '*' | 'x' | 'X' => first_number * second_number,
         _ => panic!("Invalid operator used. ") 
     }
 }

 fn output_formatter(operator: char, first_number: f32, second_number: f32, result: f32) -> String{
     format!("{} {} {} = {}", first_number, operator, second_number, result)
 }