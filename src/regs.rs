#[macro_export]
macro_rules! read_reg {
    ( $($reg: expr)? ) => {
        {
            let mut temp: u64;
            core::arch::asm!(concat!("lea {}, [", $($reg)?, "]"), out(reg) temp);
            temp
        }
    };
}

#[macro_export]
macro_rules! write_reg {
    ($($reg: expr, $val: expr)?) => {
        core::arch::asm!(concat!("mov ", $($reg)?, ", ", $($val)?));
    };
}