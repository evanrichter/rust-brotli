#![no_main]
use libfuzzer_sys::fuzz_target;
use std::io::{Read, Write};

fuzz_target!(|data: (u16, u32, u32, &[u8], u16)| {
  let mut compressed = Vec::new();
  let mut writer = brotli::CompressorWriter::new(&mut compressed, data.0.into(), data.1, data.2);
  writer.write_all(data.3).unwrap();
  drop(writer);
  let mut reader = brotli::Decompressor::new(compressed.as_slice(), data.4.into());
  let mut decompressed = Vec::with_capacity(data.3.len());
  let _ = reader.read_to_end(&mut decompressed);
  assert_eq!(data.3, decompressed, "roundtrip failed");
});
