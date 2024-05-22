use lox::common::OpCode;
use lox::chunk::Chunk;

fn main() {
    let mut chunk = Chunk::new();
    chunk.write(OpCode::OpReturn);

    chunk.disassemble("test chunk");
}
