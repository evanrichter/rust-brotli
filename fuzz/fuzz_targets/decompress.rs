#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::{Read, Write};

fuzz_target!(|data: (u16, &[u8])| {
  let sink = std::io::sink();
  let mut decompressor = brotli::DecompressorWriter::new(sink, data.0.into());
  let _ = decompressor.write_all(data.1);
});
