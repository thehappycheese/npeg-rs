use std::sync::atomic::{AtomicUsize, Ordering};

/// `OpaqueIdentifier` is used as a key in the BTreeMap for 'memoizing' the parser.
/// It uses fancy atomic types to generate a unique usize when the `new()` function is called.
/// 
/// Thanks to cdhowie at https://stackoverflow.com/questions/72148631/how-can-i-hash-by-a-raw-pointer for this snippet
/// 
/// ```ignore
/// use std::collections::BTreeMap;
/// use std::hash::{Hash, Hasher};
/// use npeg_rs_trait::core::opaque_identifier::OpaqueIdentifier;
/// struct UnHashableData {/* You can't hash me! */};
/// struct KeyStruct {
///     identifier:OpaqueIdentifier,
///     some_unhashable_data:UnHashableData
/// }
/// impl Hash for KeyStruct{
///     fn hash<H:Hasher>(&self, state:&mut H){
///         self.identifier.hash(state);
///     }
/// }
/// ```
/// 
/// 
/// > Note: `Clone` and `Copy` are not implemented to avoid accidentally making a non-unique instance.
#[derive(Debug, Hash, PartialEq, Eq)]
///
/// `#[repr(transparent)]` is used to make this type use the same  memory layout as a bare `usize` type.
/// I think this is perhaps a minor optimization and not needed? It was in the code I copy pasted.
#[repr(transparent)]
pub struct OpaqueIdentifier(usize);

impl OpaqueIdentifier {
    pub fn new() -> Self {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        
        Self(COUNTER.fetch_add(1, Ordering::Relaxed))
    }
    
    pub fn id(&self) -> usize {
        self.0
    }
}

impl Default for OpaqueIdentifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests{
    use super::OpaqueIdentifier;
    #[test]
    fn test_manual_hash_impl(){
        //use std::collections::BTreeMap;

        use std::hash::{Hash, Hasher};

        struct UnHashableData {/* can't hash me! */}
        struct KeyStruct {
            identifier:OpaqueIdentifier,
            _some_unhashable_data:UnHashableData
        }
        impl Hash for KeyStruct{
            fn hash<H:Hasher>(&self, state:&mut H){
                self.identifier.hash(state);
            }
        }
    }
}