use std::fmt;
use num_bigint::BigUint;
use num_traits::cast::ToPrimitive;
use std::rc::Rc;

#[derive(Clone)]
pub struct Rational{
    pub numerator: BigUint,
    pub denominator: BigUint,
    opcount: u8
}

impl fmt::Display for Rational{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f,"{}",self.numerator)?;
        if self.denominator > BigUint::from(0u32){
            write!(f,"/{}", self.denominator)?;
        }
        write!(f,"")
    }
}

impl Into<f64> for Rational{
    fn into(self) -> f64{
        return self.numerator.to_f64().unwrap() / self.denominator.to_f64().unwrap();
    }
}

fn gcd(a: &BigUint, b: &BigUint) -> BigUint{
    if b == &BigUint::from(0u32){
        return a.clone();
    }else{
        return gcd(b, &(a % b));
    }
}

impl Rational{
    fn simplify(self) -> Self{
        if self.opcount < 10{
            return self;
        }
        let divisor = gcd(&self.numerator, &self.denominator);
        if divisor == BigUint::from(1u32){
            return self;
        }else{
            let numerator = self.numerator / &divisor;
            let denominator = self.denominator / &divisor;
            Rational{
                numerator,
                denominator,
                opcount: 0
            }
        }
    }

    fn simplify_mut(&mut self){
        if self.opcount < 10{
            return;
        }
        let divisor = gcd(&self.numerator, &self.denominator);
        if divisor == BigUint::from(1u32){
            return;
        }else{
            self.numerator /= &divisor;
            self.denominator /= &divisor;
            self.opcount = 0;
        }
    }

    pub fn from(numerator: u64, denominator: u64) -> Self{
        Rational{
            numerator: BigUint::from(numerator),
            denominator: BigUint::from(denominator),
            opcount: 0
        }.simplify()
    }
}

impl std::ops::Add for Rational{
    type Output = Self;
    fn add(self, rhs: Self) -> Self{
        let a1 = self.numerator * &rhs.denominator;
        let b1 = rhs.numerator * &self.denominator;
        let ab2 = self.denominator * rhs.denominator;
        let frac = Rational{
            numerator: a1 + b1,
            denominator: ab2,
            opcount: self.opcount + rhs.opcount
        };
        return frac.simplify();
    }
}

impl std::ops::AddAssign for Rational{
    fn add_assign(&mut self, other: Self){
        self.numerator *= &other.denominator;
        self.denominator *= &other.denominator;
        self.numerator += other.numerator * &self.denominator;
        self.opcount += other.opcount;
        self.simplify_mut()
    }
}

impl std::ops::Sub for Rational{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self{
        let a1 = self.numerator * &rhs.denominator;
        let b1 = rhs.numerator * &self.denominator;
        let ab2 = self.denominator * rhs.denominator;
        let frac = Rational{
            numerator: a1 - b1,
            denominator: ab2,
            opcount: self.opcount + rhs.opcount
        };
        return frac.simplify();
    }
}

impl std::ops::Mul for Rational{
    type Output = Self;
    fn mul(self, rhs: Self) -> Self{
        let frac = Rational{
            numerator: self.numerator * rhs.numerator,
            denominator: self.denominator * rhs.denominator,
            opcount: self.opcount + rhs.opcount
        };
        return frac.simplify();
    }
}

impl std::ops::MulAssign for Rational{
    fn mul_assign(&mut self, other: Self){
        self.numerator *= other.numerator;
        self.denominator *= other.denominator;
        self.opcount += other.opcount;
        self.simplify_mut();
    }
}

impl std::ops::Div for Rational{
    type Output = Self;
    fn div(self, rhs: Self) -> Self{
        let frac = Rational{
            numerator: self.numerator * rhs.denominator,
            denominator: self.denominator * rhs.numerator,
            opcount: self.opcount + rhs.opcount
        };
        return frac.simplify();
    }
}

impl std::cmp::PartialEq for Rational{
    fn eq(&self, other: &Self) -> bool{
        return self.denominator == other.denominator && self.numerator == other.numerator;
    }
}

impl std::cmp::PartialOrd for Rational{
    fn partial_cmp(&self, other: &Self) -> std::option::Option<std::cmp::Ordering>{
        let a1 = &self.numerator * &other.denominator;
        let a2 = &other.numerator * &self.denominator;
        return a1.partial_cmp(&a2);
    }
}