pub fn main() {
    let thingue = "batata";
    let thingue_2: Vec<char> = thingue.chars().collect();

    thingue.chars().for_each(|x| println!("{}", x));

    println!("-------");

    // for i in thingue.chars().step_by(2) {
    //     println!("{}", i)
    // }

    println!("{:?}", thingue);
    println!("{:?}", thingue.chars());
    println!("{:?}", thingue_2);
}
