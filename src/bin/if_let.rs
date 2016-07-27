fn main() {
    let number = Some(7);
    let letter:Option<i32> = None;
    let emoticon:Option<i32> = None;

    if let Some(i) = number{
    	println!("Matched {:?}", i);
    }

    if let Some(i) = letter{
    	println!("Matched {:?}", i);
    }else{
    	println!("Not matched a number, let's go with a letter.");
    }

    let i_like_letters = false;

    if let Some(i) = emoticon{
    	println!("Matched {:?}", i);
    }else if i_like_letters {
    	println!("Not matched a number, let's go with a letter.");
    }else{
    	println!("Not matched a number, I don't like letters, let's go with an emoticon :) !");
    }

    number.map(|i| {
    	println!("{:?}", i)
    });

}