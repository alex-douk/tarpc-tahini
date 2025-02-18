// use proc_macro::Ident;
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, spanned::Spanned, Data, DataStruct, DeriveInput, Field, Visibility, ItemStruct};
use attribute_derive::FromAttr;

pub type Error = (Span, &'static str);


#[derive(FromAttr)]
#[attribute(ident = alohomora_out_type)]
struct AlohomoraTypeArgs {
  name: Option<Ident>,
  to_derive: Option<Vec<Ident>>,
  verbatim: Option<Vec<Ident>>,
}
impl AlohomoraTypeArgs {
    pub fn is_verbatim(&self, ident: &str) -> bool {
        match &self.verbatim {
            None => false,
            Some(v) => {
                for i in v {
                    if &i.to_string() == ident {
                        return true;
                    }
                }
                false
            },
        }
    }
}

pub fn parse_derive_input_struct(input: DeriveInput) -> Result<ItemStruct, Error> {
    match input.data {
        Data::Enum(_) => Err((input.ident.span(), "derive(AlohomoraType) only works on structs")),
        Data::Union(_) => Err((input.ident.span(), "derive(AlohomoraType) only works on structs")),
        Data::Struct(data_struct) => Ok(
            ItemStruct {
                attrs: input.attrs,
                vis: input.vis,
                struct_token: data_struct.struct_token,
                ident: input.ident,
                generics: input.generics,
                fields: data_struct.fields,
                semi_token: data_struct.semi_token,
            }
        ),
    }
}

fn serialize_struct(sct: DataStruct) -> Result<TokenStream, Error>{
    match sct.fields {
        syn::Fields::Named(named) => {
            let fields = named.named;
            let field_idents : Vec<_> = fields.iter().map(|x| x.ident.as_ref().unwrap().to_string()).collect();
            let pub_idents = fields.iter()
                .map(|field| (field.clone().vis, field.clone().ident.unwrap())).collect::<Vec<(Visibility, Ident)>>();




            Err((sct.struct_token.span(), "derive(TahiniType) only works on named structs"))
        },
        syn::Fields::Unnamed(_) => Err((sct.struct_token.span(), "derive(TahiniType) only works on named struct")),
        syn::Fields::Unit => Err((sct.struct_token.span(), "derive(TahiniType) only works on named struct")),
    }
}

pub fn derive_tahini_type_impl(input: DeriveInput) -> Result<TokenStream, Error> {
    // Parse the provided input attributes.
    let attrs = AlohomoraTypeArgs::from_attributes(&input.attrs).unwrap();

    // Parse the input struct.
    let input = parse_derive_input_struct(input)?;

    // The generics of the input type.
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    // Expand needed variables.
    let input_ident = &input.ident;

    // Find all fields.
    let fields: Vec<_> = input.fields.iter()
        .map(|field| (
            field.ident.as_ref().unwrap().clone(),
            field.ident.as_ref().unwrap().to_string(),
            field.ty.clone(),
        ))
        .collect();

    // Filter into those that are AlohomoraTypes themselves, and those who are kept verbatim.
    let tahini_fields: Vec<_> = fields
        .iter()
        .filter(|(_, string, _)| !attrs.is_verbatim(string))
        .cloned()
        .collect();
    let verbatium_fields: Vec<_> = fields
        .iter()
        .filter(|(_, string, _)| attrs.is_verbatim(string))
        .cloned()
        .collect();

    // Split field components.
    let tahini_fields_idents: Vec<_> = tahini_fields
        .iter()
        .map(|(ident, _, _)| ident.clone())
        .collect();

    let mut tahini_fields_strings: Vec<_> = Vec::new();
    for triplet in tahini_fields.iter(){
        tahini_fields_strings.push(triplet.1.as_str());
    }
    let tahini_fields_types: Vec<_> = tahini_fields
        .iter()
        .map(|(_, _, ty)| ty.clone())
        .collect();

    let verbatim_fields_idents: Vec<_> = verbatium_fields
        .iter()
        .map(|(ident, _, _)| ident.clone())
        .collect();

    let mut verbatim_fields_strings: Vec<_> = Vec::new();
    for triplet in verbatium_fields.iter(){
        verbatim_fields_strings.push(triplet.1.as_str());
    }

    let binding = input_ident.to_string();
    let ident_str = binding.as_str();



    // Generate implementation.
    Ok(quote! {
        #[automatically_derived]
        #[automatically_derived]
        #[doc = "Library implementation of TahiniType. Do not copy this docstring!"]
        impl #impl_generics ::alohomora::tarpc::TahiniType for #input_ident #ty_generics #where_clause {
            fn to_tahini_enum(&self) -> ::alohomora::tarpc::TahiniEnum {
                let mut map: ::std::collections::HashMap<&'static str, ::alohomora::tarpc::TahiniEnum> = ::std::collections::HashMap::new();
                ::alohomora::tarpc::TahiniEnum::Struct(#ident_str, ::std::collections::HashMap::from([
                    #((#tahini_fields_strings, <#tahini_fields_types as TahiniType>::to_tahini_enum(&self.#tahini_fields_idents)),)*
                    // #((#verbatim_fields_strings, <#tahini_fields_types as TahiniType>::to_tahini_enum(&self.#verbatim_fields_strings)),)*
                    #((#verbatim_fields_strings, ::alohomora::tarpc::TahiniEnum::Value(Box::new(self.#verbatim_fields_idents))),)*
                ]))
            }
        }
    })
}
