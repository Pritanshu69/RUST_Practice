// fn main(){
//     println!("Hello World!");

//     another_func();
// }

// fn another_func() {
//     println!(", How are you?");
// }

fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
}