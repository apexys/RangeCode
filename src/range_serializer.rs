use std::io::Write;
use std::io::Read;
use std::io::BufWriter;
use std::io::BufReader;
use std::error::Error;
use num_bigint::BigUint;
use crate::range_encoder::RangeEncoder;
use crate::rational::Rational;
pub struct RangeSerializer{}

impl RangeSerializer{
    pub fn write(re: RangeEncoder, data: Rational, output: &mut dyn Write)-> Result<(), Box<dyn Error>>{
        let mut bfr = BufWriter::new(output);
        //Write file length
        RangeSerializer::write_u64(re.length, &mut bfr)?;
        //Write density map
        for i in 0 .. 256{
            RangeSerializer::write_u64(re.density_map[i], &mut bfr)?;
        }
        //Write numerator length
        let numerator_bytes = data.numerator.to_bytes_le();
        RangeSerializer::write_u64(numerator_bytes.len() as u64, &mut bfr)?;
        //Write numerator bytes
        bfr.write_all(&numerator_bytes)?;
        //Write denominator length
        let denominator_bytes = data.denominator.to_bytes_le();
        RangeSerializer::write_u64(denominator_bytes.len() as u64, &mut bfr)?;
        //Write denominator bytes
        bfr.write_all(&denominator_bytes)?;
        Ok(())
    }

    pub fn read(input: &mut dyn Read) -> Result<(RangeEncoder, Rational), Box<dyn Error>>{
        let mut brr = BufReader::new(input);
        //Create dummy objects
        let mut re = RangeEncoder{
            length: 0,
            density_map: [0u64;256]
        };
        let mut data = Rational::from(1, 1);

        //Read file length
        re.length = RangeSerializer::read_u64(&mut brr)?;
        //Read density map
        for i in 0 .. 256{
            re.density_map[i] = RangeSerializer::read_u64(&mut brr)?;
        }
        //Read numerator length
        let numerator_length = RangeSerializer::read_u64(&mut brr)?;
        //Read numerator bytes
        let mut numerator_bytes = vec![0u8; numerator_length as usize];
        brr.read_exact(&mut numerator_bytes)?;
        data.numerator = BigUint::from_bytes_le(&numerator_bytes);
        //Read denominator length
        let denominator_length = RangeSerializer::read_u64(&mut brr)?;
        //Read denominator bytes
        let mut denominator_bytes = vec![0u8; denominator_length as usize];
        brr.read_exact(&mut denominator_bytes)?;
        data.denominator = BigUint::from_bytes_le(&denominator_bytes);

        Ok((re, data))
    }

    fn write_u64(val: u64, output: &mut dyn Write)-> Result<(), Box<dyn Error>>{
        let mut temp = val;
        for _ in 0..8{
            output.write(&[(temp & 0xFF) as u8])?;
            temp = temp >> 8;
        }
        Ok(())
    }

    fn read_u64(input: &mut dyn Read) -> Result<u64, Box<dyn Error>>{
        let mut temp = 0u64;
        let mut buf: [u8;1] = [0u8;1];
        for _ in 0 .. 8{
            input.read_exact(&mut buf)?;
            temp = temp << 8;
            temp = temp | buf[0] as u64;
        }
        Ok(temp)
    }
}