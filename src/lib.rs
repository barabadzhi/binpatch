#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate data_encoding;
extern crate rayon;

#[derive(Debug, Default, PartialEq)]
pub struct Patch<'a> {
    file: Option<String>,
    pairs: Vec<PatchPair<'a>>,
    output: Option<String>,
}

#[derive(Debug, PartialEq)]
struct PatchPair<'a> {
    from: &'a [u8],
    to: &'a [u8],
}

impl<'a> Patch<'a> {
    pub fn new() -> Patch<'a> {
        Patch {
            ..Default::default()
        }
    }

    pub fn file(&mut self, s: &str) -> &mut Patch<'a> {
        self.file = Some(String::from(s));
        self
    }

    pub fn replace(&mut self, from: &'a [u8], to: &'a [u8]) -> &mut Patch<'a> {
        self.pairs.push(PatchPair { from, to });
        self
    }

    pub fn output(&mut self, s: &str) -> &mut Patch<'a> {
        self.output = Some(String::from(s));
        self
    }

    pub fn execute(&mut self) -> Result<(), std::io::Error> {
        use std::io::prelude::*;
        use std::fs::File;

        use data_encoding::HEXUPPER;
        use rayon::prelude::*;

        if let Some(ref file) = self.file {
            let mut buffer = Vec::new();

            File::open(file)?.read_to_end(&mut buffer)?;

            for pair in &self.pairs {
                let from = HEXUPPER.decode(pair.from).unwrap();
                let to = HEXUPPER.decode(pair.to).unwrap();

                buffer
                    .par_chunks_mut(from.len())
                    .for_each(|chunk| if chunk == from.as_slice() {
                        chunk.copy_from_slice(&to);
                    });
            }

            if let Some(ref output) = self.output {
                File::create(output)?.write_all(&buffer)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_new_patch() {
        assert_eq!(
            Patch::new(),
            Patch {
                ..Default::default()
            }
        );
    }

    #[test]
    fn set_file_to_patch() {
        assert_eq!(
            *Patch::new().file("lib.so"),
            Patch {
                file: Some(String::from("lib.so")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn set_patch_output() {
        assert_eq!(
            *Patch::new().output("lib.patched.so"),
            Patch {
                output: Some(String::from("lib.patched.so")),
                ..Default::default()
            }
        );
    }

    #[test]
    fn add_patch_pair() {
        assert_eq!(
            *Patch::new()
                .file("lib.so")
                .replace(b"FF", b"C3")
                .output("lib.patched.so"),
            Patch {
                file: Some(String::from("lib.so")),
                output: Some(String::from("lib.patched.so")),
                pairs: vec![
                    PatchPair {
                        from: b"FF",
                        to: b"C3",
                    },
                ],
            }
        );
    }

    // #[test]
    // fn execute_patch() {
    //     unimplemented!();
    // }
}
