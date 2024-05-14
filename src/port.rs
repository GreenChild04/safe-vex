//! Defines safe(er) functions for dealing with the robot's ports

use vex_rt::smart_port::SmartPort;

/// Represents the ports on a robot in a safe manner that enforces rust mutability and borrow rules
pub struct PortManager([Option<SmartPort>; 21]); // `21` smart ports

#[derive(Debug, Clone)]
pub enum PortError {
    /// Occurs when there is an invalid id *(must be `0..21`)*
    PortInvalid,
    /// Occurs when an 
    PortTaken,
}

impl PortManager {
    /// Creates a new `PortManager`
    ///
    /// **note:** it's private to prevent multiple instances of the PortManager which would defeat the whole purpose of it
    #[inline]
    pub(crate) fn new() -> Self {
        // stupid, I know, but it works
        let mut ports = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];
        
        ports.iter_mut()
            .enumerate()
            .for_each(|(i, port)| *port = Some(unsafe { SmartPort::new(i as u8 + 1) }));
        Self(ports)
    }

    /// Gets a non-mutable reference to a smart port of the robot
    #[inline]
    pub fn get(&self, port: u8) -> Result<&SmartPort, PortError> {
        self.0
            .get(port as usize -1)
            .ok_or(PortError::PortInvalid)?
            .as_ref()
            .ok_or(PortError::PortTaken)
    }

    /// Gets a mutable reference to a smart port of the robot
    #[inline]
    pub fn get_mut(&mut self, port: u8) -> Result<&mut SmartPort, PortError> {
        self.0
            .get_mut(port as usize -1)
            .ok_or(PortError::PortInvalid)?
            .as_mut()
            .ok_or(PortError::PortTaken)
    }

    /// Takes ownership of a smart port of the robot
    #[inline]
    pub fn take(&mut self, port: u8) -> Result<SmartPort, PortError> {
        self.0
            .get_mut(port as usize -1)
            .ok_or(PortError::PortInvalid)?
            .take()
            .ok_or(PortError::PortTaken)
    }
}
