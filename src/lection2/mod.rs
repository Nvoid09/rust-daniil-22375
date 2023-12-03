pub mod task1;
pub mod task2;
pub mod task3;
//pub mod rectangle;

#[derive(Debug)]
struct Depth
{
   byte : u8,
   ushort : u16,
   uint : u32,
}

pub fn lec2_check(){
    let d1 = Depth {
        byte: 8,
        ushort: 16,
        uint: 32
    };
 
    let d2 = Depth {
        byte: 4,
        ..d1
    };
    let el: task1::Element<u32, i32> = task1::Element{re: 1, im: 2};
    println!("{:?}", el);
    let byte = d2.byte;
    let ushort = d2.ushort;
    let uint = d2.uint;
    println!("{},{},{}", byte, ushort, uint);
    let Depth {byte: byte_new, ushort: ushort_new, uint: _} = d1;
    println!("print struct {}, {}", byte_new, ushort_new);
    let Depth {uint: uint_new, .. } = d1;
    println!("print struct {}", uint_new);
    println!("{:?}", d1);
}