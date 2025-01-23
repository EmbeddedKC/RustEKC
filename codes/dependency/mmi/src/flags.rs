use crate::address::*;

bitflags! {
    pub struct MapPermission: u16 {
        const R = 1 << 1;  // read
        const W = 1 << 2;  // write
        const X = 1 << 3;  // execute
        const RWX = 0b111 << 1;
        const U = 1 << 4;  // user
        const G = 1 << 5;  // global
        const O = 1 << 9;  // copy on write
    }
    
}


impl MapPermission{
    pub fn get_bits(&self) -> u16{
        self.bits
    }
}


#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MapType {
    Identical,
    Framed,
    Raw,
    Specified(PhysPageNum)
}

impl From<MapType> for usize {
    fn from(v: MapType) -> Self {
        match v {
            MapType::Identical =>{
                return usize::MAX-1;
            }
            MapType::Framed =>{
                return usize::MAX-2;
            }
            MapType::Raw =>{
                return usize::MAX-3;
            }
            MapType::Specified(ppn) =>{
                return ppn.0;
            }
        }
    }
}
impl From<usize> for MapType{
    fn from(v: usize) -> Self {
        unsafe{
             if v == usize::MAX-1 {
            MapType::Identical
        }else if v == usize::MAX-2 {
            MapType::Framed
        }else if v == usize::MAX-3 {
            MapType::Raw
        }else{
            MapType::Specified(PhysPageNum::from(v))
        }
        }
    }
}

impl From<MapPermission> for usize{
    fn from(v: MapPermission) -> Self{
        v.bits().into()
    }
}

impl From<usize> for MapPermission{
    fn from(v: usize) -> Self{
        MapPermission { bits: v as u16}
    }
}

