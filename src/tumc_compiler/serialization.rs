//! TUMC Serialization Framework
//!
//! High-performance serialization for Tariro types including
//! binary, JSON, and binary formats.

use std::collections::{HashMap, HashSet};

/// Serializer trait
pub trait Serializer: Sized {
    /// Serialize a value
    fn serialize<T: Serialize<Self>>(&mut self, value: &T) -> Result<(), SerialError>;

    /// Serialize a boolean
    fn serialize_bool(&mut self, v: bool) -> Result<(), SerialError>;

    /// Serialize an integer
    fn serialize_i32(&mut self, v: i32) -> Result<(), SerialError>;
    fn serialize_i64(&mut self, v: i64) -> Result<(), SerialError>;
    fn serialize_u32(&mut self, v: u32) -> Result<(), SerialError>;
    fn serialize_u64(&mut self, v: u64) -> Result<(), SerialError>;

    /// Serialize a float
    fn serialize_f32(&mut self, v: f32) -> Result<(), SerialError>;
    fn serialize_f64(&mut self, v: f64) -> Result<(), SerialError>;

    /// Serialize a string
    fn serialize_str(&mut self, v: &str) -> Result<(), SerialError>;

    /// Serialize a sequence
    fn serialize_seq(&mut self, len: usize) -> Result<SeqSerializer<'_, Self>, SerialError>;

    /// Serialize a map
    fn serialize_map(&mut self, len: usize) -> Result<MapSerializer<'_, Self>, SerialError>;
}

/// Sequence serializer
pub struct SeqSerializer<'a, S: Serializer> {
    serializer: &'a mut S,
}

impl<'a, S: Serializer> SeqSerializer<'a, S> {
    /// Serialize element
    pub fn serialize_element<T: Serialize<S>>(&mut self, value: &T) -> Result<(), SerialError> {
        value.serialize(self.serializer)
    }

    /// End sequence
    pub fn end(self) -> Result<(), SerialError> {
        Ok(())
    }
}

/// Map serializer
pub struct MapSerializer<'a, S: Serializer> {
    serializer: &'a mut S,
}

impl<'a, S: Serializer> MapSerializer<'a, S> {
    /// Serialize key
    pub fn serialize_key<T: Serialize<S>>(&mut self, key: &T) -> Result<(), SerialError> {
        key.serialize(self.serializer)
    }

    /// Serialize value
    pub fn serialize_value<T: Serialize<S>>(&mut self, value: &T) -> Result<(), SerialError> {
        value.serialize(self.serializer)
    }

    /// End map
    pub fn end(self) -> Result<(), SerialError> {
        Ok(())
    }
}

/// Serializable trait
pub trait Serialize<S: Serializer> {
    /// Serialize self
    fn serialize(&self, serializer: &mut S) -> Result<(), SerialError>;
}

/// Deserializer trait
pub trait Deserializer: Sized {
    /// Deserialize a value
    fn deserialize<T: Deserialize<Self>>(self) -> Result<T, SerialError>;

    /// Deserialize a boolean
    fn deserialize_bool(self) -> Result<bool, SerialError>;

    /// Deserialize integers
    fn deserialize_i32(self) -> Result<i32, SerialError>;
    fn deserialize_i64(self) -> Result<i64, SerialError>;
    fn deserialize_u32(self) -> Result<u32, SerialError>;
    fn deserialize_u64(self) -> Result<u64, SerialError>;

    /// Deserialize floats
    fn deserialize_f32(self) -> Result<f32, SerialError>;
    fn deserialize_f64(self) -> Result<f64, SerialError>;

    /// Deserialize string
    fn deserialize_str(self) -> Result<String, SerialError>;

    /// Deserialize sequence
    fn deserialize_seq(self) -> Result<SeqDeserializer<Self>, SerialError>;

    /// Deserialize map
    fn deserialize_map(self) -> Result<MapDeserializer<Self>, SerialError>;
}

/// Sequence deserializer
pub struct SeqDeserializer<D: Deserializer> {
    data: D,
}

impl<D: Deserializer> SeqDeserializer<D> {
    /// Get next element
    pub fn next<T: Deserialize<D>>(self) -> Result<Option<(T, Self)>, SerialError> {
        Ok(None)
    }
}

/// Map deserializer
pub struct MapDeserializer<D: Deserializer> {
    data: D,
}

impl<D: Deserializer> MapDeserializer<D> {
    /// Get next entry
    pub fn next<K: Deserialize<D>, V: Deserialize<D>>(self) -> Result<Option<((K, V), Self)>, SerialError> {
        Ok(None)
    }
}

