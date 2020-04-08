use std::prelude::v1::*;

fn base() {
    info!("hello");
    info!("hello",);
}

fn base_expr_context() {
    let _ = info!("hello");
}

fn with_args() {
    info!("hello {}", "cats");
    info!("hello {}", "cats",);
    info!("hello {}", "cats",);
}

fn with_args_expr_context() {
    match "cats" {
        cats => info!("hello {}", cats),
    };
}

fn kv() {
    info!("hello {}", "cats", {
        cat_1: "chashu",
        cat_2: "nori",
    });
}

fn kv_expr_context() {
    match "chashu" {
        cat_1 => info!("hello {}", "cats", {
            cat_1: cat_1,
            cat_2: "nori",
        }),
    };
}

pub fn run_tests() {
    use sgx_tunittest::*;
    rsgx_unit_tests!(
        base,
        base_expr_context,
        with_args,
        with_args_expr_context,
        kv,
        kv_expr_context,
    );
}
