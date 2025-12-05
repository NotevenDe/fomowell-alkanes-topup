use bitcoin::opcodes::all;
use bitcoin::script::{Builder, PushBytesBuf};
use bitcoin::ScriptBuf;

pub mod alkanes_protostone {
    use super::{all, Builder, PushBytesBuf, ScriptBuf};

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct RuneId {
        pub block: u64,
        pub tx: u32,
    }

    #[derive(Debug, Clone)]
    pub struct Edict {
        pub id: RuneId,
        pub amount: u128,
        pub output: u32,
    }

    #[derive(Debug, Clone)]
    pub struct Protostone {
        pub subprotocol_id: u128,
        pub edicts: Vec<Edict>,
        pub pointer: Option<u32>,
        pub refund_pointer: Option<u32>,
        pub burn: Option<u128>,
        pub message: Option<Vec<u8>>,
        pub from: Option<Vec<u32>>,
    }

    mod prototag {
        pub const BODY: u128 = 0;
        pub const MESSAGE: u128 = 81;
        pub const BURN: u128 = 83;
        pub const POINTER: u128 = 91;
        pub const REFUND: u128 = 93;
        pub const FROM: u128 = 95;
        pub const CENOTAPH: u128 = 126;
        pub const NOP: u128 = 127;
    }
    use prototag::*;

    #[inline]
    fn push_leb128_u128(mut value: u128, out: &mut Vec<u8>) {
        out.reserve(19);

        loop {
            let mut byte = (value & 0x7f) as u8;
            value >>= 7;

            if value != 0 {
                byte |= 0x80;
                out.push(byte);
            } else {
                out.push(byte);
                break;
            }
        }
    }

    #[inline]
    fn read_leb128_u128(bytes: &[u8], idx: &mut usize) -> Option<u128> {
        let mut result: u128 = 0;
        let mut shift = 0;

        while *idx < bytes.len() {
            let byte = bytes[*idx];
            *idx += 1;

            result |= ((byte & 0x7f) as u128) << shift;

            if (byte & 0x80) == 0 {
                return Some(result);
            }

            shift += 7;
            if shift >= 128 {
                return None;
            }
        }

        None
    }

    const CHUNK_SIZE: usize = 15;

    fn pack_bytes_as_u128_chunks(bytes: &[u8]) -> Vec<u128> {
        if bytes.is_empty() {
            return Vec::new();
        }

        let chunk_count = (bytes.len() + CHUNK_SIZE - 1) / CHUNK_SIZE;
        let mut out = Vec::with_capacity(chunk_count);

        for chunk in bytes.chunks(CHUNK_SIZE) {
            let mut v: u128 = 0;
            for (j, &b) in chunk.iter().enumerate() {
                v |= (b as u128) << (j * 8);
            }
            out.push(v);
        }

        out
    }

    fn unpack_u128_chunks_to_bytes(ints: &[u128]) -> Vec<u8> {
        if ints.is_empty() {
            return Vec::new();
        }

        let mut out = Vec::with_capacity(ints.len() * CHUNK_SIZE);

        for &v in ints {
            if v == 0 {
                out.push(0);
                continue;
            }

            let mut x = v;
            for _ in 0..CHUNK_SIZE {
                out.push((x & 0xff) as u8);
                x >>= 8;
                if x == 0 {
                    break;
                }
            }
        }

        out
    }

    fn encode_edicts_to_ints(edicts: &[super::alkanes_protostone::Edict]) -> Vec<u128> {
        if edicts.is_empty() {
            return Vec::new();
        }

        let mut eds = edicts.to_vec();
        eds.sort_unstable_by_key(|e| (e.id.block, e.id.tx));

        let mut ints = Vec::with_capacity(eds.len() * 4);
        let (mut base_b, mut base_tx) = (0u64, 0u32);

        for e in eds {
            let b_delta = e.id.block - base_b;
            ints.push(b_delta as u128);

            let tx = if b_delta == 0 {
                (e.id.tx - base_tx) as u128
            } else {
                e.id.tx as u128
            };
            ints.push(tx);
            ints.push(e.amount);
            ints.push(e.output as u128);

            base_b = e.id.block;
            base_tx = e.id.tx;
        }

        ints
    }

    fn parse_edicts_from_body(ints: &[u128], mut idx: usize, end: usize) -> (Vec<super::alkanes_protostone::Edict>, usize) {
        let capacity = (end - idx) / 4;
        let mut edicts = Vec::with_capacity(capacity);
        let mut base_b: u64 = 0;
        let mut base_tx: u32 = 0;

        while idx + 3 < end {
            let b_delta = ints[idx] as u64;
            let tx_val = ints[idx + 1] as u32;
            let amount = ints[idx + 2];
            let output = ints[idx + 3] as u32;
            idx += 4;

            let block = base_b + b_delta;
            let tx = if b_delta == 0 {
                base_tx + tx_val
            } else {
                tx_val
            };

            base_b = block;
            base_tx = tx;

            edicts.push(super::alkanes_protostone::Edict {
                id: super::alkanes_protostone::RuneId { block, tx },
                amount,
                output,
            });
        }

        (edicts, idx)
    }

    const MAGIC_BYTE: u8 = 0x01;

    #[inline]
    fn encode_calldata(msg: &[u8]) -> Vec<u8> {
        let mut buf = Vec::with_capacity(msg.len() + 2);
        buf.push(MAGIC_BYTE);
        buf.extend_from_slice(msg);
        buf.push(MAGIC_BYTE);
        buf
    }

