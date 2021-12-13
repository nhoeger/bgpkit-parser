/*!
Provides IO utility functions for read bytes of different length and converting to corresponding structs.
*/
use byteorder::{BigEndian, ReadBytesExt};
use ipnetwork::{Ipv4Network, Ipv6Network, IpNetwork};
use std::{
    io,
    net::{Ipv4Addr, Ipv6Addr},
};

use num_traits::FromPrimitive;
use std::net::IpAddr;
use std::io::{Read, Take};
use bgp_models::network::{Afi, Asn, AsnLength, NetworkPrefix, Safi};
use log::debug;

use crate::error::ParserErrorKind;

/// Drop n bytes from input
macro_rules! drop_n{
    ($input:expr, $n:expr)=>{
        {
            let mut buffer = Vec::with_capacity($n as usize);
            $input.read_to_end(&mut buffer)?;
            drop(buffer);
        }
    }
}
// Allow reading IPs from Reads
pub trait ReadUtils: io::Read {
    #[inline]
    fn read_64b(&mut self) -> io::Result<u64> {
        self.read_u64::<BigEndian>()
    }

    #[inline]
    fn read_32b(&mut self) -> io::Result<u32> {
        self.read_u32::<BigEndian>()
    }

    #[inline]
    fn read_16b(&mut self) -> io::Result<u16> {
        self.read_u16::<BigEndian>()
    }

    #[inline]
    fn read_8b(&mut self) -> io::Result<u8> {
        self.read_u8()
    }

