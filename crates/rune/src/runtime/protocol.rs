use crate::compile::ItemBuf;
use crate::{Hash, InstFnInfo, InstFnKind, InstFnName, IntoTypeHash};
use std::cmp;
use std::fmt;
use std::hash;

/// A built in instance function.
#[derive(Debug, Clone, Copy)]
pub struct Protocol {
    /// The name of the builtin function.
    pub name: &'static str,
    /// The hash of the builtin function.
    pub hash: Hash,
}

impl InstFnName for Protocol {
    #[inline]
    fn name_hash(self) -> Hash {
        self.hash
    }

    #[inline]
    fn info(self) -> InstFnInfo {
        InstFnInfo {
            hash: self.hash,
            kind: InstFnKind::Protocol(self),
            parameters: Hash::EMPTY,
        }
    }
}

impl IntoTypeHash for Protocol {
    fn into_type_hash(self) -> Hash {
        self.hash
    }

    fn into_item(self) -> Option<ItemBuf> {
        None
    }

    fn hash<H>(self, hasher: &mut H)
    where
        H: hash::Hasher,
    {
        self.hash.hash(hasher);
    }
}

impl std::ops::Deref for Protocol {
    type Target = Hash;

    fn deref(&self) -> &Self::Target {
        &self.hash
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl cmp::PartialEq for Protocol {
    fn eq(&self, other: &Self) -> bool {
        self.hash.eq(&other.hash)
    }
}

impl cmp::Eq for Protocol {}

impl hash::Hash for Protocol {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state)
    }
}

impl Protocol {
    /// Check two types for equality.
    pub const EQ: Protocol = Protocol {
        name: "eq",
        hash: Hash::new(0x418f5becbf885806),
    };

    /// The function to access a field by name when Protocol::GET fails.
    pub const FALLBACK_GET: Protocol = Protocol {
        name: "fallback_get",
        hash: Hash::new(0x6dda58b140dfeaf9),
    };

    /// The function to set a field by name when Protocol::SET fails.
    pub const FALLBACK_SET: Protocol = Protocol {
        name: "fallback_set",
        hash: Hash::new(0xbe28c02896ca0b64),
    };

    /// The function to access a field.
    pub const GET: Protocol = Protocol {
        name: "get",
        hash: Hash::new(0x504007af1a8485a4),
    };

    /// The function to set a field.
    pub const SET: Protocol = Protocol {
        name: "set",
        hash: Hash::new(0x7d13d47fd8efef5a),
    };

    /// The function to access an index.
    pub const INDEX_GET: Protocol = Protocol {
        name: "index_get",
        hash: Hash::new(0xadb5b27e2a4d2dec),
    };

    /// The function to set an index.
    pub const INDEX_SET: Protocol = Protocol {
        name: "index_set",
        hash: Hash::new(0x162943f7bd03ad36),
    };

    /// The function to implement for the addition operation.
    pub const ADD: Protocol = Protocol {
        name: "+",
        hash: Hash::new(0xe4ecf51fa0bf1076),
    };

    /// The function to implement for the addition assign operation.
    pub const ADD_ASSIGN: Protocol = Protocol {
        name: "+=",
        hash: Hash::new(0x42451ccb0a2071a9),
    };

    /// The function to implement for the subtraction operation.
    pub const SUB: Protocol = Protocol {
        name: "-",
        hash: Hash::new(0x6fa86a5f18d0bf71),
    };

    /// The function to implement for the subtraction assign operation.
    pub const SUB_ASSIGN: Protocol = Protocol {
        name: "-=",
        hash: Hash::new(0x5939bb56a1415284),
    };

    /// The function to implement for the multiply operation.
    pub const MUL: Protocol = Protocol {
        name: "*",
        hash: Hash::new(0xb09e99dc94091d1c),
    };

    /// The function to implement for the multiply assign operation.
    pub const MUL_ASSIGN: Protocol = Protocol {
        name: "*=",
        hash: Hash::new(0x29a54b727f980ebf),
    };

    /// The function to implement for the division operation.
    pub const DIV: Protocol = Protocol {
        name: "/",
        hash: Hash::new(0xf26d6eea1afca6e8),
    };

