use std::borrow::Cow;

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine};
use russh::{keys::key::PublicKey, server::KexInitPacket};
use serde::{
    ser::{SerializeMap, SerializeSeq},
    Serialize, Serializer,
};

#[serde_with::serde_as]
#[derive(Serialize, Debug)]
pub struct EventContainer {
    pub session_id: uuid::Uuid,
    #[serde_as(as = "serde_with::TimestampMilliSeconds")]
    pub time: std::time::SystemTime,
    pub event: Event,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "snake_case", tag = "type", content = "data")]
pub enum Event {
    TcpConnection {
        addr: std::net::SocketAddr,
    },
    Version {
        #[serde(serialize_with = "serialize_bytes")]
        version: Vec<u8>,
    },
    KexInit {
        #[serde(
            serialize_with = "serialize_kex_init_packet",
            rename = "packet"
        )]
        packet: KexInitPacket,
    },
    PublicKeyAuth {
        #[serde(serialize_with = "serialize_bytes")]
        user: Vec<u8>,
        #[serde(serialize_with = "serialize_pubkey")]
        key: PublicKey,
    },
    PasswordAuth {
        #[serde(serialize_with = "serialize_bytes")]
        user: Vec<u8>,
        #[serde(serialize_with = "serialize_bytes")]
        password: Vec<u8>,
    },
}

struct NamesWrapper<'a>(&'a [Vec<u8>]);

impl Serialize for NamesWrapper<'_> {
    fn serialize<S>(&self, ser: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = ser.serialize_seq(Some(self.0.len()))?;

        for name in self.0 {
            seq.serialize_element(&bytes_to_json(name))?;
        }

        seq.end()
    }
}

fn bytes_to_json<T: AsRef<[u8]>>(data: &T) -> Cow<str> {
    String::from_utf8_lossy(data.as_ref())
}

fn serialize_bytes<T, S>(data: &T, ser: S) -> Result<S::Ok, S::Error>
where
    T: AsRef<[u8]>,
    S: Serializer,
{
    ser.serialize_str(&bytes_to_json(data))
}

fn serialize_pubkey<S>(key: &PublicKey, ser: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    ser.serialize_str(&format!("{} {}", key.name(), key.fingerprint()))
}

fn serialize_kex_init_packet<S>(
    packet: &KexInitPacket,
    ser: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut map = ser.serialize_map(Some(12))?;
    map.serialize_entry("cookie", &STANDARD_NO_PAD.encode(packet.cookie))?;

    for (key, value) in [
        ("kex", &packet.kex),
        ("host_key", &packet.host_key),
        ("c2s_ciphers", &packet.c2s_ciphers),
        ("s2c_ciphers", &packet.s2c_ciphers),
        ("c2s_macs", &packet.c2s_macs),
        ("s2c_macs", &packet.s2c_macs),
        ("c2s_compression", &packet.c2s_compression),
        ("s2c_compression", &packet.s2c_compression),
        ("c2s_languages", &packet.c2s_languages),
        ("s2c_languages", &packet.s2c_languages),
    ] {
        map.serialize_entry(key, &NamesWrapper(value))?;
    }

    map.serialize_entry("reserved", &packet.reserved)?;
    map.end()
}
