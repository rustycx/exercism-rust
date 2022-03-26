#[derive(Debug, PartialEq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().fold(vec![], |mut arr, &v| {
        let mut tmp = vec![];
        while v > 0 {
            tmp.push((v & 0x80) as u8);
            v >>= 7;
        }
        arr.extend(tmp.iter().rev());
        arr
    })
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    if let Some(&v) = bytes.last() {
        if v >= 0x80 {
            return Err(Error::IncompleteNumber);
        }
    }
    Err(Error::IncompleteNumber)
}
