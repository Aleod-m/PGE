use pge_macros::wrap;
use pge_macros::expose_pub_fn;
trait GlObj {
    fn id(&self) -> u8;
}

trait BufferType {
    const TYPE : u8;
}
struct BufferTypeContainer;
impl BufferType for BufferTypeContainer {
    const TYPE : u8 = 1;
}
#[wrap(
    wrapper_name = "BufferTypeOne<T>",
    wrapped_generics = "<BufferTypeContainer, T>",
)]
struct Buffer<B, T> 
where B : BufferType {
    id : u8,
    data : Vec<T>,
    _marker : std::marker::PhantomData<B>,
}


// pub struct BufWrapper<T: Display>
// where T : Display {
//     inner : Buffer<BufferTypeContainer, T>,
// }

#[expose_pub_fn(
    wrapper_name = "BufferTypeOne<T>"
)]
impl<B,T> Buffer<B,T>
where B : BufferType
{
    /// Creates a new buffer 
    pub fn new(data : T) -> Self {
        let mut id : u8 = 0;
        Self {
            id,
            data,
            _marker : std::marker::PhantomData,
        }
    }
    
    /// Set the buffer data on the GPU 
    pub fn set_data(&mut self, data : Vec<T>) {
        self.data = data;
    } 


}

impl<B, T> GlObj for Buffer<B, T> where B : BufferType {

    fn id(&self) -> u8 {
        self.id
    }
} 

fn main() {
    println!("Running");
}