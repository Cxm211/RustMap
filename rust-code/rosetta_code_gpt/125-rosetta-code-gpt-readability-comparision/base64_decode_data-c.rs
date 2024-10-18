fn find_index(val: u8) -> i32 {
    if b'A' <= val && val <= b'Z' {
        return (val - b'A') as i32;
    }
    if b'a' <= val && val <= b'z' {
        return (val - b'a' + 26) as i32;
    }
    if b'0' <= val && val <= b'9' {
        return (val - b'0' + 52) as i32;
    }
    if val == b'+' {
        return 62;
    }
    if val == b'/' {
        return 63;
    }
    -1
}

fn decode(source: &[u8], sink: &mut Vec<u8>) -> i32 {
    let length = source.len();
    
    if length % 4 != 0 {
        return 1;
    }

    let mut it = source.iter();

    while let (Some(&b1), Some(&b2), Some(&b3), Some(&b4)) = (it.next(), it.next(), it.next(), it.next()) {
        let i1 = find_index(b1);
        let i2 = find_index(b2);

        let mut acc = i1 << 2;
        acc |= i2 >> 4;
        sink.push(acc as u8);

        if b3 != b'=' {
            let i3 = find_index(b3);
            let mut acc = (i2 & 0xF) << 4;
            acc += i3 >> 2;
            sink.push(acc as u8);

            if b4 != b'=' {
                let i4 = find_index(b4);
                let mut acc = (i3 & 0x3) << 6;
                acc |= i4;
                sink.push(acc as u8);
            }
        }
    }

    0
}

fn main() {
    let data = b"VG8gZXJyIGlzIGh1bWFuLCBidXQgdG8gcmVhbGx5IGZvdWwgdGhpbmdzIHVwIHlvdSBuZWVkIGEgY29tcHV0ZXIuCiAgICAtLVBhdWwgUi5FaHJsaWNo";
    let mut decoded = Vec::new();

    println!("{:?}", std::str::from_utf8(data).unwrap());

    let result = decode(data, &mut decoded);
    if result == 0 {
        println!("{:?}", std::str::from_utf8(&decoded).unwrap());
    } else {
        println!("Invalid base64 input");
    }
}