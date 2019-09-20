use std::io::Read;
use std::io::Seek;
use std::error::Error;
use crate::rational::Rational;

pub struct RangeEncoder{
    pub density_map: [u64;256],
    pub length: u64
}

trait Input: Read + Seek{}

impl RangeEncoder{
    fn density(&self, symbol: usize) -> u64{
        let mut result = 0;
        for i in 0 .. symbol{
            result += self.density_map[i];
        }
        return result;
    }

    pub fn initialize(input: &mut dyn Read) -> Result<RangeEncoder, Box<dyn Error>>{
        let mut re = RangeEncoder{
            density_map: [0u64; 256],
            length: 0
        };
       for b in input.bytes(){
           let real_b = b?;
            re.length += 1;
            re.density_map[real_b as usize] += 1;
        }
        return Ok(re);
    }

    pub fn compress(&self, input: &mut dyn Read) -> Result<Rational, Box<dyn Error>>{
        let densities_bases = (0..256).into_iter().map(|i| Rational::from(self.density(i), self.length)).collect::<Vec<Rational>>();
        let densities_ranges = (0..256).into_iter().map(|i| Rational::from(self.density_map[i],self.length)).collect::<Vec<Rational>>();

        for i in 0 ..256{
            let base: f64 = densities_bases[i].clone().into();
            let range: f64 = densities_ranges[i].clone().into();
            eprintln!("{} => {}..{}", i, base, base + range);
        }


        let mut iter = input.bytes();
        let first_byte = iter.next().unwrap()?;
        let mut base = densities_bases[first_byte as usize].clone();
        let mut range = densities_ranges[first_byte as usize].clone();
        let mut ctr = 0u64;
        eprintln!("");
        eprintln!("0 bytes processed");
        for b in input.bytes(){
            ctr += 1;
            if ctr % 100 == 0{
                eprint!("\r{} bytes processed", ctr);
            }
            let real_b = b? as usize;
            base += densities_bases[real_b].clone() * range.clone();
            range *= densities_ranges[real_b].clone();
        }
        eprintln!("\r{} bytes processed", ctr);
        range = range * Rational::from(1,2);
        return Ok(base + range);
    }

    pub fn decompress(&self, input: Rational) -> Vec<u8>{
        let densities_bases = (0..256).into_iter().map(|i| Rational::from(self.density(i), self.length)).collect::<Vec<Rational>>();
        let densities_ranges = (0..256).into_iter().map(|i| Rational::from(self.density_map[i],self.length)).collect::<Vec<Rational>>();
        let find_range_index = |r: &Rational| {
            for i in 0 .. 256 {
                if &densities_bases[i] > r{
                    if i == 0{
                        return 0;
                    }else{
                        return i - 1;
                    }
                }
            }
            return 0;
        };
        let mut temp = input.clone();
        let mut base;
        let mut range;
        let mut result = Vec::new();
        for _ in 0 .. self.length{
            let i = find_range_index(&temp);
            base = &densities_bases[i as usize];
            range = &densities_ranges[i as usize];
            temp = ((temp - base.clone()) / range.clone()).clone();
            result.push(i as u8);
        }
        return result;
    }
}