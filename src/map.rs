const CONVERSION: [char; 16] = ['0','1','2','3',
                                '4','5','6','7',
                                '8','9','a','b',
                                'c','d','e','f'];

pub fn convert_ascii(a: u8) -> char {
    CONVERSION[a as usize]
}