[
    Meta(
        NameValue(
            MetaNameValue {
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident {
                                ident: "wrapper_name",
                                span: #0 bytes(210..222),
                            },
                            arguments: None,
                        },
                    ],
                },
                eq_token: Eq,
                lit: Str(
                    LitStr {
                        token: "BufWrapper",
                    },
                ),
            },
        ),
    ),
    Meta(
        List(
            MetaList {
                path: Path {
                    leading_colon: None,
                    segments: [
                        PathSegment {
                            ident: Ident {
                                ident: "wrapper_generics",
                                span: #0 bytes(239..255),
                            },
                            arguments: None,
                        },
                    ],
                },
                paren_token: Paren,
                nested: [
                    Lit(
                        Str(
                            LitStr {
                                token: "BufferTypeContainer",
                            },
                        ),
                    ),
                    Comma,
                    Lit(
                        Str(
                            LitStr {
                                token: "T",
                            },
                        ),
                    ),
                ],
            },
        ),
    ),
]