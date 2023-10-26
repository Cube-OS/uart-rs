//
// Copyright (C) 2018 Kubos Corporation
// Copyright (C) 2022 CUAVA
//
// Licensed under the Apache License, Version 2.0 (the "License")
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//

// #![deny(missing_docs)]

//! A generalized HAL for communicating over serial ports

mod error;
pub mod mock;
#[cfg(test)]
mod tests;

pub use ::serial::PortSettings;
pub use crate::error::*;
use serial::prelude::*;
use std::cell::RefCell;
#[allow(unused_imports)]
use std::io::prelude::*;
use std::time::Duration;
use std::thread;
use hal_stream::Stream;
use std::sync::{Arc, Mutex};

/// Wrapper for UART stream
pub struct Connection {
    /// Any boxed stream that allows for communication over serial ports
    pub stream: Box<dyn Stream<StreamError = UartError> + Send>,
}

impl Connection {
    /// Constructor to creation connection with provided stream
    pub fn new(stream: Box<dyn Stream<StreamError = UartError> + Send>) -> Connection {
        Connection { stream }
    }

    /// Convenience constructor to create connection from bus path
    pub fn from_path(
        bus: &str,
        settings: serial::PortSettings,
        timeout: Duration,
    ) -> Connection {
        Connection {
            stream: Box::new(SerialStream::new(bus, settings, timeout)),
        }
    }

    /// Writes out raw bytes to the stream
    pub fn write(&self, data: &[u8]) -> UartResult<()> {
        self.stream.write(data.to_vec())
    }

    /// Reads messages upto specified length recieved on the bus
    pub fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let mut response: Vec<u8> = vec![0; len];
        self.stream.read_timeout(&mut response, len, timeout)
    }

    /// Write multiple single bytes, without closing UART Port
    pub fn write_bytes(&self, data: &[u8]) -> UartResult<()> {
        self.stream.write_bytes(data.to_vec())
    }

    /// Write - Read transfer
    pub fn transfer(&self, data: &[u8], len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        self.stream.transfer(data.to_vec(),len,Some(timeout))
    }
}

// /// This trait is used to represent streams and allows for mocking for api unit tests
// pub trait Stream: Send {
//     /// Write raw bytes to stream
//     fn write(&self, data: &[u8]) -> UartResult<()>;

//     /// Read upto a specified amount of raw bytes from the stream
//     fn read(&self, len: usize, timeout: Duration) -> UartResult<Vec<u8>>;

//     /// Write - Read transfer
//     fn transfer(&self, data: &[u8], len: usize, timeout: Duration) -> UartResult<Vec<u8>>;
// }

// This is the actual stream that data is tranferred over
pub struct SerialStream {
    bus: String,
    settings: serial::PortSettings,
    timeout: Duration,
}

impl SerialStream {
    pub fn new(bus: &str, settings: serial::PortSettings, timeout: Duration) -> Self {
        Self {
            bus: bus.to_string(),
            settings,
            timeout,
        }
        // let mut port = serial::open(bus)?;

        // port.configure(&settings)?;

        // Ok(SerialStream {
        //     port: RefCell::new(port),
        //     timeout,
        // })
    }    
}

// Read and write implementations for the serial stream
impl Stream for SerialStream {
    type StreamError = UartError;

    fn write(&self, data: Vec<u8>) -> UartResult<()> {
        let mut port = serial::open(&self.bus)?;

        port.configure(&self.settings)?;
 
        port.set_timeout(self.timeout)?;

        Ok(port.write_all(&data)?)
    }

    fn write_bytes(&self, data: Vec<u8>) -> UartResult<()> {
        let mut port = serial::open(&self.bus)?;

        port.configure(&self.settings)?;

        port.set_timeout(self.timeout)?;

        for d in data {
            port.write_all(&[d])?;
            port.flush()?;
        }
        Ok(())
    }

    fn read(&self, data: &mut Vec<u8>, _len: usize) -> UartResult<Vec<u8>> {
        let mut port = serial::open(&self.bus)?;

        port.configure(&self.settings)?;

        port.read_exact(data.as_mut_slice())?;

        Ok(data.to_vec())
    }

    fn read_timeout(&self, data: &mut Vec<u8>, _len: usize, timeout: Duration) -> UartResult<Vec<u8>> {
        let mut port = serial::open(&self.bus)?;

        port.configure(&self.settings)?; 

        port.set_timeout(timeout)?;

        port.read_exact(data.as_mut_slice())?;

        Ok(data.to_vec())
    }

    fn transfer(&self, data: Vec<u8>, len: usize, timeout: Option<Duration>) -> UartResult<Vec<u8>> {
        let mut port = serial::open(&self.bus)?;

        port.configure(&self.settings)?;
 
        port.set_timeout(timeout.unwrap())?;

        let mut response: Vec<u8> = vec![0; len];

        port.write_all(&data)?;
        
        thread::sleep(timeout);

        port.read(response.as_mut_slice())?;

        Ok(response)
    }
}