use crate::*;

impl WebSocketFrame {
    /// decode_websocket_frame_with_length
    ///
    /// # Parameters
    /// - `data`: The raw data slice from the WebSocket stream.
    ///
    /// # Returns
    /// - An Option containing a tuple (WebSocketFrame, usize), where the WebSocketFrame is the decoded frame and usize is the number of bytes consumed.
    ///   Returns None if the frame is incomplete.
    pub fn decode_websocket_frame_with_length(data: &[u8]) -> WebsocketFrameWithLengthOption {
        if data.len() < 2 {
            return None;
        }
        let mut index: usize = 0;
        let fin: bool = (data[index] & 0b1000_0000) != 0;
        let opcode: u8 = data[index] & 0b0000_1111;
        index += 1;
        let mask: bool = (data[index] & 0b1000_0000) != 0;
        let mut payload_len: usize = (data[index] & 0b0111_1111) as usize;
        index += 1;
        if payload_len == 126 {
            if data.len() < index + 2 {
                return None;
            }
            payload_len = u16::from_be_bytes(data[index..index + 2].try_into().ok()?) as usize;
            index += 2;
        } else if payload_len == 127 {
            if data.len() < index + 8 {
                return None;
            }
            payload_len = u64::from_be_bytes(data[index..index + 8].try_into().ok()?) as usize;
            index += 8;
        }
        let mask_key: Option<[u8; 4]> = if mask {
            if data.len() < index + 4 {
                return None;
            }
            let key: [u8; 4] = data[index..index + 4].try_into().ok()?;
            index += 4;
            Some(key)
        } else {
            None
        };
        if data.len() < index + payload_len {
            return None;
        }
        let mut payload: Vec<u8> = data[index..index + payload_len].to_vec();
        if let Some(mask_key) = mask_key {
            for (i, byte) in payload.iter_mut().enumerate() {
                *byte ^= mask_key[i % 4];
            }
        }
        index += payload_len;
        let frame: WebSocketFrame = WebSocketFrame {
            fin,
            opcode,
            mask,
            payload_data: payload,
        };
        Some((frame, index))
    }

    /// create_response_frame_list
    ///
    /// - `body`: A reference to a response body (payload) as a byte slice.
    /// - Returns: A vector of response bodies (frames) representing the framed data.
    #[inline]
    pub fn create_response_frame_list(body: &ResponseBody) -> Vec<ResponseBody> {
        let total_len: usize = body.len();
        let mut offset: usize = 0;
        let mut frames_list: Vec<ResponseBody> = Vec::new();
        let mut is_first_frame: bool = true;
        let is_valid_utf8: bool = std::str::from_utf8(body).is_ok();
        let base_opcode: u8 = if is_valid_utf8 { 0x01 } else { 0x02 };
        while offset < total_len {
            let remaining: usize = total_len - offset;
            let mut frame_size: usize = remaining.min(MAX_FRAME_SIZE);
            if is_valid_utf8 && frame_size < remaining {
                while frame_size > 0 && (body[offset + frame_size] & 0xC0) == 0x80 {
                    frame_size -= 1;
                }
                if frame_size == 0 {
                    frame_size = remaining.min(MAX_FRAME_SIZE);
                }
            }
            let mut frame: ResponseBody = Vec::with_capacity(frame_size + 10);
            let opcode: u8 = if is_first_frame { base_opcode } else { 0x00 };
            let fin_bit: u8 = if remaining > frame_size { 0x00 } else { 0x80 };
            frame.push(fin_bit | opcode);
            if frame_size < 126 {
                frame.push(frame_size as u8);
            } else if frame_size <= MAX_FRAME_SIZE {
                frame.push(126);
                frame.extend_from_slice(&(frame_size as u16).to_be_bytes());
            } else {
                frame.push(127);
                frame.extend_from_slice(&(frame_size as u64).to_be_bytes());
            }
            frame.extend_from_slice(&body[offset..offset + frame_size]);
            frames_list.push(frame);
            offset += frame_size;
            is_first_frame = false;
        }
        frames_list
    }

