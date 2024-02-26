pub fn left_rotate(byte: i32, n: i32) -> i32
{
    return (byte << n)|(n >> (32-n));
}

pub fn right_rotate(byte: i32, n: i32) -> i32
{
    return (byte >> n)|(byte << (32-n)) & 0xffffffffu32 as i32;
}

pub fn sha256(word: &str) -> String
{
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    //
    // STEP 1:
    // Encoding the input into an array of 32-bits

    let mut count4 = 0;
    let mut b32: Vec<i32> = vec![];
    let mut temp: i32 = 0;
    let mut b_num: i64 = 0;
    for c in word.chars()
    {
        temp = temp << 8;
        temp += c as i32;
        count4 += 1;
        if count4 % 4 == 0
        {
            count4 = 0;
            b32.push(temp);
            temp = 0;
        }
        b_num += 1;
    }
    if count4 == 0
    {
        temp = 1;
        temp = temp << 31;
        b32.push(temp);
    }
    if count4 == 1
    {
        temp = temp << 8;
        temp += 8388608;
        b32.push(temp);
    }
    if count4 == 2
    {
        temp = temp << 8;
        temp += 32768;
        b32.push(temp);
    }
    if count4 == 3
    {
        temp = temp << 8;
        temp += 128;
        b32.push(temp);
    }
    b_num *= 8;
    let mut chunk_num = (b_num + 72) / 512; // 72 = 64 + 8
                                            //    = big endian len + trailing 1
    if (b_num + 72) % 512 != 0
    {
        chunk_num += 1;
    }

    let mut filling_lines = (chunk_num * 512 - b_num - 72) / 8;
    filling_lines -= filling_lines % 4;
    filling_lines = filling_lines / 4;
    for _i in 0..filling_lines
    {
        b32.push(0);
    }

    let step1: i32 = (b_num >> 32) as i32;
    let step2: i32 = b_num as i32;
    b32.push(step1);
    b32.push(step2);

    // STEP 1 -> OK
    //
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-
    //
    // STEP 2 : Splitting the first array into a smaller one to create
    // a message schedule, this schedule will contains 64 32-bits values


    // NE PAS TOUCHER PLS
    let mut hash_values: Vec<i32> = vec![
        0b01101010000010011110011001100111, 0b10111011011001111010111010000101u32 as i32,
        0b00111100011011101111001101110010, 0b10100101010011111111010100111010u32 as i32,
        0b01010001000011100101001001111111, 0b10011011000001010110100010001100u32 as i32,
        0b00011111100000111101100110101011, 0b01011011111000001100110100011001
    ];

    // NE PAS TOUCHER PLS
    let constantes: Vec<i32> = vec![
        0b01000010100010100010111110011000, 0b01110001001101110100010010010001,
        0b10110101110000001111101111001111u32 as i32, 0b11101001101101011101101110100101u32 as i32,
        0b00111001010101101100001001011011, 0b01011001111100010001000111110001,
        0b10010010001111111000001010100100u32 as i32, 0b10101011000111000101111011010101u32 as i32,
        0b11011000000001111010101010011000u32 as i32, 0b00010010100000110101101100000001,
        0b00100100001100011000010110111110, 0b01010101000011000111110111000011,
        0b01110010101111100101110101110100, 0b10000000110111101011000111111110u32 as i32,
        0b10011011110111000000011010100111u32 as i32, 0b11000001100110111111000101110100u32 as i32,
        0b11100100100110110110100111000001u32 as i32, 0b11101111101111100100011110000110u32 as i32,
        0b00001111110000011001110111000110, 0b00100100000011001010000111001100,
        0b00101101111010010010110001101111, 0b01001010011101001000010010101010,
        0b01011100101100001010100111011100, 0b01110110111110011000100011011010,
        0b10011000001111100101000101010010u32 as i32, 0b10101000001100011100011001101101u32 as i32,
        0b10110000000000110010011111001000u32 as i32, 0b10111111010110010111111111000111u32 as i32,
        0b11000110111000000000101111110011u32 as i32, 0b11010101101001111001000101000111u32 as i32,
        0b00000110110010100110001101010001, 0b00010100001010010010100101100111,
        0b00100111101101110000101010000101, 0b00101110000110110010000100111000,
        0b01001101001011000110110111111100, 0b01010011001110000000110100010011,
        0b01100101000010100111001101010100, 0b01110110011010100000101010111011,
        0b10000001110000101100100100101110u32 as i32, 0b10010010011100100010110010000101u32 as i32,
        0b10100010101111111110100010100001u32 as i32, 0b10101000000110100110011001001011u32 as i32,
        0b11000010010010111000101101110000u32 as i32, 0b11000111011011000101000110100011u32 as i32,
        0b11010001100100101110100000011001u32 as i32, 0b11010110100110010000011000100100u32 as i32,
        0b11110100000011100011010110000101u32 as i32, 0b00010000011010101010000001110000,
        0b00011001101001001100000100010110, 0b00011110001101110110110000001000,
        0b00100111010010000111011101001100, 0b00110100101100001011110010110101,
        0b00111001000111000000110010110011, 0b01001110110110001010101001001010,
        0b01011011100111001100101001001111, 0b01101000001011100110111111110011,
        0b01110100100011111000001011101110, 0b01111000101001010110001101101111,
        0b10000100110010000111100000010100u32 as i32, 0b10001100110001110000001000001000u32 as i32,
        0b10010000101111101111111111111010u32 as i32, 0b10100100010100000110110011101011u32 as i32,
        0b10111110111110011010001111110111u32 as i32, 0b11000110011100010111100011110010u32 as i32
            ];

    let mut a: i32 = hash_values[0];
    let mut b: i32 = hash_values[1]; 
    let mut c: i32 = hash_values[2];
    let mut d: i32 = hash_values[3];
    let mut e: i32 = hash_values[4];
    let mut f: i32 = hash_values[5];
    let mut g: i32 = hash_values[6];
    let mut h: i32 = hash_values[7];

    for i in 0..chunk_num
    {
        let mut w: Vec<i32> = b32[(i*16) as usize..((i+1)*16) as usize].to_vec();
        for j in 16..64
        {
            let t1 = right_rotate(w[j-15], 7);
            let t2 = right_rotate(w[j-15], 18);
            let t3 = w[j-15] & 0b00011111111111111111111111111111;
            let sig0 = t1 ^ t2 ^ t3;

            let t1 = right_rotate(w[j-2], 17);
            let t2 = right_rotate(w[j-2], 19);
            let t3 = w[j-2] & 0b00000000001111111111111111111111;
            let sig1 = t1 ^ t2 ^ t3;

            let sum: i32 = ((w[j-16] as i64) + (sig0 as i64)
                            + (w[j-7] as i64) + (sig1 as i64)) as i32;
            w.push(sum);
        }

        // STEP 2 -> OK
        //
        // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        //
        // STEP 3 : Doing calculus while iterating over the array created before

        for j in 0..64
        {
            let t1 = right_rotate(a, 2);
            let t2 = right_rotate(a, 13);
            let t3 = right_rotate(a, 22);
            let sig0 = t1 ^ t2 ^ t3;

            let t1 = right_rotate(e, 6);
            let t2 = right_rotate(e, 11);
            let t3 = right_rotate(e, 25);
            let sig1 = t1 ^ t2 ^ t3;

            let m: i32 = (a & b) ^ (a & c) ^ (b & c);
            let choice: i32 = (e & f) ^ ((!e) & g);

            let temp1: i32 = ((sig0 as i64) + (m as i64)) as i32;
            let temp2: i32 = ((h as i64) + (sig1 as i64) + (choice as i64)
                              + (constantes[j] as i64) + (w[j] as i64)) as i32;

            h = g;
            g = f;
            f = e;
            e = ((d as i64) + (temp1 as i64)) as i32;
            d = c;
            c = b;
            b = a;
            a = ((temp1 as i64) + (temp2 as i64)) as i32;
        }

        hash_values[0] = ((hash_values[0] as i64) + (a as i64)) as i32;
        hash_values[1] = ((hash_values[1] as i64) + (b as i64)) as i32;
        hash_values[2] = ((hash_values[2] as i64) + (c as i64)) as i32;
        hash_values[3] = ((hash_values[3] as i64) + (d as i64)) as i32;
        hash_values[4] = ((hash_values[4] as i64) + (e as i64)) as i32;
        hash_values[5] = ((hash_values[5] as i64) + (f as i64)) as i32;
        hash_values[6] = ((hash_values[6] as i64) + (g as i64)) as i32;
        hash_values[7] = ((hash_values[7] as i64) + (h as i64)) as i32;

        a = hash_values[0];
        b = hash_values[1]; 
        c = hash_values[2];
        d = hash_values[3];
        e = hash_values[4];
        f = hash_values[5];
        g = hash_values[6];
        h = hash_values[7];

        // STEP 3 -> OK
        //
        // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        //
        // STEP 4 : Formatting output
    }

    let s1 = format!("{a:x}");
    let s2 = format!("{b:x}");
    let s3 = format!("{c:x}");
    let s4 = format!("{d:x}");
    let s5 = format!("{e:x}");
    let s6 = format!("{f:x}");
    let s7 = format!("{g:x}");
    let s8 = format!("{h:x}");

    let mut res = String::new();
    res.push_str(&s1);
    res.push_str(&s2);
    res.push_str(&s3);
    res.push_str(&s4);
    res.push_str(&s5);
    res.push_str(&s6);
    res.push_str(&s7);
    res.push_str(&s8);

    return res;

    // STEP 4 -> OK
    //
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
}

