pub mod primitive;
pub mod derived;
//the compounds can hold multiple values
pub (crate) fn tuple () {
    let person: (&str, i32, f64, char, bool) = ("timothy", 12, 12.4, A, true); // defining a tuple with a string, integer,float and char
    //
    let name: &str = person.0;
    let age: i32 = person.1;
    let height: f64 = person.2;
    let bloodtype: char = person.3;
    let beauty: bool = person.4;
        println!("{:?}", name);
        println!("{:?}", age);
        println!("{:?}", height);
        println!("{:?}", bloodtype);
        println!("{:?}", beauty);
}

pub (create) fn array (){
    // it only stores one data type
    //assigning items into the array
    let people_name: [&str; 3] = ["timmy", "nathan", "titi"];
    let people_age: [i32; 3] = [12,2,3];
    let people_height: [f64; 3] = [6.3,5.4,5.6];
    let people_bloodtype: [char;3] = ['A', 'B', 'A'];
    let people_beauty:  [bool;3] = [true,true,false];
    //assigning variables from our array to people in tuple
    let person1: (&str, i32, f64, char, bool) = (
        people_name[0],
        people_age[0],
        people_height[0],
        people_bloodtype[0],
        people_beauty[0],
        );
    let person2: (&str, i32, f64, char, bool) = (
        people_name[2],
        people_name[2],
        people_height[2],
        people_bloodtype[2],
        people_beauty[2],
    );

    println;
        "1st person's details: Name "
}