    /// sha1
    ///
    /// - `data`: A byte slice containing the input data to be hashed.
    /// - Returns: A 20-byte array representing the SHA-1 hash of the input data.
    #[inline]
    pub fn sha1(data: &[u8]) -> [u8; 20] {
        let mut hash_state: [u32; 5] = HASH_STATE;
        let mut padded_data: Vec<u8> = Vec::from(data);
        let original_length_bits: u64 = (padded_data.len() * 8) as u64;
        padded_data.push(0x80);
        while (padded_data.len() + 8) % 64 != 0 {
            padded_data.push(0);
        }
        padded_data.extend_from_slice(&original_length_bits.to_be_bytes());
        for block in padded_data.chunks_exact(64) {
            let mut message_schedule: [u32; 80] = [0u32; 80];
            for (i, block_chunk) in block.chunks_exact(4).enumerate().take(16) {
                message_schedule[i] = u32::from_be_bytes([
                    block_chunk[0],
                    block_chunk[1],
                    block_chunk[2],
                    block_chunk[3],
                ]);
            }
            for i in 16..80 {
                message_schedule[i] = (message_schedule[i - 3]
                    ^ message_schedule[i - 8]
                    ^ message_schedule[i - 14]
                    ^ message_schedule[i - 16])
                    .rotate_left(1);
            }
            let [mut a, mut b, mut c, mut d, mut e] = hash_state;
            for (i, &word) in message_schedule.iter().enumerate() {
                let (f, k) = match i {
                    0..=19 => ((b & c) | (!b & d), 0x5A827999),
                    20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                    40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                    _ => (b ^ c ^ d, 0xCA62C1D6),
                };
                let temp: u32 = a
                    .rotate_left(5)
                    .wrapping_add(f)
                    .wrapping_add(e)
                    .wrapping_add(k)
                    .wrapping_add(word);
                e = d;
                d = c;
                c = b.rotate_left(30);
                b = a;
                a = temp;
            }
            hash_state[0] = hash_state[0].wrapping_add(a);
            hash_state[1] = hash_state[1].wrapping_add(b);
            hash_state[2] = hash_state[2].wrapping_add(c);
            hash_state[3] = hash_state[3].wrapping_add(d);
            hash_state[4] = hash_state[4].wrapping_add(e);
        }
        let mut result: [u8; 20] = [0u8; 20];
        for (i, &val) in hash_state.iter().enumerate() {
            result[i * 4..(i + 1) * 4].copy_from_slice(&val.to_be_bytes());
        }
        result
    }

    /// generate_accept_key
    ///
    /// - `key`: A string slice containing the client-provided key.
    /// - Returns: A string representing the generated WebSocket accept key.
    #[inline]
    pub fn generate_accept_key(key: &str) -> String {
        let mut data: [u8; 60] = [0u8; 60];
        data[..24].copy_from_slice(&key.as_bytes()[..24.min(key.len())]);
        data[24..].copy_from_slice(GUID);
        let hash: [u8; 20] = Self::sha1(&data);
        Self::base64_encode(&hash)
    }

    /// base64_encode
    ///
    /// - `data`: A byte slice containing the data to encode in base64.
    /// - Returns: A string with the base64 encoded representation of the input data.
    #[inline]
    pub fn base64_encode(data: &[u8]) -> String {
        let mut encoded_data: Vec<u8> = Vec::with_capacity((data.len() + 2) / 3 * 4);
        for chunk in data.chunks(3) {
            let mut buffer: [u8; 3] = [0u8; 3];
            buffer[..chunk.len()].copy_from_slice(chunk);
            let indices: [u8; 4] = [
                buffer[0] >> 2,
                ((buffer[0] & 0b11) << 4) | (buffer[1] >> 4),
                ((buffer[1] & 0b1111) << 2) | (buffer[2] >> 6),
                buffer[2] & 0b111111,
            ];
            for &idx in &indices[..chunk.len() + 1] {
                encoded_data.push(BASE64_CHARSET_TABLE[idx as usize]);
            }
            while encoded_data.len() % 4 != 0 {
                encoded_data.push(EQUAL_BYTES[0]);
            }
        }
        String::from_utf8(encoded_data).unwrap()
    }
}
