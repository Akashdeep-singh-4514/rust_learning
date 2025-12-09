fn main() {

    // ================integer
    let int1:i32=4;
    println!("=================INTEGER=================");
    println!("integer 1 {}",int1);


    println!("=================FLOAT=================");
    // ==================float
    let float1:f32=2.0;
    println!("float 1 {}", float1);

    //-------------numerical operations

    println!("=================NUMERICAL OPERATIONS=================");
    let sum = 5 + 10;

    println!("sum of 5 and 10 is {}",sum);
    // subtraction
    let difference = 95.5 - 4.3;
    println!("difference of 95.5 and 4.3 is {}",difference);
    // multiplication
    let product = 4 * 30;
    println!("product of 4 and 30 is {}",product);
    // division
    let quotient = 56.7 / 32.2;
    println!("quotient of 56.7 and 32.2 is {}",quotient);
    let truncated = -5 / 3; // Results in -1
    println!("truncated of -5 and 3 is {}",truncated);
    // remainder
    let remainder = 43 % 5;
    println!("remainder of 43 and 5 is {}",remainder);


    println!("=================BOOLEAN=================");
    let boolean1:bool=true;
    println!("boolean 1 {}",boolean1);

    println!("=================CHARACTER=================");
    let c = 'z';
    println!("character 1 c: {}",c);
    let z: char = 'ℤ'; // with explicit type annotation
    println!("character 2 z: {}",z);
    let heart_eyed_cat = '😻';
    println!("character 3 heart_eyed_cat: {}",heart_eyed_cat);

    println!("=================STRING=================");
    let string1:String="hello".to_string();
    println!("string 1 {}",string1);
    let string2:String="world".to_string();
    println!("string 2 {}",string2);
    let string3:String=format!("{}-{}!",string1,string2);
    println!("string 3 {}",string3);

    println!("=================TUPLE=================");
    let tuple1:(i32,f32,char,String)=(1,2.0,'a',"hello".to_string());
    println!("tuple 1 debug print {:?}",tuple1);
    println!("tuple 1 pretty print {:#?}",tuple1);
    println!("first element of tuple1 is {}",tuple1.0);

    let (x,y,z,w)=tuple1;
    println!("x: {}",x);
    println!("y: {}",y);
    println!("z: {}",z);
    println!("w: {}",w);


    println!("=================ARRAY=================");
    let array1:[i32;4]=[1,2,3,4];
    println!("array 1 debug print {:?}",array1);
    println!("array 1 pretty print {:#?}",array1);
    println!("first element of array1 is {}",array1[0]);
    println!("second element of array1 is {}",array1[1]);
    println!("third element of array1 is {}",array1[2]);



}
