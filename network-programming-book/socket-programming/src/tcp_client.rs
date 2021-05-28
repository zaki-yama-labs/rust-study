use std::io::{self, BufRead, BufReader, Write};
use std::net::TcpStream;
use std::str;

/**
 * 指定の IP アドレス、ポート番号に TCP 接続する
 */
pub fn connect(address: &str) -> Result<(), failure::Error> {
    let mut stream = TcpStream::connect(address)?;
    loop {
        // 入力データをソケットから送信
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        stream.write_all(input.as_bytes())?;

        // ソケットから受信したデータを表示
        let mut reader = BufReader::new(&stream);
        let mut buffer = Vec::new();
        reader.read_until(b'\n', &mut buffer)?;
        print!("{}", str::from_utf8(&buffer)?);
    }
}
