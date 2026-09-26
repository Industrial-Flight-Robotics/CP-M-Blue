pub struct i8080
{
    pub a: u8,

    pub b: u8,
    pub c: u8,

    pub d: u8,
    pub e: u8,

    pub h: u8,
    pub l: u8,

    pub pc: u16,
    pub sp: u16,
}


imp i8080{
    pub fn new() -> Self{
        return i8080
        {
            a: 0,

            b: 0,
            c: 0,

            d: 0,
            e: 0,

            h: 0,
            l: 0,

            pc: 0,
            sp: 0,
        };
    }
}}