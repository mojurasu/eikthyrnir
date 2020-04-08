use std::borrow::Cow;
use std::fs::{File, read};
use std::io::{BufReader, Cursor, Write};
use std::path::PathBuf;

use quick_xml::{Reader, Writer};
use quick_xml::events::{BytesEnd, BytesStart, Event};
use quick_xml::events::attributes::Attribute;

use crate::Result;
use eikthyrnir::*;
use std::fs;

pub fn relabel(files: Vec<PathBuf>) -> Result<()> {
    for world in files {
        if world.is_file() {
            let mut reader = Reader::from_file(&world).unwrap();
            reader.trim_text(true);
            let new_world = world.with_extension("relabeled");
            let mut writer = Writer::new_with_indent(File::create(&new_world).unwrap(),
                                                     b'\t', 1);
            let mut buf = Vec::new();
            let mut count = 0;
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(ref e)) if e.name() == b"IFS" => {
                        // crates a new element ... alternatively we could reuse `e` by calling
                        // `e.into_owned()`
                        let mut elem = BytesStart::owned(b"IFS".to_vec(), "IFS".len());
                        elem.extend_attributes(e.attributes().map(|attr| {
                            let mut a = attr.unwrap();
                            if a.key == b"name" {
                                match world.file_stem() {
                                    Some(name) => {
                                        let name = name.to_string_lossy();
                                        a.value = Cow::Owned(name.as_bytes().to_vec());
                                        count += 1;
                                    }
                                    None => padded_message("Skipped".bright_yellow(), format!("{} as it ends with ..", world.display()))
                                }
                            }
                            a
                        }));

                        // writes the event to the writer
                        assert!(writer.write_event(Event::Start(elem)).is_ok());
                    }
                    Ok(Event::Eof) => break,
                    Ok(e) => assert!(writer.write_event(e).is_ok()),
                    Err(e) => print_error(format!("Error at position {}: {:?}", reader.buffer_position(), e)),
                }
                buf.clear();
            }
            if count == 1 {
                writer.write(b"\n");
                fs::remove_file(&world)?;
                fs::rename(new_world, world)?;
            } else {
                padded_message("Skipped".bright_yellow(), format!("{} because it contains multiple worlds", world.display()));
                fs::remove_file(new_world)?;
            }

        }
    }
    Ok(())
}
