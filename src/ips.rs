use std;
use std::io::prelude::*;

use error::Error;

#[derive(Debug)]
enum Record {
    Normal { offset: usize, data: Vec<u8> },
    RuntimeLengthEncoded {
        offset: usize,
        size: usize,
        value: u8,
    },
}

#[derive(Debug)]
struct Patch {
    records: Vec<Record>,
}

impl Patch {
    fn load(patch_filename: &str) -> Result<Self, Error> {

        let buf = {

            let mut f = try!(std::fs::File::open(patch_filename).map_err(|e| {
                                                                             Error::Io {
                    cause: e,
                    description: format!("Failed to open patch file '{}'", patch_filename),
                }
                                                                         }));

            let mut buf = Vec::new();
            try!(f.read_to_end(&mut buf)
                     .map_err(|e| {
                                  Error::Io {
                                      cause: e,
                                      description: format!("Failed to read patch file '{}'",
                                                           patch_filename),
                                  }
                              }));

            buf
        };

        Patch::parse(&buf)
    }

    fn parse(patch: &Vec<u8>) -> Result<Self, Error> {

        if patch.len() < 5 || &patch[..5] != "PATCH".as_bytes() {
            return Err(Error::InvalidPatch { description: format!("Missing PATCH header") });
        }
        let mut patch = &patch[5..];

        let mut records = Vec::new();

        loop {
            if patch.len() == 3 && &patch[..3] == "EOF".as_bytes() {
                break;
            }

            if patch.len() < 3 {
                return Err(Error::InvalidPatch {
                               description: format!("Expecting record 'offset' field, got {} of 3 bytes \
                                          before reaching end of file",
                                                    patch.len()),
                           });
            }
            let offset = ((patch[0] as u32) << 16) + ((patch[1] as u32) << 8) + (patch[2] as u32);
            patch = &patch[3..];

            if patch.len() < 2 {
                return Err(Error::InvalidPatch {
                               description: format!("Expecting record 'size' field, got {} of 2 bytes before \
                                          reaching end of file",
                                                    patch.len()),
                           });
            }
            let size = ((patch[0] as u16) << 8) + (patch[1] as u16);
            patch = &patch[2..];

            records.push(if 0 == size {

                if patch.len() < 2 {
                    return Err(Error::InvalidPatch {
                        description: format!("Expecting record 'rle_size', got {} of 2 bytes \
                                              before reaching end of file",
                                             patch.len()),
                    });
                }
                let rle_size = ((patch[0] as u16) << 8) + (patch[1] as u16);
                patch = &patch[2..];

                if patch.len() < 1 {
                    return Err(Error::InvalidPatch {
                        description: format!("Expecting record 'rle_value' field, got end of file"),
                    });
                }

                let rle_value = patch[0];
                patch = &patch[1..];

                Record::RuntimeLengthEncoded {
                    offset: offset as usize,
                    size: rle_size as usize,
                    value: rle_value,
                }

            } else {

                if patch.len() < size as usize {
                    return Err(Error::InvalidPatch {
                        description: format!("Expecting record 'data' field, got {} of {} bytes \
                                              before reaching end of file",
                                             patch.len(),
                                             size),
                    });
                }
                let data = Vec::from(&patch[..(size as usize)]);
                patch = &patch[(size as usize)..];

                Record::Normal {
                    offset: offset as usize,
                    data: data,
                }
            });
        }

        // records.sort();

        let p = Patch { records: records };
        Ok(p)
    }

    #[allow(dead_code)]
    fn dump_records<T>(records: T)
        where T: Iterator<Item = Record>
    {
        for rec in records {
            match rec {
                Record::Normal {
                    ref offset,
                    ref data,
                } => {
                    println!("DATA : {:x}, {:x}", offset, data.len());
                }
                Record::RuntimeLengthEncoded {
                    ref offset,
                    ref size,
                    ref value,
                } => {
                    println!("RLE  : {:x}, {:x}, {:x}", offset, size, value);
                }
            }
        }
    }

    fn apply(&self, ibuf: &Vec<u8>) -> Result<Vec<u8>, Error> {
        let mut obuf = ibuf.clone();
        for rec in self.records.iter() {
            match *rec {
                Record::Normal {
                    ref offset,
                    ref data,
                } => {
                    // Special case: extend existing ROM data.
                    if obuf.len() < *offset + data.len() {
                        obuf.resize(*offset + data.len(), 0)
                    }
                    for i in 0..data.len() {
                        obuf[*offset + i] = data[i];
                    }
                }
                Record::RuntimeLengthEncoded {
                    ref offset,
                    ref size,
                    ref value,
                } => {
                    // Special case: extend existing ROM data.
                    if obuf.len() < *offset + size {
                        obuf.resize(*offset + size, 0)
                    }
                    for i in *offset..(*offset + *size) {
                        obuf[i] = *value;
                    }
                }
            }
        }
        Ok(obuf)
    }
}

pub fn patch(patch_filename: &str) -> Result<(), Error> {

    let patch = try!(Patch::load(patch_filename));

    let ibuf = {
        let mut x = Vec::new();
        try!(std::io::stdin()
                 .read_to_end(&mut x)
                 .map_err(|e| {
                              Error::Io {
                                  cause: e,
                                  description: format!("Failed to read from stdin to end"),
                              }
                          }));
        x
    };

    let obuf = try!(patch.apply(&ibuf));

    try!(std::io::stdout()
             .write_all(&obuf)
             .map_err(|e| {
                          Error::Io {
                              cause: e,
                              description: format!("Failed to write to stdout"),
                          }
                      }));

    Ok(())
}

#[cfg(test)]
mod tests {}
