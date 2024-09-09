fn main() {
    // created a string variable
    let name="Devank".chars();

    // a loop to iterate in a range similar to kotlin
    for i in 1..10 {
        println!("{}",i);
    } 

    // iterating in a string to print each character
    for i in name{
        print!("*{}-",i);
    }  

    // created a function and invokation
    add();
}

// function declaration 
fn add(){
    let a=10;
    let b=20;
    println!("Sum of two number :{}", (a+b));
}
