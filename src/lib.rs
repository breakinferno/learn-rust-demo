pub mod top_mod {
    pub mod top_mod_inner {
        pub fn say() {

        }
    }
}

// pub use 使用
pub use crate::top_mod::top_mod_inner;

pub mod top_mod_server {
    pub mod top_mod_server_inner {
        pub fn test() {
            crate::top_mod::top_mod_inner::say();
            super::private_relative();
            super::private_say();
        }
    }

    fn private_say() {
        use crate::top_mod::top_mod_inner;

        top_mod_inner::say();
    }

    fn private_relative() {
        super::top_mod::top_mod_inner::say();
    }
}