    fn read_n_bytes(&mut self, n_bytes: u64) -> io::Result<Vec<u8>>{
        let mut buffer = Vec::with_capacity(n_bytes as usize);
        self.take(n_bytes).read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn read_n_bytes_to_string(&mut self, n_bytes: u64) -> io::Result<String>{
        let mut buffer = Vec::with_capacity(n_bytes as usize);
        self.take(n_bytes).read_to_end(&mut buffer)?;
        Ok(buffer.into_iter().map(|x:u8| x as char).collect::<String>())
    }

    fn read_and_drop_n_bytes(&mut self, n_bytes: u64) -> io::Result<()>{
        let mut buffer = Vec::with_capacity(n_bytes as usize);
        self.take(n_bytes).read_to_end(&mut buffer)?;
        drop(buffer);
        Ok(())
    }

    /// Read announced/withdrawn prefix.
    ///
    /// The length in bits is 1 byte, and then based on the IP version it reads different number of bytes.
    /// If the `add_path` is true, it will also first read a 4-byte path id first; otherwise, a path-id of 0
    /// is automatically set.
    fn read_nlri_prefix(&mut self, afi: &Afi, add_path: bool) -> Result<NetworkPrefix, ParserErrorKind> {

        let path_id = if add_path {
            self.read_u32::<BigEndian>()?
        } else {
            0
        };

        // Length in bits
        let bit_len = self.read_8b()?;

        // Convert to bytes
        let byte_len: usize = (bit_len as usize + 7) / 8;
        let addr:IpAddr = match afi {
            Afi::Ipv4 => {

                // 4 bytes -- u32
                if byte_len>4 {
                    return Err(ParserErrorKind::ParseError(format!("Invalid byte length for IPv4 prefix. byte_len: {}, bit_len: {}", byte_len, bit_len)))
                }
                let mut buff = [0; 4];
                for i in 0..byte_len {
                    buff[i] = self.read_8b()?
                }
                IpAddr::V4(Ipv4Addr::from(buff))
            }
            Afi::Ipv6 => {
                // 16 bytes
                if byte_len>16 {
                    return Err(ParserErrorKind::ParseError(format!("Invalid byte length for IPv6 prefix. byte_len: {}, bit_len: {}", byte_len, bit_len)))
                }
                let mut buff = [0; 16];
                for i in 0..byte_len {
                    buff[i] = self.read_8b()?
                }
                IpAddr::V6(Ipv6Addr::from(buff))
            }
        };
        let prefix = match IpNetwork::new(addr, bit_len) {
            Ok(p) => {p}
            Err(_) => {
                return Err(ParserErrorKind::ParseError(format!("Invalid network prefix length: {}", bit_len)))
            }
        };

        Ok(NetworkPrefix::new(prefix, path_id))
    }

    fn read_address(&mut self, afi: &Afi) -> io::Result<IpAddr> {
        match afi {
            Afi::Ipv4 => {
                match self.read_ipv4_address(){
                    Ok(ip) => Ok(IpAddr::V4(ip)),
                    _ => return Err(io::Error::new(io::ErrorKind::Other, "Cannot parse IPv4 address".to_string()))
                }
            },
            Afi::Ipv6 => {
                match self.read_ipv6_address(){
                    Ok(ip) => Ok(IpAddr::V6(ip)),
                    _ => return Err(io::Error::new(io::ErrorKind::Other, "Cannot parse IPv6 address".to_string()))
                }
            },
        }
    }

    fn read_ipv4_address(&mut self) -> io::Result<Ipv4Addr> {
        let addr = self.read_u32::<BigEndian>()?;
        Ok(Ipv4Addr::from(addr))
    }

    fn read_ipv6_address(&mut self) -> io::Result<Ipv6Addr> {
        let mut buf = [0; 16];
        self.read_exact(&mut buf)?;
        Ok(Ipv6Addr::from(buf))
    }

    fn read_ipv4_prefix(&mut self) -> io::Result<Ipv4Network> {
        let addr = self.read_ipv4_address()?;
        let mask = self.read_u8()?;
        match Ipv4Network::new(addr, mask) {
            Ok(n) => Ok(n),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid prefix mask")),
        }
    }

    fn read_ipv6_prefix(&mut self) -> io::Result<Ipv6Network> {
        let addr = self.read_ipv6_address()?;
        let mask = self.read_u8()?;
        match Ipv6Network::new(addr, mask) {
            Ok(n) => Ok(n),
            Err(_) => Err(io::Error::new(io::ErrorKind::Other, "Invalid prefix mask")),
        }
    }

    fn read_asn(&mut self, as_length: &AsnLength) -> io::Result<Asn> {
        match as_length {
            AsnLength::Bits16 => Ok(self.read_u16::<BigEndian>()? as u32),
            AsnLength::Bits32 => self.read_u32::<BigEndian>(),
        }
    }

    fn read_asns(&mut self, as_length: &AsnLength, count: usize) -> io::Result<Vec<Asn>> {
        let mut path = Vec::with_capacity(count);
        match as_length {
            AsnLength::Bits16 => {
                for _ in 0..count {
                    path.push(self.read_u16::<BigEndian>()? as u32);
                }
            }
            AsnLength::Bits32 => {
                for _ in 0..count {
                    path.push(self.read_u32::<BigEndian>()?);
                }
            }
        };
        Ok(path)
    }

    fn read_afi(&mut self) -> Result<Afi, ParserErrorKind> {
        let afi = self.read_u16::<BigEndian>()?;
        match Afi::from_i16(afi as i16) {
            Some(afi) => Ok(afi),
            None => {
                Err(crate::error::ParserErrorKind::Unsupported(format!("Unknown AFI type: {}", afi)))
            },
        }
    }

    fn read_safi(&mut self) -> Result<Safi, ParserErrorKind> {
        let safi = self.read_u8()?;
        match Safi::from_u8(safi) {
            Some(safi) => Ok(safi),
            None => Err(crate::error::ParserErrorKind::Unsupported(format!("Unknown SAFI type: {}", safi)))
        }
    }
}

pub(crate) fn parse_nlri_list(
    input: &mut Take<&[u8]>,
    add_path: bool,
    afi: &Afi,
) -> Result<Vec<NetworkPrefix>, ParserErrorKind> {

    let mut bytes_copy = Vec::with_capacity(input.limit() as usize);
    input.read_to_end(&mut bytes_copy)?;

    let mut is_add_path = add_path;
    let mut prefixes = Vec::new();

    let mut retry = false;
    let mut guessed = false;

    let mut new_input = bytes_copy.take(bytes_copy.len() as u64);
    while new_input.limit() > 0 {
        if !is_add_path && new_input.get_ref()[0]==0 {
            // it's likely that this is a add-path wrongfully wrapped in non-add-path msg
            debug!("not add-path but with NLRI size to be 0, likely add-path msg in wrong msg type, treat as add-path now");
            is_add_path = true;
            guessed = true;
        }
        let prefix = match new_input.read_nlri_prefix(afi, is_add_path){
            Ok(p) => {p}
            Err(e) => {
                if guessed {
                    retry = true;
                    break;
                } else {
                    return Err(e);
                }
            }
        };
        prefixes.push(prefix);
    }

    if retry {
        prefixes.clear();
        // try again without attempt to guess add-path
        let mut new_input = bytes_copy.take(bytes_copy.len() as u64);
        while new_input.limit() > 0 {
            let prefix = new_input.read_nlri_prefix(afi, add_path)?;
            prefixes.push(prefix);
        }
    }

    Ok(prefixes)
}

// All types that implement Read can now read prefixes
impl<R: io::Read> ReadUtils for R {}