/// Deserializable trait
pub trait Deserialize<D: Deserializer> {
    /// Deserialize from deserializer
    fn deserialize(deserializer: D) -> Result<Self, SerialError>
    where
        Self: Sized;
}

/// Serialization error
#[derive(Debug)]
pub enum SerialError {
    Message(String),
    EndOfInput,
    InvalidType,
    UnknownField(String),
    Custom(String),
}

impl std::fmt::Display for SerialError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SerialError::Message(s) => write!(f, "{}", s),
            SerialError::EndOfInput => write!(f, "end of input"),
            SerialError::InvalidType => write!(f, "invalid type"),
            SerialError::UnknownField(s) => write!(f, "unknown field: {}", s),
            SerialError::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl std::error::Error for SerialError {}

// =============================================================================
// Binary Serializer
// =============================================================================

/// Binary serializer
pub struct BinarySerializer {
    buffer: Vec<u8>,
}

impl BinarySerializer {
    /// Create new binary serializer
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    /// Get bytes
    pub fn into_bytes(self) -> Vec<u8> {
        self.buffer
    }
}

impl Default for BinarySerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for BinarySerializer {
    fn serialize<T: Serialize<Self>>(&mut self, value: &T) -> Result<(), SerialError> {
        value.serialize(self)
    }

    fn serialize_bool(&mut self, v: bool) -> Result<(), SerialError> {
        self.buffer.push(if v { 1 } else { 0 });
        Ok(())
    }

    fn serialize_i32(&mut self, v: i32) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_i64(&mut self, v: i64) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u32(&mut self, v: u32) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_u64(&mut self, v: u64) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f32(&mut self, v: f32) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<(), SerialError> {
        self.buffer.extend_from_slice(&v.to_le_bytes());
        Ok(())
    }

    fn serialize_str(&mut self, v: &str) -> Result<(), SerialError> {
        let bytes = v.as_bytes();
        self.serialize_u32(bytes.len() as u32)?;
        self.buffer.extend_from_slice(bytes);
        Ok(())
    }

    fn serialize_seq(&mut self, len: usize) -> Result<SeqSerializer<'_, Self>, SerialError> {
        self.serialize_u32(len as u32)?;
        Ok(SeqSerializer { serializer: self })
    }

    fn serialize_map(&mut self, len: usize) -> Result<MapSerializer<'_, Self>, SerialError> {
        self.serialize_u32(len as u32)?;
        Ok(MapSerializer { serializer: self })
    }
}

// =============================================================================
// Binary Deserializer
// =============================================================================

/// Binary deserializer
pub struct BinaryDeserializer {
    buffer: Vec<u8>,
    position: usize,
}

impl BinaryDeserializer {
    /// Create from bytes
    pub fn from_bytes(buffer: Vec<u8>) -> Self {
        Self { buffer, position: 0 }
    }

    /// Get remaining bytes
    fn remaining(&self) -> &[u8] {
        &self.buffer[self.position..]
    }

    /// Read bytes
    fn read_bytes(&mut self, len: usize) -> Result<&[u8], SerialError> {
        if self.position + len > self.buffer.len() {
            return Err(SerialError::EndOfInput);
        }
        let result = &self.buffer[self.position..self.position + len];
        self.position += len;
        Ok(result)
    }
}

impl Deserializer for BinaryDeserializer {
    fn deserialize<T: Deserialize<Self>>(self) -> Result<T, SerialError> {
        T::deserialize(self)
    }

    fn deserialize_bool(self) -> Result<bool, SerialError> {
        let bytes = self.read_bytes(1)?;
        Ok(bytes[0] != 0)
    }

