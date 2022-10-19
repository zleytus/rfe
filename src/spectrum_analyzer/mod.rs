mod command;
mod config;
mod dsp_mode;
mod parsers;
mod setup_info;
mod spectrum_analyzer;
mod sweep;
mod tracking_status;

pub(crate) use command::Command;
pub use config::{CalcMode, Config, Mode, RadioModule};
pub use dsp_mode::DspMode;
pub use setup_info::SetupInfo;
pub use spectrum_analyzer::SpectrumAnalyzer;
pub use sweep::Sweep;
pub use tracking_status::TrackingStatus;


#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum InputStage {
    Bypass = b'0',
    Attenuator30dB = b'1',
    Lna25dB = b'2',
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive)]
#[repr(u8)]
pub enum WifiBand {
    TwoPointFourGhz = 1,
    FiveGhz,
}

}

}

    }
}

}

    }
}
