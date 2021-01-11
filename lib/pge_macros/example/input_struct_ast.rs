DeriveInput {
    attrs: [],
    vis: Inherited,
    ident: Ident {
        ident: "Buffer",
        span: #0 bytes(292..298),
    },
    generics: Generics {
        lt_token: Some(
            Lt,
        ),
        params: [
            Type(
                TypeParam {
                    attrs: [],
                    ident: Ident {
                        ident: "B",
                        span: #0 bytes(299..300),
                    },
                    colon_token: None,
                    bounds: [],
                    eq_token: None,
                    default: None,
                },
            ),
            Comma,
            Type(
                TypeParam {
                    attrs: [],
                    ident: Ident {
                        ident: "T",
                        span: #0 bytes(302..303),
                    },
                    colon_token: None,
                    bounds: [],
                    eq_token: None,
                    default: None,
                },
            ),
        ],
        gt_token: Some(
            Gt,
        ),
        where_clause: Some(
            WhereClause {
                where_token: Where,
                predicates: [
                    Type(
                        PredicateType {
                            lifetimes: None,
                            bounded_ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "B",
                                                    span: #0 bytes(317..318),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                            colon_token: Colon,
                            bounds: [
                                Trait(
                                    TraitBound {
                                        paren_token: None,
                                        modifier: None,
                                        lifetimes: None,
                                        path: Path {
                                            leading_colon: None,
                                            segments: [
                                                PathSegment {
                                                    ident: Ident {
                                                        ident: "BufferType",
                                                        span: #0 bytes(321..331),
                                                    },
                                                    arguments: None,
                                                },
                                            ],
                                        },
                                    },
                                ),
                            ],
                        },
                    ),
                ],
            },
        ),
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Named(
                FieldsNamed {
                    brace_token: Brace,
                    named: [
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "id",
                                    span: #0 bytes(339..341),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "u8",
                                                    span: #0 bytes(344..346),
                                                },
                                                arguments: None,
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "data",
                                    span: #0 bytes(352..356),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "Vec",
                                                    span: #0 bytes(359..362),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "T",
                                                                                        span: #0 bytes(363..364),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                        Field {
                            attrs: [],
                            vis: Inherited,
                            ident: Some(
                                Ident {
                                    ident: "_marker",
                                    span: #0 bytes(371..378),
                                },
                            ),
                            colon_token: Some(
                                Colon,
                            ),
                            ty: Path(
                                TypePath {
                                    qself: None,
                                    path: Path {
                                        leading_colon: None,
                                        segments: [
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "std",
                                                    span: #0 bytes(381..384),
                                                },
                                                arguments: None,
                                            },
                                            Colon2,
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "marker",
                                                    span: #0 bytes(386..392),
                                                },
                                                arguments: None,
                                            },
                                            Colon2,
                                            PathSegment {
                                                ident: Ident {
                                                    ident: "PhantomData",
                                                    span: #0 bytes(394..405),
                                                },
                                                arguments: AngleBracketed(
                                                    AngleBracketedGenericArguments {
                                                        colon2_token: None,
                                                        lt_token: Lt,
                                                        args: [
                                                            Type(
                                                                Path(
                                                                    TypePath {
                                                                        qself: None,
                                                                        path: Path {
                                                                            leading_colon: None,
                                                                            segments: [
                                                                                PathSegment {
                                                                                    ident: Ident {
                                                                                        ident: "B",
                                                                                        span: #0 bytes(406..407),
                                                                                    },
                                                                                    arguments: None,
                                                                                },
                                                                            ],
                                                                        },
                                                                    },
                                                                ),
                                                            ),
                                                        ],
                                                        gt_token: Gt,
                                                    },
                                                ),
                                            },
                                        ],
                                    },
                                },
                            ),
                        },
                        Comma,
                    ],
                },
            ),
            semi_token: None,
        },
    ),
}

