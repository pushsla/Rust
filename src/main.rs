
trait HasArea{
    fn area(&self) -> f64;
}

struct Circle{
    x: f64,
    y: f64,
    r: f64,
}

impl Circle{
    fn new(x: f64, y: f64, r: f64) -> Circle{
        Circle{x, y, r}
    }
    fn set_x(&mut self, x: f64){
        self.x = x;
    }
    fn set_y(&mut self, y: f64){
        self.y = y;
    }
    fn set_r(&mut self, r: f64){
        self.r = r;
    }
}

impl HasArea for Circle{
    fn area(&self) -> f64{
        std::f64::consts::PI * (self.r * self.r)
    }
}

struct Square{
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Square{
    fn new(x: f64, y: f64, width: f64, height: f64) -> Square{
        Square{x, y, width, height}
    }
}

impl HasArea for Square{
    fn area(&self) -> f64{
        self.width*self.height
    }
}

fn print_area<T: HasArea>(fig: &T){
    println!("{}", fig.area());
}

fn main(){
    let mut c1 = Circle::new(0f64, 0f64, 10f64);
    let s1 = Square::new(0f64, 10f64, 20f64, 31f64);
    print_area(&c1);
    c1.set_r(100f64);
    print_area(&c1);
    print_area(&s1);
}