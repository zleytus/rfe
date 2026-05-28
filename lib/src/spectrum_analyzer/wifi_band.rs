use num_enum::IntoPrimitive;

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
/// Wi-Fi band used by Wi-Fi analyzer mode.
pub enum WifiBand {
    /// 2.4 GHz Wi-Fi band.
    TwoPointFourGhz = 1,
    /// 5 GHz Wi-Fi band.
    FiveGhz,
}
