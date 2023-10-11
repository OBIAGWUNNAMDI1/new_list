struct Car{
    Name: String,
    tyres:i8,
    colour: String,

}
fn _build_car(mut car: Car, colour: String){
    car.colour = String::from("Red");
    println!("The car name is {}", car.Name );
    println!("The car has {} number of tyres", car.tyres );

}
fn main() {
    let Name = String::from("Toyota");
    let car = Car{Name, tyres: 3, colour: String::from("Blue")};
    let color = String::from("White");
    _build_car(car, color);
}