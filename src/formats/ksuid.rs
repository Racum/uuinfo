use std::fmt::Write;

use crate::schema::{Args, IDInfo};
use crate::utils::milliseconds_to_seconds_and_iso8601;

mod custom_base62 {
    // Based on https://github.com/ecstatic-morse/ksuid/blob/master/src/base62.rs

    use resize_slice::ResizeSlice;
    use std::io;
    const CHAR_MAP: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";
    #[rustfmt::skip]
    const BYTE_MAP: &[i8] = &[
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
        -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1, -1,
         0,  1,  2,  3,  4,  5,  6,  7,  8,  9, -1, -1, -1, -1, -1, -1,
        -1, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
        25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, -1, -1, -1, -1, -1,
        -1, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50,
        51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, -1, -1, -1, -1, -1,
    ];

    fn b62_to_bin(c: u8) -> i8 {
        BYTE_MAP[usize::from(c)]
    }

    fn change_base(mut num: &mut [u8], out: &mut [u8], in_base: usize, out_base: usize) {
        debug_assert!(out.iter().all(|&b| b == 0));
        let mut k = out.len();
        while !num.is_empty() {
            let mut rem = 0;
            let mut i = 0;
            for j in 0..num.len() {
                let acc = usize::from(num[j]) + in_base * rem;
                let div = acc / out_base;
                rem = acc % out_base;
                if i != 0 || div != 0 {
                    num[i] = div as u8;
                    i += 1;
                }
            }
            k -= 1;
            *out.get_mut(k).expect("Output buffer not large enough") = rem as u8;
            out[k] = rem as u8;
            num.resize_to(i);
        }
    }

    pub fn encode_raw(input: &mut [u8], output: &mut [u8]) {
        change_base(input, output, 256, 62);
        for b in output.iter_mut() {
            *b = CHAR_MAP[usize::from(*b)];
        }
    }

    pub fn decode_raw(input: &mut [u8], output: &mut [u8]) -> io::Result<()> {
        for c in input.iter_mut() {
            if *c & 0x80 != 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Non-ASCII character in input"));
            }
            let b = b62_to_bin(*c);
            if b < 0 {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid base62 character in input"));
            }
            *c = b as u8;
        }
        change_base(input, output, 62, 256);
        Ok(())
    }
}

mod custom_ksuid {
    // Based on https://github.com/ecstatic-morse/ksuid/blob/master/src/lib.rs

    use std::io;

    use super::*;

    const LEN: usize = 20;
    const EMPTY: [u8; LEN] = [0; LEN];
    const BASE62_LEN: usize = 27;
    const HEX_LEN: usize = 40;
    const MAX_BASE62_KSUID: &[u8] = b"aWgEPTl1tmebfsQzFP4bxwgy80V";

    #[derive(Debug)]
    pub struct Ksuid([u8; LEN]);

    impl Ksuid {
        pub fn from_base62(s: &str) -> io::Result<Self> {
            let bytes = s.as_bytes();
            if bytes.len() != BASE62_LEN || bytes > MAX_BASE62_KSUID {
                return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid id"));
            }
            let mut ret = Ksuid(EMPTY);
            let mut scratch = [0; BASE62_LEN];
            scratch.clone_from_slice(bytes);
            custom_base62::decode_raw(scratch.as_mut(), ret.0.as_mut())?;
            Ok(ret)
        }

        pub fn from_hex(hex: &str) -> Result<Ksuid, String> {
            if hex.len() != HEX_LEN {
                return Err("Hex string must be 40 bytes long".to_string());
            }
            match hex::decode(hex) {
                Ok(bytes) => Ok(Ksuid(bytes.try_into().unwrap())),
                Err(_) => Err("Error decoding hex".to_string()),
            }
        }

        pub fn as_bytes(&self) -> &[u8] {
            self.0.as_ref()
        }

        pub fn millis(&self) -> u64 {
            let ts_hex: [u8; 4] = self.0[0..4].try_into().unwrap();
            let ts_secs = (u32::from_be_bytes(ts_hex) as u64) + 1_400_000_000;
            ts_secs * 1_000
        }

