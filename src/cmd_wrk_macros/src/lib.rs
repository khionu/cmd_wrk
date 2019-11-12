use proc_macro::{TokenStream};
use syn::{parse_macro_input, Ident, Item, FnArg, Type, ItemFn, Visibility, PatType, PatIdent};
use syn::token::Token;
use syn::ItemMod;
use syn::export::ToTokens;
use syn::parse::{Parse, ParseBuffer, ParseStream};
use std::convert::{TryFrom, TryInto};
use std::borrow::Borrow;
use std::collections::HashMap;
use quote::format_ident;

static INJECT_IDENT: Ident = format_ident!("inject");
static NAME_IDENT: Ident = format_ident!("name");

pub fn cli(args: TokenStream, input: TokenStream) -> TokenStream {
    let tree: ItemMod = parse_macro_input!(input as ItemMod);

    let mut ident_stack = Vec::new();

    let cmds = parse_group(tree, &mut ident_stack);
}

fn parse_group(group: ItemMod, path: &mut Vec<Ident>) -> HashMap<String, Cmd> {
    let mut out = HashMap::new();

    path.push(group.ident.clone().into());

    let (_, group) = group.content.expect("empty command group");

    for i in group {
        match i {
            Item::Mod(g) => out.extend(parse_group(g, path)),
            Item::Fn(cmd) => {
                let path = path.iter().fold(String::new(), |mut s, i| {
                    s.push_str(i.as_ref()); s
                });

                out.insert(path, cmd.into());
            },
            _ => continue
        }
    }

    path.pop();

    out
}

struct Cmd {
    name: Ident,
    args: Vec<Arg>,
}

struct Arg {
    ty: Box<Type>,
    name: Ident,
    inject: bool,
    src: PatType,
}

impl ToTokens for Cmd {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        unimplemented!()
    }
}

impl ToTokens for Arg {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        unimplemented!()
    }
}

impl From<ItemFn> for Cmd {
    fn from(value: ItemFn) -> Self {
        match value.vis {
            Visibility::Restricted(_) | Visibility::Inherited =>
                panic!("Commands must be pub or pub(crate)"),
            _ => (),
        }

        let name = value.sig.ident;

        let mut args = Vec::new();

        for (i, a) in value.sig.inputs.iter().enumerate() {
            let arg = match a {
                FnArg::Receiver(r) => panic!("Commands mustn't be members"),
                FnArg::Typed(p) => {
                    let mut name = None;
                    let mut inject = false;

                    for attr in p.attrs {
                        if attr.path.is_ident(&NAME_IDENT) {
                            if let Some(_) = name { panic!("Args must only have one name") }

                            name = Some(format_ident!("{}", attr.tokens));

                            break
                        } else if attr.path.is_ident(&INJECT_IDENT) {
                            inject = true;

                            break
                        }
                    }

                    if let None = name {
                        name = if let syn::Pat::Ident(id) = *(p.pat) {
                            Some(id)
                        } else {
                            Some(format_ident!("_{}_{}", i, p.ty))
                        };
                    }

                    Arg {
                        ty: p.ty.clone(),
                        name: name.unwrap(),
                        inject,
                        src: p.clone(),
                    }
                }
            };

            args.push(arg);
        }

        Cmd {
            name,
            args,
        }
    }
}
