#[link(name = "case_0_monolithic", vers = "0.1", author = "nejucomo@gmail.com")];
#[crate_type = "lib"];


pub mod a {
    pub mod b {
        pub static x: uint = 42u;
    }
}
