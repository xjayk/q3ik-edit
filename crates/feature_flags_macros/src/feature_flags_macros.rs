use proc_macro::TokenStream;

#[proc_macro_derive(EnumFeatureFlag)]
pub fn derive_enum_feature_flag(input: TokenStream) -> TokenStream {
    input
}