        pub fn to_base62(&self) -> String {
            let mut scratch = self.0;
            let mut out = vec![0; BASE62_LEN];
            custom_base62::encode_raw(scratch.as_mut(), out.as_mut());
            // This is valid because base 62 encoded data contains only ASCII alphanumeric characters.
            unsafe { String::from_utf8_unchecked(out) }
        }

        pub fn to_hex(&self) -> String {
            hex::encode(self.0)
        }
    }
}

use custom_ksuid::Ksuid;

pub fn parse_ksuid(args: &Args) -> Option<IDInfo> {
    let version: Option<String>;
    let parsed: Option<String>;
    let ksuid: Ksuid = match Ksuid::from_base62(&args.id) {
        Ok(value) => {
            version = Some("Base62-encoded".to_string());
            parsed = Some("from base62".to_string());
            value
        }
        Err(_) => match Ksuid::from_hex(&args.id) {
            Ok(value) => {
                version = Some("Hex-encoded".to_string());
                parsed = Some("from hex".to_string());
                value
            }
            Err(_) => return None,
        },
    };

    let formatted_time = milliseconds_to_seconds_and_iso8601(ksuid.millis(), None);
    let timestamp = Some(formatted_time.0);
    let datetime = Some(formatted_time.1);

    Some(IDInfo {
        id_type: "KSUID".to_string(),
        version,
        standard: ksuid.to_base62(),
        parsed,
        size: 160,
        entropy: 128,
        datetime,
        timestamp,
        hex: Some(ksuid.to_hex()),
        bits: Some(ksuid.as_bytes().iter().fold(String::new(), |mut output, c| {
            let _ = write!(output, "{c:08b}");
            output
        })),
        color_map: Some("3333333333333333333333333333333322222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222222".to_string()),
        high_confidence: true,
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_ksuid_base62() {
        let result = parse_ksuid(&Args {
            id: "2ozUlIq2UpZdpYDMOGAZf9i7oya".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.id_type, "KSUID".to_string());
        assert_eq!(result.version.unwrap(), "Base62-encoded".to_string());
        assert_eq!(result.timestamp.unwrap(), "1731872046.000".to_string());
        assert_eq!(result.standard, "2ozUlIq2UpZdpYDMOGAZf9i7oya".to_string());
        assert_eq!(result.hex.unwrap(), "13c7f72eff938c3cba49cfb88fa840868effca9c".to_string());
    }

    #[test]
    fn test_parse_ksuid_hex() {
        let result = parse_ksuid(&Args {
            id: "13c7f72eff938c3cba49cfb88fa840868effca9c".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.id_type, "KSUID".to_string());
        assert_eq!(result.version.unwrap(), "Hex-encoded".to_string());
        assert_eq!(result.timestamp.unwrap(), "1731872046.000".to_string());
        assert_eq!(result.standard, "2ozUlIq2UpZdpYDMOGAZf9i7oya".to_string());
        assert_eq!(result.hex.unwrap(), "13c7f72eff938c3cba49cfb88fa840868effca9c".to_string());
    }

    #[test]
    fn test_parse_ksuid_min() {
        let result = parse_ksuid(&Args {
            id: "0000000000000000000000000000000000000000".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.id_type, "KSUID".to_string());
        assert_eq!(result.version.unwrap(), "Hex-encoded".to_string());
        assert_eq!(result.timestamp.unwrap(), "1400000000.000".to_string());
        assert_eq!(result.standard, "000000000000000000000000000".to_string());
        assert_eq!(result.hex.unwrap(), "0000000000000000000000000000000000000000".to_string());
    }

    #[test]
    fn test_parse_ksuid_max() {
        let result = parse_ksuid(&Args {
            id: "ffffffffffffffffffffffffffffffffffffffff".to_string(),
            ..Default::default()
        })
        .unwrap();
        assert_eq!(result.id_type, "KSUID".to_string());
        assert_eq!(result.version.unwrap(), "Hex-encoded".to_string());
        assert_eq!(result.timestamp.unwrap(), "5694967295.000".to_string());
        assert_eq!(result.standard, "aWgEPTl1tmebfsQzFP4bxwgy80V".to_string());
        assert_eq!(result.hex.unwrap(), "ffffffffffffffffffffffffffffffffffffffff".to_string());
    }
}
