use gnuplot::{Figure, Caption, Color,PointSymbol};
use std::f64;
use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

struct complex {
    real : f64,
    imag : f64
}
impl complex{
    fn add(&mut self, b : &mut complex) -> complex{
        let mut c = complex{ real : 0.0,imag : 0.0};
        c.real = self.real + b.real;
        c.imag = self.imag + b.imag;
        c
    }
    fn sub(&mut self, b : &mut complex) -> complex{
        let mut c = complex{ real : 0.0,imag : 0.0};
        c.real = self.real - b.real;
        c.imag = self.imag - b.imag;
        c
    }

    fn mul(&mut self, b : &mut complex) -> complex{
        let mut c = complex{ real : 0.0,imag : 0.0};
        c.real = (self.real*b.real)-(self.imag*b.imag);
        c.imag = self.real*b.imag+self.imag*b.real;
        c
    }
    fn pow2(&mut self) -> complex{
        let mut c = complex{ real : 0.0,imag : 0.0};
        c.real = (self.real*self.real)-(self.imag*self.imag);
        c.imag = self.real*self.imag+self.imag*self.real;
        c
    }
    fn abs(&mut self) -> complex{
        let mut c = complex{ real : 0.0,imag : 0.0};
        c.real=(self.real*self.real+self.imag*self.imag).sqrt();
        c.imag=0.0;
        c
    }   

}

fn main() {
    let mut fg = Figure::new();
    let mut _real : Vec<f64> = Vec::new();
    let mut _imag : Vec<f64> = Vec::new();
    
    let mut a : f64 = -2.000;
    let mut b : f64 = -2.000;
    let mut i : i32 = 0;
    let digit : f64 = 0.002;
    let max_calc = 100;
    let mut writer = BufWriter::new(File::create("complex.csv").unwrap());
    while a < 2.000 {
        b=-2.0;
        while b < 2.000 {
            let mut cplx = complex{real:a,imag:b};
            
            i=0;
            let mut flag=1;
            let mut prev_cplx  = complex{real:0.0,imag:0.0};
            while i < max_calc { 
                let mut cplx_1=prev_cplx.pow2().add(&mut cplx);
                let abs_cplx=cplx_1.abs().real;
               // println!("{}",abs_cplx);
                if abs_cplx > 2.0 {
                   flag=0;
                    break;
                }
                prev_cplx = cplx_1;
                
                i+=1;
                //println!("{}",i);
            }
            if flag == 1 {
                println!("real={},imag={}",cplx.real,cplx.imag);
                _real.push(a);
                _imag.push(b);
                writer.write(a.to_string().as_bytes()).unwrap();
                writer.write(String::from(",").as_bytes()).unwrap();
                writer.write(b.to_string().as_bytes()).unwrap();
                writer.write(String::from("\r\n").as_bytes()).unwrap();
            }
           //println!("real={},imag={}",cplx.real,cplx.imag);
           // println!("real={},imag={}",a,b);
           
            b+=digit;
        }
        a+=digit;
    }
    fg.axes2d().points(&_real, &_imag, &[ Caption("out"), Color("green"),PointSymbol('.')]);
    fg.show(); 
}

