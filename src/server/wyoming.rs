//! Wyoming JSON-line header, optional JSON data, then exact binary payload.
use serde_json::{json, Map, Value};
use std::io::{self, BufRead, Write};

pub const MAX_HEADER: usize = 16 * 1024;
pub const MAX_DATA: usize = 64 * 1024;
pub const MAX_PAYLOAD: usize = 64 * 1024;

#[derive(Debug)]
pub struct Event { pub kind: String, pub data: Map<String, Value>, pub payload: Vec<u8> }
fn invalid(text: &str) -> io::Error { io::Error::new(io::ErrorKind::InvalidData, text) }
fn length(value: Option<&Value>, maximum: usize) -> io::Result<usize> {
    match value {
        None | Some(Value::Null) => Ok(0),
        Some(v) => v.as_u64().filter(|&n| n <= maximum as u64).map(|n| n as usize).ok_or_else(|| invalid("Invalid or oversized Wyoming length")),
    }
}
pub fn read_event(reader: &mut impl BufRead) -> io::Result<Option<Event>> {
    let mut line = Vec::new();
    loop {
        let buffer = reader.fill_buf()?;
        if buffer.is_empty() {
            if line.is_empty() { return Ok(None); }
            return Err(invalid("Truncated Wyoming header"));
        }
        let end = buffer.iter().position(|&b| b == b'\n').map(|n| n + 1);
        let n = end.unwrap_or(buffer.len());
        if line.len() + n > MAX_HEADER { return Err(invalid("Wyoming header exceeds limit")); }
        line.extend_from_slice(&buffer[..n]);
        reader.consume(n);
        if end.is_some() { break; }
    }
    let header: Value = serde_json::from_slice(&line).map_err(|_| invalid("Invalid Wyoming JSON header"))?;
    let object = header.as_object().ok_or_else(|| invalid("Header must be an object"))?;
    let kind = object.get("type").and_then(Value::as_str).filter(|s| !s.is_empty()).ok_or_else(|| invalid("Missing Wyoming type"))?.to_owned();
    let mut data = match object.get("data") {
        None => Map::new(),
        Some(v) => v.as_object().cloned().ok_or_else(|| invalid("Wyoming data must be an object"))?,
    };
    let data_len = length(object.get("data_length"), MAX_DATA)?;
    let payload_len = length(object.get("payload_length"), MAX_PAYLOAD)?;
    if data_len > 0 {
        let mut bytes = vec![0; data_len]; reader.read_exact(&mut bytes)?;
        let value: Value = serde_json::from_slice(&bytes).map_err(|_| invalid("Invalid additional Wyoming JSON data"))?;
        data.extend(value.as_object().ok_or_else(|| invalid("Additional data must be an object"))?.clone());
    }
    let mut payload = vec![0; payload_len]; reader.read_exact(&mut payload)?;
    Ok(Some(Event { kind, data, payload }))
}
pub fn write_event(writer: &mut impl Write, kind: &str, data: Value) -> io::Result<()> {
    serde_json::to_writer(&mut *writer, &json!({"type":kind,"data":data}))?;
    writer.write_all(b"\n")?; writer.flush()
}
pub fn error(writer: &mut impl Write, code: &str, text: &str) -> io::Result<()> {
    write_event(writer, "error", json!({"code":code,"text":text}))
}
pub fn audio_format(event: &Event) -> Result<(), String> {
    if event.data.get("rate").and_then(Value::as_u64) != Some(16000)
        || event.data.get("width").and_then(Value::as_u64) != Some(2)
        || event.data.get("channels").and_then(Value::as_u64) != Some(1) {
        return Err("Expected signed little-endian PCM16: rate=16000, width=2 bytes, channels=1; conversion is not supported".into());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{BufReader, Cursor};
    #[test]
    fn merged_data_and_binary_next_event() {
        let additional = br#"{"rate":16000,"width":2,"channels":1}"#;
        let mut bytes = format!("{{\"type\":\"audio-chunk\",\"data\":{{\"rate\":8000}},\"data_length\":{},\"payload_length\":4}}\n",additional.len()).into_bytes();
        bytes.extend_from_slice(additional); bytes.extend_from_slice(&[0,10,255,0]); bytes.extend_from_slice(b"{\"type\":\"audio-stop\"}\n");
        let mut r = BufReader::with_capacity(1, Cursor::new(bytes));
        let e = read_event(&mut r).unwrap().unwrap(); audio_format(&e).unwrap(); assert_eq!(e.payload, vec![0,10,255,0]);
        assert_eq!(read_event(&mut r).unwrap().unwrap().kind,"audio-stop"); assert!(read_event(&mut r).unwrap().is_none());
    }
    #[test]
    fn reject_oversized_negative_and_truncated() {
        for input in ["{\"type\":\"audio-chunk\",\"payload_length\":-1}\n", "{\"type\":\"x\",\"payload_length\":65537}\n", "{\"type\":\"x\",\"payload_length\":3}\nX", "{\"type\":\"x\"}"] {
            assert!(read_event(&mut Cursor::new(input)).is_err());
        }
        assert!(read_event(&mut Cursor::new(vec![b'x';MAX_HEADER+1])).is_err());
    }
}
