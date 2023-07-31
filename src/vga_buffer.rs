#[allow(dead_code)] // NOTE: 使われてないコードの警告を抑止
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u8)] // NOTE: 色は4bitで表現されているので u4 で十分だけど存在しないので u8
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LIghtBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // NOTE: C言語の構造体と同じようにFieldを並べるのを保証してくれる
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]    // NOTE: Fieldが単一もしくは0個の時に同じメモリレイアウトを保証してくれるやつ
struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize, // NOTE: どこまで書き込んだかの行を記録
    color_code: ColorCode,
    buffer: &'static mut Buffer,    // NOTE: プログラム起動中はずっと保持される
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col] = ScreenChar {  // NOTE: 行目一杯まで読む
                    ascii_character: byte,
                    color_code
                };
                self.column_position += 1;  // 読み進め
            }
        }
    }

    // 文字列全体を出力
    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7a | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe), // NOTE: 出力不可能なAsciiバイトの場合■を出力
            }
        }
    }

    pub fn print_something() {
        let mut writer = Writer {
            column_position: 0,
            color_code: ColorCode::new(Color::Yellow, Color::Black),
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer)},
        };

        writer.write_byte(b'H');
        writer.write_string("ello ");
        writer.write_string("Wörld!");
    }

    fn new_line(&mut self) { }
}