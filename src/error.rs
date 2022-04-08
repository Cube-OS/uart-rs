//
// Copyright (C) 2018 Kubos Corporation
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

use failure::Fail;

/// Custom errors for UART actions
#[derive(Fail, Debug, Clone, PartialEq)]
pub enum UartError {
    /// Catch-all error case
    #[fail(display = "Generic Error")]
    GenericError,
    /// A read/write call was made while another call was already in-progress
    #[fail(display = "Serial port already in-use")]
    PortBusy,
    /// An error was thrown by the serial driver
    #[fail(display = "Serial Error")]
    SerialError(serial::ErrorKind),
}

impl From<std::io::Error> for UartError {
    fn from(error: std::io::Error) -> Self {
        UartError::SerialError(serial::ErrorKind::Io(error.kind()))
    }
}

impl From<serial::Error> for UartError {
    fn from(error: serial::Error) -> Self {
        UartError::SerialError(error.kind())
    }
}

/// Errors that occur while reading from and writing to stream
pub type UartResult<T> = Result<T, UartError>;