    fn decode_calldata(data: &[u8]) -> Option<Vec<u8>> {
        if data.len() < 2 || data[0] != MAGIC_BYTE {
            return None;
        }
        let end = data.iter().rposition(|&b| b != 0x00)?;
        if data[end] != MAGIC_BYTE {
            return None;
        }
        if end <= 1 {
            return Some(Vec::new());
        }
        Some(data[1..end].to_vec())
    }

    fn encode_protostone_to_ints(p: &super::alkanes_protostone::Protostone) -> Vec<u128> {
        let mut ints = Vec::with_capacity(64);

        ints.push(p.subprotocol_id);

        let len_index = ints.len();
        ints.push(0);

        if let Some(b) = p.burn {
            ints.push(BURN);
            ints.push(b);
        }

        if let Some(ptr) = p.pointer {
            ints.push(POINTER);
            ints.push(ptr as u128);
        }

        if let Some(rp) = p.refund_pointer {
            ints.push(REFUND);
            ints.push(rp as u128);
        }

        if let Some(ref v) = p.from {
            for &idx in v {
                ints.push(FROM);
                ints.push(idx as u128);
            }
        }

        if let Some(ref msg_raw) = p.message {
            let buf = encode_calldata(msg_raw);
            let chunks = pack_bytes_as_u128_chunks(&buf);
            for ch in chunks {
                ints.push(MESSAGE);
                ints.push(ch);
            }
        }

        if !p.edicts.is_empty() {
            ints.push(BODY);
            ints.extend(encode_edicts_to_ints(&p.edicts));
        }

        ints[len_index] = (ints.len() - len_index - 1) as u128;

        ints
    }

    pub fn encode_protocol_field(protos: &[super::alkanes_protostone::Protostone]) -> Vec<u128> {
        if protos.is_empty() {
            return Vec::new();
        }
        let mut internal = Vec::new();
        for p in protos {
            internal.extend(encode_protostone_to_ints(p));
        }
        let mut bytes = Vec::new();
        for x in internal {
            push_leb128_u128(x, &mut bytes);
        }
        pack_bytes_as_u128_chunks(&bytes)
    }

    fn decode_protocol_chunks_to_ints(chunks: &[u128]) -> Vec<u128> {
        let bytes = unpack_u128_chunks_to_bytes(chunks);
        let mut ints = Vec::new();
        let mut idx = 0;

        while idx < bytes.len() {
            if let Some(v) = read_leb128_u128(&bytes, &mut idx) {
                ints.push(v);
            } else {
                break;
            }
        }

        ints
    }

    pub fn parse_protostones(chunks: &[u128]) -> Vec<super::alkanes_protostone::Protostone> {
        let ints = decode_protocol_chunks_to_ints(chunks);
        parse_protostones_from_ints(&ints)
    }

    pub fn parse_protostones_from_ints(ints: &[u128]) -> Vec<super::alkanes_protostone::Protostone> {
        let mut res = Vec::new();
        let mut pos = 0;

        while pos + 2 <= ints.len() {
            let subprotocol_id = ints[pos];
            let field_len = ints[pos + 1] as usize;

            if subprotocol_id == 0 || field_len == 0 {
                break;
            }

            if pos + 2 + field_len > ints.len() {
                break;
            }

            let start = pos + 2;
            let end = start + field_len;

            let mut p = super::alkanes_protostone::Protostone {
                subprotocol_id,
                edicts: Vec::new(),
                pointer: None,
                refund_pointer: None,
                burn: None,
                message: None,
                from: None,
            };

            let mut i = start;
            let mut from_vec = Vec::new();
            let mut msg_chunks = Vec::new();

            while i < end {
                let tag = ints[i];
                i += 1;

                match tag {
                    BURN if i < end => {
                        p.burn = Some(ints[i]);
                        i += 1;
                    }
                    POINTER if i < end => {
                        p.pointer = Some(ints[i] as u32);
                        i += 1;
                    }
                    REFUND if i < end => {
                        p.refund_pointer = Some(ints[i] as u32);
                        i += 1;
                    }
                    FROM if i < end => {
                        from_vec.push(ints[i] as u32);
                        i += 1;
                    }
                    MESSAGE if i < end => {
                        msg_chunks.push(ints[i]);
                        i += 1;
                    }
                    BODY => {
                        let (eds, new_i) = parse_edicts_from_body(ints, i, end);
                        p.edicts = eds;
                        i = new_i;
                    }
                    _ => {
                        if i < end {
                            i += 1;
                        }
                    }
                }
            }

            if !from_vec.is_empty() {
                p.from = Some(from_vec);
            }

            if !msg_chunks.is_empty() {
                let raw_bytes = unpack_u128_chunks_to_bytes(&msg_chunks);
                if let Some(decoded) = decode_calldata(&raw_bytes) {
                    p.message = Some(decoded);
                }
            }

            res.push(p);
            pos = end;
        }
        res
    }

    pub fn build_alkanes_transfer_script(proto: &super::alkanes_protostone::Protostone) -> ScriptBuf {
        let chunks = encode_protocol_field(&[proto.clone()]);

        let mut payload = Vec::new();
        push_leb128_u128(16383, &mut payload);

        for v in chunks {
            push_leb128_u128(v, &mut payload);
        }

        let pb = PushBytesBuf::try_from(payload).expect("invalid push bytes for protocol field");

        Builder::new()
            .push_opcode(all::OP_RETURN)
            .push_opcode(all::OP_PUSHNUM_13)
            .push_slice(pb)
            .into_script()
    }
}
