use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use quick_xml::{Reader, Writer};
use quick_xml::events::{BytesDecl, BytesEnd, BytesStart, Event};

use eikthyrnir::*;

pub fn combine(file: PathBuf, files: Vec<PathBuf>) -> Result<()> {
    let mut writer = Writer::new_with_indent(File::create(file).unwrap(),
                                             b'\t', 1);
    writer.write_event(Event::Decl(BytesDecl::new(b"1.0", Some(b"utf-8"), None)))?;

    let mut elem = BytesStart::owned("world", 4);
    elem.push_attribute(("name", "World"));
    let root_start = Event::Start(elem);
    writer.write_event(root_start)?;

    for world in files {
        if world.is_file() {
            let mut reader = Reader::from_file(&world).unwrap();
            reader.trim_text(true);
            let mut buf = Vec::new();
            let mut ifs = false;
            loop {
                match reader.read_event(&mut buf) {
                    Ok(Event::Start(e)) if e.name() == b"IFS" => {
                        ifs = true;
                        assert!(writer.write_event(Event::Start(e.into_owned())).is_ok())
                    }
                    Ok(Event::End(e)) if e.name() == b"IFS" => {
                        ifs = false;
                        assert!(writer.write_event(Event::End(e.into_owned())).is_ok())
                    }
                    Ok(Event::Eof) => break,
                    Ok(e) => {
                        if ifs {
                            assert!(writer.write_event(e).is_ok())
                        }
                    }
                    Err(e) => print_error(format!("Error at position {}: {:?}", reader.buffer_position(), e)),
                }
                buf.clear();
            }
        }
    }
    writer.write_event(Event::End(BytesEnd::borrowed(b"world")))?;
    Ok(())
}
