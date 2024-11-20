// @generated
// @@protoc_insertion_point(attribute:aelf)
pub mod aelf {
    include!("aelf.rs");
    // @@protoc_insertion_point(aelf)
}
pub mod sf {
    // @@protoc_insertion_point(attribute:sf.substreams)
    pub mod substreams {
        include!("sf.substreams.rs");
        // @@protoc_insertion_point(sf.substreams)
        pub mod aelf {
            pub mod token {
                // @@protoc_insertion_point(attribute:sf.substreams.aelf.token.v1)
                pub mod v1 {
                    include!("sf.substreams.aelf.token.v1.rs");
                    // @@protoc_insertion_point(sf.substreams.aelf.token.v1)
                }
            }
        }
        pub mod index {
            // @@protoc_insertion_point(attribute:sf.substreams.index.v1)
            pub mod v1 {
                include!("sf.substreams.index.v1.rs");
                // @@protoc_insertion_point(sf.substreams.index.v1)
            }
        }
        // @@protoc_insertion_point(attribute:sf.substreams.v1)
        pub mod v1 {
            include!("sf.substreams.v1.rs");
            // @@protoc_insertion_point(sf.substreams.v1)
        }
    }
}
// @@protoc_insertion_point(attribute:token)
pub mod token {
    include!("token.rs");
    // @@protoc_insertion_point(token)
}