    /// The function to implement for the division assign operation.
    pub const DIV_ASSIGN: Protocol = Protocol {
        name: "/=",
        hash: Hash::new(0x4dd087a8281c04e6),
    };

    /// The function to implement for the remainder operation.
    pub const REM: Protocol = Protocol {
        name: "%",
        hash: Hash::new(0x5c6293639c74e671),
    };

    /// The function to implement for the remainder assign operation.
    pub const REM_ASSIGN: Protocol = Protocol {
        name: "%=",
        hash: Hash::new(0x3a8695980e77baf4),
    };

    /// The function to implement for the bitwise and operation.
    pub const BIT_AND: Protocol = Protocol {
        name: "&",
        hash: Hash::new(0x0e11f20d940eebe8),
    };

    /// The function to implement for the bitwise and assign operation.
    pub const BIT_AND_ASSIGN: Protocol = Protocol {
        name: "&=",
        hash: Hash::new(0x95cb1ba235dfb5ec),
    };

    /// The function to implement for the bitwise xor operation.
    pub const BIT_XOR: Protocol = Protocol {
        name: "^",
        hash: Hash::new(0xa3099c54e1de4cbf),
    };

    /// The function to implement for the bitwise xor assign operation.
    pub const BIT_XOR_ASSIGN: Protocol = Protocol {
        name: "^=",
        hash: Hash::new(0x01fa9706738f9867),
    };

    /// The function to implement for the bitwise or operation.
    pub const BIT_OR: Protocol = Protocol {
        name: "|",
        hash: Hash::new(0x05010afceb4a03d0),
    };

    /// The function to implement for the bitwise xor assign operation.
    pub const BIT_OR_ASSIGN: Protocol = Protocol {
        name: "|=",
        hash: Hash::new(0x606d79ff1750a7ec),
    };

    /// The function to implement for the bitwise shift left operation.
    pub const SHL: Protocol = Protocol {
        name: "<<",
        hash: Hash::new(0x6845f7d0cc9e002d),
    };

    /// The function to implement for the bitwise shift left assign operation.
    pub const SHL_ASSIGN: Protocol = Protocol {
        name: "<<=",
        hash: Hash::new(0xdc4702d0307ba27b),
    };

    /// The function to implement for the bitwise shift right operation.
    pub const SHR: Protocol = Protocol {
        name: ">>",
        hash: Hash::new(0x6b485e8e6e58fbc8),
    };

    /// The function to implement for the bitwise shift right assign operation.
    pub const SHR_ASSIGN: Protocol = Protocol {
        name: ">>=",
        hash: Hash::new(0x61ff7c46ff00e74a),
    };

    /// Protocol function used by template strings.
    pub const STRING_DISPLAY: Protocol = Protocol {
        name: "string_display",
        hash: Hash::new(0x811b62957ea9d9f9),
    };

    /// Protocol function used by custom debug impls.
    pub const STRING_DEBUG: Protocol = Protocol {
        name: "string_debug",
        hash: Hash::new(0x4064e3867aaa0717),
    };

    /// Function used to convert an argument into an iterator.
    pub const INTO_ITER: Protocol = Protocol {
        name: "into_iter",
        hash: Hash::new(0x15a85c8d774b4065),
    };

    /// The function to call to continue iteration.
    pub const NEXT: Protocol = Protocol {
        name: "next",
        hash: Hash::new(0xc3cde069de2ba320),
    };

    /// Function used to convert an argument into a future.
    ///
    /// Signature: `fn(Value) -> Future`.
    pub const INTO_FUTURE: Protocol = Protocol {
        name: "into_future",
        hash: Hash::new(0x596e6428deabfda2),
    };

    /// Coerce a value into a type name. This is stored as a constant.
    pub const INTO_TYPE_NAME: Protocol = Protocol {
        name: "into_type_name",
        hash: Hash::new(0xbffd08b816c24682),
    };

    /// Function used to test if a value is a specific variant.
    ///
    /// Signature: `fn(self, usize) -> bool`.
    pub const IS_VARIANT: Protocol = Protocol {
        name: "is_variant",
        hash: Hash::new(0xc030d82bbd4dabe8),
    };
}
