trait Convertible<T> {                                      // later we will reduce `T` to concrete types
    fn convert(&self) -> T;
}
 
struct Celsius(f64);
struct Fahrenheit(f64);
 
impl Convertible<Fahrenheit> for Celsius {                  // notice `Fahrenheit`
    fn convert(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 1.8 + 32.0)
    }
}
 
impl Convertible<Celsius> for Fahrenheit {                  // notice `Celsius`
    fn convert(&self) -> Celsius {
        Celsius((self.0 - 32.0) / 1.8)
    }
}
 
fn main() {
    let celsius = Celsius(100.0);
    let converted_fahrenheit: Fahrenheit = celsius.convert();
    println!("100°C in Fahrenheit: {:.2}°F", converted_fahrenheit.0);
 
 
    let fahrenheit = Fahrenheit(212.0);
    let converted_celsius: Celsius = fahrenheit.convert();
    println!("212°F in Celsius: {:.2}°C", converted_celsius.0);
}