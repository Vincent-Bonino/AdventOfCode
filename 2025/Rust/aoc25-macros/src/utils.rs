use syn::{Ident, Path, Type, TypePath};

/// Extract the type identifier from a [`syn::Type`].
pub fn extract_type_ident(ty: &Type) -> Result<&Ident, ()> {
    if let Type::Path(TypePath {
        path: Path { segments, .. },
        ..
    }) = ty
    {
        segments.first().ok_or(()).map(|path_segment| &path_segment.ident)
    } else {
        Err(())
    }
}
