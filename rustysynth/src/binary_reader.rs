use std::error;
use std::io;
use std::slice;
use std::str;

#[allow(unused)]
#[non_exhaustive]
pub(crate) struct BinaryReader
{
}

impl BinaryReader
{
    pub(crate) fn read_i8<R: io::Read>(reader: &mut R) -> Result<i8, io::Error>
    {
        let mut data: [u8; 1] = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(i8::from_le_bytes(data))
    }

    pub(crate) fn read_u8<R: io::Read>(reader: &mut R) -> Result<u8, io::Error>
    {
        let mut data: [u8; 1] = [0; 1];
        reader.read_exact(&mut data)?;
        Ok(u8::from_le_bytes(data))
    }

    pub(crate) fn read_i16<R: io::Read>(reader: &mut R) -> Result<i16, io::Error>
    {
        let mut data: [u8; 2] = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(i16::from_le_bytes(data))
    }

    pub(crate) fn read_u16<R: io::Read>(reader: &mut R) -> Result<u16, io::Error>
    {
        let mut data: [u8; 2] = [0; 2];
        reader.read_exact(&mut data)?;
        Ok(u16::from_le_bytes(data))
    }

    pub(crate) fn read_i32<R: io::Read>(reader: &mut R) -> Result<i32, io::Error>
    {
        let mut data: [u8; 4] = [0; 4];
        reader.read_exact(&mut data)?;
        Ok(i32::from_le_bytes(data))
    }

    pub(crate) fn read_four_cc<R: io::Read>(reader: &mut R) -> Result<String, Box<dyn error::Error>>
    {
        let mut data: [u8; 4] = [0; 4];
        reader.read_exact(&mut data)?;

        for i in 0..4
        {
            let value = data[i];
            if !(32 <= value && value <= 126)
            {
                data[i] = 63; // '?'
            }
        }
        
        Ok(str::from_utf8(&data)?.to_string())
    }

    pub(crate) fn read_fixed_length_string<R: io::Read>(reader: &mut R, length: i32) -> Result<String, Box<dyn error::Error>>
    {
        let mut data: Vec<u8> = vec![0; length as usize];
        reader.read_exact(&mut data)?;

        let mut actual_length: i32 = 0;
        for i in 0..length
        {
            if data[i as usize] == 0
            {
                break;
            }
            actual_length += 1;
        }

        Ok(str::from_utf8(&data[0..actual_length as usize])?.to_string())
    }

    pub(crate) fn discard_data<R: io::Read>(reader: &mut R, size: i32) -> Result<(), io::Error>
    {
        let mut data: Vec<u8> = vec![0; size as usize];
        reader.read_exact(&mut data)
    }

    pub(crate) fn read_wave_data<R: io::Read>(reader: &mut R, size: i32) -> Result<Vec<i16>, io::Error>
    {
        let length = size as usize / 2;
        let mut samples: Vec<i16> = vec![0; length];

        let ptr = samples.as_mut_ptr() as *mut u8;
        let data = unsafe { slice::from_raw_parts_mut(ptr, size as usize) };
        reader.read_exact(data)?;

        Ok(samples)
    }
}
