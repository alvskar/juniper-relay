#[macro_use]
extern crate juniper;

#[derive(GraphQLObject)]
pub struct PageInfo {
    pub has_previous_page: bool,
    pub has_next_page: bool
}

#[macro_export]
macro_rules! relay_connection {
    ($name:ident, $edgename:ident, $type:ty) => {
        relay_connection!($name, $edgename, $type, ());
    };
    ($name:ident, $edgename:ident, $type:ty, $context:ty) => {
        pub struct $edgename {
            node: $type,
            cursor: String
        }

        graphql_object!($edgename: $context |&self| {
            field node() -> &$type {
                &self.node
            }

            field cursor() -> &String {
                &self.cursor
            }
        });

        impl $edgename {
            #[allow(dead_code)]
            pub fn new(node: $type, cursor: String) -> $edgename {
                $edgename {
                    node: node,
                    cursor: cursor
                }
            }
        }

        pub struct $name {
            page_info: $crate::PageInfo,
            edges: Vec<$edgename>
        }

        graphql_object!($name: $context |&self| {
            field page_info() -> &$crate::PageInfo {
                &self.page_info
            }

            field edges() -> &Vec<$edgename> {
                &self.edges
            }
        });

        impl $name {
            #[allow(dead_code)]
            pub fn new(page_info: $crate::PageInfo, edges: Vec<$edgename>) -> $name {
                $name {
                    page_info: page_info,
                    edges: edges
                }
            }
        }
    }
}
