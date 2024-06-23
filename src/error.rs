#[derive(thiserror::Error, Debug)]
pub enum CalError {
    //#[error(transparent)]
    //GlobSyntax(#[from] glob::PatternError),

    #[error("Can't make Month with {0}")]
    MonthRange(u32),
}

