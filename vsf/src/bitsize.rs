use bitvec::{array::BitArray, order::Msb0};


pub trait BitQuantity{
    #[allow(dead_code)]
    fn get_bit_quantity(&self) -> usize;
}

macro_rules! impl_bitQuantity {
    ($struct:ident, $size:expr) => {

        #[derive(Debug, Clone, Copy)]
        pub struct $struct;

        impl BitQuantity for $struct {
            fn get_bit_quantity(&self) -> usize {
                let value: usize = $size;
                return value
            }
        }
    }
}

macro_rules! impl_bitQuantity_primitives {
    (for $($t:ty),+) => {
        $(impl BitQuantity for $t {
            fn get_bit_quantity(&self) -> usize {
                std::mem::size_of::<$t>() * 8 
            }
        })*
    }
}

impl_bitQuantity_primitives!(for u8, u16, u32, u64, u128);
impl_bitQuantity!(BitQ1, 1);
impl_bitQuantity!(BitQ2, 2);
impl_bitQuantity!(BitQ3, 3);
impl_bitQuantity!(BitQ4, 4);
impl_bitQuantity!(BitQ5, 5);
impl_bitQuantity!(BitQ6, 6);
impl_bitQuantity!(BitQ7, 7);
impl_bitQuantity!(BitQ8, 8);


#[allow(dead_code)]
pub struct BitSize<Q>(BitArray<u8, Msb0>, Q) where Q: BitQuantity;


impl<BitQuantity> BitSize<BitQuantity> where BitQuantity: self::BitQuantity {
    #[allow(dead_code)]
    pub fn new(value: u8, bit_quantity: BitQuantity ) -> Self {
        //let value: &BitSlice<u8, Msb0> = BitSlice::<u8, Msb0>::from_element(&value);
        let mut value = BitArray::<u8, Msb0>::new(value);
        let quantity: usize = bit_quantity.get_bit_quantity();
        for i in (1..=8 as usize).rev(){
            if i > quantity {
                value.set(8 - i, false);
                
            }
            else{ break; }        }
        Self(value, bit_quantity)
    }
    #[allow(dead_code)]
    pub fn to_byte(&self) -> u8{
        let bitarr = &self.0;
        bitarr.data
    }
}