    fn deserialize_i32(self) -> Result<i32, SerialError> {
        let bytes = self.read_bytes(4)?;
        Ok(i32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_i64(self) -> Result<i64, SerialError> {
        let bytes = self.read_bytes(8)?;
        Ok(i64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_u32(self) -> Result<u32, SerialError> {
        let bytes = self.read_bytes(4)?;
        Ok(u32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_u64(self) -> Result<u64, SerialError> {
        let bytes = self.read_bytes(8)?;
        Ok(u64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_f32(self) -> Result<f32, SerialError> {
        let bytes = self.read_bytes(4)?;
        Ok(f32::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_f64(self) -> Result<f64, SerialError> {
        let bytes = self.read_bytes(8)?;
        Ok(f64::from_le_bytes(bytes.try_into().unwrap()))
    }

    fn deserialize_str(self) -> Result<String, SerialError> {
        let len = self.deserialize_u32()?;
        let bytes = self.read_bytes(len as usize)?;
        String::from_utf8(bytes.to_vec()).map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_seq(self) -> Result<SeqDeserializer<Self>, SerialError> {
        Ok(SeqDeserializer { data: self })
    }

    fn deserialize_map(self) -> Result<MapDeserializer<Self>, SerialError> {
        Ok(MapDeserializer { data: self })
    }
}

// =============================================================================
// JSON Serializer
// =============================================================================

/// JSON serializer
pub struct JsonSerializer {
    buffer: String,
    pretty: bool,
    indent: usize,
}

impl JsonSerializer {
    /// Create new JSON serializer
    pub fn new() -> Self {
        Self {
            buffer: String::new(),
            pretty: false,
            indent: 0,
        }
    }

    /// Enable pretty printing
    pub fn pretty(mut self) -> Self {
        self.pretty = true;
        self
    }

    /// Get JSON string
    pub fn into_string(self) -> String {
        self.buffer
    }

    fn indent(&mut self) {
        if self.pretty {
            self.buffer.push_str(&"  ".repeat(self.indent));
        }
    }

    fn newline(&mut self) {
        if self.pretty {
            self.buffer.push('\n');
        }
    }
}

impl Default for JsonSerializer {
    fn default() -> Self {
        Self::new()
    }
}

impl Serializer for JsonSerializer {
    fn serialize<T: Serialize<Self>>(&mut self, value: &T) -> Result<(), SerialError> {
        value.serialize(self)
    }

    fn serialize_bool(&mut self, v: bool) -> Result<(), SerialError> {
        self.buffer.push_str(if v { "true" } else { "false" });
        Ok(())
    }

    fn serialize_i32(&mut self, v: i32) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_i64(&mut self, v: i64) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u32(&mut self, v: u32) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_u64(&mut self, v: u64) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f32(&mut self, v: f32) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_f64(&mut self, v: f64) -> Result<(), SerialError> {
        self.buffer.push_str(&v.to_string());
        Ok(())
    }

    fn serialize_str(&mut self, v: &str) -> Result<(), SerialError> {
        self.buffer.push('"');
        self.buffer.push_str(&escape_json_string(v));
        self.buffer.push('"');
        Ok(())
    }

    fn serialize_seq(&mut self, len: usize) -> Result<SeqSerializer<'_, Self>, SerialError> {
        self.buffer.push('[');
        self.newline();
        self.indent += 1;
        Ok(SeqSerializer { serializer: self })
    }

    fn serialize_map(&mut self, len: usize) -> Result<MapSerializer<'_, Self>, SerialError> {
        self.buffer.push('{');
        self.newline();
        self.indent += 1;
        Ok(MapSerializer { serializer: self })
    }
}

impl<'a, S: Serializer> Drop for SeqSerializer<'a, S> {
    fn drop(&mut self) {
        // Cleanup handled manually via end()
    }
}

impl<'a, S: Serializer> Drop for MapSerializer<'a, S> {
    fn drop(&mut self) {
        // Cleanup handled manually via end()
    }
}

/// Escape JSON string
fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            c if c.is_control() => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            c => result.push(c),
        }
    }
    result
}

// =============================================================================
// JSON Deserializer
// =============================================================================

/// JSON deserializer
pub struct JsonDeserializer {
    input: Vec<char>,
    position: usize,
}

impl JsonDeserializer {
    /// Create from string
    pub fn from_str(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    /// Peek next character
    fn peek(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    /// Consume next character
    fn next(&mut self) -> Option<char> {
        let c = self.input.get(self.position).copied();
        self.position += 1;
        c
    }

    /// Skip whitespace
    fn skip_ws(&mut self) {
        while let Some(c) = self.peek() {
            if c.is_whitespace() {
                self.next();
            } else {
                break;
            }
        }
    }
}

impl Deserializer for JsonDeserializer {
    fn deserialize<T: Deserialize<Self>>(self) -> Result<T, SerialError> {
        T::deserialize(self)
    }

    fn deserialize_bool(self) -> Result<bool, SerialError> {
        let mut d = self;
        d.skip_ws();

        let remaining: String = d.input[d.position..].iter().take(5).collect();
        if remaining.starts_with("true") {
            d.position += 4;
            Ok(true)
        } else if remaining.starts_with("false") {
            d.position += 5;
            Ok(false)
        } else {
            Err(SerialError::InvalidType)
        }
    }

    fn deserialize_str(self) -> Result<String, SerialError> {
        let mut d = self;
        d.skip_ws();

        if d.next() != Some('"') {
            return Err(SerialError::InvalidType);
        }

        let mut result = String::new();
        while let Some(c) = d.next() {
            match c {
                '"' => return Ok(result),
                '\\' => {
                    if let Some(escaped) = d.next() {
                        match escaped {
                            '"' => result.push('"'),
                            '\\' => result.push('\\'),
                            'n' => result.push('\n'),
                            'r' => result.push('\r'),
                            't' => result.push('\t'),
                            'u' => {
                                // Unicode escape
                                let mut hex = String::new();
                                for _ in 0..4 {
                                    if let Some(h) = d.next() {
                                        hex.push(h);
                                    }
                                }
                                if let Ok(code) = u32::from_str_radix(&hex, 16) {
                                    result.push(char::from_u32(code).unwrap_or('\u{FFFD}'));
                                }
                            }
                            c => result.push(c),
                        }
                    }
                }
                c => result.push(c),
            }
        }

        Err(SerialError::EndOfInput)
    }

    fn deserialize_seq(self) -> Result<SeqDeserializer<Self>, SerialError> {
        let mut d = self;
        d.skip_ws();

        if d.next() != Some('[') {
            return Err(SerialError::InvalidType);
        }

        Ok(SeqDeserializer { data: d })
    }

    fn deserialize_map(self) -> Result<MapDeserializer<Self>, SerialError> {
        let mut d = self;
        d.skip_ws();

        if d.next() != Some('{') {
            return Err(SerialError::InvalidType);
        }

        Ok(MapDeserializer { data: d })
    }

    fn deserialize_i32(self) -> Result<i32, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_i64(self) -> Result<i64, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_u32(self) -> Result<u32, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_u64(self) -> Result<u64, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_f32(self) -> Result<f32, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }

    fn deserialize_f64(self) -> Result<f64, SerialError> {
        self.deserialize_str()?.parse().map_err(|_| SerialError::InvalidType)
    }
}

// =============================================================================
// Built-in Serialize/Deserialize implementations
// =============================================================================

impl<S: Serializer> Serialize<S> for i32 {
    fn serialize(&self, serializer: &mut S) -> Result<(), SerialError> {
        serializer.serialize_i32(*self)
    }
}

impl<D: Deserializer> Deserialize<D> for i32 {
    fn deserialize(deserializer: D) -> Result<Self, SerialError> {
        deserializer.deserialize_i32()
    }
}

impl<S: Serializer> Serialize<S> for String {
    fn serialize(&self, serializer: &mut S) -> Result<(), SerialError> {
        serializer.serialize_str(self)
    }
}

impl<D: Deserializer> Deserialize<D> for String {
    fn deserialize(deserializer: D) -> Result<Self, SerialError> {
        deserializer.deserialize_str()
    }
}

impl<S: Serializer, T: Serialize<S>> Serialize<S> for Vec<T> {
    fn serialize(&self, serializer: &mut S) -> Result<(), SerialError> {
        let mut seq = serializer.serialize_seq(self.len())?;
        for item in self {
            seq.serialize_element(item)?;
        }
        seq.end()
    }
}

impl<D: Deserializer, T: Deserialize<D>> Deserialize<D> for Vec<T> {
    fn deserialize(deserializer: D) -> Result<Self, SerialError> {
        let mut seq = deserializer.deserialize_seq()?;
        let mut result = Vec::new();

        loop {
            match seq.next::<T>() {
                Ok(Some((item, next))) => {
                    result.push(item);
                    seq = next;
                }
                Ok(None) => break,
                Err(e) => return Err(e),
            }
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_serialize_bool() {
        let mut ser = BinarySerializer::new();
        ser.serialize_bool(true).unwrap();
        ser.serialize_bool(false).unwrap();
        let bytes = ser.into_bytes();
        assert_eq!(bytes, vec![1, 0]);
    }

    #[test]
    fn test_binary_roundtrip() {
        let mut ser = BinarySerializer::new();
        ser.serialize_i32(42).unwrap();
        ser.serialize_str("hello").unwrap();

        let mut de = BinaryDeserializer::from_bytes(ser.into_bytes());
        assert_eq!(de.deserialize_i32().unwrap(), 42);
        assert_eq!(de.deserialize_str().unwrap(), "hello");
    }

    #[test]
    fn test_json_serialize() {
        let mut ser = JsonSerializer::new().pretty();
        ser.serialize_str("hello").unwrap();
        let json = ser.into_string();
        assert_eq!(json, "\"hello\"");
    }
}