pub fn md5(word: &str) -> String
{
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    //
    // STEP 1:
    // Encoding the input into an array of 32-bits

    let mut count4 = 0;
    let mut b32: Vec<i32> = vec![];
    let mut temp: i32 = 0;
    let mut b_num: i64 = 0;
    for c in word.chars()
    {
        temp = temp << 8;
        temp += c as i32;
        count4 += 1;
        if count4 % 4 == 0
        {
            count4 = 0;
            b32.push(temp);
            temp = 0;
        }
        b_num += 1;
    }
    if count4 == 0
    {
        temp = 1;
        temp = temp << 31;
        b32.push(temp);
    }
    if count4 == 1
    {
        temp = temp << 8;
        temp += 8388608;
        b32.push(temp);
    }
    if count4 == 2
    {
        temp = temp << 8;
        temp += 32768;
        b32.push(temp);
    }
    if count4 == 3
    {
        temp = temp << 8;
        temp += 128;
        b32.push(temp);
    }
    b_num *= 8;
    let mut chunk_num = (b_num + 72) / 512; // 72 = 64 + 8
                                            //    = big endian len + trailing 1
    if (b_num + 72) % 512 != 0
    {
        chunk_num += 1;
    }

    let mut filling_lines = (chunk_num * 512 - b_num - 72) / 8;
    filling_lines -= filling_lines % 4;
    filling_lines = filling_lines / 4;
    for _i in 0..filling_lines
    {
        b32.push(0);
    }

    let step1: i32 = (b_num >> 32) as i32;
    let step2: i32 = b_num as i32;
    b32.push(step1);
    b32.push(step2);

    // STEP 1 -> OK
    //
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-
    //
    // STEP 2 : Declaring constant arrays

    // NE PAS TOUCHER PLS
    let mut hash_values: Vec<i32> = vec![
        0x67452301, 0xefcdab89u32 as i32, 0x98badcfeu32 as i32, 0x10325476
    ];

    let r: Vec<i32> = vec![
        7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,  7, 12, 17, 22,
        5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,  5,  9, 14, 20,
        4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,  4, 11, 16, 23,
        6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21,  6, 10, 15, 21
    ];

    // NE PAS TOUCHER PLS
    let constantes: Vec<i32> = vec![
        0xd76aa478u32 as i32, 0xe8c7b756u32 as i32, 0x242070db, 0xc1bdceeeu32 as i32,
        0xf57c0fafu32 as i32, 0x4787c62a, 0xa8304613u32 as i32, 0xfd469501u32 as i32,
        0x698098d8, 0x8b44f7afu32 as i32, 0xffff5bb1u32 as i32, 0x895cd7beu32 as i32,
        0x6b901122, 0xfd987193u32 as i32, 0xa679438eu32 as i32, 0x49b40821u32 as i32,
        0xf61e2562u32 as i32, 0xc040b340u32 as i32, 0x265e5a51, 0xe9b6c7aau32 as i32,	
        0xd62f105du32 as i32, 0x02441453, 0xd8a1e681u32 as i32, 0xe7d3fbc8u32 as i32,
        0x21e1cde6, 0xc33707d6u32 as i32, 0xf4d50d87u32 as i32, 0x455a14edu32 as i32,
        0xa9e3e905u32 as i32, 0xfcefa3f8u32 as i32, 0x676f02d9, 0x8d2a4c8au32 as i32,
        0xfffa3942u32 as i32, 0x8771f681u32 as i32, 0x6d9d6122, 0xfde5380cu32 as i32,
        0xa4beea44u32 as i32, 0x4bdecfa9, 0xf6bb4b60u32 as i32, 0xbebfbc70u32 as i32,
        0x289b7ec6, 0xeaa127fau32 as i32, 0xd4ef3085u32 as i32, 0x04881d05u32 as i32,
        0xd9d4d039u32 as i32, 0xe6db99e5u32 as i32, 0x1fa27cf8, 0xc4ac5665u32 as i32,
        0xf4292244u32 as i32, 0x432aff97, 0xab9423a7u32 as i32, 0xfc93a039u32 as i32,
        0x655b59c3, 0x8f0ccc92u32 as i32, 0xffeff47du32 as i32, 0x85845dd1u32 as i32,
        0x6fa87e4f, 0xfe2ce6e0u32 as i32, 0xa3014314u32 as i32, 0x4e0811a1u32 as i32,
        0xf7537e82u32 as i32, 0xbd3af235u32 as i32, 0x2ad7d2bb, 0xeb86d391u32 as i32
    ];

    let mut a: i32 = hash_values[0];
    let mut b: i32 = hash_values[1];
    let mut c: i32 = hash_values[2];
    let mut d: i32 = hash_values[3];

    // STEP 2 -> OK
    //
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
    //
    // STEP 3 : Doing calculus while iterating over the array (message splitted)

    for i in 0..chunk_num
    {
        let w: Vec<i32> = b32[(i*16) as usize..((i+1)*16) as usize].to_vec();
        for j in 0..64
        {
            let f: i32;
            let g: i32;
            if j < 16
            {
                f = (b & c)|((!b) & d);
                g = j;
            }
            else if j < 32
            {
                f = (d & b)|((!d) & c);
                g = (5*j + 1) % 16;
            }
            else if j < 48
            {
                f = b ^ c ^ d;
                g = (3*j + 5) % 16;
            }
            else
            {
                f = c ^ (b|(!d));
                g = (7*j) % 16;
            }
            let temp1 = d;
            d = c;
            c = b;
            let temp2 = ((a as i64) + (f as i64) + 
                         (constantes[j as usize] as i64) + (w[g as usize] as i64)) as i32;
            b = ((left_rotate(temp2, r[j as usize]) as i64) + (b as i64)) as i32;
            a = temp1;
        }

        // STEP 2 -> OK
        //
        // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        //
        // STEP 3 : Update hash values

        hash_values[0] = ((hash_values[0] as i64) + (a as i64)) as i32;
        hash_values[1] = ((hash_values[1] as i64) + (b as i64)) as i32;
        hash_values[2] = ((hash_values[2] as i64) + (c as i64)) as i32;
        hash_values[3] = ((hash_values[3] as i64) + (d as i64)) as i32;

        a = hash_values[0];
        b = hash_values[1]; 
        c = hash_values[2];
        d = hash_values[3];

        // STEP 3 -> OK
        //
        // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
        //
        // STEP 4 : Formatting output
    }

    let s1 = format!("{a:x}");
    let s2 = format!("{b:x}");
    let s3 = format!("{c:x}");
    let s4 = format!("{d:x}");

    let mut res = String::new();
    res.push_str(&s1);
    res.push_str(&s2);
    res.push_str(&s3);
    res.push_str(&s4);

    return res;

    // STEP 4 -> OK
    //
    // *-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*-*
}
