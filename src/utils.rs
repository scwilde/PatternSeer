use std::cmp::PartialOrd;

/// Traits for internal `utils` use only
mod sealed {
    /// Sealed trait that encapsulates the f32 and f64 types
    pub trait Float {  }
    impl Float for f32 {  }
    impl Float for f64 {  }
}

/// Min function to be used on f32 and f64 where a.min(b) is more ambiguous.
/// 
/// # Returns
/// 
/// Either an f32 or an f64 depending on which input parameter is selected.
/// If either `a` or `b` is `NaN` then `NaN` of that parameter's type will be returned.
pub fn minf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a < b { a } 
    else { b }
}
/// Max function to be used on f32 and f64 where a.max(b) is more ambiguous.
/// 
/// # Returns
/// 
/// Either an f32 or an f64 depending on which input parameter is selected.
/// If either `a` or `b` is `NaN` then `NaN` of that parameter's type will be returned.
pub fn maxf<F: sealed::Float + PartialOrd>(a: F, b: F) -> F  {
    if a != a { a } 
    else if b != b { b }
    else if a > b { a } 
    else { b }
}
