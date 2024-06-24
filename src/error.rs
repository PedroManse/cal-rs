// could let each module define own error and #transparent here

#[derive(thiserror::Error, Debug)]
pub enum CalError {
    //#[error(transparent)]
    //GlobSyntax(#[from] glob::PatternError),

    #[error("Can't make Month with {0}")]
    MonthRange(u32),
    #[error("Can't get first day of month")]
    FirstDayError,
}

