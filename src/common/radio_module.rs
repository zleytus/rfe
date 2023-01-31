use crate::spectrum_analyzer::Model;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RadioModule<M: Copy + Clone = Model> {
    Main { model: M },
    Expansion { model: M },
}

impl<M: Copy + Clone> RadioModule<M> {
    pub fn model(&self) -> M {
        match self {
            Self::Main { model, .. } => *model,
            Self::Expansion { model, .. } => *model,
        }
    }

    pub fn is_main(&self) -> bool {
        match self {
            Self::Main { .. } => true,
            Self::Expansion { .. } => false,
        }
    }

    pub fn is_expansion(&self) -> bool {
        !self.is_main()
    }
}
