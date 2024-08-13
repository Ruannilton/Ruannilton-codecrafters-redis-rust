use std::{
    fs::File,
    io::{Read, Seek},
};

use crate::redis::redis_error::RedisError;

pub trait FileExt {
    fn next_string(&mut self, len: usize) -> Result<String, RedisError>;
    fn next_u64(&mut self) -> Result<u64, RedisError>;
    fn next_u32(&mut self) -> Result<u32, RedisError>;
    fn next_u8(&mut self) -> Result<u8, RedisError>;

    fn next_i32(&mut self) -> Result<i32, RedisError>;
    fn next_i16(&mut self) -> Result<i16, RedisError>;
    fn next_i8(&mut self) -> Result<i8, RedisError>;
    fn peek(&mut self) -> Result<u8, RedisError>;
}

impl FileExt for File {
    fn next_string(&mut self, len: usize) -> Result<String, RedisError> {
        let mut buffer = Vec::with_capacity(len);

        self.read(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;

        let parsed = String::from_utf8(buffer.into()).map_err(|_| RedisError::ParsingError)?;
        Ok(parsed)
    }

    fn next_u64(&mut self) -> Result<u64, RedisError> {
        let mut buffer = [0u8; 8];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;

        let integer_value = u64::from_be_bytes(buffer);
        Ok(integer_value)
    }

    fn next_u32(&mut self) -> Result<u32, RedisError> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;

        let integer_value = u32::from_be_bytes(buffer);
        Ok(integer_value)
    }

    fn next_u8(&mut self) -> Result<u8, RedisError> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;
        Ok(buffer[0])
    }

    fn next_i32(&mut self) -> Result<i32, RedisError> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;
        let value = i32::from_be_bytes(buffer);
        Ok(value)
    }

    fn next_i16(&mut self) -> Result<i16, RedisError> {
        let mut buffer = [0u8; 2];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;
        let value = i16::from_be_bytes(buffer);
        Ok(value)
    }

    fn next_i8(&mut self) -> Result<i8, RedisError> {
        let mut buffer = [0u8; 1];
        self.read_exact(&mut buffer)
            .map_err(|err| RedisError::IOError(err))?;
        let value = i8::from_be_bytes(buffer);
        Ok(value)
    }

    fn peek(&mut self) -> Result<u8, RedisError> {
        let val = self.next_u8()?;
        _ = self
            .seek(std::io::SeekFrom::Current(-1))
            .map_err(|err| RedisError::IOError(err))?;
        Ok(val)
    }
}
