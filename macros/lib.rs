use darling::{FromDeriveInput, FromMeta};

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct Lorem {
    #[darling(rename = "sit")]
    ipsum: bool,
    dolor: Option<String>,
}

#[derive(FromDeriveInput)]
#[darling(attributes(my_crate), forward_attrs(OPSow, doc, cfg))]
pub struct MyTraitOpts {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    lorem: Lorem,
}

pub type OpsBitflag = u32;
pub const ADD:          OpsBitflag = 1 << 0;
pub const ADD_ASSIGN:   OpsBitflag = 1 << 1;
pub const SUB:          OpsBitflag = 1 << 2;
pub const SUB_ASSIGN:   OpsBitflag = 1 << 3;
pub const MUL:          OpsBitflag = 1 << 4;
pub const MUL_ASSIGN:   OpsBitflag = 1 << 5;
pub const DIV:          OpsBitflag = 1 << 6;
pub const DIV_ASSIGN:   OpsBitflag = 1 << 7;
pub const REM:          OpsBitflag = 1 << 8;
pub const REM_ASSIGN:   OpsBitflag = 1 << 9;
pub const NEG:          OpsBitflag = 1 << 10;



pub const ADD_OPS:      OpsBitflag = ADD | ADD_ASSIGN;
pub const SUB_OPS:      OpsBitflag = SUB | SUB_ASSIGN;
pub const MUL_OPS:      OpsBitflag = MUL | MUL_ASSIGN;
pub const DIV_OPS:      OpsBitflag = DIV | DIV_ASSIGN;
pub const REM_OPS:      OpsBitflag = REM | REM_ASSIGN;

pub const PLUS_MINUS:   OpsBitflag = ADD_OPS | SUB_OPS;

pub const ARITH_WO_DIV: OpsBitflag = PLUS_MINUS | MUL_OPS;
pub const ARITH_OPS:    OpsBitflag = PLUS_MINUS | MUL_OPS | DIV_OPS | REM_OPS;
#[cfg(feature = "bit_ops")]
pub mod bit_ops {
    pub const NOT:              OpsBitflag = 1 << 11;
    pub const BIT_AND:          OpsBitflag = 1 << 12;
    pub const BIT_AND_ASSIGN:   OpsBitflag = 1 << 13;
    pub const BIT_OR:           OpsBitflag = 1 << 14;
    pub const BIT_OR_ASSIGN:    OpsBitflag = 1 << 15;
    pub const BIT_XOR:          OpsBitflag = 1 << 16;
    pub const BIT_XOR_ASSIGN:   OpsBitflag = 1 << 17;
    pub const SHL:              OpsBitflag = 1 << 18;
    pub const SHL_ASSIGN:       OpsBitflag = 1 << 19;
    pub const SHR:              OpsBitflag = 1 << 20;
    pub const SHR_ASSIGN:       OpsBitflag = 1 << 21;



    pub const BIT_AND_OPS:      OpsBitflag = BIT_AND | BIT_AND_ASSIGN;
    pub const BIT_OR_OPS:       OpsBitflag = BIT_OR | BIT_OR_ASSIGN;
    pub const BIT_XOR_OPS:      OpsBitflag = BIT_XOR | BIT_XOR_ASSIGN;
    pub const BIT_BASIC_OPS:    OpsBitflag = BIT_NOT | BIT_AND_OPS | BIT_OR_OPS | BIT_XOR_OPS;

    pub const SHL_OPS:          OpsBitflag = SHL | SHL_ASSIGN;
    pub const SHR_OPS:          OpsBitflag = SHR | SHR_ASSIGN;
    pub const SHIFT_OPS:        OpsBitflag = SHL_OPS | SHR_OPS;

    pub const BIT_OPS:          OpsBitflag = BIT_BASIC_OPS | SHIFT_OPS;

    pub const ARITH_BIT_OPS:    OpsBitflag = ARITH_OPS | BIT_OPS;
}
#[cfg(feature = "iter_ops")]
pub mod iter_ops {
    pub const SUM:      OpsBitflag = 1 << 22;
    pub const PRODUCT:  OpsBitflag = 1 << 23;

    pub const ITER_OPS: OpsBitflag = SUM | PRODUCT;

    pub const ARITH_ITER_OPS:   OpsBitflag = ARITH_OPS | ITER_OPS;
}
#[cfg(all(feature = "bit_ops", feature = "iter_ops"))]
pub const ARITH_BIT_ITER_OPS: ARITH_OPS | BIT_OPS | ITER_OPS;





#[derive(Default, FromMeta)]
#[darling(default)]
pub struct ConvertReciever {
    target: Target,
    from: TokenStream,
    into: TokenStream,
}

#[derive(Default, FromMeta)]

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct OpsSelf {
    ops: OpsBitflag,
    #[darling(map = )]
    suffix:
    suffix_assign:
}

#[derive(Default, FromMeta)]
#[darling(default)]
pub struct OpsWith {
    ops: OpsBitflag,
    targets: &[Ident],
    into:
    from:
}

#[derive(FromDeriveInput)]
#[darling(
    attributes(auto_ops),
    supports(struct_any),
    // forward_attrs(doc, cfg)
)]
pub struct AutoOps {
    ident: syn::Ident,
    attrs: Vec<syn::Attribute>,
    convert: ConvertImpl,
    partial_eq: PartialEqImpl,
    partial_ord: PartialOrdImpl,
    ops_self: OpsSelf,
    ops_with: OpsWith,
}

#[derive(AutoOps, PartialEq, Eq, PartialOrd, Ord)]
#[auto_ops(
    convert(
        target=
        into=
        from=
    ),
    // partial_eq
    // partial_ord
    ops_self(
        ops=ALL_OPS,
        suffix=deg_suffix
        
    ),
    ops_with(
        ops=ALL_OPS,
        targets=ALL,
        suffix=
    ),
)]
pub struct Deg(isize);

fn deg_suffix(fields: &[Ident]) -> proc_macro2::TokenStream {
    quote! {
        let res = self.#fields % 360;
        if self.#field >= 0 { res } else { res + 360 }
    }
}

fn deg_suffix_assign(fields: &[Ident]) -> proc_macro2::TokenStream {
    quote! {
        #(
            self.#fields %= 360;
            if self.#fields < 0 { self.#fields += 360 }
        )
    }
}